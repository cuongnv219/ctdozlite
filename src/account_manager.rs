//! Multi-account management for ctdozlite.
//!
//! Stores isolated account sessions, thread caches, and history databases in `accounts/<uin>.*`.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccountProfile {
    pub uin: String,
    /// Human-readable label: profile name, else phone number, else a generic label.
    /// Never the raw account id.
    pub display_name: String,
    /// Phone number of the account, empty when unknown.
    pub phone: String,
    pub avatar: Option<String>,
    pub last_active_ts: i64,
    pub is_current: bool,
}

impl AccountProfile {
    /// Secondary line under the label: the phone number when it adds information,
    /// otherwise the account id.
    pub fn subtitle(&self) -> String {
        if !self.phone.is_empty() && self.phone != self.display_name {
            self.phone.clone()
        } else {
            format!("ID: {}", self.uin)
        }
    }
}

/// Label shown for an account that has neither a profile name nor a phone number.
const UNNAMED_ACCOUNT_LABEL: &str = "Tài khoản Zalo";

/// Pick the label of a saved account: profile name, else phone number, else a generic
/// label. A name that merely repeats the account id counts as missing.
fn account_label(uin: &str, display_name: &str, phone: &str) -> String {
    let name = display_name.trim();
    if !name.is_empty() && name != uin {
        return name.to_string();
    }
    let phone = phone.trim();
    if !phone.is_empty() {
        return phone.to_string();
    }
    UNNAMED_ACCOUNT_LABEL.to_string()
}

pub fn accounts_dir() -> PathBuf {
    PathBuf::from("accounts")
}

/// Return the accounts directory inside `data_dir`.
fn accounts_dir_in(data_dir: &Path) -> PathBuf {
    data_dir.join("accounts")
}

/// Return the isolated session snapshot file path (`<data_dir>/accounts/<uin>.json`).
fn session_snapshot_path_in(data_dir: &Path, uin: &str) -> PathBuf {
    accounts_dir_in(data_dir).join(format!("{uin}.json"))
}

/// Return the isolated threads snapshot file path (`<data_dir>/accounts/<uin>_threads.json`).
fn threads_snapshot_path_in(data_dir: &Path, uin: &str) -> PathBuf {
    accounts_dir_in(data_dir).join(format!("{uin}_threads.json"))
}

/// Copy the live `session.json` under `data_dir` over the snapshot of `uin` without taking
/// [`CREDENTIAL_WIPE_LOCK`]; callers must already hold it.
fn refresh_session_snapshot_unlocked(data_dir: &Path, uin: &str) -> Result<bool> {
    if !is_safe_uin(uin) {
        return Ok(false);
    }
    let live = data_dir.join("session.json");
    let snapshot = session_snapshot_path_in(data_dir, uin);
    let modified = |p: &Path| std::fs::metadata(p).and_then(|m| m.modified()).ok();
    let Some(live_ts) = modified(&live) else {
        return Ok(false);
    };
    if modified(&snapshot).is_some_and(|snap_ts| snap_ts >= live_ts) {
        return Ok(false);
    }
    let text = std::fs::read_to_string(&live)?;
    // The file on disk may already belong to another account (login in progress).
    let owner = serde_json::from_str::<serde_json::Value>(&text)
        .ok()
        .and_then(|v| v.pointer("/login/uin").and_then(|u| u.as_str()).map(str::to_string));
    if owner.as_deref() != Some(uin) {
        return Ok(false);
    }
    std::fs::create_dir_all(accounts_dir_in(data_dir))?;
    crate::data_dir::write_private_atomic(&snapshot, text.as_bytes())?;
    Ok(true)
}

/// Copy the live `session.json` over the snapshot of `uin` when it is newer.
///
/// The profile name, avatar and phone number are filled in after login, so a snapshot
/// taken earlier would keep labelling the account by its id. Returns `true` when the
/// snapshot was (re)written.
///
/// # Errors
/// Returns an error when the live session cannot be read or the snapshot cannot be written.
pub fn refresh_session_snapshot(uin: &str) -> Result<bool> {
    let _wipe_guard = lock_credential_wipe();
    refresh_session_snapshot_unlocked(Path::new("."), uin)
}

/// Legacy shared image cache, still used while no account is signed in.
const SHARED_MEDIA_CACHE_DIR: &str = "stickers_cache";

