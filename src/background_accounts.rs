//! Keep signed-in but inactive accounts listening so they still raise notifications.
//!
//! Every saved account other than the active one gets a passive `zpw_ws` socket
//! (`socket::spawn_passive`). Incoming chat frames are written to that account's own
//! history database and reported to the UI as a [`BackgroundActivity`].

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

use crate::account_manager::{self, AccountProfile};
use crate::chat::Thread;
use crate::session::Session;

/// How often the supervisor compares running listeners with the saved accounts.
const RECONCILE_SECS: u64 = 3;
/// Wait before a listener that could not start is retried by the supervisor.
const START_RETRY_SECS: u64 = 30;
/// Messages older than this before the listener started are backlog, not news.
const FRESH_SLACK_MS: i64 = 60_000;
/// Upper bound of remembered message ids per listener.
const NOTIFIED_CAP: usize = 4_096;
/// Longest wait for resolving display names of threads first seen in the background.
const NAME_RESOLVE_SECS: u64 = 10;
/// How long a history write waits for another writer of the same database.
const DB_BUSY_SECS: u64 = 5;

/// One conversation of an inactive account that received new messages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackgroundBanner {
    pub thread_id: String,
    pub title: String,
    pub body: String,
}

/// New incoming messages received by an account that is not shown in the UI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackgroundActivity {
    pub uin: String,
    pub account_name: String,
    /// One entry per conversation, newest message of that conversation.
    pub banners: Vec<BackgroundBanner>,
    /// Total number of new incoming messages across `banners`.
    pub incoming: usize,
}

/// Receives activity from listener tasks; must not block.
pub type ActivitySink = Arc<dyn Fn(BackgroundActivity) + Send + Sync>;

// Wakes the supervisor as soon as the worker changes the connected account, so the
/// account that became active does not keep a second socket until the next tick.
static RECONCILE: tokio::sync::Notify = tokio::sync::Notify::const_new();

/// Ask the supervisor to re-evaluate its listeners now instead of at the next tick.
pub fn request_reconcile() {
    RECONCILE.notify_one();
}

/// Accounts that need a listener: every saved account except the connected one.
///
/// `ui_uin` is the account the UI belongs to and `connected_uin` the account the
/// worker session is signed in as. They differ while the login screen is open to add
/// an account: the worker session is signed out, so the UI's account listens too.
/// Nothing listens after an explicit logout (`ui_uin` empty).
fn accounts_to_listen(
    ui_uin: &str,
    connected_uin: &str,
    saved: Vec<AccountProfile>,
) -> Vec<AccountProfile> {
    if ui_uin.is_empty() {
        return Vec::new();
    }
    saved
        .into_iter()
        .filter(|a| a.uin != connected_uin && account_manager::is_safe_uin(&a.uin))
        .collect()
}

struct Listener {
    /// Snapshot modification time the listener was started from.
    snapshot_ts: i64,
    task: tokio::task::JoinHandle<()>,
}

/// Run listeners for inactive accounts until the worker runtime shuts down.
///
/// Owned by the worker: it polls on a fixed cadence, starts a listener for each saved
/// account that is not active, and aborts listeners whose account became active, was
/// removed, or signed in again (newer snapshot).
pub async fn supervise(session: Arc<tokio::sync::Mutex<Session>>, sink: ActivitySink) {
    let mut listeners: HashMap<String, Listener> = HashMap::new();
    let mut tick = tokio::time::interval(Duration::from_secs(RECONCILE_SECS));
    tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    loop {
        tokio::select! {
            _ = tick.tick() => {}
            _ = RECONCILE.notified() => {}
        }
        let wipe_epoch = account_manager::credential_wipe_epoch();
        let connected = session
            .lock()
            .await
            .login
            .as_ref()
            .map(|l| l.uin.clone())
            .unwrap_or_default();
        let ui_uin = account_manager::active_uin();
        let wanted = tokio::task::spawn_blocking(move || {
            if account_manager::is_safe_uin(&connected)
                && account_manager::credential_wipe_epoch() == wipe_epoch
            {
                let _ = account_manager::sync_background_account_snapshot(&connected, wipe_epoch);
            }
            if account_manager::credential_wipe_epoch() != wipe_epoch {
                return Vec::new();
            }
            let saved = account_manager::list_saved_accounts(&connected);
            accounts_to_listen(&ui_uin, &connected, saved)
        })
        .await
        .unwrap_or_default();

        listeners.retain(|uin, listener| {
            let keep = !listener.task.is_finished()
                && wanted
                    .iter()
                    .any(|a| &a.uin == uin && a.last_active_ts == listener.snapshot_ts);
            if !keep {
                listener.task.abort();
            }
            keep
        });
        for account in wanted {
            if listeners.contains_key(&account.uin) {
                continue;
            }
            let uin = account.uin.clone();
            let snapshot_ts = account.last_active_ts;
            let task = tokio::spawn(listen(account, sink.clone()));
            listeners.insert(uin, Listener { snapshot_ts, task });
        }
    }
}

/// Thread shells (no messages) used to name and classify incoming frames.
fn thread_shells(threads: Vec<Thread>) -> Vec<Thread> {
    threads
        .into_iter()
        .map(|mut t| {
            t.messages.clear();
            t
        })
        .collect()
}

