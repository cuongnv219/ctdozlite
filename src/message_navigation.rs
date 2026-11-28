//! Exact quoted-message lookup within the currently loaded conversation.
use crate::chat::ChatMessage;
use serde_json::Value;

fn exact_id(value: &Value) -> Option<String> {
    match value {
        Value::String(id) if !id.is_empty() => Some(id.clone()),
        Value::Number(id) if id.is_u64() || id.is_i64() => Some(id.to_string()),
        _ => None,
    }
}

/// Resolve a reference only when exactly one loaded message matches any ID namespace.
/// Text, timestamps, numeric coercion and namespace preference never disambiguate IDs.
pub fn unique_reference_index(messages: &[ChatMessage], reference: &str) -> Option<usize> {
    if !valid_reference(reference) {
        return None;
    }
    let mut matches = messages.iter().enumerate().filter(|(_, message)| {
        message.id == reference
            || message.client_message_id == reference
            || [&message.wire_ref.server_id, &message.wire_ref.client_id]
                .into_iter()
                .flatten()
                .any(|id| exact_id(id).as_deref() == Some(reference))
    });
    let (index, _) = matches.next()?;
    matches.next().is_none().then_some(index)
}

/// Khớp 1 tin với reference: id → client_message_id → wire ids.
/// Dùng cho MessageTranslated (msg id có thể là client id khi tin vừa gửi).
pub fn message_matches_reference(message: &ChatMessage, reference: &str) -> bool {
    if !valid_reference(reference) {
        return false;
    }
    if message.id == reference || message.client_message_id == reference {
        return true;
    }
    [&message.wire_ref.server_id, &message.wire_ref.client_id]
        .into_iter()
        .flatten()
        .any(|id| exact_id(id).as_deref() == Some(reference))
}

fn valid_reference(reference: &str) -> bool {
    !reference.trim().is_empty() && !reference.bytes().all(|byte| byte == b'0')
}

fn reply_reference(quote: &Value) -> Option<String> {
    ["cliMsgId", "globalMsgId", "msgId"]
        .into_iter()
        .filter_map(|key| quote.get(key).and_then(exact_id))
        .find(|id| valid_reference(id))
}

/// Read an explicit reply reference, preserving its original ID representation.
pub fn message_reply_reference(message: &ChatMessage) -> Option<String> {
    message
        .reply_to_id
        .as_ref()
        .filter(|id| valid_reference(id))
        .cloned()
        .or_else(|| {
            message
                .wire_ref
                .fields
                .get("quote")
                .and_then(reply_reference)
        })
        .or_else(|| {
            message
                .history_source
                .as_ref()
                .and_then(|raw| raw.pointer("/meta/quote"))
                .and_then(reply_reference)
        })
}

/// Widen the newest-message window enough to lay out the target and preceding context.
pub(crate) fn render_limit_for_target(current: usize, count: usize, target: usize) -> usize {
    current.max(count.saturating_sub(target.saturating_sub(3)))
}