// The media cache is written by worker tasks and probed by the UI when it paints
// avatars, so the active account id is a tiny shared-read value rather than a
/// message; the critical section only clones or replaces one short string.
static ACTIVE_UIN: std::sync::RwLock<String> = std::sync::RwLock::new(String::new());
static ACCOUNT_EPOCH: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
static CREDENTIAL_WIPE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
static CREDENTIAL_WIPE_EPOCH: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);

/// Acquire the cross-thread lock serializing credential snapshot writes against wipes.
fn lock_credential_wipe() -> std::sync::MutexGuard<'static, ()> {
    CREDENTIAL_WIPE_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Monotonic generation counter incremented every time saved credentials are wiped.
pub fn credential_wipe_epoch() -> u64 {
    CREDENTIAL_WIPE_EPOCH.load(std::sync::atomic::Ordering::SeqCst)
}

/// `true` when `uin` is safe to embed in a file name (ASCII letters, digits, `-`, `_`).
///
/// Account ids come from `session.json`, which is untrusted input for path building.
pub fn is_safe_uin(uin: &str) -> bool {
    !uin.is_empty()
        && uin.len() <= 64
        && uin
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

/// Record which account owns the UI and media written from now on (empty = signed out).
/// Bumps [`account_epoch`] whenever the active account changes so in-flight async tasks
/// from the previous account can be discarded cleanly.
pub fn set_active_uin(uin: &str) {
    let next = if is_safe_uin(uin) { uin } else { "" };
    if let Ok(mut guard) = ACTIVE_UIN.write() {
        if guard.as_str() != next {
            *guard = next.to_string();
            ACCOUNT_EPOCH.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
    }
}

/// Monotonic generation counter incremented every time [`set_active_uin`] switches account.
pub fn account_epoch() -> u64 {
    ACCOUNT_EPOCH.load(std::sync::atomic::Ordering::SeqCst)
}

/// The account that currently owns the UI, or an empty string when signed out.
pub fn active_uin() -> String {
    ACTIVE_UIN.read().map(|g| g.clone()).unwrap_or_default()
}

/// Per-account Sync V2 Curve25519 identity key path (`accounts/<uin>/sync_identity.json`).
pub fn sync_identity_path_for(uin: &str) -> PathBuf {
    if is_safe_uin(uin) {
        let dir = account_dir(uin);
        let _ = std::fs::create_dir_all(&dir);
        dir.join("sync_identity.json")
    } else {
        PathBuf::from("sync_identity.json")
    }
}

/// Per-account Sync V1 RSA-2048 identity key path (`accounts/<uin>/sync_rsa_identity.json`).
pub fn sync_rsa_path_for(uin: &str) -> PathBuf {
    if is_safe_uin(uin) {
        let dir = account_dir(uin);
        let _ = std::fs::create_dir_all(&dir);
        dir.join("sync_rsa_identity.json")
    } else {
        PathBuf::from("sync_rsa_identity.json")
    }
}

/// Saved session snapshot of `uin` (`accounts/<uin>.json`).
pub fn session_snapshot_path(uin: &str) -> PathBuf {
    accounts_dir().join(format!("{uin}.json"))
}

/// Saved thread-list snapshot of `uin` (`accounts/<uin>_threads.json`).
pub fn threads_snapshot_path(uin: &str) -> PathBuf {
    accounts_dir().join(format!("{uin}_threads.json"))
}

/// Private storage root of one account (`accounts/<uin>/`).
pub fn account_dir(uin: &str) -> PathBuf {
    accounts_dir().join(uin)
}

/// Image cache directory of `uin`, or the shared cache under `<storage_root>/cache`.
///
/// The directory is not created; use [`ensure_media_cache_dir`] before writing.
pub fn media_cache_dir_for(uin: &str) -> PathBuf {
    let _ = SHARED_MEDIA_CACHE_DIR;
    crate::storage_manager::cache_dir_for(uin)
}

/// Image cache directory of the active account (avatars, thumbnails, stickers).
pub fn media_cache_dir() -> PathBuf {
    media_cache_dir_for(&active_uin())
}

/// [`media_cache_dir`], created on demand so callers can write into it directly.
pub fn ensure_media_cache_dir() -> PathBuf {
    let dir = media_cache_dir();
    // Best effort: a missing cache directory only costs a re-download later.
    let _ = std::fs::create_dir_all(&dir);
    dir
}

/// The OS user's own Downloads folder, where photos and videos saved from a chat land.
///
/// Linux honours `XDG_DOWNLOAD_DIR` from `user-dirs.dirs`, so a localised folder name
/// (e.g. `~/Tải về`) is respected; other platforms use `<home>/Downloads`. Falls back to the
/// home directory, then the working directory, when no Downloads folder can be resolved.
pub fn os_downloads_dir() -> PathBuf {
    let home = std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from);
    #[cfg(target_os = "linux")]
    if let Some(home) = &home {
        let config = std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| home.join(".config"));
        if let Ok(contents) = std::fs::read_to_string(config.join("user-dirs.dirs")) {
            if let Some(dir) = parse_xdg_download_dir(&contents, home) {
                return dir;
            }
        }
    }
    match home {
        Some(home) => {
            let downloads = home.join("Downloads");
            if downloads.is_dir() {
                downloads
            } else {
                home
            }
        }
        None => PathBuf::from("."),
    }
}

