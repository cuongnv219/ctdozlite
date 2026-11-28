//! Group polls: the poll a `group.poll` event points to, its detail, and vote state.
//!
//! Protocol evidence (endpoints, params, response fields) is in `re-notes/44_group_poll.md`.

use serde_json::{json, Value};

use crate::chat::ChatMessage;

/// Server error for a vote on a closed or locked poll (`STR_BLOCKED_POLL` in Zalo Web).
pub const ERR_POLL_BLOCKED: i64 = 166;

/// The poll a `group.poll` message refers to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PollRef {
    pub poll_id: String,
    /// `content.action`: `create`, `vote`, … (empty when the server sent none).
    pub action: String,
    /// `params.question`, shown until the detail arrives.
    pub question: String,
}

/// Read the poll id and action from a message's stored wire content.
///
/// Only `group.poll` rows qualify; `content` and `content.params` may each be a JSON object
/// or a JSON string.
pub fn poll_ref(msg: &ChatMessage) -> Option<PollRef> {
    if msg.wire_ref.original_type.as_ref().and_then(Value::as_str) != Some("group.poll") {
        return None;
    }
    poll_ref_from_content(msg.wire_ref.fields.get("content")?)
}

fn as_object(v: &Value) -> Option<Value> {
    match v {
        Value::Object(_) => Some(v.clone()),
        Value::String(s) if s.trim_start().starts_with('{') => serde_json::from_str(s).ok(),
        _ => None,
    }
}

fn id_string(v: &Value) -> Option<String> {
    let s = match v {
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.trim().to_string(),
        _ => return None,
    };
    (!s.is_empty() && s != "0").then_some(s)
}

/// [`poll_ref`] on a raw `content` value.
pub fn poll_ref_from_content(content: &Value) -> Option<PollRef> {
    let content = as_object(content)?;
    let params = content.get("params").and_then(as_object)?;
    let poll_id = params.get("pollId").and_then(id_string)?;
    let action = content
        .get("action")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let question = params
        .get("question")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .trim()
        .to_string();
    Some(PollRef {
        poll_id,
        action,
        question,
    })
}

/// One answer of a poll.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PollOption {
    pub id: i64,
    pub content: String,
    pub votes: i64,
    /// Whether the current user picked this option, as the server reports it.
    pub voted: bool,
    /// Voter uids; empty for anonymous polls.
    pub voters: Vec<String>,
}

/// A poll as `/api/poll/detail` returns it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PollDetail {
    pub poll_id: String,
    pub question: String,
    pub creator: String,
    pub options: Vec<PollOption>,
    pub joined: bool,
    pub closed: bool,
    pub allow_multi_choices: bool,
    pub is_anonymous: bool,
    pub is_hide_vote_preview: bool,
    /// Deadline in ms since the epoch; `0` means none.
    pub expired_time: i64,
    /// Distinct voters, when the server sends it.
    pub num_vote: Option<i64>,
}

fn as_i64(v: Option<&Value>) -> Option<i64> {
    match v? {
        Value::Number(n) => n.as_i64(),
        Value::String(s) => s.trim().parse().ok(),
        _ => None,
    }
}

fn as_bool(v: Option<&Value>) -> bool {
    match v {
        Some(Value::Bool(b)) => *b,
        Some(Value::Number(n)) => n.as_i64().unwrap_or(0) != 0,
        _ => false,
    }
}

fn parse_option(v: &Value) -> Option<PollOption> {
    let id = as_i64(v.get("option_id"))?;
    let voters = v
        .get("voters")
        .and_then(Value::as_array)
        .map(|a| a.iter().filter_map(id_string).collect())
        .unwrap_or_default();
    Some(PollOption {
        id,
        content: v
            .get("content")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string(),
        votes: as_i64(v.get("votes")).unwrap_or(0).max(0),
        voted: as_bool(v.get("voted")),
        voters,
    })
}

/// Unwrap a decrypted poll response to its `data` object.
///
/// # Errors
/// Returns a Vietnamese message when `error_code` is non-zero (code `166` is a closed or
/// locked poll) or when `data` is missing.
pub fn poll_response_data(resp: &Value) -> anyhow::Result<Value> {
    let code = resp.get("error_code").and_then(Value::as_i64).unwrap_or(0);
    if code == ERR_POLL_BLOCKED {
        anyhow::bail!("Bình chọn đã bị khóa");
    }
    if code != 0 {
        let msg = resp
            .get("error_message")
            .and_then(Value::as_str)
            .filter(|m| !m.is_empty())
            .unwrap_or("yêu cầu bình chọn bị từ chối");
        anyhow::bail!("mã lỗi {code}: {msg}");
    }
    resp.get("data")
        .and_then(as_object)
        .ok_or_else(|| anyhow::anyhow!("phản hồi bình chọn không có dữ liệu"))
}

