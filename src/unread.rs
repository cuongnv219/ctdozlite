//! Unread state follows server read cursors, never the size of a history batch.
use crate::chat::{ChatMessage, Thread};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnreadState {
    /// None means that no authoritative read point has been received.
    pub last_seen_id: Option<String>,
    #[serde(default)]
    pub unread_ids: BTreeSet<String>,
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub mention_ids: BTreeSet<String>,
    #[serde(default)]
    pub muted: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub muted_until: Option<i64>,
    #[serde(default)]
    pub marked_unread: bool,
    /// Network metadata copies may update state; ordinary history copies may not.
    #[serde(skip)]
    pub fresh: bool,
    #[serde(skip)]
    pub settings_fresh: bool,
}

fn numeric_id(id: &str) -> Option<&str> {
    if id.is_empty() || !id.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let id = id.trim_start_matches('0');
    Some(if id.is_empty() { "0" } else { id })
}

pub fn compare_ids(a: &str, b: &str) -> Option<std::cmp::Ordering> {
    let (a, b) = (numeric_id(a)?, numeric_id(b)?);
    Some(a.len().cmp(&b.len()).then_with(|| a.cmp(b)))
}

impl UnreadState {
    pub fn has_unread_mentions(&self) -> bool {
        !self.mention_ids.is_empty()
    }
    pub fn count(&self) -> usize {
        self.unread_ids.len()
    }
    pub fn is_muted(&self) -> bool {
        self.is_muted_at(crate::login::unix_now() as i64)
    }

    /// Returns whether this thread is muted at `now_unix` seconds.
    pub fn is_muted_at(&self, now_unix: i64) -> bool {
        self.muted && self.muted_until.is_none_or(|until| until > now_unix)
    }
    pub fn has_badge(&self) -> bool {
        self.count() > 0 || self.marked_unread
    }
    pub fn badge(&self, is_group: bool) -> Option<String> {
        if !self.has_badge() {
            return None;
        }
        if self.is_muted() || self.count() == 0 {
            return Some(String::new());
        }
        Some(if is_group && self.count() > 5 {
            "5+".into()
        } else if self.count() > 99 {
            "99+".into()
        } else {
            self.count().to_string()
        })
    }
    pub fn merge_metadata(&mut self, incoming: &Self) -> bool {
        let before = self.clone();
        if incoming.fresh {
            if let Some(point) = &incoming.last_seen_id {
                self.advance_seen(point);
            }
            if let Some(point) = self.last_seen_id.as_deref() {
                for id in &incoming.unread_ids {
                    if compare_ids(id, point).is_some_and(|order| order.is_gt()) {
                        self.unread_ids.insert(id.clone());
                    }
                }
            }
            if incoming.settings_fresh {
                self.muted = incoming.muted;
                self.muted_until = incoming.muted_until;
                self.marked_unread = incoming.marked_unread;
            }
            self.fresh = true;
        } else if self.last_seen_id.is_none() && incoming.last_seen_id.is_some() {
            *self = incoming.clone();
        }
        if let Some(point) = self.last_seen_id.as_deref() {
            for id in &incoming.mention_ids {
                if self.unread_ids.contains(id)
                    && compare_ids(id, point).is_some_and(|order| order.is_gt())
                {
                    self.mention_ids.insert(id.clone());
                }
            }
        }
        *self != before
    }
    pub fn advance_seen(&mut self, point: &str) {
        if numeric_id(point).is_none() {
            return;
        }
        if self
            .last_seen_id
            .as_ref()
            .is_none_or(|old| compare_ids(point, old).is_some_and(|v| !v.is_lt()))
        {
            self.last_seen_id = Some(point.into());
            self.unread_ids
                .retain(|id| compare_ids(id, point).is_some_and(|v| v.is_gt()));
            self.mention_ids
                .retain(|id| compare_ids(id, point).is_some_and(|order| order.is_gt()));
        }
    }
    /// Called only for new live messages, or a read-only server-cursor snapshot.
    pub fn observe(&mut self, messages: &[ChatMessage]) {
        let Some(point) = &self.last_seen_id else {
            return;
        };
        for message in messages {
            if !message.is_outgoing
                && message.kind != crate::message_model::MessageKind::System
                && message.unread_policy.counts_unread()
                && compare_ids(&message.id, point).is_some_and(|v| v.is_gt())
            {
                self.unread_ids.insert(message.id.clone());
            }
        }
    }
    pub fn observe_for_account(&mut self, messages: &[ChatMessage], account: &str) {
        self.observe(messages);
        self.observe_mentions(messages, account);
    }
    /// Missing mention metadata never erases a previously verified off-screen mention.
    pub fn observe_mentions(&mut self, messages: &[ChatMessage], account: &str) {
        for message in messages {
            if message.is_outgoing
                || message.kind == crate::message_model::MessageKind::System
                || !message.unread_policy.counts_unread()
            {
                self.mention_ids.remove(&message.id);
                continue;
            }
            if !self.unread_ids.contains(&message.id) {
                continue;
            }
            match classify_message_mention(message, account) {
                MentionDetection::Mention => {
                    self.mention_ids.insert(message.id.clone());
                }
                MentionDetection::NoMention => {
                    self.mention_ids.remove(&message.id);
                }
                MentionDetection::Unknown => {}
            }
        }
    }
    /// Local foreground reading does not send a receipt or mutate the account.
    pub fn read_visible(&mut self, messages: &[ChatMessage]) {
        if let Some(last) = messages
            .iter()
            .filter(|m| numeric_id(&m.id).is_some())
            .max_by(|a, b| compare_ids(&a.id, &b.id).unwrap())
        {
            self.advance_seen(&last.id);
            self.marked_unread = false;
        }
        // Only the visible cursor is advanced; off-screen unread IDs remain.
    }
}