/// Extract `XDG_DOWNLOAD_DIR` from a `user-dirs.dirs` file. The spec only allows `$HOME/...`
/// or absolute paths; a value equal to `$HOME` itself means "disabled" and is ignored.
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
fn parse_xdg_download_dir(contents: &str, home: &Path) -> Option<PathBuf> {
    let value = contents
        .lines()
        .map(str::trim)
        .filter(|l| !l.starts_with('#'))
        .find_map(|l| l.strip_prefix("XDG_DOWNLOAD_DIR="))?
        .trim()
        .trim_matches('"');
    let path = if let Some(rest) = value.strip_prefix("$HOME") {
        let rest = rest.trim_start_matches('/');
        if rest.is_empty() {
            return None;
        }
        home.join(rest)
    } else if value.starts_with('/') {
        PathBuf::from(value)
    } else {
        return None;
    };
    Some(path)
}

/// Folder that receives files downloaded by `uin`, nested under the user's download root
/// so attachments of different accounts never mix.
pub fn account_download_dir(base: &Path, uin: &str) -> PathBuf {
    if is_safe_uin(uin) {
        base.join(uin)
    } else {
        base.to_path_buf()
    }
}

/// True when `path` is an isolated per-account session snapshot (`accounts/<uin>.json`),
/// excluding thread cache snapshots (`*_threads.json`) and subdirectories.
fn is_account_credential_snapshot_file(path: &Path) -> bool {
    path.is_file()
        && path.extension().and_then(|s| s.to_str()) == Some("json")
        && path
            .file_stem()
            .and_then(|s| s.to_str())
            .is_some_and(|stem| !stem.is_empty() && !stem.ends_with("_threads"))
}

/// Remove `path` if present, returning `Ok(true)` if removed and `Ok(false)` when absent.
fn remove_file_if_exists(path: &Path) -> Result<bool> {
    match std::fs::remove_file(path) {
        Ok(()) => Ok(true),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e.into()),
    }
}

/// True when `data_dir` contains a root `session.json` or any `accounts/<uin>.json` snapshot.
pub fn has_any_saved_credentials_in(data_dir: &Path) -> bool {
    if data_dir.join("session.json").is_file() {
        return true;
    }
    let Ok(entries) = std::fs::read_dir(data_dir.join("accounts")) else {
        return false;
    };
    entries
        .flatten()
        .any(|entry| is_account_credential_snapshot_file(&entry.path()))
}

/// True when the active data directory contains any saved session or account snapshot.
pub fn has_any_saved_credentials() -> bool {
    has_any_saved_credentials_in(Path::new("."))
}

/// Delete `session.json`, `app_lock.json`, and all `accounts/<uin>.json` credential snapshots
/// under `data_dir`, preserving `accounts/*_threads.json`, `accounts/*_history.db*`, and
/// `accounts/<uin>/` subdirectories.
pub fn wipe_all_saved_session_credentials_in(data_dir: &Path) -> Result<usize> {
    let _wipe_guard = lock_credential_wipe();
    CREDENTIAL_WIPE_EPOCH.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let mut removed = 0usize;
    for target in [
        data_dir.join("session.json"),
        data_dir.join(crate::app_lock::APP_LOCK_FILE),
    ] {
        if remove_file_if_exists(&target)? {
            removed += 1;
        }
    }
    let entries = match std::fs::read_dir(data_dir.join("accounts")) {
        Ok(entries) => entries,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(removed),
        Err(e) => return Err(e.into()),
    };
    for entry_res in entries {
        let entry = entry_res?;
        let path = entry.path();
        if is_account_credential_snapshot_file(&path) && remove_file_if_exists(&path)? {
            removed += 1;
        }
    }
    Ok(removed)
}