/// Parse a decrypted `/api/poll/detail` response.
///
/// # Errors
/// See [`poll_response_data`]; also fails when `data` has no options.
pub fn parse_poll_detail(resp: &Value) -> anyhow::Result<PollDetail> {
    let data = poll_response_data(resp)?;
    let options = parse_options(&data)
        .ok_or_else(|| anyhow::anyhow!("phản hồi bình chọn không có phương án"))?;
    Ok(PollDetail {
        poll_id: data.get("poll_id").and_then(id_string).unwrap_or_default(),
        question: data
            .get("question")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string(),
        creator: data.get("creator").and_then(id_string).unwrap_or_default(),
        options,
        joined: as_bool(data.get("joined")),
        closed: as_bool(data.get("closed")),
        allow_multi_choices: as_bool(data.get("allow_multi_choices")),
        is_anonymous: as_bool(data.get("is_anonymous")),
        is_hide_vote_preview: as_bool(data.get("is_hide_vote_preview")),
        expired_time: as_i64(data.get("expired_time")).unwrap_or(0),
        num_vote: as_i64(data.get("num_vote")),
    })
}

/// Options from a response `data` object (`/api/poll/vote` answers with these only).
pub fn parse_options(data: &Value) -> Option<Vec<PollOption>> {
    let list = data.get("options")?.as_array()?;
    Some(list.iter().filter_map(parse_option).collect())
}

/// `getPollDetail` params: `{poll_id, imei}`.
pub fn detail_params(poll_id: &str, imei: &str) -> Value {
    json!({ "poll_id": poll_id_value(poll_id), "imei": imei })
}

/// `vote` params: `{poll_id, option_ids, imei}`; `option_ids` is a plain array.
pub fn vote_params(poll_id: &str, option_ids: &[i64], imei: &str) -> Value {
    json!({ "poll_id": poll_id_value(poll_id), "option_ids": option_ids, "imei": imei })
}

/// The web client sends the numeric `pollId` from the event unchanged.
fn poll_id_value(poll_id: &str) -> Value {
    poll_id
        .trim()
        .parse::<i64>()
        .map(Value::from)
        .unwrap_or_else(|_| Value::from(poll_id.trim()))
}

/// Why the user cannot vote right now, if anything.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoteBlock {
    Closed,
    Expired,
    /// "Hide results until voted" polls cannot be changed after voting.
    Final,
}

impl VoteBlock {
    /// User-facing reason.
    pub fn label(self) -> &'static str {
        match self {
            Self::Closed => "Bình chọn đã đóng",
            Self::Expired => "Bình chọn đã hết hạn",
            Self::Final => "Không thể thay đổi bình chọn",
        }
    }
}

impl PollDetail {
    /// Option ids the user has picked: `my_uid` among the voters, or the server's `voted`
    /// flag when the voter list is withheld (anonymous polls).
    pub fn my_choices(&self, my_uid: &str) -> Vec<i64> {
        self.options
            .iter()
            .filter(|o| {
                (!my_uid.is_empty() && o.voters.iter().any(|v| v == my_uid))
                    || (o.voted && o.voters.is_empty())
            })
            .map(|o| o.id)
            .collect()
    }

    /// Distinct voters: `num_vote`, else the union of the voter lists.
    pub fn voter_count(&self) -> i64 {
        if let Some(n) = self.num_vote.filter(|n| *n > 0) {
            return n;
        }
        let mut uids: Vec<&str> = self
            .options
            .iter()
            .flat_map(|o| o.voters.iter().map(String::as_str))
            .collect();
        uids.sort_unstable();
        uids.dedup();
        uids.len() as i64
    }

    /// Total votes across options.
    pub fn vote_total(&self) -> i64 {
        self.options.iter().map(|o| o.votes).sum()
    }

    /// Results stay hidden for "hide results until voted" polls the user has not joined.
    pub fn results_hidden(&self, my_uid: &str) -> bool {
        self.is_hide_vote_preview && !self.joined && self.my_choices(my_uid).is_empty()
    }

    /// `None` when the user may vote (or change their vote) at `now_ms`.
    pub fn vote_block(&self, my_uid: &str, now_ms: i64) -> Option<VoteBlock> {
        if self.closed {
            return Some(VoteBlock::Closed);
        }
        if self.expired_time > 0 && self.expired_time < now_ms {
            return Some(VoteBlock::Expired);
        }
        if self.is_hide_vote_preview && !self.my_choices(my_uid).is_empty() {
            return Some(VoteBlock::Final);
        }
        None
    }

    /// Replace the options with a vote reply's, marking the poll as joined.
    pub fn apply_vote_reply(&mut self, options: Vec<PollOption>) {
        if !options.is_empty() {
            self.options = options;
        }
        self.joined = true;
    }
}

/// Toggle `id` in a pending selection, honouring single-choice polls.
pub fn toggle_choice(selection: &mut Vec<i64>, id: i64, multi: bool) {
    if let Some(pos) = selection.iter().position(|x| *x == id) {
        selection.remove(pos);
    } else if multi {
        selection.push(id);
    } else {
        selection.clear();
        selection.push(id);
    }
}

/// Whether `selection` differs from `current` as a set.
pub fn selection_changed(selection: &[i64], current: &[i64]) -> bool {
    let mut a = selection.to_vec();
    let mut b = current.to_vec();
    a.sort_unstable();
    a.dedup();
    b.sort_unstable();
    b.dedup();
    a != b
}