fn string(value: &Value, key: &str) -> Option<String> {
    value.get(key).and_then(|v| {
        v.as_str()
            .map(str::to_owned)
            .or_else(|| v.as_u64().map(|v| v.to_string()))
    })
}

/// Original desktop consumer ignores subchat cursors (`type=2`, `sct=1/2/3`).
pub fn merge_read_points(threads: &mut [Thread], data: &Value) {
    let entries = data
        .get("clearUnreads")
        .or_else(|| data.get("clearUnreadInfo"))
        .or_else(|| data.get("statusMap").and_then(|v| v.get("clearUnreads")))
        .and_then(Value::as_array);
    for entry in entries.into_iter().flatten() {
        if entry.get("type").and_then(Value::as_i64) == Some(2) && entry.get("sct").is_some() {
            continue;
        }
        let (Some(id), Some(point)) = (string(entry, "idTo"), string(entry, "lastMsgId")) else {
            continue;
        };
        if numeric_id(&point).is_none_or(|id| id == "0") || id.is_empty() || id == "0" {
            continue;
        }
        let group = entry
            .get("isGroup")
            .is_some_and(|v| v.as_bool() == Some(true) || v.as_i64() == Some(1));
        let tid = if group {
            format!("g{}", id.trim_start_matches('g'))
        } else {
            id
        };
        if let Some(thread) = threads.iter_mut().find(|t| {
            t.id == tid
                || (group
                    && t.is_group
                    && t.id.trim_start_matches('g') == tid.trim_start_matches('g'))
        }) {
            thread.unread.advance_seen(&point);
            thread.unread.fresh = true;
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MuteKeyKind {
    Group,
    User,
    Bare,
}

fn classify_mute_key(raw: &str) -> (MuteKeyKind, &str) {
    let s = raw.trim();
    if let Some(rest) = s
        .strip_prefix("group_")
        .or_else(|| s.strip_prefix("group/"))
    {
        let rest = rest.trim();
        let trimmed = rest.trim_start_matches(['g', 'G']);
        if !trimmed.is_empty() && trimmed.bytes().all(|b| b.is_ascii_digit()) {
            return (MuteKeyKind::Group, trimmed);
        }
        return (MuteKeyKind::Group, rest);
    }
    if s.starts_with(['g', 'G']) {
        let rest = s.trim_start_matches(['g', 'G']);
        if !rest.is_empty() && rest.bytes().all(|b| b.is_ascii_digit()) {
            return (MuteKeyKind::Group, rest);
        }
    }
    if s.starts_with(['u', 'U']) {
        let rest = s.trim_start_matches(['u', 'U']);
        if !rest.is_empty() && rest.bytes().all(|b| b.is_ascii_digit()) {
            return (MuteKeyKind::User, rest);
        }
    }
    (MuteKeyKind::Bare, s)
}

/// Canonical `muted_threads` map key (`g<id>` for groups, `<id>` for 1-1 peers).
pub fn canonical_mute_key(thread_id: &str, is_group: bool) -> String {
    let (_, clean) = classify_mute_key(thread_id);
    if clean.is_empty() || clean == "0" {
        return String::new();
    }
    if is_group {
        format!("g{clean}")
    } else {
        clean.to_string()
    }
}

/// Match a `muted_threads` key or control event ID against `(thread_id, is_group)`.
/// Group threads match `group_<id>`, `g<id>`, `G<id>`, and unadorned `<id>` (never `u<id>`);
/// 1-1 DM threads match `u<id>`, `U<id>`, and unadorned `<id>` (never `g`/`G`/`group_`).
pub fn mute_key_matches(key: &str, thread_id: &str, is_group: bool) -> bool {
    let (k_kind, k_clean) = classify_mute_key(key);
    let (t_kind, t_clean) = classify_mute_key(thread_id);
    if k_clean.is_empty()
        || k_clean == "0"
        || t_clean.is_empty()
        || t_clean == "0"
        || k_clean != t_clean
    {
        return false;
    }
    if is_group {
        k_kind != MuteKeyKind::User && t_kind != MuteKeyKind::User
    } else {
        k_kind != MuteKeyKind::Group && t_kind != MuteKeyKind::Group
    }
}

/// True when `until` represents an active mute at `now_unix` (`-1` indefinite or future timestamp).
pub fn is_mute_until_active(until: i64, now_unix: i64) -> bool {
    until == -1 || (until > 0 && until > now_unix)
}

fn stronger_mute_until(a: i64, b: i64) -> i64 {
    if a == -1 || b == -1 {
        -1
    } else {
        a.max(b)
    }
}

/// Look up an active mute entry for `(thread_id, is_group)` across all valid alias keys.
pub fn active_mute_until(
    muted_threads: &HashMap<String, i64>,
    thread_id: &str,
    is_group: bool,
    now_unix: i64,
) -> Option<i64> {
    if muted_threads.is_empty() {
        return None;
    }
    let mut best: Option<i64> = None;
    for (k, &until) in muted_threads {
        if is_mute_until_active(until, now_unix) && mute_key_matches(k, thread_id, is_group) {
            best = Some(match best {
                Some(prev) => stronger_mute_until(prev, until),
                None => until,
            });
        }
    }
    best
}

/// Unified mute check across `settings.muted_threads`, `presentation.muted`, and `unread.muted`.
pub fn is_muted_state(
    muted_threads: &HashMap<String, i64>,
    thread_id: &str,
    is_group: bool,
    presentation_muted: bool,
    unread_muted: bool,
    unread_muted_until: Option<i64>,
    now_unix: i64,
) -> bool {
    if active_mute_until(muted_threads, thread_id, is_group, now_unix).is_some() {
        return true;
    }
    if unread_muted && unread_muted_until.is_none_or(|until| until > now_unix) {
        return true;
    }
    if !presentation_muted {
        return false;
    }
    if unread_muted_until.is_some_and(|until| until <= now_unix) {
        return false;
    }
    let has_expired_settings = !muted_threads.is_empty()
        && muted_threads
            .iter()
            .any(|(k, &u)| !is_mute_until_active(u, now_unix) && mute_key_matches(k, thread_id, is_group));
    !has_expired_settings
}

/// Convenience wrapper for `is_muted_state` taking a `&Thread`.
pub fn is_thread_muted(
    muted_threads: &HashMap<String, i64>,
    thread: &Thread,
    now_unix: i64,
) -> bool {
    is_muted_state(
        muted_threads,
        &thread.id,
        thread.is_group,
        thread.presentation.muted,
        thread.unread.muted,
        thread.unread.muted_until,
        now_unix,
    )
}

/// Insert or update a mute entry under the canonical key while removing stale alias duplicates.
pub fn set_muted_thread(
    muted_threads: &mut HashMap<String, i64>,
    thread_id: &str,
    is_group: bool,
    until: i64,
) {
    clear_muted_thread(muted_threads, thread_id, is_group);
    if until != -1 && until <= 0 {
        return;
    }
    let key = canonical_mute_key(thread_id, is_group);
    if !key.is_empty() {
        muted_threads.insert(key, until);
    }
}

/// Remove all alias keys matching `(thread_id, is_group)` from `muted_threads`.
pub fn clear_muted_thread(
    muted_threads: &mut HashMap<String, i64>,
    thread_id: &str,
    is_group: bool,
) {
    muted_threads.retain(|k, _| !mute_key_matches(k, thread_id, is_group));
}

/// Normalize and deduplicate `muted_threads` keys (`group_`/`G` -> `g`, `u`/`U` -> bare) and
/// prune expired entries without guessing whether ambiguous bare IDs belong to a group or DM.
pub fn normalize_muted_threads(muted_threads: &mut HashMap<String, i64>, now_unix: i64) {
    if muted_threads.is_empty() {
        return;
    }
    let entries: Vec<(String, i64)> = muted_threads.drain().collect();
    for (raw_key, until) in entries {
        if !is_mute_until_active(until, now_unix) {
            continue;
        }
        let (kind, clean) = classify_mute_key(&raw_key);
        if clean.is_empty() || clean == "0" {
            continue;
        }
        let canonical = match kind {
            MuteKeyKind::Group => format!("g{clean}"),
            MuteKeyKind::User | MuteKeyKind::Bare => clean.to_string(),
        };
        muted_threads
            .entry(canonical)
            .and_modify(|prev| *prev = stronger_mute_until(*prev, until))
            .or_insert(until);
    }
}

/// Normalize `muted_threads` and promote bare keys that uniquely identify a group thread to `g<id>`.
pub fn reconcile_muted_threads_with_threads(
    muted_threads: &mut HashMap<String, i64>,
    threads: &[Thread],
    now_unix: i64,
) {
    normalize_muted_threads(muted_threads, now_unix);
    if muted_threads.is_empty() || threads.is_empty() {
        return;
    }
    let bare_keys: Vec<String> = muted_threads
        .keys()
        .filter(|k| classify_mute_key(k).0 == MuteKeyKind::Bare)
        .cloned()
        .collect();
    for key in bare_keys {
        let (_, clean) = classify_mute_key(&key);
        let has_group = threads
            .iter()
            .any(|t| t.is_group && classify_mute_key(&t.id).1 == clean);
        let has_dm = threads
            .iter()
            .any(|t| !t.is_group && classify_mute_key(&t.id).1 == clean);
        if has_group && !has_dm {
            if let Some(until) = muted_threads.remove(&key) {
                let g_key = format!("g{clean}");
                muted_threads
                    .entry(g_key)
                    .and_modify(|prev| *prev = stronger_mute_until(*prev, until))
                    .or_insert(until);
            }
        }
    }
}

/// Hydrate a thread's mute state from `muted_threads` while clearing expired timed mutes.
pub fn apply_muted_settings_to_thread(
    muted_threads: &HashMap<String, i64>,
    thread: &mut Thread,
    now_unix: i64,
) {
    if let Some(until) = active_mute_until(muted_threads, &thread.id, thread.is_group, now_unix) {
        thread.presentation.muted = true;
        thread.unread.muted = true;
        thread.unread.muted_until = (until > 0).then_some(until);
    } else if thread.unread.is_muted_at(now_unix) {
        thread.presentation.muted = true;
    } else if thread.unread.muted_until.is_some_and(|u| u <= now_unix) {
        thread.presentation.muted = false;
        thread.unread.muted = false;
        thread.unread.muted_until = None;
    }
}

/// Protect a just-succeeded `setmute` action against eventual-consistency lag in an immediate
/// follow-up `getmute` snapshot.
pub fn reconcile_post_setmute_snapshot(
    mute_map: &mut HashMap<String, i64>,
    thread_id: &str,
    is_group: bool,
    mute: bool,
    duration_sec: i64,
    action_now_unix: i64,
) {
    if mute {
        if active_mute_until(mute_map, thread_id, is_group, action_now_unix).is_none() {
            let expected_until = if duration_sec < 0 {
                -1
            } else {
                action_now_unix.saturating_add(duration_sec)
            };
            set_muted_thread(mute_map, thread_id, is_group, expected_until);
        }
    } else {
        clear_muted_thread(mute_map, thread_id, is_group);
    }
}

fn payload_error_code(value: &Value) -> Option<i64> {
    value
        .get("error_code")
        .and_then(|v| v.as_i64().or_else(|| v.as_str()?.trim().parse().ok()))
        .filter(|c| *c != 0)
}

fn unwrap_mute_payload<'a>(data: &'a Value, decoded: &'a mut Option<Value>) -> Option<&'a Value> {
    if payload_error_code(data).is_some() {
        return None;
    }
    let raw = data.get("data").unwrap_or(data);
    if let Some(text) = raw.as_str() {
        let Ok(parsed) = serde_json::from_str::<Value>(text) else {
            return None;
        };
        if payload_error_code(&parsed).is_some() {
            return None;
        }
        *decoded = Some(parsed);
        if let Some(ref d) = *decoded {
            let inner = if d.get("chatEntries").is_none() && d.get("groupChatEntries").is_none() {
                d.get("data").unwrap_or(d)
            } else {
                d
            };
            if payload_error_code(inner).is_some() {
                return None;
            }
            return Some(inner);
        }
    }
    if payload_error_code(raw).is_some() {
        return None;
    }
    let inner = if raw.get("chatEntries").is_none() && raw.get("groupChatEntries").is_none() {
        raw.get("data").unwrap_or(raw)
    } else {
        raw
    };
    if payload_error_code(inner).is_some() {
        return None;
    }
    Some(inner)
}

fn extract_mute_entry_arrays(payload: &Value) -> Option<(&[Value], &[Value])> {
    let peers_val = payload.get("chatEntries");
    let groups_val = payload.get("groupChatEntries");
    let peers = peers_val.and_then(Value::as_array);
    let groups = groups_val.and_then(Value::as_array);
    let both_explicit_null =
        peers_val.is_some_and(Value::is_null) && groups_val.is_some_and(Value::is_null);
    if peers.is_none() && groups.is_none() && !both_explicit_null {
        return None;
    }
    Some((
        peers.map(Vec::as_slice).unwrap_or(&[]),
        groups.map(Vec::as_slice).unwrap_or(&[]),
    ))
}

fn json_i64(value: &Value, key: &str) -> Option<i64> {
    value
        .get(key)
        .and_then(|v| v.as_i64().or_else(|| v.as_str()?.trim().parse().ok()))
}

fn entry_thread_id(entry: &Value) -> Option<String> {
    for key in ["id", "toId", "toid", "grid", "groupId"] {
        if let Some(v) = entry.get(key) {
            let s = v
                .as_str()
                .map(str::to_owned)
                .or_else(|| v.as_u64().map(|n| n.to_string()))
                .or_else(|| v.as_i64().filter(|&n| n > 0).map(|n| n.to_string()));
            if let Some(val) = s {
                let (_, clean) = classify_mute_key(&val);
                if !clean.is_empty() && clean != "0" {
                    return Some(clean.to_string());
                }
            }
        }
    }
    None
}

fn normalize_unix_sec(ts: i64) -> i64 {
    if ts > 100_000_000_000 {
        ts / 1000
    } else {
        ts
    }
}

struct ParsedMuteEntry {
    clean_id: String,
    muted: bool,
    until: Option<i64>,
}

fn parse_mute_entry(entry: &Value, now_unix: i64) -> Option<ParsedMuteEntry> {
    let clean_id = entry_thread_id(entry)?;
    let mode = json_i64(entry, "muteMode");
    if mode.is_some_and(|m| m != 0 && m != 1) {
        return None;
    }
    let duration = json_i64(entry, "duration");
    let start = json_i64(entry, "startTime")
        .or_else(|| json_i64(entry, "systemTime"))
        .map(normalize_unix_sec);
    let is_muted_mode = match mode {
        Some(m) => m == 1 && duration != Some(0),
        None => duration.unwrap_or(-1) != 0,
    };
    let until = if is_muted_mode {
        let dur = duration.unwrap_or(-1);
        if dur >= 0 {
            Some(start.unwrap_or(now_unix).saturating_add(dur))
        } else {
            None
        }
    } else {
        None
    };
    Some(ParsedMuteEntry {
        clean_id,
        muted: is_muted_mode,
        until,
    })
}

/// Complete read-only mute snapshots use mode 1 and duration -1 for indefinite mute.
pub fn merge_mute_metadata(threads: &mut [Thread], data: &Value) {
    merge_mute_metadata_at(threads, data, crate::login::unix_now() as i64);
}

/// Deterministic `merge_mute_metadata` at an explicit `now_unix` timestamp.
pub fn merge_mute_metadata_at(threads: &mut [Thread], data: &Value, now_unix: i64) {
    let mut decoded = None;
    let Some(payload) = unwrap_mute_payload(data, &mut decoded) else {
        return;
    };
    let Some((peers, groups)) = extract_mute_entry_arrays(payload) else {
        return;
    };
    for thread in threads {
        let entries = if thread.is_group { groups } else { peers };
        let (_, clean_tid) = classify_mute_key(&thread.id);
        let parsed = entries
            .iter()
            .filter_map(|e| parse_mute_entry(e, now_unix))
            .find(|p| p.clean_id == clean_tid);
        if let Some(p) = parsed {
            thread.unread.muted = p.muted;
            thread.unread.muted_until = p.until;
        } else {
            let has_unknown_mode_match = entries.iter().any(|e| {
                entry_thread_id(e).is_some_and(|cid| cid == clean_tid)
                    && json_i64(e, "muteMode").is_some_and(|m| m != 0 && m != 1)
            });
            if has_unknown_mode_match {
                continue;
            }
            thread.unread.muted = false;
            thread.unread.muted_until = None;
        }
        thread.unread.settings_fresh = true;
        thread.unread.fresh = true;
        thread.presentation.muted = thread.unread.is_muted_at(now_unix);
    }
}

/// Extract an authoritative `thread_id -> until_unix` map (`-1` for indefinite mute) from a
/// `/api/social/profile/getmute` payload so `AppSettings::muted_threads` stays synchronized
/// with Official Zalo PC (#67).
pub fn extract_mute_map(data: &Value, now_unix: i64) -> Option<HashMap<String, i64>> {
    let mut decoded = None;
    let payload = unwrap_mute_payload(data, &mut decoded)?;
    let (peers, groups) = extract_mute_entry_arrays(payload)?;
    let mut map = HashMap::new();
    for (entries, is_group) in [(peers, false), (groups, true)] {
        for entry in entries {
            let Some(p) = parse_mute_entry(entry, now_unix) else {
                continue;
            };
            if !p.muted {
                continue;
            }
            let until = match p.until {
                None => -1,
                Some(exp) if exp > now_unix => exp,
                Some(_) => continue,
            };
            let key = canonical_mute_key(&p.clean_id, is_group);
            if !key.is_empty() {
                map.entry(key)
                    .and_modify(|prev| *prev = stronger_mute_until(*prev, until))
                    .or_insert(until);
            }
        }
    }
    Some(map)
}


/// Complete read-only manual-mark snapshot. No conversations are created by this adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManualUnreadMark {
    pub thread_id: String,
    pub client_message_id: String,
    pub from_uid: String,
}

pub fn parse_manual_unread_marks(response: &Value) -> anyhow::Result<Vec<ManualUnreadMark>> {
    fn checked(value: &Value) -> anyhow::Result<()> {
        if let Some(code) = value.get("error_code") {
            if code.as_i64().or_else(|| code.as_str()?.parse().ok()) != Some(0) {
                anyhow::bail!("Manual unread request failed");
            }
        }
        Ok(())
    }
    checked(response)?;
    let raw = response.get("data").unwrap_or(response);
    let decoded;
    let data = if let Some(text) = raw.as_str() {
        decoded = serde_json::from_str::<Value>(text)
            .map_err(|_| anyhow::anyhow!("Invalid manual unread payload"))?;
        &decoded
    } else {
        raw
    };
    checked(data)?;
    let (Some(groups), Some(users)) = (
        data.get("convsGroup").and_then(Value::as_array),
        data.get("convsUser").and_then(Value::as_array),
    ) else {
        anyhow::bail!("Incomplete manual unread snapshot");
    };
    let mut marks = Vec::new();
    for (rows, group) in [(groups, true), (users, false)] {
        for row in rows {
            let id = string(row, "id")
                .filter(|id| !id.is_empty() && id != "0")
                .ok_or_else(|| anyhow::anyhow!("Invalid manual unread identity"))?;
            marks.push(ManualUnreadMark {
                thread_id: if group {
                    format!("g{}", id.trim_start_matches('g'))
                } else {
                    id
                },
                client_message_id: string(row, "cliMsgId").unwrap_or_default(),
                from_uid: string(row, "fromUid").unwrap_or_default(),
            });
        }
    }
    Ok(marks)
}

pub fn merge_manual_unread_marks(threads: &mut [Thread], marks: &[ManualUnreadMark]) {
    for thread in threads {
        thread.unread.marked_unread = marks.iter().any(|mark| mark.thread_id == thread.id);
        thread.unread.settings_fresh = true;
        thread.unread.fresh = true;
    }
}

/// Shipped static getMarkAsUnread, command 12011. Reading does not modify account state.
pub async fn fetch_manual_unread_marks(
    session: &crate::session::Session,
) -> anyhow::Result<Vec<ManualUnreadMark>> {
    if session.login.is_none() {
        anyhow::bail!("No active session");
    }
    let params = session.encoded_params(&serde_json::json!({}))?;
    let url = format!(
        "{}/api/conv/getUnreadMark?{}&params={params}",
        session.domains.conversation.trim_end_matches('/'),
        session.common_params()
    );
    let raw = session.get(&url).await?;
    let response = session
        .maybe_decrypt(raw, session.session_key_bytes())
        .await;
    parse_manual_unread_marks(&response)
}

/// Unknown is distinct from a verified empty/nonmatching raw mentions array.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MentionDetection {
    Mention,
    NoMention,
    Unknown,
}

/// Verified original desktop MENTION_ALL_ID, not a display-name/text heuristic.
pub const MENTION_ALL_ID: &str = "-1";

pub fn detect_message_mention(message: &ChatMessage, account: &str) -> bool {
    classify_message_mention(message, account) == MentionDetection::Mention
}

pub fn classify_message_mention(message: &ChatMessage, account: &str) -> MentionDetection {
    if account.is_empty() || account == "0" {
        return MentionDetection::Unknown;
    }
    if message.is_outgoing {
        return MentionDetection::NoMention;
    }
    // Original lazy/default-login-main-startup-shared-worker-znotification.d914...:
    // U(e) tests mentions[].uid == session UID || uid === MENTION_ALL_ID; ji="-1".
    // Original shared-worker.06caa1ffa528545b8f43.js syncMessage2SVMessage:
    // meta.mentionsList becomes mentions; opaque UIDs are decoded, while "-1" stays.
    if ["uidFrom", "fromUid"]
        .iter()
        .any(|key| message.wire_ref.fields.get(*key).and_then(Value::as_str) == Some("0"))
    {
        return MentionDetection::NoMention;
    }
    let wire = message.wire_ref.fields.get("mentions");
    let archive = message
        .history_source
        .as_ref()
        .and_then(|source| source.pointer("/meta/mentionsList"));
    let Some(raw) = wire.or(archive) else {
        return MentionDetection::Unknown;
    };
    let Some(mentions) = raw.as_array() else {
        return MentionDetection::Unknown;
    };
    let mut unknown = false;
    for mention in mentions {
        let Some(uid) = mention.get("uid") else {
            unknown = true;
            continue;
        };
        let target = uid
            .as_str()
            .map(str::to_owned)
            .or_else(|| uid.as_u64().map(|uid| uid.to_string()));
        if uid.as_str() == Some(MENTION_ALL_ID) || target.as_deref() == Some(account) {
            return MentionDetection::Mention;
        }
        if target.is_none() {
            unknown = true;
        }
        // An archive noised UID that does not exactly match the account is not a
        // verified negative without the original decoder's identity mapping.
        if archive.is_some()
            && target
                .as_ref()
                .is_some_and(|uid| !uid.bytes().all(|b| b.is_ascii_digit()))
        {
            unknown = true;
        }
    }
    if unknown {
        MentionDetection::Unknown
    } else {
        MentionDetection::NoMention
    }
}