/// Wipe all saved session credentials and `app_lock.json` in the active data directory.
pub fn wipe_all_saved_session_credentials() -> Result<usize> {
    wipe_all_saved_session_credentials_in(Path::new("."))
}

/// List all saved accounts in the `accounts/` directory.
pub fn list_saved_accounts(current_uin: &str) -> Vec<AccountProfile> {
    let dir = accounts_dir();
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return out;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if is_account_credential_snapshot_file(&path) {
            if let Ok(text) = std::fs::read_to_string(&path) {
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some(login) = val.get("login") {
                        let uin = login
                            .get("uin")
                            .and_then(|v| v.as_str())
                            .unwrap_or_default()
                            .to_string();
                        if !uin.is_empty() {
                            let phone = login
                                .get("phone")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .trim()
                                .to_string();
                            let display_name = account_label(
                                &uin,
                                login
                                    .get("display_name")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or_default(),
                                &phone,
                            );
                            let avatar = login
                                .get("avatar")
                                .and_then(|v| v.as_str())
                                .map(str::to_string);
                            let is_current = uin == current_uin;
                            let last_active_ts = entry
                                .metadata()
                                .and_then(|m| m.modified())
                                .ok()
                                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                                .map(|d| d.as_secs() as i64)
                                .unwrap_or(0);
                            out.push(AccountProfile {
                                uin,
                                display_name,
                                phone,
                                avatar,
                                last_active_ts,
                                is_current,
                            });
                        }
                    }
                }
            }
        }
    }
    out.sort_by(|a, b| b.last_active_ts.cmp(&a.last_active_ts));
    out
}

/// Remove legacy root cache/DB files under `data_dir` (`threads_cache.json`, `zalo_history.db`
/// + WAL/SHM) so multi-account switches or QR logins never inherit another account's data.
pub fn remove_legacy_shared_caches_in(data_dir: &Path) {
    for legacy in [
        "threads_cache.json",
        "zalo_history.db",
        "zalo_history.db-wal",
        "zalo_history.db-shm",
    ] {
        let _ = std::fs::remove_file(data_dir.join(legacy));
    }
}

/// Remove legacy root cache/DB files (`threads_cache.json`, `zalo_history.db` + WAL/SHM)
/// so multi-account switches or QR logins never inherit another account's data.
pub fn remove_legacy_shared_caches() {
    remove_legacy_shared_caches_in(Path::new("."));
}

fn any_account_history_db_exists(dir: &Path) -> bool {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return false;
    };
    entries.flatten().any(|e| {
        e.path()
            .file_name()
            .and_then(|s| s.to_str())
            .is_some_and(|n| n.ends_with("_history.db"))
    })
}

/// Save the live `session.json` under `data_dir` into `accounts/<uin>.json` without acquiring
/// [`CREDENTIAL_WIPE_LOCK`]; callers must already hold it.
fn save_current_account_snapshot_unlocked(data_dir: &Path, uin: &str) -> Result<()> {
    if !is_safe_uin(uin) {
        return Ok(());
    }
    let dir = accounts_dir_in(data_dir);
    std::fs::create_dir_all(&dir)?;

    let live = data_dir.join("session.json");
    if let Ok(text) = std::fs::read_to_string(&live) {
        let owner = serde_json::from_str::<serde_json::Value>(&text)
            .ok()
            .and_then(|v| {
                v.pointer("/login/uin")
                    .and_then(|u| u.as_str())
                    .map(str::to_string)
            });
        if owner.as_deref() == Some(uin) {
            crate::data_dir::write_private_atomic(
                &session_snapshot_path_in(data_dir, uin),
                text.as_bytes(),
            )?;
        }
    }

    // One-time migration when upgrading from legacy single-account (`zalo_history.db` in root)
    // before any `accounts/*_history.db` exists at all.
    let acc_db = dir.join(format!("{uin}_history.db"));
    if !acc_db.exists() && !any_account_history_db_exists(&dir) {
        let legacy_db = data_dir.join("zalo_history.db");
        if legacy_db.exists() {
            let _ = std::fs::copy(&legacy_db, &acc_db);
        }
        let acc_threads = threads_snapshot_path_in(data_dir, uin);
        let legacy_threads = data_dir.join("threads_cache.json");
        if !acc_threads.exists() && legacy_threads.exists() {
            let _ = std::fs::copy(&legacy_threads, &acc_threads);
        }
        remove_legacy_shared_caches_in(data_dir);
    }
    Ok(())
}