fn load_account(uin: &str) -> anyhow::Result<(Session, Vec<Thread>)> {
    let mut session = Session::new(false, false, false)?;
    let path = account_manager::session_snapshot_path(uin);
    session.load_from_file(&path.to_string_lossy())?;
    // A missing or unreadable thread snapshot only costs placeholder names.
    let threads = std::fs::read_to_string(account_manager::threads_snapshot_path(uin))
        .ok()
        .and_then(|text| serde_json::from_str::<Vec<Thread>>(&text).ok())
        .unwrap_or_default();
    Ok((session, thread_shells(threads)))
}

async fn listen(account: AccountProfile, sink: ActivitySink) {
    let uin = account.uin.clone();
    let loaded = {
        let uin = uin.clone();
        tokio::task::spawn_blocking(move || load_account(&uin)).await
    };
    let (session, mut known) = match loaded {
        Ok(Ok(pair)) => pair,
        _ => {
            // Finishing immediately would make the supervisor retry every tick.
            tokio::time::sleep(Duration::from_secs(START_RETRY_SECS)).await;
            return;
        }
    };
    let has_cookie = session.zpw_sek.as_deref().is_some_and(|s| !s.is_empty());
    if session.login.is_none() || !has_cookie {
        tokio::time::sleep(Duration::from_secs(START_RETRY_SECS)).await;
        return;
    }

    let started_ms = crate::clock::normalize_ts_ms(crate::login::unix_now());
    let mut notified: HashSet<String> = HashSet::new();
    let mut events = crate::socket::spawn_passive(session.clone());
    while let Some(event) = events.recv().await {
        if !event.is_chat_ish() {
            continue;
        }
        let mut touched = known.clone();
        crate::chat::merge_previews(&mut touched, &event.payload);
        touched.retain(|t| !t.messages.is_empty());
        if touched.is_empty() {
            continue;
        }

        let has_new_thread = touched
            .iter()
            .any(|t| !known.iter().any(|k| k.id == t.id));
        if has_new_thread {
            // Best effort: on timeout the placeholder name ("Người dùng <id>") is kept.
            let _ = tokio::time::timeout(
                Duration::from_secs(NAME_RESOLVE_SECS),
                crate::chat::resolve_thread_names(&session, &mut touched),
            )
            .await;
            for thread in &touched {
                if !known.iter().any(|k| k.id == thread.id) {
                    let mut shell = thread.clone();
                    shell.messages.clear();
                    known.push(shell);
                }
            }
        }

        let realtime = matches!(
            event.cmd,
            crate::socket::CMD_PUSH_MSG_1_1 | crate::socket::CMD_PUSH_MSG_GROUP
        );
        let activity = collect_activity(&account, &touched, realtime, started_ms, &mut notified);

        let db_path = crate::store::db_path_for_account(&uin);
        let snapshot = account_manager::session_snapshot_path(&uin);
        // History writes fail open: a lost cache row must not stop notifications.
        let _ = tokio::task::spawn_blocking(move || {
            // The account may have been removed since this frame arrived; opening the
            // store would recreate its deleted database.
            if !snapshot.exists() {
                return;
            }
            if let Ok(mut store) = crate::store::MessageStore::open(&db_path) {
                // The active account may be writing the same file right after a switch.
                let _ = store.conn.busy_timeout(Duration::from_secs(DB_BUSY_SECS));
                let _ = crate::store::persist_received_messages(&mut store, &touched);
            }
        })
        .await;

        if let Some(activity) = activity {
            sink(activity);
        }
    }
}

fn is_fresh(ts: i64, started_ms: i64) -> bool {
    ts <= 0 || crate::clock::normalize_ts_ms(ts) >= started_ms - FRESH_SLACK_MS
}

/// Build the notification payload for the threads touched by one socket frame.
///
/// Only real-time push frames notify: queue pulls right after connecting replay
/// messages that arrived while the account was offline. Outgoing, muted, stale and
/// already-reported messages are skipped.
fn collect_activity(
    account: &AccountProfile,
    touched: &[Thread],
    realtime: bool,
    started_ms: i64,
    notified: &mut HashSet<String>,
) -> Option<BackgroundActivity> {
    if !realtime {
        return None;
    }
    if notified.len() > NOTIFIED_CAP {
        notified.clear();
    }
    let mut banners = Vec::new();
    let mut incoming = 0usize;
    for thread in touched {
        if thread.presentation.muted || thread.unread.is_muted() {
            continue;
        }
        let mut latest = None;
        for message in &thread.messages {
            if message.is_outgoing || !is_fresh(message.ts, started_ms) {
                continue;
            }
            let key = if message.id.is_empty() {
                format!("{}_{}", thread.id, message.ts)
            } else {
                message.id.clone()
            };
            if notified.insert(key) {
                incoming += 1;
                latest = Some(message);
            }
        }
        let Some(latest) = latest else { continue };
        let title = if thread.is_group && !latest.sender.is_empty() {
            format!("{} · {}", latest.sender, thread.name)
        } else {
            thread.name.clone()
        };
        let body = crate::os_notify::compact_text(&latest.text, 96);
        banners.push(BackgroundBanner {
            thread_id: thread.id.clone(),
            title,
            body: if body.is_empty() {
                "[ Đính kèm / Nhãn dán ]".to_string()
            } else {
                body
            },
        });
    }
    if banners.is_empty() {
        return None;
    }
    Some(BackgroundActivity {
        uin: account.uin.clone(),
        account_name: account.display_name.clone(),
        banners,
        incoming,
    })
}