/// Save the current session into `accounts/<uin>.json` (only if `session.json` belongs to `uin`).
///
/// Runtime history and thread caches already write directly to `accounts/<uin>_history.db` and
/// `accounts/<uin>_threads.json` — never copy root `zalo_history.db` or `threads_cache.json`
/// over per-account files (except one-time migration from legacy single-account root storage
/// when zero `accounts/*_history.db` existed before).
pub fn save_current_account_snapshot(uin: &str) -> Result<()> {
    let _wipe_guard = lock_credential_wipe();
    save_current_account_snapshot_unlocked(Path::new("."), uin)
}

/// Guarded background snapshot sync parameterized by `data_dir`, epoch source, and active UIN
/// lookup so tests stay isolated from global `ACTIVE_UIN` and the working directory.
fn sync_background_account_snapshot_in_with<F>(
    data_dir: &Path,
    uin: &str,
    epoch_at_read: u64,
    epoch_counter: &std::sync::atomic::AtomicU64,
    active_uin_fn: F,
) -> Result<bool>
where
    F: FnOnce() -> String,
{
    if !is_safe_uin(uin) {
        return Ok(false);
    }
    let _wipe_guard = lock_credential_wipe();
    if epoch_counter.load(std::sync::atomic::Ordering::SeqCst) != epoch_at_read {
        return Ok(false);
    }
    if active_uin_fn().is_empty() {
        return Ok(false);
    }
    if !data_dir.join("session.json").exists() {
        return Ok(false);
    }
    let snapshot = session_snapshot_path_in(data_dir, uin);
    if !snapshot.exists() {
        save_current_account_snapshot_unlocked(data_dir, uin)?;
        return Ok(snapshot.exists());
    }
    refresh_session_snapshot_unlocked(data_dir, uin)
}

/// Create or refresh `accounts/<uin>.json` for `background_accounts::supervise`, aborting
/// immediately if a credential wipe bumped [`credential_wipe_epoch`] since `epoch_at_read`,
/// if [`active_uin`] was cleared, or if `session.json` no longer exists.
pub fn sync_background_account_snapshot(uin: &str, epoch_at_read: u64) -> Result<bool> {
    sync_background_account_snapshot_in_with(
        Path::new("."),
        uin,
        epoch_at_read,
        &CREDENTIAL_WIPE_EPOCH,
        active_uin,
    )
}

/// Switch to a target account:
/// 1. Saves current account's `session.json` snapshot (if `session.json` matches `current_uin`)
/// 2. Removes stale root `threads_cache.json` / `zalo_history.db` so accounts stay strictly isolated
/// 3. Restores `accounts/<target_uin>.json` into `session.json`
pub fn switch_to_account(current_uin: &str, target_uin: &str) -> Result<()> {
    if !is_safe_uin(target_uin) {
        anyhow::bail!("Target UIN không hợp lệ");
    }
    if is_safe_uin(current_uin) {
        let _ = save_current_account_snapshot(current_uin);
    }
    remove_legacy_shared_caches();

    let target_session = session_snapshot_path(target_uin);
    if !target_session.exists() {
        anyhow::bail!("Không tìm thấy dữ liệu phiên của tài khoản {target_uin}");
    }
    let text = std::fs::read_to_string(&target_session)?;
    crate::data_dir::write_private_atomic(Path::new("session.json"), text.as_bytes())?;
    set_active_uin(target_uin);
    Ok(())
}

/// Delete a saved account and all of its isolated history/cache/sync files from `accounts/`
pub fn delete_saved_account(uin: &str) -> Result<()> {
    if !is_safe_uin(uin) {
        return Ok(());
    }
    let dir = accounts_dir();
    let _ = std::fs::remove_file(dir.join(format!("{uin}.json")));
    let _ = std::fs::remove_file(dir.join(format!("{uin}_threads.json")));
    let _ = std::fs::remove_file(dir.join(format!("{uin}_history.db")));
    let _ = std::fs::remove_file(dir.join(format!("{uin}_history.db-wal")));
    let _ = std::fs::remove_file(dir.join(format!("{uin}_history.db-shm")));
    let _ = std::fs::remove_dir_all(account_dir(uin));
    Ok(())
}
