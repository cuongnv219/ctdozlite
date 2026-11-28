//! Chat data: friend/conversation list + last messages, all over REST.
//!
//! Notes: `re-notes/02_chat_data.md`.

use anyhow::Result;
use serde_json::{json, Value};

use crate::session::{obj, Session};

/// Single sub-article or list item inside an OA or MiniApp rich card.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct OaCardItem {
    #[serde(default)]
    pub is_header: bool,
    #[serde(default)]
    pub title: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumb: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Action chip or CTA button on an OA, MiniApp, ZInstant, or ECard message.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct OaCardButton {
    #[serde(default)]
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

/// Structured rich card payload for OA (`chat.list.action`), ZInstant/MiniApp
/// (`chat.webcontent`, `chat.webcontent.v2`), ECard (`chat.ecard`), and banner OA notices.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct OaRichCard {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub card_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner_thumb: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_label: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<OaCardItem>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub buttons: Vec<OaCardButton>,
}

impl OaRichCard {
    /// Returns true when the card has no renderable banner, text, list items, or action buttons.
    pub fn is_empty(&self) -> bool {
        let has_banner = self
            .banner_thumb
            .as_deref()
            .is_some_and(|s| !s.trim().is_empty());
        let has_title = self.title.as_deref().is_some_and(|s| !s.trim().is_empty());
        let has_desc = self
            .description
            .as_deref()
            .is_some_and(|s| !s.trim().is_empty());
        !has_banner && !has_title && !has_desc && self.items.is_empty() && self.buttons.is_empty()
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ChatMessage {
    #[serde(default)]
    pub kind: crate::message_model::MessageKind,
    #[serde(default)]
    pub wire_ref: crate::message_model::WireMessageRef,
    #[serde(default)]
    pub unread_policy: crate::message_model::UnreadPolicy,
    #[serde(default)]
    pub delivery: crate::message_model::DeliveryState,
    pub id: String,
    pub sender: String,
    pub text: String,
    pub time: String,
    pub is_outgoing: bool,
    pub ts: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sticker_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sticker_cate_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sticker_type: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to_sender: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to_text: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reactions: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reaction_summaries: Vec<crate::reactions::ReactionSummary>,
    #[serde(default)]
    pub reaction_snapshot: bool,
    #[serde(default)]
    pub reaction_authoritative_snapshot: bool,
    #[serde(skip)]
    pub reaction_snapshot_fresh: bool,
    #[serde(default)]
    pub reaction_snapshot_provenance: crate::reactions::ReactionSnapshotProvenance,
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub reaction_versions: std::collections::BTreeMap<String, crate::reactions::ReactionVersion>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub client_message_id: String,
    #[serde(default)]
    pub is_pinned: bool,
    /// Group-board topic id backing a remote pin (`/api/board/topic/createv2`), needed to unpin.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub board_topic_id: String,
    #[serde(default)]
    pub is_starred: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_thumb: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_src: Option<String>,
    /// Rich card payload for OA (`chat.list.action`), ZInstant/MiniApp (`chat.webcontent`), ECard, or banner notices.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oa_card: Option<OaRichCard>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history_source: Option<Value>,
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub resolved_mention_uids: std::collections::BTreeMap<String, String>,
    /// Photo album membership (`group_layout_id`); consecutive photos sharing it render as one grid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_group: Option<MediaGroupLayout>,
    /// Pixel size the sender declared for a photo (`params.width`/`height`). Lets the chat
    /// reserve the final size before the thumbnail loads, so the list does not jump.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_size: Option<[u32; 2]>,
    /// Bản dịch hiển thị dưới tin gốc (session-only, không persist).
    #[serde(skip)]
    pub translation: Option<String>,
    /// Tin đã qua sửa (recall + gửi lại). Lịch sử các bản cũ nằm ở SQLite
    /// bảng `message_edits`; RAM chỉ giữ cờ để vẽ nhãn "(đã sửa)".
    #[serde(default)]
    pub edited: bool,
}

impl ChatMessage {
    pub fn merge_message_metadata(&mut self, incoming: &Self) -> bool {
        let previous_mentions = self.resolved_mention_uids.clone();
        self.resolved_mention_uids
            .extend(incoming.resolved_mention_uids.clone());
        let before = (
            self.kind,
            self.wire_ref.clone(),
            self.unread_policy.clone(),
            self.delivery,
            self.history_source.clone(),
        );
        if incoming.kind != crate::message_model::MessageKind::Unknown {
            self.kind = incoming.kind;
        }
        // A system notice's text is built by the parser (e.g. poll votes); a stale label
        // such as "[Hình ảnh]" saved by an older build must not survive the merge.
        let notice_text_changed = incoming.kind == crate::message_model::MessageKind::System
            && !incoming.text.is_empty()
            && self.text != incoming.text;
        if notice_text_changed {
            self.text = incoming.text.clone();
        }
        if !incoming.wire_ref.fields.is_empty() {
            let mut fields = self.wire_ref.fields.clone();
            fields.extend(incoming.wire_ref.fields.clone());
            self.wire_ref = crate::message_model::WireMessageRef::from_raw(&Value::Object(
                fields.into_iter().collect(),
            ));
            if incoming.unread_policy.params_ext_present {
                self.unread_policy = incoming.unread_policy.clone();
            }
        }
        if incoming.history_source.is_some() {
            self.history_source = incoming.history_source.clone();
        }
        let gained_media_group = self.media_group.is_none() && incoming.media_group.is_some();
        if gained_media_group {
            self.media_group = incoming.media_group.clone();
        }
        let gained_oa_card = self.oa_card.is_none() && incoming.oa_card.is_some();
        if gained_oa_card {
            self.oa_card = incoming.oa_card.clone();
        }
        let gained_media_size = self.media_size.is_none() && incoming.media_size.is_some();
        if gained_media_size {
            self.media_size = incoming.media_size;
        }
        self.delivery = self.delivery.merge(incoming.delivery);
        gained_media_group
            || notice_text_changed
            || gained_oa_card
            || gained_media_size
            || previous_mentions != self.resolved_mention_uids
            || before
                != (
                    self.kind,
                    self.wire_ref.clone(),
                    self.unread_policy.clone(),
                    self.delivery,
                    self.history_source.clone(),
                )
    }

    /// Returns a renderable `OaRichCard`: either the explicit `oa_card`, or a synthesized
    /// `"banner"` card for cached OA notifications that carry `link_thumb` + `link_title`/`link_desc`
    /// without a `link_url`. Returns `None` for empty cards or legacy call bubbles.
    pub fn effective_oa_card(&self) -> Option<OaRichCard> {
        if let Some(card) = &self.oa_card {
            if !card.is_empty() {
                return Some(card.clone());
            }
        }
        if matches!(
            self.kind,
            crate::message_model::MessageKind::Sticker
                | crate::message_model::MessageKind::Photo
                | crate::message_model::MessageKind::Video
                | crate::message_model::MessageKind::Voice
                | crate::message_model::MessageKind::File
                | crate::message_model::MessageKind::Recall
                | crate::message_model::MessageKind::Delete
                | crate::message_model::MessageKind::System
        ) {
            return None;
        }
        if self.link_title.as_deref() == Some("sendBubbleMessage") {
            return None;
        }
        if self.link_url.as_deref().is_some_and(|u| !u.trim().is_empty()) {
            return None;
        }
        let banner_thumb = self
            .link_thumb
            .as_deref()
            .map(str::trim)
            .filter(|u| u.starts_with("http://") || u.starts_with("https://"))
            .map(str::to_string)?;
        let title = self
            .link_title
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string);
        let description = self
            .link_desc
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string);
        if title.is_none() && description.is_none() {
            return None;
        }
        Some(OaRichCard {
            card_type: "banner".to_string(),
            banner_thumb: Some(banner_thumb),
            title,
            description,
            url: None,
            source_label: self
                .link_src
                .as_deref()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string),
            items: Vec::new(),
            buttons: Vec::new(),
        })
    }

    /// Keep aggregate counts and event replay cursors across cache/network merges.
    pub fn merge_reaction_metadata(&mut self, incoming: &Self) -> bool {
        let before_total = crate::reactions::total_count(&self.reaction_summaries);
        let changed = self.merge_reaction_metadata_inner(incoming);
        crate::reactions::trace_count_change(
            &format!(
                "merge_reaction_metadata incoming_total={} incoming_snapshot={} incoming_authoritative={} incoming_versions={}",
                crate::reactions::total_count(&incoming.reaction_summaries),
                incoming.reaction_snapshot,
                incoming.reaction_authoritative_snapshot,
                incoming.reaction_versions.len()
            ),
            &self.id,
            before_total,
            crate::reactions::total_count(&self.reaction_summaries),
        );
        changed
    }

    fn merge_reaction_metadata_inner(&mut self, incoming: &Self) -> bool {
        let model_changed = self.merge_message_metadata(incoming);
        let previous_provenance = self.reaction_snapshot_provenance.clone();
        let before = (
            self.reaction_summaries.clone(),
            self.reactions.clone(),
            self.client_message_id.clone(),
            self.reaction_snapshot,
            self.reaction_authoritative_snapshot,
            self.reaction_versions.clone(),
        );
        if self.client_message_id.is_empty() {
            self.client_message_id = incoming.client_message_id.clone();
        }
        let accept_snapshot = incoming.reaction_authoritative_snapshot
            && (incoming.reaction_snapshot_fresh || !self.reaction_authoritative_snapshot)
            && self.reaction_snapshot_provenance.accepts_snapshot(
                &incoming.reaction_snapshot_provenance,
                &self.reaction_versions,
                incoming.reaction_snapshot_fresh,
            );

        if accept_snapshot {
            self.reaction_snapshot_provenance = incoming.reaction_snapshot_provenance.clone();
            self.reaction_summaries = incoming.reaction_summaries.clone();
            self.reaction_authoritative_snapshot = true;
            self.reaction_snapshot = true;
            for (user, version) in &mut self.reaction_versions {
                version.covered_events.append(&mut version.events);
                version.baseline = incoming
                    .reaction_summaries
                    .iter()
                    .filter_map(|s| Some((s.r_type?, *s.reactor_counts.get(user)?)))
                    .collect();
                version.ledger_initialized = true;
                version.snapshot_baseline = true;
            }
            for (user, incoming_version) in &incoming.reaction_versions {
                let covered = self
                    .reaction_versions
                    .get(user)
                    .map(|v| v.covered_events.clone())
                    .unwrap_or_default();
                let mut version = incoming_version.clone();
                version.covered_events.extend(covered);
                if incoming.reaction_snapshot_fresh {
                    version.covered_events.append(&mut version.events);
                    version.baseline = incoming
                        .reaction_summaries
                        .iter()
                        .filter_map(|s| Some((s.r_type?, *s.reactor_counts.get(user)?)))
                        .collect();
                    version.ledger_initialized = true;
                    version.snapshot_baseline = true;
                }
                self.reaction_versions.insert(user.clone(), version);
            }
        }
        if !self.reaction_authoritative_snapshot
            && !incoming.reaction_authoritative_snapshot
            && (incoming.reaction_snapshot || !incoming.reaction_summaries.is_empty())
            && self.reaction_versions.is_empty()
        {
            self.reaction_summaries = incoming.reaction_summaries.clone();
            self.reactions = crate::reactions::legacy_emojis(&self.reaction_summaries);
            self.reaction_snapshot = incoming.reaction_snapshot;
        }
        if !self.reaction_authoritative_snapshot
            && !incoming.reaction_authoritative_snapshot
            && !self.reaction_versions.is_empty()
            && (incoming.reaction_snapshot || !incoming.reaction_summaries.is_empty())
        {
            crate::reactions::merge_untracked_snapshot(
                &mut self.reaction_summaries,
                &incoming.reaction_summaries,
                &self.reaction_versions,
                incoming.reaction_snapshot && incoming.reaction_versions.is_empty(),
            );
        }
        for (user, version) in &incoming.reaction_versions {
            if accept_snapshot {
                continue;
            }
            if !self.reaction_versions.contains_key(user) {
                if self.reaction_authoritative_snapshot {
                    for event in &version.events {
                        if event.r_type >= -1 {
                            self.apply_reaction(event);
                        }
                    }
                    continue;
                }
                self.reaction_versions.insert(user.clone(), version.clone());
                if version.ledger_initialized {
                    version.rebuild(&mut self.reaction_summaries, user);
                }
            } else {
                for event in &version.events {
                    if event.r_type >= -1 {
                        self.apply_reaction(event);
                    }
                }
            }
        }
        if !self.reaction_summaries.is_empty()
            || incoming.reaction_snapshot
            || !incoming.reaction_versions.is_empty()
        {
            self.reactions = crate::reactions::legacy_emojis(&self.reaction_summaries);
        } else if self.reactions.is_empty()
            && self.reaction_versions.is_empty()
            && !self.reaction_snapshot
        {
            self.reactions = incoming.reactions.clone();
        }
        model_changed
            || previous_provenance != self.reaction_snapshot_provenance
            || before
                != (
                    self.reaction_summaries.clone(),
                    self.reactions.clone(),
                    self.client_message_id.clone(),
                    self.reaction_snapshot,
                    self.reaction_authoritative_snapshot,
                    self.reaction_versions.clone(),
                )
    }

    pub fn apply_reaction(&mut self, event: &crate::reactions::ReactionEvent) -> bool {
        let before_total = crate::reactions::total_count(&self.reaction_summaries);
        let changed = self.apply_reaction_inner(event);
        crate::reactions::trace_count_change(
            &format!(
                "apply_reaction event={} sender={}",
                event.event_id, event.sender_id
            ),
            &self.id,
            before_total,
            crate::reactions::total_count(&self.reaction_summaries),
        );
        changed
    }

    fn apply_reaction_inner(&mut self, event: &crate::reactions::ReactionEvent) -> bool {
        if event.sender_id.is_empty() || event.r_type < -1 {
            return false;
        }
        if self.reaction_summaries.is_empty() && !self.reactions.is_empty() {
            self.reaction_summaries =
                crate::reactions::parse_reactions(&json!({"reactions":self.reactions}));
        }
        let changed = self
            .reaction_versions
            .entry(event.sender_id.clone())
            .or_default()
            .record(
                &mut self.reaction_summaries,
                event,
                self.reaction_authoritative_snapshot,
            );
        self.reactions = crate::reactions::legacy_emojis(&self.reaction_summaries);
        self.reaction_snapshot = true;
        changed
    }

    /// Repair cached media metadata without changing captions or signed URL bytes.
    pub fn hydrate_media_metadata(&mut self) -> bool {
        if self.text == "[Tin nhắn đã thu hồi]" {
            return false;
        }
        let before = (
            self.link_url.clone(),
            self.link_thumb.clone(),
            self.link_src.clone(),
        );
        let source_kind = self.history_source.as_ref().and_then(|source| {
            let kind = source
                .get("msgType")
                .and_then(|v| v.as_i64().or_else(|| v.as_str()?.parse().ok()));
            match kind {
                Some(2) | Some(11) => Some("Hình ảnh Zalo"),
                Some(3) => Some("Thoại Zalo"),
                Some(4) | Some(13) => Some("Sticker Zalo"),
                Some(18) => Some("Video Zalo"),
                Some(19) => Some("Tập tin Zalo"),
                _ => None,
            }
        });
        if let Some(source) = &self.history_source {
            if let Some(attachment) = source
                .get("meta")
                .and_then(|m| m.get("attachsList"))
                .and_then(Value::as_array)
                .and_then(|a| a.first())
            {
                let (url, _, _, thumb, _) = extract_link_info(attachment);
                if self.link_url.is_none() {
                    self.link_url = url;
                }
                if self.link_thumb.is_none() {
                    self.link_thumb = thumb;
                }
            }
        }
        let label_kind = match self.text.trim() {
            "[Photo]" | "[Hình ảnh]" => Some("Hình ảnh Zalo"),
            "[Video]" => Some("Video Zalo"),
            _ => None,
        };
        if let Some(kind) = source_kind.or(label_kind) {
            self.link_src = Some(kind.into());
            if kind == "Hình ảnh Zalo" && self.link_thumb.is_none() {
                self.link_thumb = self.link_url.clone();
            }
        }
        before
            != (
                self.link_url.clone(),
                self.link_thumb.clone(),
                self.link_src.clone(),
            )
    }

    /// Fill independently missing card fields when an existing URL already exists.
    /// The parser layer wires this helper in the dependent protocol change.
    #[allow(dead_code)]
    fn merge_media_metadata(&mut self, incoming: &Self) -> bool {
        if self.text == "[Tin nhắn đã thu hồi]" {
            return false;
        }
        let before = (
            self.link_url.clone(),
            self.link_title.clone(),
            self.link_desc.clone(),
            self.link_thumb.clone(),
            self.link_src.clone(),
        );
        if self.link_title.is_none() {
            self.link_title = incoming.link_title.clone();
        }
        if self.link_desc.is_none() {
            self.link_desc = incoming.link_desc.clone();
        }
        if self.link_thumb.is_none() {
            self.link_thumb = incoming.link_thumb.clone();
        }
        if self.link_src.is_none()
            || matches!(
                incoming.link_src.as_deref(),
                Some("Hình ảnh Zalo" | "Video Zalo")
            )
        {
            self.link_src = incoming.link_src.clone().or_else(|| self.link_src.clone());
        }
        self.hydrate_media_metadata();
        before
            != (
                self.link_url.clone(),
                self.link_title.clone(),
                self.link_desc.clone(),
                self.link_thumb.clone(),
                self.link_src.clone(),
            )
    }

    /// Compute the static PNG URL for a Zalo sticker (`compact-app-pc.b7bec208c4c8f73e09f8.js:7739583` & `18589292`).
    pub fn sticker_image_url(&self) -> Option<String> {
        let sid = self.sticker_id?;
        if sid == 0 {
            return None;
        }
        if self.sticker_cate_id == Some(0) && (1..=30).contains(&sid) {
            Some(format!(
                "https://stc-chat.zdn.vn/images/stickers/default/thumb/{sid}.png"
            ))
        } else {
            Some(format!(
                "https://api.zaloapp.com/api/emoticon/sticker/webpc?eid={sid}&size=130"
            ))
        }
    }

    pub fn sticker_cache_key(&self) -> Option<String> {
        let sid = self.sticker_id?;
        if sid == 0 {
            return None;
        }
        let cid = self.sticker_cate_id.unwrap_or(0);
        Some(format!("stk_{cid}_{sid}"))
    }
}

/// Sửa tin đã gửi: Zalo không có endpoint edit-in-place nên ctdozlite
/// recall tin cũ + gửi tin mới. Hạn chế PHẢI nói rõ với user: phía bên kia
/// thấy "tin bị thu hồi" + một tin mới, không phải edit liền mạch như Telegram.
/// Giới hạn 15 phút kể từ `msg.ts` (ms) — giống Zalo Web thu hồi có giới hạn thời gian.
pub const EDIT_WINDOW_SECS: i64 = 15 * 60;

/// `true` khi tin của mình, là text thuần (không sticker/link/file/quote),
/// chưa thu hồi, và còn trong 15 phút.
pub fn can_edit_message(msg: &ChatMessage, now_ms: i64) -> bool {
    if !msg.is_outgoing {
        return false;
    }
    if msg.sticker_id.is_some() {
        return false;
    }
    if msg.link_url.as_deref().is_some_and(|u| !u.is_empty()) {
        return false;
    }
    if msg.reply_to_id.is_some() {
        return false;
    }
    let text = msg.text.trim();
    if text.is_empty()
        || text == "[Tin nhắn đã thu hồi]"
        || text.starts_with("[Đã thu hồi]")
        || text == VANISH_EXPIRED_TEXT
    {
        return false;
    }
    if msg.ts <= 0 {
        return false;
    }
    let age_ms = now_ms.saturating_sub(msg.ts);
    age_ms >= 0 && age_ms <= EDIT_WINDOW_SECS * 1000
}


/// Tin nhắn tự xoá (disappearing messages) — client-side local-only.
// Tham chiếu hành vi Zalo PC (`MSG_VANISH`, TTL picker, `expireOn` cột TTLItem):
// - Chế độ theo hội thoại: bật/tắt + 1 thời gian chung cho mọi tin trong thread.
// - Durations theo config server `chat_privacy_ttl_durations` của Zalo PC:
//   `[36e5, 864e5, 7*864e5, 14*864e5]` = 1 giờ / 1 ngày / 7 ngày / 14 ngày.
// - Zalo thật: server giữ `ttl`/`expireOn` nên phía bên kia vẫn thấy tin hết hạn
//   (khi server xoá). ctdozlite KHÔNG tìm thấy endpoint/cmd nào để gửi TTL lên
//   server trong bundle PC nên chỉ tự ẩn phía mình, phía bên kia vẫn thấy tin.
// Thời điểm bắt đầu (`started_at_sec`, epoch giây) là lúc bật chế độ; mọi tin có
// `ts` (ms, chuẩn hoá về ms) >= `started_at_sec * 1000` đều chịu chế độ tự xoá và
/// hết hạn sau `duration_sec`. Tin cũ hơn lúc bật không bị ảnh hưởng.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct VanishSetting {
    /// Thời gian tự xoá chung của thread, tính bằng giây. `0` = tắt.
    pub duration_sec: i64,
    /// Epoch giây lúc bật chế độ (mốc tính tin nào chịu tự xoá).
    pub started_at_sec: i64,
}

/// Các mốc thời gian Zalo cho phép, theo `chat_privacy_ttl_durations` trong bundle PC.
pub const VANISH_DURATIONS: [(i64, &str); 4] = [
    (3600, "1 giờ"),
    (86_400, "1 ngày"),
    (604_800, "7 ngày"),
    (1_209_600, "14 ngày"),
];

/// Text thay thế khi tin hết hạn — khớp `STR_VANISH_MSG` của Zalo PC.
pub const VANISH_EXPIRED_TEXT: &str = "[Tin nhắn đã biến mất]";

impl VanishSetting {
    pub fn enabled(&self) -> bool {
        self.duration_sec > 0
    }

    /// Tin có timestamp `msg_ts_ms` (đã chuẩn hoá ms) có chịu chế độ tự xoá không.
    pub fn covers(&self, msg_ts_ms: i64) -> bool {
        self.enabled() && msg_ts_ms >= self.started_at_sec.saturating_mul(1000)
    }

    /// Epoch ms lúc tin `msg_ts_ms` hết hạn. `None` khi tắt hoặc tin ngoài chế độ.
    pub fn expire_at_ms(&self, msg_ts_ms: i64) -> Option<i64> {
        if !self.covers(msg_ts_ms) {
            return None;
        }
        Some(msg_ts_ms.saturating_add(self.duration_sec.saturating_mul(1000)))
    }

    /// Số ms còn lại tới khi tin hết hạn tại `now_ms`. `None` khi đã hết hạn / ngoài chế độ.
    pub fn remaining_ms(&self, msg_ts_ms: i64, now_ms: i64) -> Option<i64> {
        self.expire_at_ms(msg_ts_ms)
            .map(|exp| exp.saturating_sub(now_ms))
            .filter(|&r| r > 0)
    }

    /// Tin đã hết hạn tại `now_ms` chưa.
    pub fn is_expired(&self, msg_ts_ms: i64, now_ms: i64) -> bool {
        match self.expire_at_ms(msg_ts_ms) {
            Some(exp) => now_ms >= exp,
            None => false,
        }
    }
}

/// Đọc cấu hình vanish của thread. `None` = tắt.
pub fn vanish_of(thread: &Thread) -> Option<&VanishSetting> {
    thread.vanish.as_ref().filter(|v| v.enabled())
}

/// Đặt chế độ tự xoá cho thread. `duration_sec <= 0` = tắt.
/// Trả về `true` khi cấu hình thực sự đổi.
pub fn set_thread_vanish(thread: &mut Thread, duration_sec: i64, now_sec: i64) -> bool {
    if duration_sec <= 0 {
        let changed = thread.vanish.as_ref().is_some_and(|v| v.enabled());
        thread.vanish = None;
        return changed;
    }
    let changed = thread
        .vanish
        .as_ref()
        .map(|v| (v.duration_sec, v.started_at_sec))
        != Some((duration_sec, now_sec));
    thread.vanish = Some(VanishSetting {
        duration_sec,
        started_at_sec: now_sec,
    });
    changed
}

/// Tin có đang hiển thị dưới dạng đã biến mất không.
pub fn is_vanished_message(msg: &ChatMessage) -> bool {
    msg.text.trim() == VANISH_EXPIRED_TEXT
}

/// Lật tin sang trạng thái đã biến mất: text thành `[Tin nhắn đã biến mất]`,
/// vô hiệu media/link (xoá thumb/url/desc/sticker) để không mở lại được.
/// Giữ `id`/`ts`/`sender` để merge/dedup/history không vỡ.
pub fn apply_vanish_expiry(msg: &mut ChatMessage) -> bool {
    if is_vanished_message(msg) {
        return false;
    }
    msg.text = VANISH_EXPIRED_TEXT.to_string();
    msg.translation = None;
    msg.link_url = None;
    msg.link_title = None;
    msg.link_desc = None;
    msg.link_thumb = None;
    msg.link_src = None;
    msg.sticker_id = None;
    msg.sticker_cate_id = None;
    msg.sticker_type = None;
    msg.oa_card = None;
    msg.media_group = None;
    msg.media_size = None;
    true
}

/// Quét 1 thread, lật mọi tin đã hết hạn tại `now_ms`.
/// Trả về id các tin vừa lật (để persist + báo GUI).
pub fn sweep_thread_vanish(thread: &mut Thread, now_ms: i64) -> Vec<String> {
    let Some(v) = vanish_of(thread) else {
        return Vec::new();
    };
    let duration_ms = v.duration_sec.saturating_mul(1000);
    let start_ms = v.started_at_sec.saturating_mul(1000);
    let mut expired = Vec::new();
    for m in thread.messages.iter_mut() {
        if is_vanished_message(m) {
            continue;
        }
        let ts = crate::clock::normalize_ts_ms(m.ts);
        if ts < start_ms {
            continue;
        }
        if now_ms.saturating_sub(ts) >= duration_ms {
            apply_vanish_expiry(m);
            expired.push(m.id.clone());
        }
    }
    expired
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Thread {
    #[serde(default)]
    pub presentation: SidebarPresentation,
    pub id: String,
    pub name: String,
    pub preview: String,
    pub time: String,
    pub avatar: Option<String>,
    pub is_group: bool,
    pub messages: Vec<ChatMessage>,
    #[serde(default)]
    pub unread: crate::unread::UnreadState,
    /// Chế độ tin nhắn tự xoá của thread (`None` = tắt). Persist SQLite + cache.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vanish: Option<VanishSetting>,
}

/// Durable sidebar metadata, including metadata-only startup snapshots.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SidebarPresentation {
    /// Whether the current user is restricted from posting in this group.
    pub posting_restricted: Option<bool>,
    pub members: Vec<SidebarMember>,
    /// Server totalMember only; never inferred from message senders or avatar samples.
    pub member_count: Option<u64>,
    pub last_message_ts: i64,
    pub last_message_id: String,
    pub last_sender: String,
    pub last_sender_id: String,
    pub last_outgoing: bool,
    pub pinned: bool,
    pub muted: bool,
    pub hidden: bool,
    pub contact_name: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SidebarMember {
    pub id: String,
    pub name: String,
    pub avatar: Option<String>,
    #[serde(default)]
    pub role: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarFontStyle {
    Regular,
    Semibold,
}

impl Thread {
    pub fn sidebar_font_style(&self) -> SidebarFontStyle {
        if self.unread.has_badge() {
            SidebarFontStyle::Semibold
        } else {
            SidebarFontStyle::Regular
        }
    }

    pub fn refresh_sidebar_presentation(&mut self) {
        if let Some(message) = self
            .messages
            .iter()
            .max_by_key(|m| crate::clock::normalize_ts_ms(m.ts))
        {
            let ts = crate::clock::normalize_ts_ms(message.ts);
            if ts >= crate::clock::normalize_ts_ms(self.presentation.last_message_ts) {
                let sender_id = sidebar_sender_id(message)
                    .or_else(|| {
                        (message.id == self.presentation.last_message_id
                            && ts
                                == crate::clock::normalize_ts_ms(self.presentation.last_message_ts))
                        .then(|| self.presentation.last_sender_id.clone())
                    })
                    .unwrap_or_default();
                let keep_resolved = message.id == self.presentation.last_message_id
                    && ts == crate::clock::normalize_ts_ms(self.presentation.last_message_ts)
                    && sender_id == self.presentation.last_sender_id
                    && !self.presentation.last_sender.is_empty()
                    && !self
                        .presentation
                        .last_sender
                        .bytes()
                        .all(|b| b.is_ascii_digit())
                    && message.sender.bytes().all(|b| b.is_ascii_digit());
                self.presentation.last_message_ts = message.ts;
                self.presentation.last_message_id = message.id.clone();
                if !keep_resolved {
                    self.presentation.last_sender = message.sender.clone();
                }
                self.presentation.last_sender_id = sender_id;
                self.presentation.last_outgoing = message.is_outgoing;
            }
        }
    }

    pub fn sidebar_preview(&self) -> String {
        let latest = self
            .messages
            .iter()
            .max_by_key(|m| crate::clock::normalize_ts_ms(m.ts))
            .filter(|m| {
                crate::clock::normalize_ts_ms(m.ts)
                    > crate::clock::normalize_ts_ms(self.presentation.last_message_ts)
                    || self.presentation.last_sender.is_empty()
            });
        let (sender, outgoing) = latest
            .map(|m| (m.sender.as_str(), m.is_outgoing))
            .unwrap_or((
                &self.presentation.last_sender,
                self.presentation.last_outgoing,
            ));
        let sender = self
            .presentation
            .members
            .iter()
            .find(|member| member.id == sender && !member.name.trim().is_empty())
            .map(|member| member.name.as_str())
            .unwrap_or(sender);
        let text = self.preview.replace('\n', " ");
        if text.is_empty() {
            return text;
        }
        if outgoing {
            if text.starts_with("Bạn: ") {
                text
            } else {
                format!("Bạn: {text}")
            }
        } else if self.is_group
            && !sender.trim().is_empty()
            && !sender.trim().chars().all(|c| c.is_ascii_digit())
        {
            let prefix = format!("{}: ", sender.trim());
            if text.starts_with(&prefix) {
                text
            } else {
                format!("{prefix}{text}")
            }
        } else {
            text
        }
    }

    pub fn sidebar_activity_ts(&self) -> i64 {
        self.messages
            .iter()
            .map(|m| crate::clock::normalize_ts_ms(m.ts))
            .chain(std::iter::once(crate::clock::normalize_ts_ms(
                self.presentation.last_message_ts,
            )))
            .max()
            .unwrap_or(0)
    }

    pub fn sidebar_timestamp(&self, now_ms: i64) -> String {
        let ts = self.sidebar_activity_ts();
        if ts > 0 {
            crate::clock::sidebar_time_at(ts, now_ms)
        } else {
            self.time.clone()
        }
    }

    /// Parse only group roster metadata observed in the original getmg-v2 consumer.
    pub fn apply_group_sidebar_metadata(&mut self, info: &Value) {
        if let Some(count) = info
            .get("totalMember")
            .and_then(|v| v.as_u64().or_else(|| v.as_str()?.parse().ok()))
        {
            self.presentation.member_count = Some(count);
        }
        let mut ids = Vec::new();
        if let Some(list) = info.get("memVerList").and_then(Value::as_array) {
            for item in list.iter().filter_map(Value::as_str) {
                let id = item.split('_').next().unwrap_or_default();
                if !id.is_empty() && !ids.iter().any(|s| s == id) {
                    ids.push(id.to_owned());
                }
            }
        } else if let Some(map) = info.get("memberVerMap").and_then(Value::as_object) {
            ids.extend(map.keys().cloned());
        }
        let mut leaders = Vec::new();
        if let Some(creator) = pick_str(info, &["creatorId"]) {
            leaders.push(creator);
        }
        if let Some(admins) = info.get("adminIds").and_then(Value::as_array) {
            leaders.extend(admins.iter().filter_map(Value::as_str).map(str::to_owned));
        }
        let mut ordered = Vec::new();
        for id in leaders.into_iter().chain(ids.iter().cloned()) {
            if ids.contains(&id) && !ordered.contains(&id) {
                ordered.push(id);
            }
        }
        ids = ordered;
        let top = info.get("topMember").and_then(Value::as_array);
        if let Some(top) = top {
            for profile in top {
                if let Some(id) = pick_str(profile, &["id", "userId", "uid"]) {
                    if !ids.contains(&id) {
                        ids.push(id);
                    }
                }
            }
        }
        if !ids.is_empty() {
            let previous = std::mem::take(&mut self.presentation.members);
            self.presentation.members =
                ids.into_iter()
                    .map(|id| {
                        let profile = top.and_then(|list| {
                            list.iter().find(|p| {
                                pick_str(p, &["id", "userId", "uid"]).as_ref() == Some(&id)
                            })
                        });
                        let mut member = previous.iter().find(|m| m.id == id).cloned().unwrap_or(
                            SidebarMember {
                                id,
                                ..Default::default()
                            },
                        );
                        if let Some(profile) = profile {
                            if let Some(name) =
                                pick_str(profile, &["displayName", "zaloName", "name"])
                            {
                                member.name = name;
                            }
                            if let Some(avatar) = pick_str(profile, &["avatar", "avt"]) {
                                member.avatar = Some(avatar);
                            }
                        }
                        member
                    })
                    .collect();
        }
    }
}

pub fn sidebar_sender_id(message: &ChatMessage) -> Option<String> {
    let raw = message.wire_ref.sender.as_ref().or_else(|| {
        message
            .history_source
            .as_ref()
            .and_then(|v| v.get("senderId"))
    })?;
    let id = match raw {
        Value::String(v) => v.clone(),
        Value::Number(v) if v.is_u64() => v.to_string(),
        _ => return None,
    };
    (!id.is_empty() && id != "0" && id.bytes().all(|b| b.is_ascii_digit())).then_some(id)
}

/// Return `true` if a thread name is empty or a generic placeholder (`"Người dùng ..."`, `"Nhóm ..."`).
pub fn is_generic_thread_name(name: &str) -> bool {
    let t = name.trim();
    t.is_empty()
        || t.starts_with("Người dùng ")
        || t.starts_with("Nhóm ")
        || t.chars().all(|c| c.is_ascii_digit())
}

/// Return `true` if `name` is empty or a generated placeholder for `thread_id`
/// (`"Người dùng {id}"`, `"Nhóm {id}"`, or bare `{id}`), while preserving genuine
/// contact/group aliases like `"123"` or `"Nhóm học"` (#126).
pub fn is_placeholder_thread_name(name: &str, thread_id: &str) -> bool {
    let t = name.trim();
    if t.is_empty() {
        return true;
    }
    let tid = thread_id.trim();
    let clean_tid = tid
        .trim_start_matches("group_")
        .trim_start_matches('g')
        .trim_start_matches('G');
    if !tid.is_empty() && (t == tid || (!clean_tid.is_empty() && t == clean_tid)) {
        return true;
    }
    if let Some(rest) = t.strip_prefix("Người dùng ").map(str::trim) {
        return rest.is_empty()
            || (!tid.is_empty() && (rest == tid || rest == clean_tid))
            || rest.chars().all(|c| c.is_ascii_digit());
    }
    if let Some(rest) = t.strip_prefix("Nhóm ").map(str::trim) {
        return rest.is_empty()
            || (!tid.is_empty() && (rest == tid || rest == clean_tid))
            || rest.chars().all(|c| c.is_ascii_digit());
    }
    false
}

/// Extract the stable avatar filename (e.g. `"00000000000000000000000000000001.jpg"`) from a Zalo CDN URL.
pub fn avatar_file_key(avatar: Option<&str>) -> Option<String> {
    let url = avatar?.trim();
    if url.is_empty() {
        return None;
    }
    let path = url.split('?').next()?;
    let fname = path.rsplit('/').next()?.trim();
    if fname.len() >= 12
        && (fname.ends_with(".jpg") || fname.ends_with(".png") || fname.ends_with(".webp"))
    {
        Some(fname.to_ascii_lowercase())
    } else {
        None
    }
}

/// Extract the `?time=...` expiry timestamp from a Zalo avatar URL so newer session avatars win.
pub fn avatar_url_time(avatar: Option<&str>) -> i64 {
    let Some(url) = avatar else {
        return 0;
    };
    let mut max_ts = 0i64;
    for seg in url.split(';') {
        let Some(query) = seg.split('?').nth(1) else {
            continue;
        };
        for pair in query.split('&') {
            if let Some(val) = pair.strip_prefix("time=") {
                let digits: String = val.chars().take_while(|c| c.is_ascii_digit()).collect();
                if let Ok(ts) = digits.parse::<i64>() {
                    if ts > max_ts {
                        max_ts = ts;
                    }
                }
            }
        }
    }
    max_ts
}

/// Count how many real `http(s)://` image URLs (excluding `/default`) an avatar string contains.
pub fn avatar_http_url_count(avatar: Option<&str>) -> usize {
    let Some(raw) = avatar.map(str::trim).filter(|s| !s.is_empty()) else {
        return 0;
    };
    if let Some(rest) = raw.strip_prefix("multi:") {
        rest.split(';')
            .filter_map(|seg| seg.split_once('|').map(|(_, u)| u.trim()))
            .filter(|u| {
                (u.starts_with("http://") || u.starts_with("https://")) && !u.ends_with("/default")
            })
            .count()
    } else if (raw.starts_with("http://") || raw.starts_with("https://"))
        && !raw.ends_with("/default")
    {
        1
    } else {
        0
    }
}

/// Normalize an avatar string by extracting stable image filenames (`avatar_file_key`) so `s120`
/// vs `s160` CDN subdomains and rotating `?key=...&time=...` tokens compare equal.
fn normalize_avatar_signature(avatar: &str) -> String {
    let raw = avatar.trim();
    if let Some(rest) = raw.strip_prefix("multi:") {
        let mut out = String::from("multi:");
        for (idx, seg) in rest.split(';').enumerate() {
            if idx > 0 {
                out.push(';');
            }
            if let Some((name, url)) = seg.split_once('|') {
                let u_trim = url.trim();
                let file_key = avatar_file_key(Some(u_trim))
                    .unwrap_or_else(|| u_trim.split('?').next().unwrap_or("").to_string());
                out.push_str(name.trim());
                out.push('|');
                out.push_str(&file_key);
            } else {
                out.push_str(seg.trim());
            }
        }
        out
    } else {
        avatar_file_key(Some(raw))
            .unwrap_or_else(|| raw.split('?').next().unwrap_or(raw).to_string())
    }
}

/// Decide whether `incoming` should replace `current` avatar without downgrading a rich
/// `multi:` composite avatar (with member photo URLs) or flipping between `s120` and `s160` URLs.
pub fn should_replace_avatar(current: Option<&str>, incoming: Option<&str>) -> bool {
    let Some(inc) = incoming.map(str::trim).filter(|s| !s.is_empty()) else {
        return false;
    };
    let Some(cur) = current.map(str::trim).filter(|s| !s.is_empty()) else {
        return true;
    };
    let cur_urls = avatar_http_url_count(Some(cur));
    let inc_urls = avatar_http_url_count(Some(inc));
    if inc_urls < cur_urls {
        return false;
    }
    if inc_urls > cur_urls {
        return true;
    }
    if cur.starts_with("multi:") && inc.starts_with("multi:") && cur_urls > 0 {
        return false;
    }
    if normalize_avatar_signature(cur) == normalize_avatar_signature(inc) {
        return false;
    }
    avatar_url_time(Some(inc)) > avatar_url_time(Some(cur))
}

/// Return `true` if two `Thread` entries represent the same underlying contact or group,
/// even if Zalo issued a new session-scoped `userId` after a QR re-login.
///
/// Two threads with distinct non-empty IDs are NEVER treated as the same contact solely
/// because their display names match (#63 — multiple Zalo contacts/groups frequently share
/// the same display name). Across different IDs, only the singleton "Cloud của tôi" thread
/// or contacts sharing the same non-default CDN avatar hash (`avatar_file_key`) and name match.
pub fn threads_represent_same_contact(a: &Thread, b: &Thread) -> bool {
    if !a.id.is_empty() && a.id == b.id {
        return true;
    }
    if a.is_group != b.is_group {
        return false;
    }
    if !a.is_group && a.name.contains("Cloud của tôi") && b.name.contains("Cloud của tôi") {
        return true;
    }
    if let (Some(ka), Some(kb)) = (
        avatar_file_key(a.avatar.as_deref()),
        avatar_file_key(b.avatar.as_deref()),
    ) {
        if !ka.contains("default") && ka == kb {
            return is_generic_thread_name(&a.name)
                || is_generic_thread_name(&b.name)
                || a.name.trim() == b.name.trim();
        }
        return false;
    }
    false
}

fn wire_identity(value: Option<&Value>) -> Option<String> {
    match value? {
        Value::String(value) if !value.trim().is_empty() && value != "0" => Some(value.clone()),
        Value::Number(value) => value.as_u64().filter(|id| *id > 0).map(|id| id.to_string()),
        _ => None,
    }
}

fn wire_sender(value: Option<&Value>) -> Option<String> {
    match value? {
        Value::String(value) if !value.is_empty() => Some(value.clone()),
        Value::Number(value) => value.as_u64().map(|id| id.to_string()),
        _ => None,
    }
}

fn server_message_id(message: &ChatMessage) -> Option<String> {
    let wire_server = wire_identity(message.wire_ref.server_id.as_ref());
    let wire_client = wire_identity(message.wire_ref.client_id.as_ref());
    if message.id.starts_with("out_")
        || message.id.is_empty()
        || wire_client.as_deref() == Some(message.id.as_str())
    {
        wire_server
    } else {
        wire_identity(Some(&Value::String(message.id.clone())))
    }
}

fn client_message_id(message: &ChatMessage) -> Option<String> {
    let wire = wire_identity(message.wire_ref.client_id.as_ref());
    if message.client_message_id.trim().is_empty() || message.client_message_id == "0" {
        wire
    } else if wire
        .as_ref()
        .is_some_and(|id| id != &message.client_message_id)
    {
        None
    } else {
        Some(message.client_message_id.clone())
    }
}

#[derive(Default)]
struct MessageIdentityIndex {
    rows: std::collections::HashMap<(u8, String), std::collections::HashSet<usize>>,
}

impl MessageIdentityIndex {
    fn keys(message: &ChatMessage) -> Vec<(u8, String)> {
        let mut keys = Vec::with_capacity(3);
        if let Some(id) = server_message_id(message) {
            keys.push((0, id));
        } else if message.id.starts_with("out_") {
            keys.push((1, message.id.clone()));
        }
        if !message.id.starts_with("out_") {
            if let Some(id) = wire_identity(Some(&Value::String(message.id.clone()))) {
                // Old caches only retain this ID; it may have come from cliMsgId.
                keys.push((3, id));
            }
        }
        if let Some(id) = client_message_id(message) {
            keys.push((2, id));
        }
        keys
    }

    fn insert(&mut self, message: &ChatMessage, index: usize) {
        for key in Self::keys(message) {
            self.rows.entry(key).or_default().insert(index);
        }
    }

    fn remove(&mut self, message: &ChatMessage, index: usize) {
        for key in Self::keys(message) {
            if let Some(rows) = self.rows.get_mut(&key) {
                rows.remove(&index);
            }
        }
    }

    fn find(&self, messages: &[ChatMessage], incoming: &ChatMessage) -> Option<usize> {
        for key in Self::keys(incoming) {
            let Some(rows) = self.rows.get(&key).filter(|rows| !rows.is_empty()) else {
                continue;
            };
            // A colliding identity is not evidence for selecting any particular row.
            if rows.len() != 1 {
                return None;
            }
            let index = *rows.iter().next()?;
            let existing = &messages[index];
            if existing.is_outgoing != incoming.is_outgoing {
                return None;
            }
            if let (Some(a), Some(b)) = (
                wire_sender(existing.wire_ref.sender.as_ref()),
                wire_sender(incoming.wire_ref.sender.as_ref()),
            ) {
                if a != b {
                    return None;
                }
            }
            let is_recall = incoming.text.trim() == "[Tin nhắn đã thu hồi]"
                || existing.text.trim() == "[Tin nhắn đã thu hồi]"
                || incoming.text.starts_with("[Đã thu hồi]")
                || existing.text.starts_with("[Đã thu hồi]");
            if key.0 != 2 {
                if key.0 == 3 && !is_recall {
                    if let (Some(a), Some(b)) = (
                        wire_identity(existing.wire_ref.server_id.as_ref()),
                        wire_identity(incoming.wire_ref.server_id.as_ref()),
                    ) {
                        if a != b {
                            return None;
                        }
                    }
                }
                return Some(index);
            }
            if is_recall {
                return Some(index);
            }
            // Client-only packets can replay before a server ID is known. A client
            // reference never collapses two distinct pending or server messages.
            let existing_server = server_message_id(existing);
            let incoming_server = server_message_id(incoming);
            let existing_local = existing.id.starts_with("out_") && existing_server.is_none();
            let incoming_local = incoming.id.starts_with("out_") && incoming_server.is_none();
            let client_only_replay = existing_server.is_none()
                && incoming_server.is_none()
                && (existing.id.is_empty() || incoming.id.is_empty());
            return (client_only_replay
                || (existing.is_outgoing
                    && ((existing_local && incoming_server.is_some())
                        || (incoming_local && existing_server.is_some()))))
            .then_some(index);
        }
        None
    }
}

fn delivery_rank(state: crate::message_model::DeliveryState) -> u8 {
    use crate::message_model::DeliveryState;
    match state {
        DeliveryState::Unknown | DeliveryState::DeliveryUnknown => 0,
        DeliveryState::Failed => 1,
        DeliveryState::Pending => 2,
        DeliveryState::ServerAccepted => 3,
        DeliveryState::Delivered => 4,
        DeliveryState::Read => 5,
    }
}

fn merge_delivery_state(
    existing: &mut crate::message_model::DeliveryState,
    incoming: crate::message_model::DeliveryState,
) {
    if delivery_rank(incoming) > delivery_rank(*existing) {
        *existing = incoming;
    }
}

pub(crate) fn apply_recall_merge(existing: &mut ChatMessage, incoming_text: &str) {
    let inc_trim = incoming_text.trim();
    let ex_trim = existing.text.trim();
    if existing.is_outgoing
        && (inc_trim == "[Tin nhắn đã thu hồi]"
            || inc_trim.starts_with("[Đã thu hồi]")
            || ex_trim == "[Tin nhắn đã thu hồi]"
            || ex_trim.starts_with("[Đã thu hồi]"))
    {
        existing.text = "[Tin nhắn đã thu hồi]".to_string();
        existing.kind = crate::message_model::MessageKind::Text;
        existing.link_url = None;
        existing.link_thumb = None;
        existing.link_title = None;
        existing.link_desc = None;
        existing.link_src = None;
        existing.sticker_id = None;
        existing.sticker_cate_id = None;
        existing.sticker_type = None;
        existing.oa_card = None;
        existing.media_group = None;
        return;
    }
    if inc_trim == "[Tin nhắn đã thu hồi]" {
        if ex_trim.is_empty() || ex_trim == "[Tin nhắn đã thu hồi]" {
            existing.text = "[Tin nhắn đã thu hồi]".to_string();
        } else if !ex_trim.starts_with("[Đã thu hồi]") {
            existing.text = format!("[Đã thu hồi] {}", existing.text);
        }
    } else if inc_trim.starts_with("[Đã thu hồi]") {
        let inc_body = inc_trim.strip_prefix("[Đã thu hồi]").unwrap_or("").trim();
        if ex_trim == "[Tin nhắn đã thu hồi]" || ex_trim.is_empty() {
            if !inc_body.is_empty() {
                existing.text = incoming_text.to_string();
            } else if ex_trim.is_empty() {
                existing.text = "[Tin nhắn đã thu hồi]".to_string();
            }
        } else if !ex_trim.starts_with("[Đã thu hồi]") {
            if !inc_body.is_empty() && ex_trim.is_empty() {
                existing.text = incoming_text.to_string();
            } else {
                existing.text = format!("[Đã thu hồi] {}", existing.text);
            }
        }
    } else if ex_trim == "[Tin nhắn đã thu hồi]" && !inc_trim.is_empty() {
        existing.text = format!("[Đã thu hồi] {incoming_text}");
    }
}

/// Merge a local send acknowledgement without replacing an existing server clock.
pub(crate) fn merge_sent_message(thread: &mut Thread, mut message: ChatMessage) {
    if message.is_outgoing
        && matches!(
            message.delivery,
            crate::message_model::DeliveryState::Unknown
                | crate::message_model::DeliveryState::DeliveryUnknown
                | crate::message_model::DeliveryState::Pending
        )
    {
        message.delivery = crate::message_model::DeliveryState::ServerAccepted;
    }
    let mut identities = MessageIdentityIndex::default();
    for (row, existing) in thread.messages.iter().enumerate() {
        identities.insert(existing, row);
    }
    if identities
        .find(&thread.messages, &message)
        .is_some_and(|row| server_message_id(&thread.messages[row]).is_some())
    {
        // The acknowledgement receipt time is not the server's message timestamp.
        message.ts = 0;
        message.time.clear();
    }
    merge_thread_into(
        thread,
        Thread {
            messages: vec![message],
            ..Default::default()
        },
        false,
    );
}

/// Update `target.name` from an incoming thread/friend entry, giving precedence to
/// contact aliases (`is_contact_name == true`) and preventing non-contact or placeholder
/// names from overwriting contact aliases or real names (#125, #126).
pub fn adopt_thread_name(target: &mut Thread, name: String, is_contact_name: bool) {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return;
    }
    if is_contact_name {
        target.name = name;
        target.presentation.contact_name = true;
        return;
    }
    if target.presentation.contact_name {
        return;
    }
    if !is_placeholder_thread_name(trimmed, &target.id)
        && (is_placeholder_thread_name(&target.name, &target.id)
            || (is_generic_thread_name(&target.name) && !is_generic_thread_name(trimmed)))
    {
        target.name = name;
    }
}

/// Merge `incoming` thread into `target`. When `incoming_id_is_authoritative` is `true`,
/// `target.id` is migrated to `incoming.id` so actions (chat/VoIP call) always target the active session's `userId`.
pub fn merge_thread_into(
    target: &mut Thread,
    incoming: Thread,
    incoming_id_is_authoritative: bool,
) {
    if incoming.presentation.posting_restricted.is_some() {
        target.presentation.posting_restricted = incoming.presentation.posting_restricted;
    }
    if incoming_id_is_authoritative && !incoming.id.is_empty() && incoming.id != "0" {
        target.id = incoming.id;
    }
    adopt_thread_name(target, incoming.name, incoming.presentation.contact_name);
    if should_replace_avatar(target.avatar.as_deref(), incoming.avatar.as_deref()) {
        target.avatar = incoming.avatar;
    }
    target.is_group = target.is_group || incoming.is_group;
    // Vanish là local-only: payload sync/poll không mang policy. Nếu thread
    // RAM chưa có mà incoming có (vd. hydrate từ DB), nhận lấy; không bao giờ
    // để merge history ghi đè/xoá policy đang bật trong RAM (#376).
    if target.vanish.as_ref().is_none_or(|v| !v.enabled())
        && incoming.vanish.as_ref().is_some_and(|v| v.enabled())
    {
        target.vanish = incoming.vanish.clone();
    }
    target.presentation.pinned = target.presentation.pinned || incoming.presentation.pinned;
    target.presentation.muted = target.presentation.muted || incoming.presentation.muted;
    target.presentation.hidden = target.presentation.hidden || incoming.presentation.hidden;
    target.unread.merge_metadata(&incoming.unread);
    if let Some(ref point) = incoming.unread.last_seen_id {
        target.unread.advance_seen(point);
    }

    // Repeated text and timestamps are not identities. Keep ambiguous keys instead of
    // choosing the first candidate, especially when multiple pending sends collide.
    let mut identities = MessageIdentityIndex::default();
    for (index, message) in target.messages.iter().enumerate() {
        identities.insert(message, index);
    }

    for msg in incoming.messages {
        if msg.text == "sendBubbleMessage" {
            continue;
        }
        let found = identities.find(&target.messages, &msg);
        if let Some(idx) = found {
            let existing_msg = &mut target.messages[idx];
            identities.remove(existing_msg, idx);
            existing_msg.merge_reaction_metadata(&msg);
            merge_delivery_state(&mut existing_msg.delivery, msg.delivery);
            merge_reply_metadata(existing_msg, &msg);
            if existing_msg.id.starts_with("out_")
                && !msg.id.is_empty()
                && !msg.id.starts_with("out_")
            {
                existing_msg.id = msg.id.clone();
            }
            let is_undo_msg = msg.text.trim() == "[Tin nhắn đã thu hồi]";
            if msg.ts > 0 && existing_msg.ts != msg.ts && (!is_undo_msg || existing_msg.ts == 0) {
                existing_msg.ts = msg.ts;
                existing_msg.time = msg.time;
            }
            apply_recall_merge(existing_msg, &msg.text);
            // Anti-recall: Retain sticker, link_url, link_thumb so user can still view recalled photos & media!
            if existing_msg.sticker_id.is_none() && msg.sticker_id.is_some() {
                existing_msg.sticker_id = msg.sticker_id;
                existing_msg.sticker_cate_id = msg.sticker_cate_id;
                existing_msg.sticker_type = msg.sticker_type;
            }
            if existing_msg.link_url.is_none() && msg.link_url.is_some() {
                existing_msg.link_url = msg.link_url;
                existing_msg.link_title = msg.link_title;
                existing_msg.link_desc = msg.link_desc;
                existing_msg.link_thumb = msg.link_thumb;
                existing_msg.link_src = msg.link_src;
                if !msg.text.is_empty()
                    && !existing_msg.text.starts_with("[Đã thu hồi]")
                    && existing_msg.text.trim() != "[Tin nhắn đã thu hồi]"
                {
                    existing_msg.text = msg.text;
                }
            }
            if existing_msg.kind == crate::message_model::MessageKind::Text
                && msg.kind != crate::message_model::MessageKind::Text
                && msg.kind != crate::message_model::MessageKind::Recall
            {
                existing_msg.kind = msg.kind;
            }
            if existing_msg.history_source.is_none() && msg.history_source.is_some() {
                existing_msg.history_source = msg.history_source;
            }
            if existing_msg.reactions.is_empty() && !msg.reactions.is_empty() {
                existing_msg.reactions = msg.reactions;
            }
            existing_msg.is_pinned = existing_msg.is_pinned || msg.is_pinned;
            existing_msg.is_starred = existing_msg.is_starred || msg.is_starred;
            identities.insert(existing_msg, idx);
        } else {
            let idx = target.messages.len();
            identities.insert(&msg, idx);
            target.messages.push(msg);
        }
    }

    target.messages.retain(|m| m.text != "sendBubbleMessage");
    target.messages.sort_by_key(|m| m.ts);

    let mut latest_incoming_ts = 0i64;
    for m in target.messages.iter() {
        if !m.is_outgoing && m.ts > latest_incoming_ts {
            latest_incoming_ts = m.ts;
        }
    }
    if latest_incoming_ts > 0 {
        for m in target.messages.iter_mut() {
            if m.is_outgoing && m.ts > 0 && m.ts <= latest_incoming_ts {
                m.delivery = crate::message_model::DeliveryState::Read;
            }
        }
    }

    repair_thread_quote_replies(&mut target.messages);
    if let Some(newest) = target.messages.last() {
        if !newest.text.is_empty() {
            target.preview = newest.text.replace('\n', " ");
            target.time = newest.time.clone();
        }
    } else if !incoming.preview.is_empty() && target.preview.is_empty() {
        target.preview = incoming.preview;
        if !incoming.time.is_empty() {
            target.time = incoming.time;
        }
    } else if target.time.is_empty() && !incoming.time.is_empty() {
        target.time = incoming.time;
    }
}

/// Deduplicate `threads` in-place, merging entries that represent the same contact/group
/// (by `id`, avatar hash, or non-generic display name) and preserving the authoritative/newer `id`.
pub fn deduplicate_threads(
    threads: &mut Vec<Thread>,
    authoritative_ids: Option<&std::collections::HashSet<String>>,
) {
    let mut merged: Vec<Thread> = Vec::with_capacity(threads.len());
    for mut t in threads.drain(..) {
        if t.id.is_empty() || t.id == "0" {
            continue;
        }
        t.messages.retain(|m| m.text != "sendBubbleMessage");
        if let Some(newest) = t.messages.last() {
            if !newest.text.is_empty() && (t.preview.is_empty() || t.preview == "sendBubbleMessage")
            {
                t.preview = newest.text.replace('\n', " ");
                t.time = newest.time.clone();
            }
        }
        if let Some(existing) = merged.iter_mut().find(|e| {
            if e.id == t.id {
                return true;
            }
            if let Some(auth) = authoritative_ids {
                if auth.contains(&e.id) && auth.contains(&t.id) {
                    return false;
                }
            }
            threads_represent_same_contact(e, &t)
        }) {
            let incoming_auth = if let Some(auth) = authoritative_ids {
                auth.contains(&t.id) && !auth.contains(&existing.id)
            } else {
                // In cache order, later entries or entries with newer avatar URL timestamps come from newer sessions
                let t_time = avatar_url_time(t.avatar.as_deref());
                let e_time = avatar_url_time(existing.avatar.as_deref());
                if t_time != e_time {
                    t_time > e_time
                } else {
                    true
                }
            };
            merge_thread_into(existing, t, incoming_auth);
        } else {
            merged.push(t);
        }
    }
    let cloud_msg_ids: std::collections::HashSet<String> = merged
        .iter()
        .filter(|t| t.name.contains("Cloud của tôi"))
        .flat_map(|t| t.messages.iter())
        .filter_map(|m| {
            if !m.id.is_empty() && !m.id.starts_with("out_") && !m.id.starts_with("local_") {
                Some(m.id.clone())
            } else {
                None
            }
        })
        .collect();
    if !cloud_msg_ids.is_empty() {
        for t in merged.iter_mut() {
            if !t.name.contains("Cloud của tôi") {
                let before = t.messages.len();
                t.messages.retain(|m| !cloud_msg_ids.contains(&m.id));
                if t.messages.len() != before {
                    if let Some(newest) = t.messages.last() {
                        t.preview = newest.text.replace('\n', " ");
                        t.time = newest.time.clone();
                    } else {
                        t.preview.clear();
                        t.time.clear();
                    }
                }
            }
        }
    }
    for t in merged.iter_mut() {
        repair_thread_quote_replies(&mut t.messages);
    }
    *threads = merged;
}

fn active_uin_from_session() -> Option<String> {
    if let Ok(text) = std::fs::read_to_string("session.json") {
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(uin) = val.pointer("/login/uin").and_then(|v| v.as_str()) {
                if !uin.is_empty() {
                    return Some(uin.to_string());
                }
            }
        }
    }
    None
}

fn active_uin_resolved() -> Option<String> {
    let mem_uin = crate::account_manager::active_uin();
    if crate::account_manager::is_safe_uin(&mem_uin) {
        return Some(mem_uin);
    }
    active_uin_from_session().filter(|u| crate::account_manager::is_safe_uin(u))
}

/// Load locally cached conversations scoped strictly to `uin` (`accounts/<uin>_threads.json`),
/// falling back to legacy `threads_cache.json` ONLY when `uin` is empty/unassigned.
pub fn load_threads_cache_for(uin: &str) -> Vec<Thread> {
    if crate::account_manager::is_safe_uin(uin) {
        let acc_path = crate::account_manager::threads_snapshot_path(uin);
        if let Ok(text) = std::fs::read_to_string(&acc_path) {
            if let Ok(mut threads) = serde_json::from_str::<Vec<Thread>>(&text) {
                deduplicate_threads(&mut threads, None);
                repair_poll_notices(&mut threads);
                return threads;
            }
        }
        return Vec::new();
    }
    if std::path::Path::new("session.json").exists() {
        if let Ok(text) = std::fs::read_to_string("threads_cache.json") {
            if let Ok(mut threads) = serde_json::from_str::<Vec<Thread>>(&text) {
                deduplicate_threads(&mut threads, None);
                repair_poll_notices(&mut threads);
                return threads;
            }
        }
    }
    Vec::new()
}

/// Notice text for a `group.poll` event, from the localized template Zalo sends in
/// `content.params.msg.vi` (`%1$s` = member name, `%2$s` = poll question). Without a
/// template it falls back to a generic Vietnamese line. `None` if params are unreadable.
pub(crate) fn poll_notice_text(content: &Value, actor: &str) -> Option<String> {
    let parse_json = |v: &Value| -> Option<Value> {
        match v {
            Value::String(s) if s.trim_start().starts_with('{') => serde_json::from_str(s).ok(),
            Value::Object(_) => Some(v.clone()),
            _ => None,
        }
    };
    let params = parse_json(content)?.get("params").and_then(parse_json)?;
    let name = params
        .get("dName")
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(actor)
        .trim();
    let name = if name.is_empty() {
        "Thành viên"
    } else {
        name
    };
    let question = params
        .get("question")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .trim();
    let template = params
        .get("msg")
        .and_then(|m| m.get("vi"))
        .and_then(Value::as_str)
        .filter(|t| t.contains("%1$s"));
    Some(match template {
        Some(t) => t.replace("%1$s", name).replace("%2$s", question),
        None if question.is_empty() => format!("{name} đã cập nhật một cuộc bình chọn"),
        None => format!("{name} đã cập nhật cuộc bình chọn: {question}"),
    })
}

/// Cached threads saved before poll events were parsed hold them as "[Hình ảnh]" text.
/// Rebuild the notice from the raw wire content kept in `wire_ref`.
fn repair_poll_notices(threads: &mut [Thread]) {
    for msg in threads.iter_mut().flat_map(|t| t.messages.iter_mut()) {
        let is_poll =
            msg.wire_ref.original_type.as_ref().and_then(Value::as_str) == Some("group.poll");
        if !is_poll {
            continue;
        }
        let actor = msg
            .wire_ref
            .fields
            .get("dName")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        if let Some(notice) = msg
            .wire_ref
            .fields
            .get("content")
            .and_then(|c| poll_notice_text(c, &actor))
        {
            msg.text = notice;
            msg.kind = crate::message_model::MessageKind::System;
        }
    }
}

/// Load locally cached conversations and message history scoped to active account.
pub fn load_threads_cache() -> Vec<Thread> {
    if let Some(uin) = active_uin_resolved() {
        return load_threads_cache_for(&uin);
    }
    if std::path::Path::new("session.json").exists() {
        return load_threads_cache_for("");
    }
    Vec::new()
}

/// Persist thread-list snapshot strictly to `accounts/<uin>_threads.json` when `uin` is known
/// (never touching a shared `threads_cache.json` across accounts).
pub fn save_threads_cache_for(uin: &str, threads: &[Thread]) {
    let keep = crate::store::MessageStore::PRELOAD_MSGS;
    let slim: Vec<Thread> = threads
        .iter()
        .map(|t| Thread {
            presentation: t.presentation.clone(),
            unread: t.unread.clone(),
            id: t.id.clone(),
            name: t.name.clone(),
            preview: t.preview.clone(),
            time: t.time.clone(),
            avatar: t.avatar.clone(),
            is_group: t.is_group,
            messages: t.messages[t.messages.len().saturating_sub(keep)..].to_vec(),
            vanish: t.vanish.clone(),
        })
        .collect();
    if let Ok(text) = serde_json::to_string(&slim) {
        if crate::account_manager::is_safe_uin(uin) {
            let _ = std::fs::create_dir_all("accounts");
            let path = crate::account_manager::threads_snapshot_path(uin);
            let _ = crate::data_dir::write_private_atomic(&path, text.as_bytes());
            let _ = std::fs::remove_file("threads_cache.json");
        } else {
            let _ = std::fs::write("threads_cache.json", &text);
        }
    }
}

/// Snapshot of the thread list for an instant first paint. Only the newest
/// `PRELOAD_MSGS` messages per thread are kept: full history lives in SQLite, and
/// rewriting every message as pretty JSON on each change cost more than the SQLite write.
pub fn save_threads_cache(threads: &[Thread]) {
    let uin = active_uin_resolved().unwrap_or_default();
    save_threads_cache_for(&uin, threads);
}

/// `/api/social/friend/getfriends` — the friend list.
///
/// Uses `s.domains.friend` (defaulting to `tt-friend-wpa.chat.zalo.me`), falling back
/// to `s.domains.profile` only if `domains.friend` is empty.
pub async fn get_friends(s: &Session) -> Result<Vec<Thread>> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let body = obj(&[
        ("incInvalid", json!(1)),
        ("page", json!(1)),
        ("count", json!(20000)),
        ("avatar_size", json!(160)),
        ("actiontime", json!(0)),
        ("imei", json!(s.imei)),
    ]);
    let params = match s.encoded_params(&body) {
        Ok(p) => format!("params={p}"),
        Err(e) => {
            crate::logger::log_error("chat::get_friends::encoded_params", &e.to_string());
            eprintln!("! get_friends: no session key for encoded_params ({e}), falling back to plain_params");
            Session::plain_params(&body)
        }
    };
    let path_and_query = format!(
        "/api/social/friend/getfriends?{}{}",
        s.common_params(),
        if params.is_empty() {
            String::new()
        } else {
            format!("&{params}")
        }
    );
    let raw = s
        .get_endpoint("friend", &path_and_query)
        .await
        .map_err(|e| e.context("không tải được danh sách bạn bè"))?;
    let resp = s.maybe_decrypt(raw, s.session_key_bytes()).await;
    let data_root = resp.get("data").unwrap_or(&resp);
    let mut threads: Vec<Thread> = match data_root {
        Value::Array(items) => items.iter().filter_map(parse_thread).collect(),
        Value::Object(o) if o.contains_key("friends") => o["friends"]
            .as_array()
            .map(|a| a.iter().filter_map(parse_thread).collect())
            .unwrap_or_default(),
        Value::Object(o) if o.get("data").and_then(Value::as_array).is_some() => o["data"]
            .as_array()
            .map(|a| a.iter().filter_map(parse_thread).collect())
            .unwrap_or_default(),
        _ => Vec::new(),
    };

    // If Cloud của tôi (send2me) is available, place it at the top
    if let Some(li) = &s.login {
        if !li.send2me_id.is_empty() && !threads.iter().any(|t| t.id == li.send2me_id) {
            threads.insert(
                0,
                Thread {
                    presentation: Default::default(),
                    unread: Default::default(),
                    id: li.send2me_id.clone(),
                    name: "Cloud của tôi (Lưu trữ)".into(),
                    preview: "".into(),
                    time: crate::ui::clock(),
                    avatar: None,
                    is_group: false,
                    messages: Vec::new(),
                    vanish: None,
                },
            );
        }
    }

    // Hydrate with locally persisted history from SQLite scoped to this account
    let uin = s.login.as_ref().map(|l| l.uin.as_str()).unwrap_or_default();
    let db_path = crate::store::db_path_for_account(uin);
    crate::store::load_history_into_threads(
        &db_path,
        &mut threads,
        crate::store::MessageStore::PRELOAD_MSGS,
    );

    // Merge user groups (`getlg/v4` + `getmg-v2`) into threads list, upgrading any placeholder group names
    if let Ok(mut groups) = get_groups(s).await {
        for g in groups.drain(..) {
            if let Some(existing) = threads.iter_mut().find(|t| t.id == g.id) {
                merge_thread_into(existing, g, true);
            } else {
                threads.push(g);
            }
        }
    }

    resolve_thread_names(s, &mut threads).await;

    Ok(threads)
}

fn parse_thread(v: &Value) -> Option<Thread> {
    let id = pick_str(
        v,
        &["userId", "uid", "id", "friendId", "globalId", "groupId"],
    )?;
    let picked_name = pick_str(v, &["displayName", "zaloName", "display_name", "name"])
        .or_else(|| {
            v.get("profile_displayName")
                .or_else(|| v.get("profile").and_then(|p| p.get("displayName")))
                .and_then(Value::as_str)
                .map(str::to_string)
        })
        .filter(|s| !s.trim().is_empty());
    let contact_name = picked_name.is_some();
    let name = picked_name.unwrap_or_else(|| id.clone());
    let avatar = pick_str(v, &["avatar", "avt", "profile_avatar"]);
    let is_group = is_group_flag(v);
    let time = v
        .get("lastActionTime")
        .and_then(|val| match val {
            Value::Number(n) => n.as_i64(),
            Value::String(s) => s.parse::<i64>().ok(),
            _ => None,
        })
        .map(crate::clock::message_time)
        .unwrap_or_default();

    Some(Thread {
        presentation: SidebarPresentation {
            contact_name,
            ..Default::default()
        },
        unread: Default::default(),
        id,
        name,
        preview: String::new(),
        time,
        avatar,
        is_group,
        messages: Vec::new(),
        vanish: None,
    })
}

pub(crate) fn pick_str(v: &Value, keys: &[&str]) -> Option<String> {
    for k in keys {
        match v.get(*k) {
            Some(Value::String(s)) if !s.trim().is_empty() => return Some(s.clone()),
            Some(Value::Number(n)) => return Some(n.to_string()),
            _ => {}
        }
    }
    None
}

/// Allocate a millisecond-shaped request ID once per send, including simultaneous sends.
pub(crate) fn next_client_message_id() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static LAST_ID: AtomicU64 = AtomicU64::new(0);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or(0);
    // CAS loop instead of `fetch_update`: deprecated on new toolchains, while its
    // replacement `try_update` is too new for the 1.75 MSRV.
    let mut last = LAST_ID.load(Ordering::Relaxed);
    loop {
        let next = now.max(last.saturating_add(1));
        match LAST_ID.compare_exchange_weak(last, next, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return next,
            Err(actual) => last = actual,
        }
    }
}

/// Convert inline Markdown (`**bold**`, `__underline__`, `~~strikethrough~~`, `*italic*`) and
/// line-prefix styles (`## `, `- `, `1. `) into Zalo PC's native `(clean_text, Option<textProperties>)`
/// format (`{"styles":[{"start":N,"len":M,"st":"b,i,u,s,f_18,lst_1,lst_2"}],"ver":0}` in `compact-app-pc.js`).
pub fn extract_zalo_rtf_text_properties(raw: &str) -> (String, Option<String>) {
    let mut out_text = String::with_capacity(raw.len());
    let mut styles: Vec<Value> = Vec::new();

    let lines: Vec<&str> = raw.split('\n').collect();
    for (line_idx, raw_line) in lines.iter().enumerate() {
        if line_idx > 0 {
            out_text.push('\n');
        }
        let mut line = *raw_line;
        let mut line_tags: Vec<&'static str> = Vec::new();
        if let Some(rest) = line.strip_prefix("### ") {
            line = rest;
            line_tags.push("b");
            line_tags.push("f_16");
        } else if let Some(rest) = line.strip_prefix("## ") {
            line = rest;
            line_tags.push("b");
            line_tags.push("f_18");
        } else if let Some(rest) = line.strip_prefix("# ") {
            line = rest;
            line_tags.push("b");
            line_tags.push("f_18");
        } else if let Some(rest) = line.strip_prefix("- ").or_else(|| line.strip_prefix("* ")) {
            line = rest;
            line_tags.push("lst_1");
        } else if let Some(rest) = line.strip_prefix("1. ") {
            line = rest;
            line_tags.push("lst_2");
        }

        let line_start_u16 = out_text.encode_utf16().count();
        let mut rest = line;
        while !rest.is_empty() {
            let mut matched_span = false;
            for (delim, st_tag) in [("`", ""), ("**", "b"), ("__", "u"), ("~~", "s"), ("*", "i")] {
                if rest.starts_with(delim) {
                    let dlen = delim.len();
                    let after = &rest[dlen..];
                    if let Some(end_pos) = after.find(delim) {
                        let inner = &after[..end_pos];
                        if !inner.is_empty() && !inner.contains('\n') {
                            let span_start = out_text.encode_utf16().count();
                            let span_len = inner.encode_utf16().count();
                            out_text.push_str(inner);
                            if !st_tag.is_empty() && span_len > 0 {
                                styles.push(json!({
                                    "start": span_start,
                                    "len": span_len,
                                    "st": st_tag,
                                }));
                            }
                            rest = &after[end_pos + dlen..];
                            matched_span = true;
                            break;
                        }
                    }
                }
            }
            if matched_span {
                continue;
            }
            let mut chars = rest.chars();
            if let Some(ch) = chars.next() {
                out_text.push(ch);
                rest = chars.as_str();
            } else {
                break;
            }
        }
        let line_len_u16 = out_text
            .encode_utf16()
            .count()
            .saturating_sub(line_start_u16);
        if !line_tags.is_empty() && line_len_u16 > 0 {
            styles.push(json!({
                "start": line_start_u16,
                "len": line_len_u16,
                "st": line_tags.join(","),
            }));
        }
    }

    if styles.is_empty() {
        (raw.to_string(), None)
    } else {
        let prop_str = json!({
            "styles": styles,
            "ver": 0,
        })
        .to_string();
        (out_text, Some(prop_str))
    }
}

/// Reconstruct Markdown markers from Zalo's incoming `rtf` object (`{"action":"rtf","title":...,"params":"{\"styles\":[...]}"}`)
/// so incoming rich-text messages sent from official Zalo PC / Mobile render with bold, underline, strikethrough, and lists.
fn format_incoming_zalo_rtf(title: &str, params_str: &str) -> String {
    let Ok(p_val) = serde_json::from_str::<Value>(params_str.trim()) else {
        return title.to_string();
    };
    let Some(styles) = p_val.get("styles").and_then(Value::as_array) else {
        return title.to_string();
    };
    if styles.is_empty() {
        return title.to_string();
    }
    let u16_units: Vec<u16> = title.encode_utf16().collect();
    let mut spans: Vec<(usize, usize, String)> = styles
        .iter()
        .filter_map(|s| {
            let start = s.get("start")?.as_u64()? as usize;
            let len = s.get("len")?.as_u64()? as usize;
            let st = s.get("st")?.as_str()?.to_string();
            if len > 0 && start + len <= u16_units.len() {
                Some((start, start + len, st))
            } else {
                None
            }
        })
        .collect();
    if spans.is_empty() {
        return title.to_string();
    }
    spans.sort_by_key(|x| x.0);
    let mut out = String::with_capacity(title.len() + spans.len() * 4);
    let mut cursor = 0usize;
    for (s, e, st) in spans {
        if s < cursor {
            continue;
        }
        if s > cursor {
            out.push_str(&String::from_utf16_lossy(&u16_units[cursor..s]));
        }
        let seg = String::from_utf16_lossy(&u16_units[s..e]);
        let tags: Vec<&str> = st.split(',').collect();
        let is_bold = tags.contains(&"b") || tags.iter().any(|t| t.starts_with("f_"));
        let is_italic = tags.contains(&"i");
        let is_under = tags.contains(&"u");
        let is_strike = tags.contains(&"s");
        let mut wrapped = seg;
        if is_italic && !is_bold {
            wrapped = format!("*{wrapped}*");
        }
        if is_bold {
            wrapped = format!("**{wrapped}**");
        }
        if is_under {
            wrapped = format!("__{wrapped}__");
        }
        if is_strike {
            wrapped = format!("~~{wrapped}~~");
        }
        out.push_str(&wrapped);
        cursor = e;
    }
    if cursor < u16_units.len() {
        out.push_str(&String::from_utf16_lossy(&u16_units[cursor..]));
    }
    out
}

/// Send a 1-1 text message: `POST {domains.chat}/api/message/sms?{common}`
pub async fn send_message(s: &Session, to_id: &str, text: &str) -> Result<Value> {
    send_message_with_client_id(s, to_id, text, next_client_message_id()).await
}

/// Preserve the request client ID for reconciliation with a later server echo.
pub(crate) async fn send_message_with_client_id(
    s: &Session,
    to_id: &str,
    text: &str,
    now: u64,
) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let (clean_text, text_props_opt) = extract_zalo_rtf_text_properties(text);
    let mut body = obj(&[
        ("toid", json!(to_id)),
        ("message", json!(clean_text)),
        ("clientId", json!(now)),
        ("imei", json!(s.imei)),
    ]);
    if let (Some(tp), Some(map)) = (text_props_opt, body.as_object_mut()) {
        map.insert("textProperties".into(), json!(tp));
    }
    let params = s.encoded_params(&body)?;

    let host = if s.domains.chat.contains("-msg-chat.")
        || s.domains.chat.contains("tt-chat2-wpa")
        || s.domains.chat.is_empty()
    {
        "https://tt-chat3-wpa.api.zaloapp.com"
    } else {
        &s.domains.chat
    };

    let url = format!("{host}/api/message/sms?{}", s.common_params());
    let form = format!("params={params}");
    let resp = s.post_form(&url, form).await?;
    let resp = s.maybe_decrypt(resp, s.session_key_bytes()).await;
    if let Some(code) = resp.get("error_code").and_then(Value::as_i64) {
        if code != 0 {
            let msg = resp
                .get("error_message")
                .and_then(Value::as_str)
                .unwrap_or("unknown");
            return Err(crate::session::SessionError::new(code, msg).into());
        }
    }
    Ok(resp)
}

/// Send a group text message: `POST {domains.group}/api/group/sendmsg?{common}`
pub async fn send_group_message(s: &Session, grid: &str, text: &str) -> Result<Value> {
    send_group_message_with_client_id(s, grid, text, next_client_message_id()).await
}

/// Preserve the request client ID for reconciliation with a later server echo.
pub(crate) async fn send_group_message_with_client_id(
    s: &Session,
    grid: &str,
    text: &str,
    now: u64,
) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let clean_grid = grid
        .trim_start_matches("group_")
        .trim_start_matches('g')
        .trim_start_matches('G');
    let (clean_text, text_props_opt) = extract_zalo_rtf_text_properties(text);
    let mut body = obj(&[
        ("grid", json!(clean_grid)),
        ("message", json!(clean_text)),
        ("clientId", json!(now)),
        ("imei", json!(s.imei)),
        ("visibility", json!(0)),
    ]);
    if let (Some(tp), Some(map)) = (text_props_opt, body.as_object_mut()) {
        map.insert("textProperties".into(), json!(tp));
    }
    let params = s.encoded_params(&body)?;
    let url = format!(
        "{}/api/group/sendmsg?{}",
        s.domains.group,
        s.common_params()
    );
    let form = format!("params={params}");
    let resp = s.post_form(&url, form).await?;
    let resp = s.maybe_decrypt(resp, s.session_key_bytes()).await;
    if let Some(code) = resp.get("error_code").and_then(Value::as_i64) {
        if code != 0 {
            let msg = resp
                .get("error_message")
                .and_then(Value::as_str)
                .unwrap_or("unknown");
            return Err(crate::session::SessionError::new(code, msg).into());
        }
    }
    Ok(resp)
}

/// Format a byte count into a human-readable size string (e.g. `"1.4 MB"`, `"320 KB"`).
pub fn format_file_size(bytes: u64) -> String {
    if bytes >= 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else if bytes >= 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{bytes} B")
    }
}

fn md5_hex_bytes(bytes: &[u8]) -> String {
    use md5::{Digest, Md5};
    let mut h = Md5::new();
    h.update(bytes);
    let out = h.finalize();
    out.iter().fold(String::with_capacity(32), |mut acc, b| {
        use std::fmt::Write;
        let _ = write!(acc, "{b:02x}");
        acc
    })
}

/// How long `send_attachment` waits for the `file_done` control event of an async file upload.
const FILE_DONE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(90);
/// How long an unclaimed `file_done` event is kept for a waiter that registers late.
const FILE_DONE_EARLY_TTL: std::time::Duration = std::time::Duration::from_secs(120);

/// Async file uploads waiting for their `act_type: "file_done"` control event, keyed by `fileId`.
///
/// `asyncfile/upload` only returns a `fileId`; Zalo pushes the final `fileUrl` later over the
/// WebSocket (`cmd 601`) or `lp_control`, and `asyncfile/msg` is rejected without it.
#[derive(Default)]
struct FileDoneRegistry {
    waiters: std::collections::HashMap<String, tokio::sync::oneshot::Sender<String>>,
    /// Events that arrived before their waiter registered (`fileId` -> (`fileUrl`, received at)).
    early: std::collections::HashMap<String, (String, std::time::Instant)>,
}

static FILE_DONE_REGISTRY: std::sync::LazyLock<std::sync::Mutex<FileDoneRegistry>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(FileDoneRegistry::default()));

/// Set once any control-event consumer (WebSocket or `lp_control` poll) feeds `file_done` events,
/// so front-ends without one (TUI) skip the wait instead of stalling for `FILE_DONE_TIMEOUT`.
static FILE_DONE_LISTENER_ACTIVE: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

fn file_id_key(v: &Value) -> Option<String> {
    match v {
        Value::String(s) if !s.trim().is_empty() => Some(s.trim().to_string()),
        Value::Number(n) => Some(n.to_string()),
        _ => None,
    }
}

/// Parse `act_type: "file_done"` items (`{fileId, data: {url}}`) into `(fileId, fileUrl)` pairs.
pub fn extract_file_done_controls(data_node: &Value) -> Vec<(String, String)> {
    let mut candidates: Vec<&Value> = Vec::new();
    if let Some(arr) = data_node.get("controls").and_then(Value::as_array) {
        candidates.extend(arr.iter());
    } else if let Some(arr) = data_node.as_array() {
        for item in arr {
            if let Some(sub) = item.get("controls").and_then(Value::as_array) {
                candidates.extend(sub.iter());
            } else {
                candidates.push(item);
            }
        }
    } else {
        candidates.push(data_node);
    }

    let mut out = Vec::new();
    for item in candidates {
        let c = item.get("content").unwrap_or(item);
        if c.get("act_type").and_then(Value::as_str) != Some("file_done") {
            continue;
        }
        let raw_data = c.get("data").unwrap_or(&Value::Null);
        let parsed_data: Value;
        let d = if let Some(s) = raw_data.as_str() {
            parsed_data = serde_json::from_str::<Value>(s).unwrap_or(Value::Null);
            &parsed_data
        } else {
            raw_data
        };
        let Some(file_id) = c
            .get("fileId")
            .or_else(|| d.get("fileId"))
            .and_then(file_id_key)
        else {
            continue;
        };
        let Some(url) = pick_str(d, &["url", "fileUrl"]) else {
            continue;
        };
        out.push((file_id, url));
    }
    out
}

/// Resolve pending async file uploads from a decrypted control payload (WebSocket `cmd 601` or
/// `lp_control`). Unclaimed events are kept briefly for uploads whose waiter registers late.
pub fn dispatch_file_done_controls(data_node: &Value) {
    FILE_DONE_LISTENER_ACTIVE.store(true, std::sync::atomic::Ordering::Relaxed);
    let events = extract_file_done_controls(data_node);
    if events.is_empty() {
        return;
    }
    let Ok(mut reg) = FILE_DONE_REGISTRY.lock() else {
        crate::logger::log_error("chat::file_done", "registry mutex poisoned");
        return;
    };
    reg.early
        .retain(|_, (_, at)| at.elapsed() < FILE_DONE_EARLY_TTL);
    for (file_id, url) in events {
        match reg.waiters.remove(&file_id) {
            Some(tx) => {
                let _ = tx.send(url);
            }
            None => {
                reg.early.insert(file_id, (url, std::time::Instant::now()));
            }
        }
    }
}

fn register_file_done_waiter(file_id: &str) -> tokio::sync::oneshot::Receiver<String> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    match FILE_DONE_REGISTRY.lock() {
        Ok(mut reg) => match reg.early.remove(file_id) {
            Some((url, _)) => {
                let _ = tx.send(url);
            }
            None => {
                reg.waiters.insert(file_id.to_string(), tx);
            }
        },
        // Dropping `tx` makes the receiver resolve immediately with an error.
        Err(_) => crate::logger::log_error("chat::file_done", "registry mutex poisoned"),
    }
    rx
}

/// Wait up to `timeout` for the `file_done` event of `file_id`; `None` on timeout.
async fn wait_for_file_done(file_id: &str, timeout: std::time::Duration) -> Option<String> {
    let rx = register_file_done_waiter(file_id);
    let res = tokio::time::timeout(timeout, rx).await;
    if let Ok(mut reg) = FILE_DONE_REGISTRY.lock() {
        reg.waiters.remove(file_id);
    }
    res.ok().and_then(Result::ok)
}

/// Upload and send a photo, video, or general file attachment to a 1-1 or group conversation.
///
/// Implements Zalo PC 26.10.11's two-stage chunked upload (`photo_original/upload` -> `photo_original/send`
/// for images; `asyncfile/upload` -> `asyncfile/msg` for videos/files) with automatic fallback to
/// single-shot `message/photo` (`11152`) and `message/sharefile_full` (`11146`).
pub async fn send_attachment(
    s: &Session,
    to_id: &str,
    is_group: bool,
    file_path: &std::path::Path,
    caption: &str,
) -> Result<(Value, ChatMessage)> {
    send_attachment_ext(s, to_id, is_group, file_path, caption, true).await
}

/// Album ("group layout") membership for one photo in a multi-photo send.
///
/// Field names follow zca-js `sendMessage.ts` (Zalo Web); see
/// `re-notes/38_photo_album_group_layout.md`. Every photo of one album shares `id`.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MediaGroupLayout {
    /// Shared album identifier; zca-js uses the send-time Unix milliseconds.
    pub id: String,
    /// Zero-based position of this photo inside the album.
    pub index: u32,
    /// Number of photos in the album.
    pub total: u32,
}

/// Parse `params` from a message `content`; both may arrive as an object or a JSON string.
fn content_params(content: &Value) -> Option<Value> {
    let parse_json = |v: &Value| -> Option<Value> {
        match v {
            Value::String(s) if s.trim_start().starts_with('{') => serde_json::from_str(s).ok(),
            Value::Object(_) => Some(v.clone()),
            _ => None,
        }
    };
    let content = parse_json(content)?;
    content.get("params").and_then(parse_json)
}

/// Read a photo's declared pixel size from `content.params` (`width`, `height`), as numbers
/// or numeric strings. Returns `None` unless both are positive.
pub fn photo_size_from_content(content: &Value) -> Option<[u32; 2]> {
    params_size(content, "width", "height")
}

/// Read a video's declared pixel size from `content.params` (`video_width`, `video_height`,
/// the fields Zalo's web client reads for `MSG_VIDEO`).
pub fn video_size_from_content(content: &Value) -> Option<[u32; 2]> {
    params_size(content, "video_width", "video_height")
}

fn params_size(content: &Value, width_key: &str, height_key: &str) -> Option<[u32; 2]> {
    let params = content_params(content)?;
    let dim = |key: &str| -> Option<u32> {
        let v = params.get(key)?;
        let n = v
            .as_u64()
            .or_else(|| v.as_f64().map(|f| f.round() as u64))
            .or_else(|| v.as_str().and_then(|s| s.trim().parse().ok()))?;
        u32::try_from(n).ok().filter(|&n| n > 0)
    };
    Some([dim(width_key)?, dim(height_key)?])
}

impl MediaGroupLayout {
    /// Build the layout for photo `index` of `total`, all sharing `id`.
    pub fn new(id: impl Into<String>, index: u32, total: u32) -> Self {
        Self {
            id: id.into(),
            index,
            total,
        }
    }

    /// Read album membership from an incoming photo's `content` (object or JSON string), whose
    /// `params` carries `group_layout_id`, `is_group_layout` and `id_in_group`. Returns `None`
    /// for photos sent on their own.
    pub fn from_content(content: &Value) -> Option<Self> {
        let params = content_params(content)?;
        let as_u64 = |key_snake: &str, key_camel: &str| -> Option<u64> {
            let v = params.get(key_snake).or_else(|| params.get(key_camel))?;
            v.as_u64()
                .or_else(|| v.as_str().and_then(|s| s.trim().parse().ok()))
                .or_else(|| v.as_bool().map(u64::from))
        };
        if as_u64("is_group_layout", "isGroupLayout").unwrap_or(0) == 0 {
            return None;
        }
        let id = pick_str(&params, &["group_layout_id", "groupLayoutId"])
            .or_else(|| as_u64("group_layout_id", "groupLayoutId").map(|n| n.to_string()))
            .filter(|s| !s.is_empty() && s != "0")?;
        let index = as_u64("id_in_group", "idInGroup").unwrap_or(0) as u32;
        let total = as_u64("total_item_in_group", "totalItemInGroup").unwrap_or(0) as u32;
        Some(Self { id, index, total })
    }

    /// Insert the album fields into a `photo_original/send` params object.
    pub fn apply_to(&self, params: &mut serde_json::Map<String, Value>) {
        params.insert("groupLayoutId".into(), json!(self.id));
        params.insert("isGroupLayout".into(), json!(1));
        params.insert("idInGroup".into(), json!(self.index));
        params.insert("totalItemInGroup".into(), json!(self.total));
        params.insert(
            "extMsgProp".into(),
            json!(json!({ "groupMediaMsg": { "groupLayoutId": self.id } }).to_string()),
        );
    }
}

pub(crate) fn probe_mp4_metadata(bytes: &[u8]) -> Option<(u64, u32, u32)> {
    fn read_be_u32(b: &[u8], off: usize) -> Option<u32> {
        let s = b.get(off..off + 4)?;
        Some(u32::from_be_bytes([s[0], s[1], s[2], s[3]]))
    }
    fn read_be_u64(b: &[u8], off: usize) -> Option<u64> {
        let s = b.get(off..off + 8)?;
        Some(u64::from_be_bytes([
            s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7],
        ]))
    }
    fn iter_boxes(buf: &[u8], mut f: impl FnMut([u8; 4], &[u8])) {
        let mut pos = 0usize;
        while pos + 8 <= buf.len() {
            let Some(size32) = read_be_u32(buf, pos) else {
                break;
            };
            let fourcc = [buf[pos + 4], buf[pos + 5], buf[pos + 6], buf[pos + 7]];
            let (hdr, total) = if size32 == 1 {
                let Some(ext) = read_be_u64(buf, pos + 8) else {
                    break;
                };
                (16usize, usize::try_from(ext).unwrap_or(buf.len() - pos))
            } else if size32 == 0 {
                (8usize, buf.len() - pos)
            } else {
                (8usize, size32 as usize)
            };
            if total < hdr {
                break;
            }
            let end = pos.saturating_add(total).min(buf.len());
            if let Some(payload) = buf.get(pos + hdr..end) {
                f(fourcc, payload);
            }
            if end <= pos {
                break;
            }
            pos = end;
        }
    }

    let mut duration_ms = 0u64;
    let mut width = 0u32;
    let mut height = 0u32;
    iter_boxes(bytes, |fourcc, payload| {
        if &fourcc == b"moov" {
            iter_boxes(payload, |sub_id, sub_payload| {
                if &sub_id == b"mvhd" && !sub_payload.is_empty() {
                    let ver = sub_payload[0];
                    if ver == 0 {
                        if let (Some(ts), Some(dur)) =
                            (read_be_u32(sub_payload, 12), read_be_u32(sub_payload, 16))
                        {
                            if ts > 0 {
                                duration_ms = (dur as u64).saturating_mul(1000) / ts as u64;
                            }
                        }
                    } else if ver == 1 {
                        if let (Some(ts), Some(dur)) =
                            (read_be_u32(sub_payload, 20), read_be_u64(sub_payload, 24))
                        {
                            if ts > 0 {
                                duration_ms = dur.saturating_mul(1000) / ts as u64;
                            }
                        }
                    }
                } else if &sub_id == b"trak" && width == 0 && height == 0 {
                    iter_boxes(sub_payload, |trak_id, trak_payload| {
                        if &trak_id == b"tkhd" && !trak_payload.is_empty() && width == 0 {
                            let ver = trak_payload[0];
                            let off = if ver == 0 { 76 } else { 88 };
                            if let (Some(w_fx), Some(h_fx)) = (
                                read_be_u32(trak_payload, off),
                                read_be_u32(trak_payload, off + 4),
                            ) {
                                let w = w_fx >> 16;
                                let h = h_fx >> 16;
                                if w > 0 && h > 0 {
                                    width = w;
                                    height = h;
                                }
                            }
                        }
                    });
                }
            });
        }
    });

    if width > 0 && height > 0 || duration_ms > 0 {
        Some((duration_ms, width, height))
    } else {
        None
    }
}

fn probe_video_metadata(path: &std::path::Path, bytes: &[u8]) -> (u64, u32, u32) {
    if let Some((dur, w, h)) = probe_mp4_metadata(bytes) {
        return (
            dur,
            if w > 0 { w } else { 1280 },
            if h > 0 { h } else { 720 },
        );
    }
    let mut cmd = std::process::Command::new("ffprobe");
    cmd.args([
        "-v",
        "error",
        "-select_streams",
        "v:0",
        "-show_entries",
        "stream=width,height",
        "-show_entries",
        "format=duration",
        "-of",
        "default=noprint_wrappers=1:nokey=1",
    ])
    .arg(path);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x0800_0000);
    }
    if let Ok(out) = cmd.output() {
        if out.status.success() {
            let txt = String::from_utf8_lossy(&out.stdout);
            let mut nums = txt.lines().filter_map(|l| l.trim().parse::<f64>().ok());
            let w = nums.next().map(|v| v as u32).unwrap_or(0);
            let h = nums.next().map(|v| v as u32).unwrap_or(0);
            let dur = nums.next().map(|v| (v * 1000.0).max(0.0) as u64).unwrap_or(0);
            if w > 0 || h > 0 || dur > 0 {
                return (
                    dur,
                    if w > 0 { w } else { 1280 },
                    if h > 0 { h } else { 720 },
                );
            }
        }
    }
    (0, 1280, 720)
}

fn generate_video_thumbnail_jpeg(
    video_path: &std::path::Path,
    width: u32,
    height: u32,
) -> Option<Vec<u8>> {
    for ss in ["00:00:00.5", "0"] {
        let mut cmd = std::process::Command::new("ffmpeg");
        cmd.args(["-y", "-ss", ss, "-i"])
            .arg(video_path)
            .args([
                "-vframes",
                "1",
                "-vf",
                "scale=640:-2",
                "-f",
                "image2",
                "-vcodec",
                "mjpeg",
                "-q:v",
                "4",
                "pipe:1",
            ]);
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(0x0800_0000);
        }
        if let Ok(out) = cmd.output() {
            if out.status.success() && !out.stdout.is_empty() {
                return Some(out.stdout);
            }
        }
    }

    let (tw, th) = if height > width { (270u32, 480u32) } else { (480u32, 270u32) };
    let mut img = image::RgbImage::from_pixel(tw, th, image::Rgb([24, 30, 42]));
    let cx = (tw / 2) as i32;
    let cy = (th / 2) as i32;
    let half_h = 26i32;
    for dy in -half_h..=half_h {
        let span = ((half_h - dy.abs()) * 42) / half_h.max(1);
        for dx in -12..=(-12 + span) {
            let px = cx + dx;
            let py = cy + dy;
            if px >= 0 && px < tw as i32 && py >= 0 && py < th as i32 {
                img.put_pixel(px as u32, py as u32, image::Rgb([238, 242, 248]));
            }
        }
    }
    let mut buf = Vec::new();
    image::DynamicImage::ImageRgb8(img)
        .write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Jpeg)
        .ok()?;
    Some(buf)
}

async fn upload_video_thumbnail(
    s: &Session,
    file_host: &str,
    clean_target: &str,
    is_group: bool,
    target_type: u8,
    client_id: u64,
    jpeg_bytes: &[u8],
) -> Option<String> {
    let upload_ep = if is_group {
        "/api/group/photo_original/upload"
    } else {
        "/api/message/photo_original/upload"
    };
    let mut m = serde_json::Map::new();
    m.insert("totalChunk".into(), json!(1));
    m.insert("fileName".into(), json!("video_thumb.jpg"));
    m.insert("clientId".into(), json!(client_id.saturating_add(1)));
    m.insert("totalSize".into(), json!(jpeg_bytes.len() as u64));
    m.insert("imei".into(), json!(s.imei));
    m.insert("isE2EE".into(), json!(0));
    m.insert("jxl".into(), json!(0));
    m.insert("chunkId".into(), json!(1));
    if is_group {
        m.insert("grid".into(), json!(clean_target));
    } else {
        m.insert("toid".into(), json!(clean_target));
    }
    let params = s.encoded_params(&Value::Object(m)).ok()?;
    let url = format!(
        "{file_host}{upload_ep}?{}&params={params}&type={target_type}",
        s.common_params()
    );
    let up_raw = s
        .post_multipart(&url, "chunkContent", "video_thumb.jpg", "image/jpeg", jpeg_bytes)
        .await
        .ok()?;
    let dec = s.maybe_decrypt(up_raw, s.session_key_bytes()).await;
    let data_node = dec.get("data").unwrap_or(&dec);
    pick_str(data_node, &["normalUrl", "thumbUrl", "hdUrl", "oriUrl", "url"])
}

pub(crate) fn build_native_video_forward_params(
    clean_target: &str,
    is_group: bool,
    client_id: u64,
    imei: &str,
    video_url: &str,
    thumb_url: &str,
    duration_ms: u64,
    width: u32,
    height: u32,
    file_size: u64,
    caption: &str,
) -> Value {
    let msg_info = json!({
        "videoUrl": video_url,
        "thumbUrl": thumb_url,
        "duration": duration_ms,
        "width": width,
        "height": height,
        "fileSize": file_size,
        "properties": {
            "color": -1,
            "size": -1,
            "type": 1003,
            "subType": 0,
            "ext": {
                "sSrcType": -1,
                "sSrcStr": "",
                "msg_warning_type": 0
            }
        },
        "title": caption.trim(),
    });
    let mut params = serde_json::Map::new();
    if is_group {
        params.insert("grid".into(), json!(clean_target));
        params.insert("visibility".into(), json!(0));
    } else {
        params.insert("toId".into(), json!(clean_target));
    }
    params.insert("clientId".into(), json!(client_id.to_string()));
    params.insert("ttl".into(), json!(0));
    params.insert("zsource".into(), json!(704));
    params.insert("msgType".into(), json!(5));
    params.insert("msgInfo".into(), json!(msg_info.to_string()));
    params.insert("imei".into(), json!(imei));
    Value::Object(params)
}

/// [`send_attachment`] with an explicit HD flag and no album membership.
pub async fn send_attachment_ext(
    s: &Session,
    to_id: &str,
    is_group: bool,
    file_path: &std::path::Path,
    caption: &str,
    is_original_hd: bool,
) -> Result<(Value, ChatMessage)> {
    send_attachment_in_layout(s, to_id, is_group, file_path, caption, is_original_hd, None).await
}

/// Upload and send one attachment; when `layout` is set and the file is a photo sent via
/// `photo_original/send`, it joins that album. Videos, files and the single-shot photo
/// fallback ignore `layout`.
///
/// # Errors
/// Returns an error when not logged in, the file is unreadable or empty, or every upload
/// and send path fails.
pub async fn send_attachment_in_layout(
    s: &Session,
    to_id: &str,
    is_group: bool,
    file_path: &std::path::Path,
    caption: &str,
    is_original_hd: bool,
    layout: Option<&MediaGroupLayout>,
) -> Result<(Value, ChatMessage)> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let file_bytes = std::fs::read(file_path)?;
    if file_bytes.is_empty() {
        anyhow::bail!("Tập tin rỗng (0 byte)");
    }
    let file_name = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("attachment.bin")
        .to_string();
    let ext = file_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    let is_image = matches!(
        ext.as_str(),
        "png" | "jpg" | "jpeg" | "webp" | "gif" | "bmp"
    );
    let is_video = matches!(ext.as_str(), "mp4" | "mov" | "mkv" | "webm" | "avi" | "m4v");

    let file_host = if s.domains.file.is_empty() {
        "https://tt-files-wpa.zaloapp.com"
    } else {
        s.domains.file.trim_end_matches('/')
    };
    let clean_target = if is_group {
        to_id
            .trim_start_matches("group_")
            .trim_start_matches('g')
            .trim_start_matches('G')
    } else {
        to_id
    };
    let client_id = next_client_message_id();
    let total_size = file_bytes.len() as u64;
    let chunk_size: usize = 1_048_576; // 1 MiB chunks
    let total_chunks = file_bytes.len().div_ceil(chunk_size).max(1);
    let target_type = if is_group { 11 } else { 2 };

    if is_image {
        let (width, height) = image::load_from_memory(&file_bytes)
            .map(|img| (img.width(), img.height()))
            .unwrap_or((800, 600));
        let mime = match ext.as_str() {
            "png" => "image/png",
            "webp" => "image/webp",
            "gif" => "image/gif",
            "bmp" => "image/bmp",
            _ => "image/jpeg",
        };
        let upload_ep = if is_group {
            "/api/group/photo_original/upload"
        } else {
            "/api/message/photo_original/upload"
        };

        let mut last_up_dec = Value::Null;
        for (idx, chunk) in file_bytes.chunks(chunk_size).enumerate() {
            let chunk_id = idx + 1;
            let mut m = serde_json::Map::new();
            m.insert("totalChunk".into(), json!(total_chunks));
            m.insert("fileName".into(), json!(file_name));
            m.insert("clientId".into(), json!(client_id));
            m.insert("totalSize".into(), json!(total_size));
            m.insert("imei".into(), json!(s.imei));
            m.insert("isE2EE".into(), json!(0));
            m.insert("jxl".into(), json!(0));
            m.insert("chunkId".into(), json!(chunk_id));
            if is_group {
                m.insert("grid".into(), json!(clean_target));
            } else {
                m.insert("toid".into(), json!(clean_target));
            }
            let params = s.encoded_params(&Value::Object(m))?;
            let url = format!(
                "{file_host}{upload_ep}?{}&params={params}&type={target_type}",
                s.common_params()
            );
            let up_raw = s
                .post_multipart(&url, "chunkContent", &file_name, mime, chunk)
                .await?;
            last_up_dec = s.maybe_decrypt(up_raw, s.session_key_bytes()).await;
        }

        let data_node = last_up_dec.get("data").unwrap_or(&last_up_dec);
        let photo_id = pick_str(data_node, &["photoId", "clientFileId"]);
        let normal_url =
            pick_str(data_node, &["normalUrl", "hdUrl", "oriUrl", "url"]).unwrap_or_default();
        let hd_url = pick_str(data_node, &["hdUrl", "normalUrl", "oriUrl"])
            .unwrap_or_else(|| normal_url.clone());
        let ori_url = pick_str(data_node, &["oriUrl", "hdUrl", "normalUrl"])
            .unwrap_or_else(|| normal_url.clone());
        let thumb_url = pick_str(data_node, &["thumbUrl", "normalUrl", "hdUrl"])
            .unwrap_or_else(|| normal_url.clone());

        // Only `photo_original/send` carries album fields; the single-shot fallback does not.
        let mut joined_album = false;
        let send_resp = if let Some(pid) = photo_id.filter(|_| !normal_url.is_empty()) {
            let send_ep = if is_group {
                "/api/group/photo_original/send"
            } else {
                "/api/message/photo_original/send"
            };
            let pid_val = pid
                .parse::<u64>()
                .map(|n| json!(n))
                .unwrap_or_else(|_| json!(pid));
            let mut send_obj = serde_json::Map::new();
            send_obj.insert("photoId".into(), pid_val);
            send_obj.insert("clientId".into(), json!(client_id));
            send_obj.insert("desc".into(), json!(caption));
            send_obj.insert("width".into(), json!(width));
            send_obj.insert("height".into(), json!(height));
            send_obj.insert("rawUrl".into(), json!(normal_url));
            send_obj.insert("hdUrl".into(), json!(hd_url));
            send_obj.insert("thumbUrl".into(), json!(thumb_url));
            send_obj.insert(
                "oriUrl".into(),
                json!(if is_original_hd {
                    &ori_url
                } else {
                    &normal_url
                }),
            );
            send_obj.insert("normalUrl".into(), json!(normal_url));
            send_obj.insert(
                "hdSize".into(),
                json!(if is_original_hd { total_size } else { 0 }),
            );
            send_obj.insert("isHD".into(), json!(if is_original_hd { 1 } else { 0 }));
            send_obj.insert("hd".into(), json!(if is_original_hd { 1 } else { 0 }));
            send_obj.insert(
                "isOriginal".into(),
                json!(if is_original_hd { 1 } else { 0 }),
            );
            send_obj.insert("zsource".into(), json!(-1));
            send_obj.insert("jcp".into(), json!("{\"convertible\":true}"));
            send_obj.insert("imei".into(), json!(s.imei));
            if let Some(layout) = layout {
                layout.apply_to(&mut send_obj);
                joined_album = true;
            }
            if is_group {
                send_obj.insert("grid".into(), json!(clean_target));
                send_obj.insert("visibility".into(), json!(0));
            } else {
                send_obj.insert("toid".into(), json!(clean_target));
            }
            let params = s.encoded_params(&Value::Object(send_obj))?;
            let url = format!("{file_host}{send_ep}?{}", s.common_params());
            let r = s.post_form(&url, format!("params={params}")).await?;
            s.maybe_decrypt(r, s.session_key_bytes()).await
        } else {
            // Fallback: single-shot `/api/message/photo` or `/api/group/sendpt`
            let single_ep = if is_group {
                "/api/group/sendpt"
            } else {
                "/api/message/photo"
            };
            let mut body_obj = serde_json::Map::new();
            body_obj.insert("clientId".into(), json!(client_id));
            body_obj.insert("desc".into(), json!(caption));
            body_obj.insert("width".into(), json!(width));
            body_obj.insert("height".into(), json!(height));
            body_obj.insert("imei".into(), json!(s.imei));
            if is_group {
                body_obj.insert("grid".into(), json!(clean_target));
                body_obj.insert("visibility".into(), json!(0));
            } else {
                body_obj.insert("toid".into(), json!(clean_target));
            }
            let params = s.encoded_params(&Value::Object(body_obj))?;
            let url = format!(
                "{file_host}{single_ep}?{}&params={params}&type={target_type}",
                s.common_params()
            );
            let r = s
                .post_multipart(&url, "fileContent", &file_name, mime, &file_bytes)
                .await?;
            s.maybe_decrypt(r, s.session_key_bytes()).await
        };

        if let Some(code) = send_resp.get("error_code").and_then(Value::as_i64) {
            if code != 0 {
                let msg = send_resp
                    .get("error_message")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown");
                return Err(crate::session::SessionError::new(code, msg).into());
            }
        }

        let final_thumb = if !thumb_url.is_empty() {
            thumb_url
        } else {
            format!("local://{}", file_path.display())
        };
        let final_url = if is_original_hd && !ori_url.is_empty() {
            ori_url
        } else if !hd_url.is_empty() {
            hd_url
        } else if !normal_url.is_empty() {
            normal_url
        } else {
            file_path.display().to_string()
        };
        // Pre-populate `<media cache>/thumb_<md5>.png` with the local image bytes so the GUI
        // renders the sent photo immediately without an extra network round-trip.
        let cache_key = format!("thumb_{}", crate::crypto::md5_hex(&final_thumb));
        // Best effort: a failed cache write only costs one thumbnail re-download.
        let _ = std::fs::write(
            crate::account_manager::ensure_media_cache_dir().join(format!("{cache_key}.png")),
            &file_bytes,
        );

        let msg_id = send_resp
            .get("data")
            .and_then(|d| pick_str(d, &["msgId"]))
            .unwrap_or_else(|| format!("out_{client_id}"));
        let now_str = crate::ui::clock();
        let display_text = if !caption.trim().is_empty() {
            caption.trim().to_string()
        } else {
            "[Hình ảnh]".to_string()
        };
        let clean_title = if file_name.starts_with("clipboard_")
            || file_name.starts_with("msedge_")
            || file_name.starts_with("ctdozlite_")
        {
            format!(
                "Hình ảnh ({width}×{height} · {})",
                format_file_size(total_size)
            )
        } else {
            format!(
                "{file_name} ({width}×{height} · {})",
                format_file_size(total_size)
            )
        };
        let cm = ChatMessage {
            id: msg_id,
            client_message_id: client_id.to_string(),
            sender: "Tôi".into(),
            text: display_text.clone(),
            time: now_str,
            is_outgoing: true,
            ts: client_id as i64,
            delivery: crate::message_model::DeliveryState::ServerAccepted,
            link_url: Some(final_url),
            link_title: Some(clean_title),
            link_desc: if caption.trim().is_empty() {
                None
            } else {
                Some(caption.trim().to_string())
            },
            link_thumb: Some(final_thumb),
            link_src: Some("Hình ảnh Zalo".into()),
            media_group: layout.filter(|_| joined_album).cloned(),
            media_size: Some([width, height]).filter(|&[w, h]| w > 0 && h > 0),
            ..Default::default()
        };
        return Ok((send_resp, cm));
    }

    // Video / Audio / Document / Archive upload (`asyncfile/upload` -> `asyncfile/msg`)
    // Zalo server processes the uploaded file asynchronously and emits a WebSocket notification
    // (`cmd = 601`, `subCmd = 0`, `act_type = "file_done"`) containing the CDN fileUrl.
    // That fileUrl is mandatory for `/api/message/asyncfile/msg` (or `/api/group/asyncfile/msg`).
    let mime = match ext.as_str() {
        "mp4" | "m4v" => "video/mp4",
        "mov" => "video/quicktime",
        "mkv" => "video/x-matroska",
        "webm" => "video/webm",
        "avi" => "video/x-msvideo",
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "pdf" => "application/pdf",
        "zip" => "application/zip",
        "txt" | "md" | "log" => "text/plain",
        _ => "application/octet-stream",
    };
    let checksum = md5_hex_bytes(&file_bytes);
    let upload_ep = if is_group {
        "/api/group/asyncfile/upload"
    } else {
        "/api/message/asyncfile/upload"
    };

    let mut last_up_dec = Value::Null;
    for (idx, chunk) in file_bytes.chunks(chunk_size).enumerate() {
        let chunk_id = idx + 1;
        let mut m = serde_json::Map::new();
        m.insert("totalChunk".into(), json!(total_chunks));
        m.insert("fileName".into(), json!(file_name));
        m.insert("clientId".into(), json!(client_id));
        m.insert("totalSize".into(), json!(total_size));
        m.insert("imei".into(), json!(s.imei));
        m.insert("isE2EE".into(), json!(0));
        m.insert("chunkId".into(), json!(chunk_id));
        if is_group {
            m.insert("grid".into(), json!(clean_target));
        } else {
            m.insert("toid".into(), json!(clean_target));
        }
        let params = s.encoded_params(&Value::Object(m))?;
        let url = format!(
            "{file_host}{upload_ep}?{}&params={params}&type={target_type}",
            s.common_params()
        );
        let up_raw = s
            .post_multipart(&url, "chunkContent", &file_name, mime, chunk)
            .await
            .map_err(|e| anyhow::anyhow!("tải lên phần {chunk_id}/{total_chunks} thất bại: {e}"))?;
        last_up_dec = s.maybe_decrypt(up_raw, s.session_key_bytes()).await;
        if let Some(code) = last_up_dec.get("error_code").and_then(Value::as_i64) {
            if code != 0 {
                let msg = last_up_dec
                    .get("error_message")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown");
                return Err(crate::session::SessionError::new(code, msg).into());
            }
        }
    }

    let data_node = last_up_dec.get("data").unwrap_or(&last_up_dec);
    let file_id = pick_str(data_node, &["fileId", "clientFileId"]).filter(|id| id != "-1");
    let client_file_id = data_node
        .get("clientFileId")
        .cloned()
        .unwrap_or_else(|| json!(client_id));
    let mut file_url = pick_str(data_node, &["fileUrl", "url", "normalUrl"]).unwrap_or_default();
    if file_url.is_empty() && FILE_DONE_LISTENER_ACTIVE.load(std::sync::atomic::Ordering::Relaxed) {
        if let Some(fid) = file_id.as_deref() {
            match wait_for_file_done(fid, FILE_DONE_TIMEOUT).await {
                Some(url) => file_url = url,
                None => crate::logger::log_error(
                    "chat::send_attachment",
                    &format!("no file_done event for fileId {fid} within {FILE_DONE_TIMEOUT:?}"),
                ),
            }
        }
    }

    if is_video && !file_url.is_empty() {
        let (duration_ms, vid_w, vid_h) = probe_video_metadata(file_path, &file_bytes);
        let thumb_url = match generate_video_thumbnail_jpeg(file_path, vid_w, vid_h) {
            Some(jpeg) => upload_video_thumbnail(
                s,
                file_host,
                clean_target,
                is_group,
                target_type,
                client_id,
                &jpeg,
            )
            .await
            .unwrap_or_default(),
            None => String::new(),
        };
        let fwd_ep = if is_group {
            "/api/group/forward"
        } else {
            "/api/message/forward"
        };
        let fwd_params = build_native_video_forward_params(
            clean_target,
            is_group,
            client_id,
            &s.imei,
            &file_url,
            &thumb_url,
            duration_ms,
            vid_w,
            vid_h,
            total_size,
            caption,
        );
        if let Ok(enc) = s.encoded_params(&fwd_params) {
            let fwd_url = format!("{file_host}{fwd_ep}?{}", s.common_params());
            match s.post_form(&fwd_url, format!("params={enc}")).await {
                Ok(raw) => {
                    let fwd_resp = s.maybe_decrypt(raw, s.session_key_bytes()).await;
                    let code = fwd_resp
                        .get("error_code")
                        .and_then(Value::as_i64)
                        .unwrap_or(-1);
                    if code == 0 {
                        let msg_id = fwd_resp
                            .get("data")
                            .and_then(|d| pick_str(d, &["msgId"]))
                            .unwrap_or_else(|| format!("out_{client_id}"));
                        let display_text = if !caption.trim().is_empty() {
                            caption.trim().to_string()
                        } else {
                            format!("[Video] {file_name}")
                        };
                        let cm = ChatMessage {
                            id: msg_id,
                            client_message_id: client_id.to_string(),
                            sender: "Tôi".into(),
                            text: display_text,
                            time: crate::ui::clock(),
                            is_outgoing: true,
                            ts: client_id as i64,
                            delivery: crate::message_model::DeliveryState::ServerAccepted,
                            kind: crate::message_model::MessageKind::Video,
                            link_url: Some(file_url),
                            link_thumb: (!thumb_url.is_empty()).then_some(thumb_url),
                            link_title: Some(format!(
                                "{file_name} ({})",
                                format_file_size(total_size)
                            )),
                            link_desc: (!caption.trim().is_empty())
                                .then(|| caption.trim().to_string()),
                            link_src: Some("Video Zalo".into()),
                            media_size: Some([vid_w, vid_h]).filter(|&[w, h]| w > 0 && h > 0),
                            ..Default::default()
                        };
                        return Ok((fwd_resp, cm));
                    }
                    crate::logger::log_error(
                        "chat::send_video_forward",
                        &format!("forward returned error_code={code}: {fwd_resp:?}"),
                    );
                }
                Err(e) => {
                    crate::logger::log_error(
                        "chat::send_video_forward",
                        &format!("forward request failed: {e}"),
                    );
                }
            }
        }
    }

    let send_resp = if let Some(fid) = file_id {
        let msg_ep = if is_group {
            "/api/group/asyncfile/msg"
        } else {
            "/api/message/asyncfile/msg"
        };
        let mut msg_obj = serde_json::Map::new();
        msg_obj.insert("fileId".into(), json!(fid));
        msg_obj.insert("checksum".into(), json!(checksum));
        msg_obj.insert("checksumSha".into(), json!(""));
        msg_obj.insert("extention".into(), json!(ext));
        msg_obj.insert("extension".into(), json!(ext));
        msg_obj.insert("totalSize".into(), json!(total_size));
        msg_obj.insert("fileName".into(), json!(file_name));
        msg_obj.insert("clientId".into(), client_file_id);
        msg_obj.insert("fType".into(), json!(if is_video { 2 } else { 1 }));
        msg_obj.insert("fileCount".into(), json!(0));
        msg_obj.insert("fdata".into(), json!("{}"));
        if is_group {
            msg_obj.insert("grid".into(), json!(clean_target));
        } else {
            msg_obj.insert("toid".into(), json!(clean_target));
        }
        msg_obj.insert("fileUrl".into(), json!(file_url));
        msg_obj.insert("zsource".into(), json!(-1));
        msg_obj.insert("ttl".into(), json!(0));

        let params = s.encoded_params(&Value::Object(msg_obj))?;
        let url = format!("{file_host}{msg_ep}?{}", s.common_params());
        let r = s.post_form(&url, format!("params={params}")).await?;
        s.maybe_decrypt(r, s.session_key_bytes()).await
    } else {
        json!({ "error_code": -1, "error_message": "Không nhận được fileId từ máy chủ Zalo" })
    };

    if send_resp
        .get("error_code")
        .and_then(Value::as_i64)
        .unwrap_or(-1)
        != 0
    {
        eprintln!("[send_attachment] asyncfile/msg failed: {send_resp:?}");
    }

    if let Some(code) = send_resp.get("error_code").and_then(Value::as_i64) {
        if code != 0 {
            let msg = send_resp
                .get("error_message")
                .and_then(Value::as_str)
                .unwrap_or("unknown");
            return Err(crate::session::SessionError::new(code, msg).into());
        }
    }

    let final_url = if !file_url.is_empty() {
        file_url
    } else {
        send_resp
            .get("data")
            .and_then(|d| pick_str(d, &["fileUrl", "url", "href"]))
            .unwrap_or_default()
    };
    let msg_id = send_resp
        .get("data")
        .and_then(|d| pick_str(d, &["msgId"]))
        .unwrap_or_else(|| format!("out_{client_id}"));
    let kind_label = if is_video { "Video" } else { "Tập tin" };
    let display_text = if !caption.trim().is_empty() {
        caption.trim().to_string()
    } else {
        format!("[{kind_label}] {file_name}")
    };
    let cm = ChatMessage {
        id: msg_id,
        client_message_id: client_id.to_string(),
        sender: "Tôi".into(),
        text: display_text,
        time: crate::ui::clock(),
        is_outgoing: true,
        ts: client_id as i64,
        delivery: crate::message_model::DeliveryState::ServerAccepted,
        link_url: if final_url.is_empty() {
            None
        } else {
            Some(final_url)
        },
        link_title: Some(format!("{file_name} ({})", format_file_size(total_size))),
        link_desc: if caption.trim().is_empty() {
            None
        } else {
            Some(caption.trim().to_string())
        },
        link_src: Some(format!("{kind_label} Zalo")),
        ..Default::default()
    };
    Ok((send_resp, cm))
}
/// Gửi tin nhắn thoại thu sẵn (`Zalo PC 26.11.10`: `sendVoice` — 1-1 `POST
/// `{file}/api/message/voicemp3` (cmd 11148), nhóm `POST {file}/api/group/sendmp3`
/// (cmd 11169); multipart field `fileContent` + `params={toid|grid, clientId-string}`).
///
/// Lựa chọn format: **M4A/AAC** (ffmpeg) — đúng chuẩn voice Zalo (bundle map
/// `MSG_VOICE → aac`). Thực nghiệm: gửi MP3 thì server tạo message nhưng không
/// gắn URL voice, bên nhận chỉ thấy chữ "[Tin nhắn thoại]".
pub async fn send_voice_message(
    s: &Session,
    to_id: &str,
    is_group: bool,
    voice_path: &std::path::Path,
    mime: &str,
    duration_secs: u64,
) -> Result<(Value, ChatMessage)> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let voice_bytes = std::fs::read(voice_path)?;
    if voice_bytes.is_empty() {
        anyhow::bail!("File ghi âm rỗng (0 byte)");
    }
    let file_name = voice_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("voice.aac")
        .to_string();
    let file_host = if s.domains.file.is_empty() {
        "https://tt-files-wpa.zaloapp.com"
    } else {
        s.domains.file.trim_end_matches('/')
    };
    let clean_target = if is_group {
        to_id
            .trim_start_matches("group_")
            .trim_start_matches('g')
            .trim_start_matches('G')
    } else {
        to_id
    };
    // Zalo PC truyền `clientId` là string trong params của `sendVoice`.
    // Endpoint mới (cmd 12060/12063): voicemp3/sendmp3 cũ đã 404 mọi host.
    let client_id = next_client_message_id();
    // Zalo yêu cầu fileType cho voice: "amr" nếu tên file chứa .amr, else "aac".
    let file_type = if file_name.to_ascii_lowercase().contains(".amr") {
        "amr"
    } else {
        "aac"
    };
    let (upload_ep, target_type, params_obj) = if is_group {
        let m = obj(&[
            ("grid", json!(clean_target)),
            ("clientId", json!(client_id.to_string())),
            ("fileName", json!(file_name.clone())),
            ("fileSize", json!(voice_bytes.len() as u64)),
            ("totalSize", json!(voice_bytes.len() as u64)),
            ("totalChunk", json!(1)),
            ("chunkId", json!(1)),
            ("duration", json!(duration_secs)),
            ("fileType", json!(file_type)),
            ("imei", json!(s.imei.clone())),
            ("isE2EE", json!(0)),
        ]);
        ("/api/group/voice/upload", 11, m)
    } else {
        let m = obj(&[
            ("toid", json!(clean_target)),
            ("clientId", json!(client_id.to_string())),
            ("fileName", json!(file_name.clone())),
            ("fileSize", json!(voice_bytes.len() as u64)),
            ("totalSize", json!(voice_bytes.len() as u64)),
            ("totalChunk", json!(1)),
            ("chunkId", json!(1)),
            ("duration", json!(duration_secs)),
            ("fileType", json!(file_type)),
            ("imei", json!(s.imei.clone())),
            ("isE2EE", json!(0)),
        ]);
        ("/api/message/voice/upload", 2, m)
    };
    let params = s.encoded_params(&params_obj)?;
    let url = format!(
        "{file_host}{upload_ep}?{}&params={params}&type={target_type}",
        s.common_params()
    );
    let up_raw = s
        .post_multipart(&url, "chunkContent", &file_name, mime, &voice_bytes)
        .await
        .map_err(|e| anyhow::anyhow!("tải lên tin nhắn thoại thất bại: {e}"))?;
    let mut send_resp = s.maybe_decrypt(up_raw, s.session_key_bytes()).await;
    // Async upload chỉ trả fileId; fileUrl tới sau qua WS file_done (như attachment).
    // Chờ event rồi nhét URL vào response để parse ra message đầy đủ.
    let need_url = send_resp
        .get("data")
        .and_then(|d| pick_str(d, &["fileUrl", "url", "normalUrl", "href"]))
        .unwrap_or_default()
        .is_empty();
    if need_url && FILE_DONE_LISTENER_ACTIVE.load(std::sync::atomic::Ordering::Relaxed) {
        let fid = send_resp
            .get("data")
            .and_then(|d| pick_str(d, &["fileId"]).filter(|id| id != "-1"))
            .unwrap_or_default();
        if !fid.is_empty() {
            if let Some(url) = wait_for_file_done(&fid, FILE_DONE_TIMEOUT).await {
                if let Some(obj) = send_resp.get_mut("data").and_then(Value::as_object_mut) {
                    obj.insert("fileUrl".into(), Value::String(url));
                }
            } else {
                crate::logger::log_error(
                    "chat::send_voice",
                    &format!("no file_done event for fileId {fid} within {FILE_DONE_TIMEOUT:?}"),
                );
            }
        }
    }
    let cm = parse_voice_send_response(&send_resp, client_id, duration_secs)?;
    Ok((send_resp, cm))
}

/// Parse envelope `sendVoice`/`sendmp3`: success giữ voice URL, error_code
/// tài liệu hoá trả Err (để test fixture không cần network, #2503).
pub fn parse_voice_send_response(
    send_resp: &Value,
    client_id: u64,
    duration_secs: u64,
) -> Result<ChatMessage> {
    if let Some(code) = send_resp.get("error_code").and_then(Value::as_i64) {
        if code != 0 {
            let msg = send_resp
                .get("error_message")
                .and_then(Value::as_str)
                .unwrap_or("unknown");
            return Err(crate::session::SessionError::new(code, msg).into());
        }
    }
    let data_node = send_resp.get("data").unwrap_or(send_resp);
    let file_url =
        pick_str(data_node, &["fileUrl", "url", "normalUrl", "href"]).unwrap_or_default();
    let msg_id = pick_str(data_node, &["msgId"]).unwrap_or_else(|| format!("out_{client_id}"));
    let mm = duration_secs / 60;
    let ss = duration_secs % 60;
    Ok(ChatMessage {
        id: msg_id,
        client_message_id: client_id.to_string(),
        sender: "Tôi".into(),
        text: "[Tin nhắn thoại]".into(),
        time: crate::ui::clock(),
        is_outgoing: true,
        ts: client_id as i64,
        delivery: crate::message_model::DeliveryState::ServerAccepted,
        kind: crate::message_model::MessageKind::Voice,
        link_url: if file_url.is_empty() {
            None
        } else {
            Some(file_url)
        },
        link_title: Some(format!("Tin nhắn thoại ({mm:02}:{ss:02})")),
        link_desc: None,
        link_src: Some("Thoại Zalo".into()),
        ..Default::default()
    })
}
/// Đoán ngôn ngữ nguồn để dịch (target mặc định vi). Trả None khi text đã
/// đúng ngôn ngữ đích hoặc quá ngắn/không có chữ → không cần dịch.
pub fn guess_translate_source(text: &str) -> Option<&'static str> {
    translate_source_for_target(text, "vi")
}

/// So sánh mã ngôn ngữ user nhập (case/space-insensitive, rỗng = vi).
fn lang_is(code: &str, want: &str) -> bool {
    let t = code.trim();
    if want == "vi" {
        return t.is_empty() || t.eq_ignore_ascii_case("vi");
    }
    t.eq_ignore_ascii_case(want)
}

/// Đoán ngôn ngữ nguồn cho target cụ thể. "Tự động" vẫn là heuristic chữ
/// viết — MyMemory autodetect không đáng tin (fr→vi cho ra EN) nên Latin
/// ngoài en (es/fr/…) cần user chọn nguồn thủ công ở menu Dịch.
pub fn translate_source_for_target(text: &str, target: &str) -> Option<&'static str> {
    let mut latin = 0u32;
    let mut vi_mark = 0u32;
    let mut cjk = 0u32;
    let mut kana = 0u32;
    let mut hangul = 0u32;
    let mut cyrillic = 0u32;
    for ch in text.chars() {
        match ch {
            // Dấu tiếng Việt (đ, ă, â, ê, ô, ơ, ư + nguyên âm dấu, cả HOA).
            'đ' | 'Đ' | 'ă' | 'Ă' | 'â' | 'Â' | 'ê' | 'Ê' | 'ô' | 'Ô' | 'ơ' | 'Ơ' | 'ư' | 'Ư'
            | 'á' | 'à' | 'ả' | 'ã' | 'ạ' | 'ắ' | 'ằ' | 'ẳ' | 'ẵ' | 'ặ' | 'ấ' | 'ầ' | 'ẩ' | 'ẫ'
            | 'ậ' | 'é' | 'è' | 'ẻ' | 'ẽ' | 'ẹ' | 'ế' | 'ề' | 'ể' | 'ễ' | 'ệ' | 'í' | 'ì' | 'ỉ'
            | 'ĩ' | 'ị' | 'ó' | 'ò' | 'ỏ' | 'õ' | 'ọ' | 'ố' | 'ồ' | 'ổ' | 'ỗ' | 'ộ' | 'ớ' | 'ờ'
            | 'ở' | 'ỡ' | 'ợ' | 'ú' | 'ù' | 'ủ' | 'ũ' | 'ụ' | 'ứ' | 'ừ' | 'ử' | 'ữ' | 'ự' | 'ý'
            | 'ỳ' | 'ỷ' | 'ỹ' | 'ỵ' | 'Á' | 'À' | 'Ả' | 'Ã' | 'Ạ' | 'Ắ' | 'Ằ' | 'Ẳ' | 'Ẵ' | 'Ặ'
            | 'Ấ' | 'Ầ' | 'Ẩ' | 'Ẫ' | 'Ậ' | 'É' | 'È' | 'Ẻ' | 'Ẽ' | 'Ẹ' | 'Ế' | 'Ề' | 'Ể' | 'Ễ'
            | 'Ệ' | 'Í' | 'Ì' | 'Ỉ' | 'Ĩ' | 'Ị' | 'Ó' | 'Ò' | 'Ỏ' | 'Õ' | 'Ọ' | 'Ố' | 'Ồ' | 'Ổ'
            | 'Ỗ' | 'Ộ' | 'Ớ' | 'Ờ' | 'Ở' | 'Ỡ' | 'Ợ' | 'Ú' | 'Ù' | 'Ủ' | 'Ũ' | 'Ụ' | 'Ứ' | 'Ừ'
            | 'Ử' | 'Ữ' | 'Ự' | 'Ý' | 'Ỳ' | 'Ỷ' | 'Ỹ' | 'Ỵ' => vi_mark += 1,
            'a'..='z' | 'A'..='Z' => latin += 1,
            '\u{3040}'..='\u{30ff}' => kana += 1,
            '\u{4e00}'..='\u{9fff}' => cjk += 1,
            '\u{ac00}'..='\u{d7af}' => hangul += 1,
            '\u{0400}'..='\u{04ff}' => cyrillic += 1,
            _ => {}
        }
    }
    // Dấu lẻ tẻ kiểu "café" chưa đủ kết luận là Việt: cần ≥2 dấu, hoặc
    // đúng 1 dấu mà chữ Latin rất ngắn; còn lại coi như Latin → dịch tiếp.
    let is_vietnamese = vi_mark >= 2 || (vi_mark == 1 && latin <= 3);
    if is_vietnamese {
        if lang_is(target, "vi") {
            return None; // Đã đúng ngôn ngữ đích.
        }
        return Some("vi");
    }
    // Ưu tiên script phi-Latin (rõ ràng nhất).
    if kana > 0 {
        return if lang_is(target, "ja") {
            None
        } else {
            Some("ja")
        };
    }
    if hangul > 0 {
        return if lang_is(target, "ko") {
            None
        } else {
            Some("ko")
        };
    }
    if cyrillic > cjk && cyrillic > latin {
        return if lang_is(target, "ru") {
            None
        } else {
            Some("ru")
        };
    }
    if cjk > 0 {
        return if lang_is(target, "zh") {
            None
        } else {
            Some("zh")
        };
    }
    if latin >= 3 {
        return if lang_is(target, "en") {
            None
        } else {
            Some("en")
        };
    }
    None
}
/// Key ổn định cho 1 request dịch (thread + msg id).
pub fn translate_request_key(thread_id: &str, msg_id: &str) -> String {
    format!("{thread_id}\u{1f}{msg_id}")
}

/// Lỗi dịch đã phân loại để GUI toast thân thiện.
#[derive(Debug, PartialEq)]
pub enum TranslateError {
    Quota,
    Empty,
    Rejected(String),
}

impl std::fmt::Display for TranslateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Quota => write!(f, "Hết quota dịch miễn phí trong ngày, thử lại mai"),
            Self::Empty => write!(f, "Không dịch được (server trả rỗng)"),
            Self::Rejected(s) => write!(f, "{s}"),
        }
    }
}

/// Parse JSON MyMemory thuần (không mạng) để test offline được.
/// Chấp nhận responseStatus số hoặc chuỗi; 403 + diagnostic trong
/// translatedText (vd "PLEASE SELECT TWO DISTINCT LANGUAGES",
/// "INVALID EMAIL") phải bị từ chối, không hiện như bản dịch.
pub fn parse_mymemory_response(resp: &Value, sent: &str) -> Result<String, TranslateError> {
    // Quota trước: 429 bản thân đã là status lỗi, không lẫn với Rejected.
    let quota_out = resp
        .get("quotaFinished")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let status_429 = matches!(
        resp.get("responseStatus"),
        Some(Value::String(s)) if s.trim() == "429"
    ) || matches!(resp.get("responseStatus"), Some(Value::Number(n)) if n.as_i64() == Some(429));
    if quota_out || status_429 {
        return Err(TranslateError::Quota);
    }
    let status_ok = match resp.get("responseStatus") {
        Some(Value::Number(n)) => n.as_i64() == Some(200),
        Some(Value::String(s)) => s.trim() == "200",
        _ => false,
    };
    if !status_ok {
        let detail = resp
            .get("responseDetails")
            .and_then(Value::as_str)
            .unwrap_or("")
            .trim();
        if detail.is_empty() {
            return Err(TranslateError::Rejected(
                "Dịch thất bại (mã lỗi không rõ)".into(),
            ));
        }
        return Err(TranslateError::Rejected(format!(
            "Dịch thất bại ({detail})"
        )));
    }
    let out = resp
        .get("responseData")
        .and_then(|d| d.get("translatedText"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .trim()
        .to_string();
    // Diagnostic MyMemory nhét vào translatedText dù status 200 (vd khi
    // langpair trùng nhau "en|en" → "PLEASE SELECT TWO DISTINCT LANGUAGES").
    let upper = out.to_ascii_uppercase();
    let is_diagnostic = out.is_empty()
        || out == sent
        || upper.contains("PLEASE SELECT TWO DISTINCT LANGUAGES")
        || upper.contains("INVALID EMAIL")
        || upper.contains("QUERY LENGTH LIMIT EXCEEDED")
        || upper.contains("NO QUERY SPECIFIED")
        || upper.contains("MYMEMORY WARNING");
    if is_diagnostic {
        return Err(TranslateError::Empty);
    }
    Ok(out)
}

/// Dịch text qua MyMemory API miễn phí (không key, ~5000 từ/ngày/IP).
/// Trả Err thân thiện khi quota hết/mạng lỗi để GUI toast.
pub async fn translate_via_mymemory(
    http: &reqwest::Client,
    text: &str,
    source: &str,
    target: &str,
) -> Result<String> {
    let text = text.trim();
    if text.is_empty() {
        anyhow::bail!("Tin nhắn trống, không có gì để dịch");
    }
    // MyMemory giới hạn ~500 ký tự/query (q>500 hay 413) → chỉ dịch 450 ký tự
    // đầu và đánh dấu "[dịch 1 phần]" để user biết phần còn lại bị bỏ.
    let truncated = text.chars().count() > 450;
    let short: String = text.chars().take(450).collect();
    let http_resp = http
        .get("https://api.mymemory.translated.net/get")
        .query(&[
            ("q", short.as_str()),
            ("langpair", &format!("{source}|{target}")),
        ])
        .timeout(std::time::Duration::from_secs(15))
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Không gọi được dịch vụ dịch ({e})"))?
        .error_for_status()
        .map_err(|e| anyhow::anyhow!("Dịch vụ dịch lỗi HTTP ({e})"))?;
    let resp: Value = http_resp
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("Dịch vụ dịch trả về không hợp lệ ({e})"))?;
    let out = parse_mymemory_response(&resp, &short).map_err(|e| anyhow::anyhow!("{e}"))?;
    if truncated {
        Ok(format!("{out}\n[dịch 1 phần]"))
    } else {
        Ok(out)
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct LinkMetadata {
    pub url: String,
    pub title: String,
    pub desc: String,
    pub thumb: String,
    pub src: String,
}

/// Extract the first `https://` or `http://` URL from a message text string.
pub fn extract_first_url(text: &str) -> Option<String> {
    let trimmed = text.trim();
    let pos = trimmed
        .find("https://")
        .or_else(|| trimmed.find("http://"))?;
    let after = &trimmed[pos..];
    let end = after.find(char::is_whitespace).unwrap_or(after.len());
    let url = after[..end].trim_end_matches(|c: char| matches!(c, ')' | ']' | ',' | ';'));
    if url.len() >= 10 {
        Some(url.to_string())
    } else {
        None
    }
}

fn unescape_html_entities(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&#x27;", "'")
        .replace("&apos;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&nbsp;", " ")
        .trim()
        .to_string()
}

fn extract_meta_attr(tag: &str, attr_name: &str) -> Option<String> {
    let lower = tag.to_ascii_lowercase();
    for pat in [
        format!("{attr_name}=\""),
        format!("{attr_name}='"),
        format!("{attr_name} = \""),
    ] {
        if let Some(idx) = lower.find(&pat) {
            let quote_char = pat.chars().last().unwrap_or('"');
            let rest = &tag[idx + pat.len()..];
            if let Some(end) = rest.find(quote_char) {
                let val = unescape_html_entities(&rest[..end]);
                if !val.is_empty() {
                    return Some(val);
                }
            }
        }
    }
    None
}

/// Scrape OpenGraph / Twitter Card / HTML `<meta>` tags directly from a webpage URL.
pub async fn scrape_opengraph_metadata(url: &str) -> Option<LinkMetadata> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(3800))
        .redirect(reqwest::redirect::Policy::limited(5))
        .build()
        .ok()?;
    let resp = client
        .get(url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (compatible; Discordbot/2.0; +https://discordapp.com)",
        )
        .header("Accept-Language", "vi-VN,vi;q=0.9,en-US;q=0.8")
        .send()
        .await
        .ok()?;
    if !resp.status().is_success() {
        return None;
    }
    let bytes = resp.bytes().await.ok()?;
    let take_len = bytes.len().min(131_072);
    let html = String::from_utf8_lossy(&bytes[..take_len]);

    let mut og_title = String::new();
    let mut og_desc = String::new();
    let mut og_thumb = String::new();
    let mut og_site = String::new();

    let mut cursor = html.as_ref();
    while let Some(start) = cursor.find('<') {
        let after = &cursor[start..];
        let Some(end) = after.find('>') else {
            break;
        };
        let tag = &after[..=end];
        cursor = &after[end + 1..];

        if tag
            .get(1..5)
            .map_or(false, |s| s.eq_ignore_ascii_case("meta"))
        {
            let key = extract_meta_attr(tag, "property")
                .or_else(|| extract_meta_attr(tag, "name"))
                .unwrap_or_default()
                .to_ascii_lowercase();
            let Some(content) = extract_meta_attr(tag, "content") else {
                continue;
            };
            match key.as_str() {
                "og:title" | "twitter:title" | "lark:url:video_title" if og_title.is_empty() => {
                    og_title = content;
                }
                "og:description" | "twitter:description" | "description" if og_desc.is_empty() => {
                    og_desc = content;
                }
                "og:image" | "twitter:image" | "lark:url:video_cover_image_url"
                    if og_thumb.is_empty() && content.starts_with("http") =>
                {
                    og_thumb = content;
                }
                "og:site_name" if og_site.is_empty() => {
                    og_site = content;
                }
                _ => {}
            }
        } else if og_title.is_empty()
            && tag
                .get(1..6)
                .map_or(false, |s| s.eq_ignore_ascii_case("title"))
        {
            if let Some(close) = cursor.find("</") {
                let raw_t = unescape_html_entities(&cursor[..close]);
                if !raw_t.is_empty() {
                    og_title = raw_t;
                }
            }
        }
    }

    let domain = url
        .split("://")
        .nth(1)
        .and_then(|r| r.split('/').next())
        .unwrap_or("Liên kết")
        .to_string();
    Some(LinkMetadata {
        url: url.to_string(),
        title: og_title,
        desc: og_desc,
        thumb: og_thumb,
        src: if !og_site.is_empty() { og_site } else { domain },
    })
}

fn is_placeholder_zalo_thumb(thumb: &str) -> bool {
    thumb.is_empty()
        || thumb.contains("feed_thumb_link")
        || thumb.contains("im_parselink")
        || thumb.contains("aHR0cHM6Ly9yZXMtemFsby56YWRuLnZuL3VwbG9hZC9tZWRpYS8yMDE5LzEwLzE1L2ZlZWRfdGh1bWJfbGlua19fMV9fMTU3MTEzMzEyMjc3OF85Mzc4MC5wbmc=")
}

/// Parse rich link metadata (`title`, `desc`, `thumb`, `src`) via Zalo's `/api/message/parselink`
/// (`cmd 11285`) combined with direct OpenGraph HTML scraping fallback.
pub async fn parse_link_metadata(s: &Session, raw_url: &str) -> Result<LinkMetadata> {
    let url = raw_url.trim().to_string();
    let domain = url
        .split("://")
        .nth(1)
        .and_then(|r| r.split('/').next())
        .unwrap_or("Liên kết")
        .to_string();
    let mut meta = LinkMetadata {
        url: url.clone(),
        title: String::new(),
        desc: String::new(),
        thumb: String::new(),
        src: domain.clone(),
    };

    if s.login.is_some() {
        let file_host = if s.domains.file.is_empty() {
            "https://tt-files-wpa.chat.zalo.me"
        } else {
            s.domains.file.trim_end_matches('/')
        };
        let body = obj(&[
            ("link", json!(url)),
            ("version", json!(1)),
            ("imei", json!(s.imei)),
        ]);
        if let Ok(params) = s.encoded_params(&body) {
            let parse_url = format!(
                "{file_host}/api/message/parselink?{}&params={params}",
                s.common_params()
            );
            if let Ok(resp) = s.get(&parse_url).await {
                let dec = s.maybe_decrypt(resp, s.session_key_bytes()).await;
                let d = dec
                    .get("data")
                    .and_then(|x| x.get("data").or(Some(x)))
                    .unwrap_or(&dec);
                if let Some(t) = d.get("title").and_then(Value::as_str).map(str::trim) {
                    if !t.is_empty() && t != url {
                        meta.title = t.to_string();
                    }
                }
                if let Some(ds) = d.get("desc").and_then(Value::as_str).map(str::trim) {
                    if !ds.is_empty() && ds != url {
                        meta.desc = ds.to_string();
                    }
                }
                if let Some(th) = d.get("thumb").and_then(Value::as_str).map(str::trim) {
                    if !is_placeholder_zalo_thumb(th) {
                        meta.thumb = th.to_string();
                    }
                }
                if let Some(sr) = d.get("src").and_then(Value::as_str).map(str::trim) {
                    if !sr.is_empty() {
                        meta.src = sr.to_string();
                    }
                }
            }
        }
    }

    if meta.title.is_empty() || meta.thumb.is_empty() || meta.desc.is_empty() {
        if let Some(og) = scrape_opengraph_metadata(&url).await {
            if meta.title.is_empty() && !og.title.is_empty() {
                meta.title = og.title;
            }
            if meta.desc.is_empty() && !og.desc.is_empty() {
                meta.desc = og.desc;
            }
            if meta.thumb.is_empty() && !og.thumb.is_empty() {
                meta.thumb = og.thumb;
            }
            if (meta.src.is_empty() || meta.src == domain) && !og.src.is_empty() {
                meta.src = og.src;
            }
        }
    }

    if meta.title.is_empty() {
        meta.title = domain;
    }
    Ok(meta)
}

/// Send a rich link message (`sendLinkMessage` @ `compact-app-pc.js:14319196`):
/// - 1-1 (`cmd 11286`): `POST {domains.chat}/api/message/link?{common}`
/// - Group (`cmd 11193`): `POST {domains.group}/api/group/sendlink?{common}`
pub async fn send_link_message(
    s: &Session,
    to_id: &str,
    text: &str,
    meta: &LinkMetadata,
    is_group: bool,
) -> Result<Value> {
    send_link_message_with_client_id(s, to_id, text, meta, is_group, next_client_message_id()).await
}

/// Preserve the request client ID for reconciliation with a later server echo.
pub async fn send_link_message_with_client_id(
    s: &Session,
    to_id: &str,
    text: &str,
    meta: &LinkMetadata,
    is_group: bool,
    now: u64,
) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let msg_str = if text.trim().is_empty() {
        meta.url.as_str()
    } else {
        text.trim()
    };
    let title_str = if meta.title.trim().is_empty() {
        meta.url.as_str()
    } else {
        meta.title.trim()
    };
    let media_json = json!({
        "type": 0,
        "count": 0,
        "mediaTitle": "",
        "artist": "",
        "streamUrl": "",
        "stream_icon": ""
    })
    .to_string();

    let res: Result<Value> = async {
        if is_group {
            let clean_grid = to_id
                .trim_start_matches("group_")
                .trim_start_matches('g')
                .trim_start_matches('G');
            let body = obj(&[
                ("grid", json!(clean_grid)),
                ("msg", json!(msg_str)),
                ("href", json!(meta.url)),
                ("src", json!(meta.src)),
                ("title", json!(title_str)),
                ("desc", json!(meta.desc)),
                ("thumb", json!(meta.thumb)),
                ("type", json!(0)),
                ("media", json!(media_json)),
                ("clientId", json!(now)),
                ("mentionInfo", json!("[]")),
                ("language", json!("vi")),
                ("ttl", json!(0)),
                ("imei", json!(s.imei)),
            ]);
            let params = s.encoded_params(&body)?;
            let url = format!(
                "{}/api/group/sendlink?{}",
                s.domains.group,
                s.common_params()
            );
            let resp = s.post_form(&url, format!("params={params}")).await?;
            let dec = s.maybe_decrypt(resp, s.session_key_bytes()).await;
            if let Some(code) = dec.get("error_code").and_then(Value::as_i64) {
                if code != 0 {
                    anyhow::bail!("sendlink group error {code}");
                }
            }
            Ok(dec)
        } else {
            let host = if s.domains.chat.contains("-msg-chat.")
                || s.domains.chat.contains("tt-chat2-wpa")
                || s.domains.chat.is_empty()
            {
                "https://tt-chat3-wpa.api.zaloapp.com"
            } else {
                &s.domains.chat
            };
            let body = obj(&[
                ("toId", json!(to_id)),
                ("msg", json!(msg_str)),
                ("href", json!(meta.url)),
                ("src", json!(meta.src)),
                ("title", json!(title_str)),
                ("desc", json!(meta.desc)),
                ("thumb", json!(meta.thumb)),
                ("type", json!(0)),
                ("media", json!(media_json)),
                ("clientId", json!(now)),
                ("mentionInfo", json!("")),
                ("language", json!("vi")),
                ("ttl", json!(0)),
                ("imei", json!(s.imei)),
            ]);
            let params = s.encoded_params(&body)?;
            let url = format!("{host}/api/message/link?{}", s.common_params());
            let resp = s.post_form(&url, format!("params={params}")).await?;
            let dec = s.maybe_decrypt(resp, s.session_key_bytes()).await;
            if let Some(code) = dec.get("error_code").and_then(Value::as_i64) {
                if code != 0 {
                    anyhow::bail!("sendlink error {code}");
                }
            }
            Ok(dec)
        }
    }
    .await;

    match res {
        Ok(v) => Ok(v),
        Err(err) => {
            crate::logger::log_error(
                "chat::send_link_message",
                &format!("sendlink failed ({err:?}), falling back to text message with URL"),
            );
            let fallback_text = if msg_str.contains(meta.url.as_str()) {
                msg_str.to_string()
            } else if msg_str.is_empty() {
                meta.url.clone()
            } else {
                format!("{msg_str}\n{}", meta.url)
            };
            if is_group {
                send_group_message_with_client_id(s, to_id, &fallback_text, now).await
            } else {
                send_message_with_client_id(s, to_id, &fallback_text, now).await
            }
        }
    }
}

/// Send a sticker (`sendSticker` in `compact-app-pc.b7bec208c4c8f73e09f8.js:14325410`):
/// - 1-1 (`cmd 11154`): `POST {domains.chat}/api/message/sticker?{common}`
/// - Group (`cmd 11165`): `POST {domains.group}/api/group/sticker?{common}`
pub async fn send_sticker(
    s: &Session,
    to_id: &str,
    sticker_id: u64,
    cate_id: u64,
    sticker_type: u32,
    is_group: bool,
) -> Result<Value> {
    send_sticker_with_client_id(
        s,
        to_id,
        sticker_id,
        cate_id,
        sticker_type,
        is_group,
        next_client_message_id(),
    )
    .await
}

/// Preserve the request client ID for reconciliation with a later server echo.
pub(crate) async fn send_sticker_with_client_id(
    s: &Session,
    to_id: &str,
    sticker_id: u64,
    cate_id: u64,
    sticker_type: u32,
    is_group: bool,
    now: u64,
) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }

    let url = if is_group {
        let clean_grid = to_id
            .trim_start_matches("group_")
            .trim_start_matches('g')
            .trim_start_matches('G');
        let body = obj(&[
            ("grid", json!(clean_grid)),
            ("stickerId", json!(sticker_id)),
            ("cateId", json!(cate_id)),
            ("type", json!(sticker_type)),
            ("clientId", json!(now)),
            ("imei", json!(s.imei)),
            ("zsource", json!(101)),
        ]);
        let params = s.encoded_params(&body)?;
        let url = format!(
            "{}/api/group/sticker?{}",
            s.domains.group,
            s.common_params()
        );
        let form = format!("params={params}");
        let resp = s.post_form(&url, form).await?;
        let resp = s.maybe_decrypt(resp, s.session_key_bytes()).await;
        if let Some(code) = resp.get("error_code").and_then(Value::as_i64) {
            if code != 0 {
                let msg = resp
                    .get("error_message")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown");
                return Err(crate::session::SessionError::new(code, msg).into());
            }
        }
        return Ok(resp);
    } else {
        let host = if s.domains.chat.contains("-msg-chat.")
            || s.domains.chat.contains("tt-chat2-wpa")
            || s.domains.chat.is_empty()
        {
            "https://tt-chat3-wpa.api.zaloapp.com"
        } else {
            &s.domains.chat
        };
        format!("{host}/api/message/sticker?{}", s.common_params())
    };

    let body = obj(&[
        ("toid", json!(to_id)),
        ("stickerId", json!(sticker_id)),
        ("cateId", json!(cate_id)),
        ("type", json!(sticker_type)),
        ("clientId", json!(now)),
        ("imei", json!(s.imei)),
        ("zsource", json!(101)),
    ]);
    let params = s.encoded_params(&body)?;
    let form = format!("params={params}");
    let resp = s.post_form(&url, form).await?;
    let resp = s.maybe_decrypt(resp, s.session_key_bytes()).await;
    if let Some(code) = resp.get("error_code").and_then(Value::as_i64) {
        if code != 0 {
            let msg = resp
                .get("error_message")
                .and_then(Value::as_str)
                .unwrap_or("unknown");
            return Err(crate::session::SessionError::new(code, msg).into());
        }
    }
    Ok(resp)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StickerPackInfo {
    pub id: u64,
    pub name: String,
    pub total_image: u32,
    pub icon_url: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StickerItem {
    pub sticker_id: u64,
    pub cate_id: u64,
    pub sticker_type: u32,
    pub label: String,
}

/// Fetch the complete Zalo Sticker Store catalog (`getStickerCatsV2` -> `getAllCate` in `compact-app-pc.js:14288962`).
///
/// Returns ~390 official Zalo sticker packs sorted by newest/featured first and caches them to
/// `store_packs.json` in the active account's media cache.
pub async fn fetch_sticker_store_packs(s: &Session) -> Result<Vec<StickerPackInfo>> {
    let cache_dir = crate::account_manager::ensure_media_cache_dir();
    let cache_path = cache_dir.join("store_packs.json");
    if let Ok(raw_cache) = std::fs::read_to_string(&cache_path) {
        if let Ok(cached) = serde_json::from_str::<Vec<StickerPackInfo>>(&raw_cache) {
            if cached.len() >= 20 {
                return Ok(cached);
            }
        }
    }

    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let sticker_host = if s.domains.sticker.is_empty() {
        "https://tt-sticker-wpa.chat.zalo.me"
    } else {
        s.domains.sticker.trim_end_matches('/')
    };
    let key = s.session_key_bytes();
    if key.is_empty() {
        anyhow::bail!("thiếu session key để tải kho nhãn dán");
    }
    let enc_empty = crate::crypto::encode_aes(key, "")?;
    let mut encoded_param = String::with_capacity(enc_empty.len() + 16);
    for b in enc_empty.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded_param.push(b as char);
            }
            _ => {
                use std::fmt::Write;
                let _ = write!(encoded_param, "%{b:02X}");
            }
        }
    }

    let list_url = format!(
        "{sticker_host}/api/message/sticker/category/list/v2?{}&params={encoded_param}",
        s.common_params()
    );
    let resp = s.get(&list_url).await?;
    let dec = s.maybe_decrypt(resp, key).await;
    let data = dec.get("data").unwrap_or(&dec);

    let ex_ids: std::collections::HashSet<u64> = data
        .get("exCateIds")
        .and_then(Value::as_array)
        .map(|arr| arr.iter().filter_map(Value::as_u64).collect())
        .unwrap_or_default();

    let data_url = data
        .get("dataUrl")
        .and_then(Value::as_str)
        .unwrap_or("https://zalo-api.zadn.vn/api/emoticon/pcweb/getAllCate")
        .replace("http://", "https://");

    let cdn_resp = s.http.get(&data_url).send().await?;
    let raw_bytes = cdn_resp.bytes().await?;
    let json_text = if raw_bytes.len() >= 2 && raw_bytes[0] == 0x1f && raw_bytes[1] == 0x8b {
        use std::io::Read;
        let mut gz = flate2::read::GzDecoder::new(&raw_bytes[..]);
        let mut out = String::new();
        gz.read_to_string(&mut out)?;
        out
    } else {
        String::from_utf8_lossy(&raw_bytes).into_owned()
    };

    let arr: Vec<Value> = serde_json::from_str(&json_text)?;
    let mut packs = Vec::with_capacity(arr.len());
    for item in arr {
        let Some(id) = item.get("id").and_then(Value::as_u64) else {
            continue;
        };
        if id == 0 || ex_ids.contains(&id) {
            continue;
        }
        let name = item
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or("")
            .trim()
            .to_string();
        if name.is_empty() || name.to_lowercase().contains("test") {
            continue;
        }
        let total_image = item.get("totalImage").and_then(Value::as_u64).unwrap_or(16) as u32;
        let icon_url = item
            .get("iconUrl")
            .or_else(|| item.get("thumbImg"))
            .and_then(Value::as_str)
            .unwrap_or("")
            .replace("http://", "https://");
        packs.push(StickerPackInfo {
            id,
            name,
            total_image,
            icon_url,
        });
    }

    // Sort newest packs first (`id` descending) so modern Zalo packs appear at the top of the store
    packs.sort_by(|a, b| b.id.cmp(&a.id));
    if let Ok(txt) = serde_json::to_string(&packs) {
        let _ = std::fs::write(&cache_path, txt);
    }
    Ok(packs)
}

/// Fetch all stickers belonging to a specific category (`getStickerCategoryDetail` — `cmd 11284`).
pub async fn fetch_sticker_pack_items(
    s: &Session,
    cate_id: u64,
    pack_name: &str,
) -> Result<Vec<StickerItem>> {
    if cate_id == 0 {
        return Ok((1..=30)
            .map(|id| StickerItem {
                sticker_id: id,
                cate_id: 0,
                sticker_type: 7,
                label: format!("Củ hành #{id}"),
            })
            .collect());
    }

    let cache_dir = crate::account_manager::ensure_media_cache_dir();
    let cache_path = cache_dir.join(format!("pack_{cate_id}.json"));
    if let Ok(raw_cache) = std::fs::read_to_string(&cache_path) {
        if let Ok(cached) = serde_json::from_str::<Vec<StickerItem>>(&raw_cache) {
            if !cached.is_empty() {
                return Ok(cached);
            }
        }
    }

    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let sticker_host = if s.domains.sticker.is_empty() {
        "https://tt-sticker-wpa.chat.zalo.me"
    } else {
        s.domains.sticker.trim_end_matches('/')
    };
    let params = s.encoded_params(&json!({ "cid": cate_id.to_string() }))?;
    let url = format!(
        "{sticker_host}/api/message/sticker/category/sticker_detail?{}&params={params}",
        s.common_params()
    );
    let resp = s.get(&url).await?;
    let dec = s.maybe_decrypt(resp, s.session_key_bytes()).await;
    let arr = dec
        .get("data")
        .and_then(Value::as_array)
        .or_else(|| dec.as_array())
        .cloned()
        .unwrap_or_default();

    let mut items = Vec::with_capacity(arr.len());
    for (idx, it) in arr.iter().enumerate() {
        let Some(sid) = it
            .get("id")
            .or_else(|| it.get("stickerId"))
            .and_then(Value::as_u64)
        else {
            continue;
        };
        let cid = it.get("cateId").and_then(Value::as_u64).unwrap_or(cate_id);
        let stype = it.get("type").and_then(Value::as_u64).unwrap_or(7) as u32;
        items.push(StickerItem {
            sticker_id: sid,
            cate_id: cid,
            sticker_type: stype,
            label: format!("{pack_name} #{}", idx + 1),
        });
    }

    if !items.is_empty() {
        if let Ok(txt) = serde_json::to_string(&items) {
            let _ = std::fs::write(&cache_path, txt);
        }
    }
    Ok(items)
}

/// Normalize a sticker keyword by converting non-breaking spaces (`\u{a0}`), collapsing
/// consecutive whitespace to a single ASCII space, and converting to lowercase.
pub fn normalize_sticker_keyword(s: &str) -> String {
    s.replace('\u{a0}', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// Strip Vietnamese diacritics to lowercase ASCII (`simpleStripVietnamese` in `compact-app-pc.js:12955017`).
pub fn strip_vn_for_sticker_kw(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for raw in s.chars() {
        for c in raw.to_lowercase() {
            let mapped = match c {
                'à' | 'á' | 'ạ' | 'ả' | 'ã' | 'â' | 'ầ' | 'ấ' | 'ậ' | 'ẩ' | 'ẫ' | 'ă' | 'ằ'
                | 'ắ' | 'ặ' | 'ẳ' | 'ẵ' => 'a',
                'è' | 'é' | 'ẹ' | 'ẻ' | 'ẽ' | 'ê' | 'ề' | 'ế' | 'ệ' | 'ể' | 'ễ' => {
                    'e'
                }
                'ì' | 'í' | 'ị' | 'ỉ' | 'ĩ' => 'i',
                'ò' | 'ó' | 'ọ' | 'ỏ' | 'õ' | 'ô' | 'ồ' | 'ố' | 'ộ' | 'ổ' | 'ỗ' | 'ơ' | 'ờ'
                | 'ớ' | 'ợ' | 'ở' | 'ỡ' => 'o',
                'ù' | 'ú' | 'ụ' | 'ủ' | 'ũ' | 'ư' | 'ừ' | 'ứ' | 'ự' | 'ử' | 'ữ' => {
                    'u'
                }
                'ỳ' | 'ý' | 'ỵ' | 'ỷ' | 'ỹ' => 'y',
                'đ' => 'd',
                '\u{a0}' => ' ',
                other => other,
            };
            out.push(mapped);
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Map common unaccented Vietnamese keywords, English terms, or chat slang to the accented
/// Vietnamese form indexed by Zalo's `/api/message/sticker/suggest/stickers` (`cmd 11741`) and
/// `/api/message/sticker/search` (`cmd 11747`) endpoints.
pub fn canonical_accented_vn_for_sticker(stripped: &str) -> Option<&'static str> {
    match stripped {
        "nho" | "nho em" | "nho anh" | "nho qua" | "miss" | "miss u" | "miss you" => Some("nhớ"),
        "khoc" | "huhu" | "huhuhu" | "hic" | "cry" => Some("khóc"),
        "buon" | "buon qua" | "sad" | "tui than" => Some("buồn"),
        "vui" | "vui ve" | "vui qua" | "happy" => Some("vui"),
        "cuoi" | "mac cuoi" | "buon cuoi" => Some("cười"),
        "yeu" | "iu" | "yeu em" | "yeu anh" | "iu em" | "iu anh" | "love" | "love u"
        | "love you" | "tha tim" | "tim" => Some("yêu"),
        "thuong" | "thuong em" | "thuong anh" | "thuong qua" => Some("thương"),
        "gian" | "gian doi" | "doi hon" | "tuc" | "buc" | "buc minh" | "cay" | "angry" => {
            Some("giận")
        }
        "met" | "met qua" | "met moi" | "duoi" | "duoi qua" | "tired" => Some("mệt"),
        "doi" | "doi bung" | "doi qua" | "hungry" => Some("đói"),
        "an" | "an com" | "di an" => Some("ăn cơm"),
        "ca phe" | "cf" | "cafe" | "coffee" => Some("cà phê"),
        "tra sua" | "trasua" => Some("trà sữa"),
        "nhau" | "di nhau" | "bia" | "dzoo" | "dzo" => Some("nhậu"),
        "ngu" | "di ngu" | "buon ngu" | "ngu ngon" | "ngu ngon nhe" | "ngu ngon nha"
        | "good night" | "goodnight" | "gn" | "g9" => Some("ngủ"),
        "chao" | "xin chao" | "chao ban" | "chao anh" | "chao chi" | "chao em" | "he lo"
        | "helo" => Some("xin chào"),
        "cam on" | "cam on ban" | "cam on anh" | "cam on chi" | "cam on em" | "cam on nhieu"
        | "thank" | "thanks" | "thank you" | "tks" | "tkss" | "thank kiu" => Some("cảm ơn"),
        "xin loi" | "xin loi nha" | "xin loi nhe" | "sorry" | "sr" | "so ry" => Some("xin lỗi"),
        "tam biet" | "tam biet nhe" | "bye" | "byebye" | "good bye" | "goodbye" | "bibi" | "pp" => {
            Some("tạm biệt")
        }
        "chuc mung" | "happy birthday" | "hpbd" | "sinh nhat" | "cmsn" => Some("chúc mừng"),
        "tet" | "chuc mung nam moi" | "nam moi" => Some("tết"),
        "co len" | "co gang" | "fighting" => Some("cố lên"),
        "tuyet" | "tuyet voi" | "xuat sac" | "dinh" | "dinh qua" | "chuan" | "duoc" => {
            Some("tuyệt vời")
        }
        "ngai" | "ngai qua" | "ngai nhi" | "xau ho" => Some("ngại"),
        "so" | "so qua" | "scared" => Some("sợ"),
        "lanh" | "lanh qua" | "lanh khong" | "ret" | "ret qua" => Some("lạnh"),
        "nong" | "nong qua" | "nong buc" => Some("nóng"),
        "tien" | "luong" => Some("tiền"),
        "het tien" | "ngheo" | "ngheo qua" | "vi rong" => Some("hết tiền"),
        "quy" | "lay" | "xin" | "van xin" => Some("lạy"),
        "ban" | "ban qua" | "busy" => Some("bận"),
        "chan" | "chan qua" | "chan doi" | "boring" => Some("chán"),
        "cho" | "cho chut" | "cho xiu" | "doi chut" | "doi xiu" | "wait" => Some("chờ chút"),
        "di choi" | "choi" | "dao pho" => Some("đi chơi"),
        "di lam" | "lam viec" | "deadline" | "code" => Some("làm"),
        "hong" | "hong chuyen" | "ngac nhien" => Some("ngạc nhiên"),
        _ => None,
    }
}

/// Build the deduplicated list of query variants (accented + unaccented + canonical synonym)
/// to maximize recall across both Zalo Sticker Suggest (`cmd 11741`) and Search (`cmd 11747`).
pub fn canonical_vn_sticker_queries(keyword: &str) -> Vec<String> {
    let norm = normalize_sticker_keyword(keyword);
    if norm.is_empty() {
        return Vec::new();
    }
    let stripped = strip_vn_for_sticker_kw(&norm);
    let mut queries = vec![norm.clone()];
    if !stripped.is_empty() && stripped != norm && !queries.contains(&stripped) {
        queries.push(stripped.clone());
    }
    if let Some(accented) = canonical_accented_vn_for_sticker(&stripped) {
        let acc_s = accented.to_string();
        if !queries.contains(&acc_s) {
            queries.push(acc_s.clone());
        }
        let acc_stripped = strip_vn_for_sticker_kw(&acc_s);
        if !acc_stripped.is_empty() && !queries.contains(&acc_stripped) {
            queries.push(acc_stripped);
        }
    }
    queries.truncate(4);
    queries
}

/// Fetch official Zalo contextual sticker trigger keywords (`getStickerListKeyWords` — `cmd 11740`).
/// Includes both normalized accented/raw forms and ASCII-folded forms so typing accented or
/// unaccented Vietnamese triggers suggestions reliably.
pub async fn fetch_sticker_suggest_keywords(s: &Session) -> Result<Vec<String>> {
    let cache_dir = crate::account_manager::ensure_media_cache_dir();
    let cache_path = cache_dir.join("suggest_keywords_v2.json");
    if let Ok(raw_cache) = std::fs::read_to_string(&cache_path) {
        if let Ok(cached) = serde_json::from_str::<Vec<String>>(&raw_cache) {
            if cached.len() >= 100 {
                return Ok(cached);
            }
        }
    }
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let sticker_host = if s.domains.sticker.is_empty() {
        "https://tt-sticker-wpa.chat.zalo.me"
    } else {
        s.domains.sticker.trim_end_matches('/')
    };
    let params = s.encoded_params(&json!({ "imei": s.imei }))?;
    let url = format!(
        "{sticker_host}/api/message/sticker/suggest/keywords?{}&params={params}",
        s.common_params()
    );
    let resp = match s.get(&url).await {
        Ok(r) => r,
        Err(_) => {
            let fallback_url = format!(
                "https://tt-sticker-wpa.zaloapp.com/api/message/sticker/suggest/keywords?{}&params={params}",
                s.common_params()
            );
            s.get(&fallback_url).await?
        }
    };
    let dec = s.maybe_decrypt(resp, s.session_key_bytes()).await;
    let mut seen = std::collections::HashSet::new();
    let mut kws: Vec<String> = Vec::new();
    if let Some(arr) = dec
        .get("data")
        .or(Some(&dec))
        .and_then(|d| d.get("keywords"))
        .and_then(Value::as_array)
    {
        for raw_v in arr {
            let Some(raw_str) = raw_v.as_str() else {
                continue;
            };
            let norm = normalize_sticker_keyword(raw_str);
            if norm.is_empty() || norm.starts_with("zkwd_") {
                continue;
            }
            if seen.insert(norm.clone()) {
                kws.push(norm.clone());
            }
            let stripped = strip_vn_for_sticker_kw(&norm);
            if !stripped.is_empty() && seen.insert(stripped.clone()) {
                kws.push(stripped);
            }
        }
    }
    if !kws.is_empty() {
        if let Ok(txt) = serde_json::to_string(&kws) {
            let _ = std::fs::write(&cache_path, txt);
        }
    }
    Ok(kws)
}

async fn query_sticker_endpoints_once(
    s: &Session,
    sticker_host: &str,
    queries: &[String],
) -> Vec<(u64, u64, u32, u8)> {
    let mut futs = Vec::with_capacity(queries.len() * 2);
    for (q_idx, q) in queries.iter().enumerate() {
        // 1) Contextual suggest endpoint (`getStickerSuggestion` — `cmd 11741`)
        let sugg_body = obj(&[
            ("keyword", json!(q)),
            ("gif", json!(1)),
            ("guggy", json!(0)),
            ("imei", json!(s.imei)),
        ]);
        if let Ok(params) = s.encoded_params(&sugg_body) {
            let url = format!(
                "{sticker_host}/api/message/sticker/suggest/stickers?{}&params={params}",
                s.common_params()
            );
            let is_primary = q_idx == 0;
            futs.push(async move {
                let Ok(resp) = s.get(&url).await else {
                    return Vec::new();
                };
                let dec = s.maybe_decrypt(resp, s.session_key_bytes()).await;
                let arr = dec
                    .get("data")
                    .or(Some(&dec))
                    .and_then(|d| d.get("sugg_sticker"))
                    .and_then(Value::as_array)
                    .cloned()
                    .unwrap_or_default();
                let mut items = Vec::with_capacity(arr.len());
                for it in arr {
                    let Some(sid) = it
                        .get("sticker_id")
                        .or_else(|| it.get("id"))
                        .and_then(Value::as_u64)
                    else {
                        continue;
                    };
                    let cid = it
                        .get("cate_id")
                        .or_else(|| it.get("cateId"))
                        .and_then(Value::as_u64)
                        .unwrap_or(0);
                    let stype = it.get("type").and_then(Value::as_u64).unwrap_or(7) as u32;
                    items.push((cid, sid, stype, if is_primary { 3u8 } else { 2u8 }));
                }
                items
            });
        }
    }

    let mut search_futs = Vec::with_capacity(queries.len());
    for (q_idx, q) in queries.iter().enumerate() {
        // 2) Full sticker tag search endpoint (`searchSticker` — `cmd 11747`)
        let search_body = obj(&[
            ("keyword", json!(q)),
            ("limit", json!(30)),
            ("srcType", json!(0)),
            ("imei", json!(s.imei)),
        ]);
        if let Ok(params) = s.encoded_params(&search_body) {
            let url = format!(
                "{sticker_host}/api/message/sticker/search?{}&params={params}",
                s.common_params()
            );
            let is_primary = q_idx == 0;
            search_futs.push(async move {
                let Ok(resp) = s.get(&url).await else {
                    return Vec::new();
                };
                let dec = s.maybe_decrypt(resp, s.session_key_bytes()).await;
                let arr = dec
                    .get("data")
                    .and_then(Value::as_array)
                    .or_else(|| dec.as_array())
                    .cloned()
                    .unwrap_or_default();
                let mut items = Vec::with_capacity(arr.len());
                for it in arr {
                    let Some(sid) = it
                        .get("sticker_id")
                        .or_else(|| it.get("id"))
                        .and_then(Value::as_u64)
                    else {
                        continue;
                    };
                    let cid = it
                        .get("cate_id")
                        .or_else(|| it.get("cateId"))
                        .and_then(Value::as_u64)
                        .unwrap_or(0);
                    let stype = it.get("type").and_then(Value::as_u64).unwrap_or(7) as u32;
                    items.push((cid, sid, stype, if is_primary { 3u8 } else { 2u8 }));
                }
                items
            });
        }
    }

    let (sugg_batches, search_batches) = tokio::join!(
        futures_util::future::join_all(futs),
        futures_util::future::join_all(search_futs)
    );

    // Merge and boost stickers that matched across multiple endpoints/queries
    let mut score_map: std::collections::HashMap<u64, (u64, u32, u8)> =
        std::collections::HashMap::new();
    for batch in sugg_batches.into_iter().chain(search_batches.into_iter()) {
        for (cid, sid, stype, weight) in batch {
            score_map
                .entry(sid)
                .and_modify(|e| {
                    e.2 = e.2.saturating_add(weight);
                    if e.0 == 0 && cid > 0 {
                        e.0 = cid;
                    }
                })
                .or_insert((cid, stype, weight));
        }
    }

    score_map
        .into_iter()
        .map(|(sid, (cid, stype, score))| (cid, sid, stype, score))
        .collect()
}

/// Search Zalo stickers by Vietnamese, English, or emoji keyword by querying both
/// `/api/message/sticker/suggest/stickers` (`cmd 11741`) and `/api/message/sticker/search` (`cmd 11747`)
/// concurrently across accented and unaccented Vietnamese variants.
pub async fn search_stickers_by_keyword(s: &Session, keyword: &str) -> Result<Vec<StickerItem>> {
    let kw = normalize_sticker_keyword(keyword);
    if kw.is_empty() {
        return Ok(Vec::new());
    }
    let cache_dir = crate::account_manager::ensure_media_cache_dir();
    let cache_path = cache_dir.join(format!("suggest_v2_{}.json", crate::crypto::md5_hex(&kw)));
    if let Ok(raw_cache) = std::fs::read_to_string(&cache_path) {
        if let Ok(cached) = serde_json::from_str::<Vec<StickerItem>>(&raw_cache) {
            if !cached.is_empty() {
                return Ok(cached);
            }
        }
    }
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let sticker_host = if s.domains.sticker.is_empty() {
        "https://tt-sticker-wpa.chat.zalo.me"
    } else {
        s.domains.sticker.trim_end_matches('/')
    };

    let queries = canonical_vn_sticker_queries(&kw);
    let mut raw_items = query_sticker_endpoints_once(s, sticker_host, &queries).await;

    // If a multi-word sentence/phrase (e.g. "anh ơi đi chơi", "hôm nay mệt quá") had no direct match,
    // automatically fallback to its trailing 2-word phrase (`tail2`) or trailing word (`last_word`).
    if raw_items.is_empty() {
        let words: Vec<&str> = kw.split_whitespace().collect();
        if words.len() >= 2 && words.len() <= 5 {
            let mut fallback_phrases = Vec::new();
            if words.len() >= 3 {
                fallback_phrases.push(format!(
                    "{} {}",
                    words[words.len() - 2],
                    words[words.len() - 1]
                ));
            }
            let last_w = words[words.len() - 1];
            if last_w.chars().count() >= 2 {
                fallback_phrases.push(last_w.to_string());
            }
            for fb in fallback_phrases {
                let fb_queries = canonical_vn_sticker_queries(&fb);
                raw_items = query_sticker_endpoints_once(s, sticker_host, &fb_queries).await;
                if !raw_items.is_empty() {
                    break;
                }
            }
        }
    }

    // Prioritize modern illustrated Zalo character packs (`cate_id` descending, matching Zalo PC's
    /// `pointCate` ranking) and exclude legacy round-emoji/photo-meme packs when character packs exist.
    const LEGACY_NON_CHAR_PACKS: &[u64] = &[
        0, 50, 10788, 11190, 11984, 11998, 12002, 12003, 12024, 12658,
    ];
    let has_modern_char = raw_items
        .iter()
        .any(|(cid, _, _, _)| *cid >= 10000 && !LEGACY_NON_CHAR_PACKS.contains(cid));
    if has_modern_char {
        raw_items.retain(|(cid, _, _, _)| *cid >= 10000 && !LEGACY_NON_CHAR_PACKS.contains(cid));
    }
    // Sort by relevance score first (matched in both search + suggest), then newest pack (`cate_id` desc)
    raw_items.sort_by(|a, b| b.3.cmp(&a.3).then_with(|| b.0.cmp(&a.0)));

    // Keep at most 2 stickers from the same pack in the first pass so suggestions show diverse characters,
    // then fill up to 24 items if needed for Sticker Picker search grids.
    let mut pack_counts: std::collections::HashMap<u64, usize> = std::collections::HashMap::new();
    let mut out = Vec::with_capacity(24);
    let mut deferred = Vec::new();
    for &(cid, sid, stype, _) in &raw_items {
        let cnt = pack_counts.entry(cid).or_insert(0);
        if *cnt >= 2 {
            deferred.push((cid, sid, stype));
            continue;
        }
        *cnt += 1;
        let idx = out.len() + 1;
        out.push(StickerItem {
            sticker_id: sid,
            cate_id: cid,
            sticker_type: stype,
            label: format!("{kw} #{idx}"),
        });
        if out.len() >= 24 {
            break;
        }
    }
    for (cid, sid, stype) in deferred {
        if out.len() >= 24 {
            break;
        }
        let idx = out.len() + 1;
        out.push(StickerItem {
            sticker_id: sid,
            cate_id: cid,
            sticker_type: stype,
            label: format!("{kw} #{idx}"),
        });
    }

    if !out.is_empty() {
        if let Ok(txt) = serde_json::to_string(&out) {
            let _ = std::fs::write(&cache_path, txt);
        }
    }
    Ok(out)
}

/// Send a quote/reply message (`sendQuoteMessage` / `sendGroupQuoteMessage` in `compact-app-pc.b7bec208c4c8f73e09f8.js:14317494`):
/// - 1-1 (`cmd 11771`): `POST {domains.chat}/api/message/quote?{common}`
/// - Group (`cmd 11197`): `POST {domains.group}/api/group/quote?{common}`
/// Automatically falls back to `send_message` / `send_group_message` if the quoted message lacks a server ID.
pub async fn send_quote_message(
    s: &Session,
    to_id: &str,
    text: &str,
    quoted: &ChatMessage,
    is_group: bool,
) -> Result<Value> {
    send_quote_message_with_client_id(s, to_id, text, quoted, is_group, next_client_message_id())
        .await
}

/// Wire identity of a quoted message, as the quote endpoints expect it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct QuoteTarget {
    /// `qmsgOwner`: uid of the quoted message's sender.
    pub owner: String,
    /// `qmsgId`: server `msgId`.
    pub msg_id: Option<String>,
    /// `qmsgCliId`: sender's `cliMsgId`.
    pub cli_msg_id: Option<String>,
}

/// Resolve `qmsgOwner` / `qmsgId` / `qmsgCliId` from wire identity.
///
/// `ChatMessage::sender` is a display name for many group rows, and `ChatMessage::id`
/// may hold the `cliMsgId` (`35_exact_message_identity.md`), so neither is used directly.
pub(crate) fn quote_target(
    quoted: &ChatMessage,
    my_uid: &str,
    to_id: &str,
    is_group: bool,
) -> QuoteTarget {
    let owner = if quoted.is_outgoing {
        my_uid.to_string()
    } else if let Some(uid) = sidebar_sender_id(quoted) {
        uid
    } else if !quoted.sender.is_empty() && quoted.sender.bytes().all(|b| b.is_ascii_digit()) {
        quoted.sender.clone()
    } else if !is_group {
        to_id.to_string()
    } else {
        "0".to_string()
    };
    QuoteTarget {
        owner,
        msg_id: server_message_id(quoted),
        cli_msg_id: client_message_id(quoted),
    }
}

fn is_video_url_like(url: &str) -> bool {
    let l = url.trim().to_ascii_lowercase();
    (l.starts_with("http://") || l.starts_with("https://"))
        && (l.ends_with(".mp4")
            || l.ends_with(".mov")
            || l.ends_with(".webm")
            || l.ends_with(".mkv")
            || l.ends_with(".avi")
            || l.contains("/video/")
            || l.contains("videomedia")
            || l.contains("video-")
            || l.contains("video.")
            || l.contains(".dlmd.me"))
}

fn is_image_filename_only(s: &str) -> bool {
    let l = s.trim().to_ascii_lowercase();
    [
        ".png", ".jpg", ".jpeg", ".webp", ".gif", ".bmp", ".jfif", ".tiff", ".heic", ".avif",
    ]
    .iter()
    .any(|ext| l.ends_with(ext))
}

fn is_raw_json_literal(s: &str) -> bool {
    let t = s.trim();
    ((t.starts_with('{') && t.ends_with('}')) || (t.starts_with('[') && t.ends_with(']')))
        && serde_json::from_str::<Value>(t).is_ok()
}

fn clean_quote_text_candidate(raw: &str) -> Option<String> {
    let t = raw.trim();
    if t.is_empty() || t == "sendBubbleMessage" || is_raw_json_literal(t) {
        return None;
    }
    Some(t.to_string())
}

fn clean_media_quote_caption(
    raw: Option<&str>,
    reject_image_filenames: bool,
    extra_skip: &[&str],
) -> Option<String> {
    let t = clean_quote_text_candidate(raw?)?;
    if t.starts_with("http://") || t.starts_with("https://") {
        return None;
    }
    if t.len() > 20 && !t.contains(' ') && !t.contains('.') {
        return None;
    }
    if extra_skip.iter().any(|s| t.eq_ignore_ascii_case(s)) {
        return None;
    }
    if reject_image_filenames && is_image_filename_only(&t) {
        return None;
    }
    Some(t)
}

fn format_media_quote_label(
    prefix: &str,
    alt_prefix: Option<&str>,
    caption: Option<String>,
) -> String {
    let Some(cap) = caption else {
        return prefix.to_string();
    };
    let c = cap.trim();
    if c.is_empty() || c == prefix || alt_prefix.is_some_and(|a| c == a) {
        return prefix.to_string();
    }
    if c.starts_with(prefix) && c[prefix.len()..].starts_with(char::is_whitespace) {
        let rest = c[prefix.len()..].trim();
        return if rest.is_empty() {
            prefix.to_string()
        } else {
            format!("{prefix} {rest}")
        };
    }
    if let Some(alt) = alt_prefix {
        if c.starts_with(alt) && c[alt.len()..].starts_with(char::is_whitespace) {
            let rest = c[alt.len()..].trim();
            return if rest.is_empty() {
                prefix.to_string()
            } else {
                format!("{prefix} {rest}")
            };
        }
    }
    format!("{prefix} {c}")
}

/// Returns `true` when `reply_to_text` is absent, blank, or carries unparsed wire JSON / CDN URLs.
pub(crate) fn is_missing_or_raw_quote_text(text: Option<&str>) -> bool {
    match text.map(str::trim) {
        None | Some("") => true,
        Some(t) => {
            is_raw_json_literal(t)
                || t == "sendBubbleMessage"
                || is_zalo_photo_cdn_url(t)
                || is_video_url_like(t)
        }
    }
}

/// Canonical display summary for a `ChatMessage` when it is quoted in a reply.
pub(crate) fn quote_preview_text(quoted: &ChatMessage) -> String {
    let raw_text = quoted
        .text
        .strip_prefix("[Đã thu hồi] ")
        .unwrap_or(&quoted.text)
        .trim();

    if quoted.sticker_id.is_some()
        || quoted.kind == crate::message_model::MessageKind::Sticker
        || raw_text == "[Nhãn dán]"
    {
        return "[Nhãn dán]".to_string();
    }
    if quoted.kind == crate::message_model::MessageKind::Voice
        || quoted.link_src.as_deref() == Some("Thoại Zalo")
        || raw_text == "[Tin nhắn thoại]"
    {
        return "[Tin nhắn thoại]".to_string();
    }
    if quoted.kind == crate::message_model::MessageKind::Video
        || quoted.link_src.as_deref() == Some("Video Zalo")
        || raw_text == "[Video]"
        || raw_text.starts_with("[Video] ")
        || quoted.link_url.as_deref().is_some_and(is_video_url_like)
    {
        let cap = clean_media_quote_caption(Some(raw_text), false, &["Video", "[Tập tin]"])
            .filter(|s| !s.starts_with("[Tập tin]"))
            .or_else(|| clean_media_quote_caption(quoted.link_desc.as_deref(), false, &["Video"]));
        return format_media_quote_label("[Video]", None, cap);
    }
    let is_photo_like = quoted.kind == crate::message_model::MessageKind::Photo
        || quoted.kind == crate::message_model::MessageKind::Gif
        || quoted.link_src.as_deref() == Some("Hình ảnh Zalo")
        || raw_text == "[Hình ảnh]"
        || raw_text.starts_with("[Hình ảnh] ")
        || raw_text == "[Photo]"
        || raw_text.starts_with("[Photo] ")
        || (!matches!(
            quoted.kind,
            crate::message_model::MessageKind::Link
                | crate::message_model::MessageKind::File
                | crate::message_model::MessageKind::Action
                | crate::message_model::MessageKind::WebContent
        ) && (quoted
            .link_url
            .as_deref()
            .is_some_and(is_zalo_photo_cdn_url)
            || quoted
                .link_thumb
                .as_deref()
                .is_some_and(is_zalo_photo_cdn_url)));
    if is_photo_like {
        let cap = clean_media_quote_caption(Some(raw_text), false, &["Hình ảnh", "Photo"])
            .or_else(|| {
                clean_media_quote_caption(
                    quoted.link_desc.as_deref(),
                    true,
                    &["Hình ảnh", "Photo"],
                )
            });
        return format_media_quote_label("[Hình ảnh]", Some("[Photo]"), cap);
    }
    if quoted.kind == crate::message_model::MessageKind::File
        || quoted.link_src.as_deref() == Some("Tập tin Zalo")
        || raw_text == "[Tập tin]"
        || raw_text.starts_with("[Tập tin] ")
    {
        if raw_text.starts_with("[Tập tin] ") && !raw_text.to_ascii_lowercase().contains("renewmedia")
        {
            return raw_text.to_string();
        }
        let has_file_meta = quoted.link_title.is_some() || quoted.link_url.is_some();
        if has_file_meta {
            let resolved = resolve_attachment_file_name(
                quoted.link_title.as_deref(),
                raw_text,
                quoted.link_url.as_deref().unwrap_or_default(),
            );
            if resolved != "tap_tin_zalo" {
                return format!("[Tập tin] {resolved}");
            }
        }
        return "[Tập tin]".to_string();
    }
    if is_raw_json_literal(raw_text) {
        return normalize_quote_raw_msg(raw_text).unwrap_or_default();
    }
    quoted.text.clone()
}

fn normalize_quote_type(q: &Value) -> String {
    let raw = q
        .get("cliMsgType")
        .or_else(|| q.get("msgType"))
        .or_else(|| q.get("type"));
    let numeric = raw.and_then(|v| match v {
        Value::Number(n) => n.as_i64(),
        Value::String(s) => s.trim().parse::<i64>().ok(),
        _ => None,
    });
    if let Some(code) = numeric {
        return match code {
            1 => "chat.text",
            2 | 11 | 32 => "chat.photo",
            3 | 31 => "chat.voice",
            4 | 13 | 36 | 401..=404 => "chat.sticker",
            5 | 37 => "chat.doodle",
            6 | 38 => "chat.contact",
            7 | 23 | 49 => "chat.gif",
            17 | 24 | 43 => "chat.location",
            18 | 44 => "chat.video",
            19 | 46 => "chat.file",
            20 | 45 => "chat.undo",
            -27 => "chat.deleted",
            -28 => "chat.vanish",
            21 | 52 | 53 => "chat.webcontent",
            25 => "chat.ecard",
            26 => "group.poll",
            40 => "chat.link",
            _ => "",
        }
        .to_string();
    }
    raw.and_then(Value::as_str)
        .map(|s| s.trim().to_ascii_lowercase())
        .unwrap_or_default()
}

fn quote_attach_str<'a>(obj: &'a serde_json::Map<String, Value>, keys: &[&str]) -> Option<&'a str> {
    keys.iter()
        .filter_map(|k| obj.get(*k).and_then(Value::as_str))
        .map(str::trim)
        .find(|s| !s.is_empty())
}

fn quote_has_voice_params(v: &Value) -> bool {
    let Some(obj) = v.as_object() else {
        return false;
    };
    if obj.get("m4a").is_some() || obj.get("voice").is_some() || obj.get("waveform").is_some() {
        return true;
    }
    obj.get("params")
        .and_then(parse_embedded_json)
        .and_then(|p| p.as_object().cloned())
        .is_some_and(|p| {
            p.get("m4a").is_some() || p.get("voice").is_some() || p.get("waveform").is_some()
        })
}

fn quote_has_file_params(v: &Value, link_url: Option<&str>) -> bool {
    let Some(obj) = v.as_object() else {
        return false;
    };
    if obj.get("fileName").is_some()
        || obj.get("fileExt").is_some()
        || obj.get("fileSize").is_some()
        || link_url.is_some_and(|u| u.to_ascii_lowercase().contains("/renewmedia"))
    {
        return true;
    }
    obj.get("params")
        .and_then(parse_embedded_json)
        .and_then(|p| p.as_object().cloned())
        .is_some_and(|p| {
            p.get("fileName").is_some() || p.get("fileExt").is_some() || p.get("fileSize").is_some()
        })
}

/// Normalize a raw quote text field (`q.msg`) when it may hold empty whitespace, JSON, or CDN URLs.
pub(crate) fn normalize_quote_raw_msg(raw: &str) -> Option<String> {
    let t = raw.trim();
    if t.is_empty() || t == "sendBubbleMessage" {
        return None;
    }
    if is_raw_json_literal(t) {
        let parsed = serde_json::from_str::<Value>(t).ok()?;
        let synthetic = json!({ "attach": parsed });
        return resolve_quote_text(&synthetic);
    }
    if is_zalo_photo_cdn_url(t) {
        return Some("[Hình ảnh]".to_string());
    }
    if is_video_url_like(t) {
        return Some("[Video]".to_string());
    }
    Some(t.to_string())
}

fn resolve_quote_text(q: &Value) -> Option<String> {
    let q_type = normalize_quote_type(q);
    let attach_val = q.get("attach").and_then(parse_embedded_json);
    let raw_msg_val = q
        .get("msg")
        .or_else(|| q.get("message"))
        .or_else(|| q.get("content"));
    let raw_msg_json = raw_msg_val.and_then(|v| {
        if v.is_object() || v.is_array() {
            Some(v.clone())
        } else {
            parse_embedded_json(v)
        }
    });
    let raw_msg_str = raw_msg_val
        .and_then(Value::as_str)
        .and_then(clean_quote_text_candidate);

    let meta_val = attach_val.as_ref().or(raw_msg_json.as_ref());
    let meta_obj = meta_val.and_then(Value::as_object);
    let (link_url, _link_title, link_desc, link_thumb, link_src) = meta_val
        .map(extract_link_info)
        .unwrap_or((None, None, None, None, None));

    let attach_action = meta_obj
        .and_then(|o| quote_attach_str(o, &["action"]))
        .unwrap_or_default()
        .to_ascii_lowercase();
    let attach_title_raw = meta_obj.and_then(|o| quote_attach_str(o, &["fileName", "name", "title"]));
    let attach_desc_raw = link_desc
        .as_deref()
        .or_else(|| meta_obj.and_then(|o| quote_attach_str(o, &["description", "caption", "desc"])));

    if matches!(q_type.as_str(), "chat.undo" | "undo") {
        return Some("[Tin nhắn đã thu hồi]".to_string());
    }
    if q_type == "chat.deleted" {
        return Some("[Tin nhắn đã bị xóa]".to_string());
    }
    if q_type == "chat.vanish" {
        return Some("[Tin nhắn đã tự xóa]".to_string());
    }

    let is_sticker_quote = matches!(q_type.as_str(), "chat.sticker" | "sticker")
        || meta_val.and_then(extract_sticker_info).is_some()
        || attach_action == "recommened.stickerset"
        || raw_msg_str
            .as_deref()
            .is_some_and(|s| s.contains("ami_fat_cat_"))
        || (q_type == "chat.doodle"
            && link_url.is_none()
            && link_thumb.is_none()
            && raw_msg_str.is_none());
    if is_sticker_quote {
        return Some("[Nhãn dán]".to_string());
    }

    let is_voice_quote = matches!(q_type.as_str(), "chat.voice" | "voice")
        || link_src.as_deref() == Some("Thoại Zalo")
        || meta_val.is_some_and(quote_has_voice_params);
    if is_voice_quote {
        return Some("[Tin nhắn thoại]".to_string());
    }

    let is_video_quote = matches!(
        q_type.as_str(),
        "chat.video" | "video" | "chat.video.msg" | "chat.video.live.msg"
    ) || link_src.as_deref() == Some("Video Zalo")
        || meta_obj.is_some_and(|o| {
            o.get("videoUrl").is_some()
                || o.get("video_width").is_some()
                || o.get("video_height").is_some()
        })
        || link_url.as_deref().is_some_and(is_video_url_like)
        || raw_msg_str.as_deref().is_some_and(is_video_url_like);
    if is_video_quote {
        let cap = clean_media_quote_caption(raw_msg_str.as_deref(), false, &["Video"])
            .or_else(|| clean_media_quote_caption(attach_desc_raw, false, &["Video"]))
            .or_else(|| clean_media_quote_caption(attach_title_raw, true, &["Video"]));
        return Some(format_media_quote_label("[Video]", None, cap));
    }

    let is_contact_or_call = matches!(q_type.as_str(), "chat.contact" | "contact");
    if is_contact_or_call {
        if attach_action == "recommened.link" {
            let fallback = clean_quote_text_candidate(
                raw_msg_str
                    .as_deref()
                    .or(attach_title_raw)
                    .or(attach_desc_raw)
                    .or(link_url.as_deref())
                    .unwrap_or_default(),
            );
            return Some(fallback.unwrap_or_else(|| "[Liên kết]".to_string()));
        }
        if attach_action == "recommened.misscall" {
            return Some("[Cuộc gọi nhỡ]".to_string());
        }
        if attach_action.starts_with("recommened.calling")
            || attach_action.starts_with("recommened.receivecall")
            || attach_action.starts_with("recommened.calltime")
        {
            return Some("[Cuộc gọi]".to_string());
        }
        let detail = clean_media_quote_caption(raw_msg_str.as_deref(), false, &["Danh thiếp"])
            .or_else(|| clean_media_quote_caption(attach_title_raw, false, &["Danh thiếp"]))
            .or_else(|| clean_media_quote_caption(attach_desc_raw, false, &["Danh thiếp"]));
        return Some(format_media_quote_label("[Danh thiếp]", None, detail));
    }

    let is_location_quote = matches!(
        q_type.as_str(),
        "chat.location" | "chat.location.new" | "location"
    );
    if is_location_quote {
        let detail = clean_media_quote_caption(raw_msg_str.as_deref(), false, &["Vị trí"])
            .or_else(|| clean_media_quote_caption(attach_title_raw, false, &["Vị trí"]))
            .or_else(|| clean_media_quote_caption(attach_desc_raw, false, &["Vị trí"]));
        return Some(format_media_quote_label("[Vị trí]", None, detail));
    }

    let is_file_quote = matches!(q_type.as_str(), "chat.file" | "file" | "share.file")
        || link_src.as_deref() == Some("Tập tin Zalo")
        || (!matches!(
            q_type.as_str(),
            "chat.photo" | "photo" | "chat.doodle" | "doodle" | "chat.gif" | "gif"
        ) && meta_val.is_some_and(|v| quote_has_file_params(v, link_url.as_deref())));
    if is_file_quote {
        if let Some(msg) = raw_msg_str.as_deref() {
            if msg.starts_with("[Tập tin]") && !msg.to_ascii_lowercase().contains("renewmedia") {
                return Some(msg.to_string());
            }
        }
        let title_for_file = meta_obj.and_then(|o| quote_attach_str(o, &["fileName", "name", "title"]));
        let owned_param_name = meta_obj
            .and_then(|o| o.get("params"))
            .and_then(parse_embedded_json)
            .and_then(|p| {
                p.get("fileName")
                    .or_else(|| p.get("mediaTitle"))
                    .and_then(Value::as_str)
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
            });
        let effective_title = title_for_file.or(owned_param_name.as_deref());
        if effective_title.is_some() || link_url.is_some() {
            let resolved = resolve_attachment_file_name(
                effective_title,
                raw_msg_str.as_deref().unwrap_or_default(),
                link_url.as_deref().unwrap_or_default(),
            );
            if resolved != "tap_tin_zalo" {
                return Some(format!("[Tập tin] {resolved}"));
            }
        }
        if let Some(msg) = raw_msg_str.as_deref().filter(|m| {
            !m.starts_with("http://")
                && !m.starts_with("https://")
                && !m.to_ascii_lowercase().contains("renewmedia")
        }) {
            return Some(format!("[Tập tin] {msg}"));
        }
        return Some("[Tập tin]".to_string());
    }

    let is_link_quote = matches!(
        q_type.as_str(),
        "chat.link" | "link" | "chat.recommended" | "share.link"
    ) || attach_action == "recommened.link";
    if is_link_quote {
        return raw_msg_str
            .or_else(|| attach_title_raw.map(str::to_string))
            .or_else(|| attach_desc_raw.map(str::to_string))
            .or(link_url)
            .or_else(|| Some("[Liên kết]".to_string()));
    }

    let is_oa_or_poll = matches!(
        q_type.as_str(),
        "chat.webcontent"
            | "chat.webcontent.v2"
            | "chat.ecard"
            | "chat.list.action"
            | "group.poll"
    );
    if is_oa_or_poll {
        if let Some(msg) = raw_msg_str {
            return Some(msg);
        }
        if let Some(card) = extract_oa_rich_card(&q_type, meta_val) {
            if let Some(t) = card
                .title
                .or(card.description)
                .or_else(|| card.items.into_iter().find(|it| !it.title.is_empty()).map(|it| it.title))
            {
                return Some(t);
            }
        }
        if let Some(extracted) = meta_val.and_then(extract_content_text) {
            if let Some(clean) = clean_quote_text_candidate(&extracted) {
                return Some(clean);
            }
        }
        return Some(if q_type == "group.poll" {
            "[Bình chọn]".to_string()
        } else {
            "[Tin nhắn]".to_string()
        });
    }

    let has_photo_media_url = link_src.as_deref() == Some("Hình ảnh Zalo")
        || link_url.as_deref().is_some_and(is_zalo_photo_cdn_url)
        || link_thumb.as_deref().is_some_and(is_zalo_photo_cdn_url)
        || raw_msg_str.as_deref().is_some_and(is_zalo_photo_cdn_url)
        || (q_type.is_empty()
            && raw_msg_str.is_none()
            && attach_title_raw.is_none()
            && (link_url.is_some() || link_thumb.is_some()));
    let is_photo_quote = matches!(
        q_type.as_str(),
        "chat.photo" | "photo" | "chat.doodle" | "doodle" | "chat.gif" | "gif"
    ) || has_photo_media_url;
    if is_photo_quote {
        let cap = clean_media_quote_caption(raw_msg_str.as_deref(), false, &["Hình ảnh", "Photo"])
            .or_else(|| {
                clean_media_quote_caption(attach_desc_raw, true, &["Hình ảnh", "Photo"])
            })
            .or_else(|| {
                clean_media_quote_caption(attach_title_raw, true, &["Hình ảnh", "Photo"])
            });
        return Some(format_media_quote_label("[Hình ảnh]", Some("[Photo]"), cap));
    }

    if let Some(o) = meta_obj {
        if o.get("action").and_then(Value::as_str) == Some("rtf") {
            if let Some(title) = o.get("title").and_then(Value::as_str).map(str::trim).filter(|s| !s.is_empty()) {
                if let Some(params_s) = o.get("params").and_then(Value::as_str) {
                    let formatted = format_incoming_zalo_rtf(title, params_s);
                    if !formatted.trim().is_empty() {
                        return Some(formatted);
                    }
                }
                return Some(title.to_string());
            }
        }
    }

    if let Some(msg) = raw_msg_str {
        return Some(msg);
    }

    if let Some(mv) = meta_val {
        if let Some(extracted) = extract_content_text(mv) {
            return normalize_quote_raw_msg(&extracted);
        }
    }

    None
}

/// Extract `(reply_to_id, reply_to_sender, reply_to_text)` from a Zalo `quote` payload.
pub(crate) fn extract_quote_reply_fields(
    q: &Value,
) -> (Option<String>, Option<String>, Option<String>) {
    let parsed = if q.is_object() {
        None
    } else {
        parse_embedded_json(q)
    };
    let q_obj = parsed.as_ref().unwrap_or(q);
    if !q_obj.is_object() {
        return (None, None, None);
    }
    let q_id = ["cliMsgId", "globalMsgId", "msgId"]
        .iter()
        .find_map(|key| wire_identity(q_obj.get(*key)))
        .or_else(|| pick_str(q_obj, &["cliMsgId", "globalMsgId", "msgId"]));
    let q_sender = pick_str(q_obj, &["fromD", "dName", "ownerId", "fromUid", "uidFrom"]);
    let q_text = resolve_quote_text(q_obj);
    (q_id, q_sender, q_text)
}

/// Extract an optional thumbnail image URL and `is_video` flag from a Zalo `quote` payload.
pub(crate) fn extract_quote_thumb_url(q: &Value) -> Option<(String, bool)> {
    let parsed = if q.is_object() {
        None
    } else {
        parse_embedded_json(q)
    };
    let q_obj = parsed.as_ref().unwrap_or(q);
    if !q_obj.is_object() {
        return None;
    }
    let q_type = normalize_quote_type(q_obj);
    if matches!(
        q_type.as_str(),
        "chat.undo" | "undo" | "chat.deleted" | "chat.vanish" | "chat.sticker" | "sticker"
            | "chat.voice" | "voice" | "chat.file" | "file" | "share.file"
    ) {
        return None;
    }
    let attach_val = q_obj.get("attach").and_then(parse_embedded_json).or_else(|| {
        q_obj
            .get("msg")
            .or_else(|| q_obj.get("content"))
            .and_then(|v| {
                if v.is_object() || v.is_array() {
                    Some(v.clone())
                } else {
                    parse_embedded_json(v)
                }
            })
    })?;
    if extract_sticker_info(&attach_val).is_some() || quote_has_voice_params(&attach_val) {
        return None;
    }
    let (link_url, _title, _desc, link_thumb, link_src) = extract_link_info(&attach_val);
    let is_video = matches!(
        q_type.as_str(),
        "chat.video" | "video" | "chat.video.msg" | "chat.video.live.msg"
    ) || link_src.as_deref() == Some("Video Zalo")
        || attach_val.as_object().is_some_and(|o| {
            o.get("videoUrl").is_some()
                || o.get("video_width").is_some()
                || o.get("video_height").is_some()
        })
        || link_url.as_deref().is_some_and(is_video_url_like);
    let is_link_like = matches!(
        q_type.as_str(),
        "chat.link" | "link" | "chat.recommended" | "share.link" | "chat.contact" | "contact"
    );
    let candidate = if is_video || is_link_like {
        link_thumb
    } else {
        link_thumb.or_else(|| {
            link_url.filter(|u| {
                !is_video_url_like(u)
                    && (matches!(
                        q_type.as_str(),
                        "chat.photo" | "photo" | "chat.doodle" | "doodle" | "chat.gif" | "gif"
                    ) || is_zalo_photo_cdn_url(u))
            })
        })
    }?;
    if !is_safe_external_image_url(&candidate)
        || candidate.to_ascii_lowercase().contains("/renewmedia")
        || is_video_url_like(&candidate)
    {
        return None;
    }
    Some((candidate, is_video))
}

fn merge_reply_metadata(existing: &mut ChatMessage, incoming: &ChatMessage) {
    if existing.reply_to_id.is_none() && incoming.reply_to_id.is_some() {
        existing.reply_to_id = incoming.reply_to_id.clone();
    }
    if existing.reply_to_sender.is_none() && incoming.reply_to_sender.is_some() {
        existing.reply_to_sender = incoming.reply_to_sender.clone();
    }
    if is_missing_or_raw_quote_text(existing.reply_to_text.as_deref())
        && !is_missing_or_raw_quote_text(incoming.reply_to_text.as_deref())
    {
        if existing.reply_to_id.is_none() || incoming.reply_to_id.is_some() {
            existing.reply_to_id = incoming.reply_to_id.clone().or_else(|| existing.reply_to_id.clone());
        }
        if existing.reply_to_sender.is_none() || incoming.reply_to_sender.is_some() {
            existing.reply_to_sender = incoming
                .reply_to_sender
                .clone()
                .or_else(|| existing.reply_to_sender.clone());
        }
        existing.reply_to_text = incoming.reply_to_text.clone();
    }
    if existing.wire_ref.fields.get("quote").is_none() {
        if let Some(q) = incoming.wire_ref.fields.get("quote") {
            existing.wire_ref.fields.insert("quote".into(), q.clone());
        }
    }
}

/// Repair missing, blank, or unparsed quote previews on loaded thread messages from wire metadata
/// and from matching target messages within the loaded conversation.
pub(crate) fn repair_thread_quote_replies(messages: &mut [ChatMessage]) {
    for msg in messages.iter_mut() {
        if msg
            .reply_to_text
            .as_deref()
            .is_some_and(|t| t.trim().is_empty())
        {
            msg.reply_to_text = None;
        }
        if is_missing_or_raw_quote_text(msg.reply_to_text.as_deref()) {
            let wire_q = msg
                .wire_ref
                .fields
                .get("quote")
                .or_else(|| msg.history_source.as_ref().and_then(|v| v.pointer("/meta/quote")));
            if let Some(q) = wire_q {
                let (q_id, q_sender, q_text) = extract_quote_reply_fields(q);
                if msg.reply_to_id.is_none() {
                    msg.reply_to_id = q_id;
                }
                if msg.reply_to_sender.is_none() {
                    msg.reply_to_sender = q_sender;
                }
                if let Some(txt) = q_text {
                    msg.reply_to_text = Some(txt);
                }
            } else if let Some(raw) = msg.reply_to_text.as_deref() {
                msg.reply_to_text = normalize_quote_raw_msg(raw);
            }
        }
    }

    let len = messages.len();
    let mut backfills: Vec<(usize, String)> = Vec::new();
    for idx in 0..len {
        if !is_missing_or_raw_quote_text(messages[idx].reply_to_text.as_deref()) {
            continue;
        }
        let Some(ref rid) = messages[idx].reply_to_id else {
            continue;
        };
        if let Some(target_idx) = crate::message_navigation::unique_reference_index(messages, rid) {
            if target_idx != idx {
                let preview = quote_preview_text(&messages[target_idx]);
                if !preview.trim().is_empty() {
                    backfills.push((idx, preview));
                }
            }
        }
    }
    for (idx, preview) in backfills {
        messages[idx].reply_to_text = Some(preview);
    }
}

/// Preserve the request client ID for reconciliation with a later server echo.
pub(crate) async fn send_quote_message_with_client_id(
    s: &Session,
    to_id: &str,
    text: &str,
    quoted: &ChatMessage,
    is_group: bool,
    now: u64,
) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }

    let my_uid = s
        .login
        .as_ref()
        .map(|l| l.uin.clone())
        .unwrap_or_else(|| "0".into());

    let target = quote_target(quoted, &my_uid, to_id, is_group);
    let qmsg_owner = target.owner.as_str();
    let qmsg_id_num = target
        .msg_id
        .as_deref()
        .and_then(|id| id.parse::<u64>().ok())
        .unwrap_or(0);
    let qmsg_cli_id = target
        .cli_msg_id
        .as_deref()
        .and_then(|id| id.parse::<u64>().ok())
        .unwrap_or(qmsg_id_num);
    let qmsg_ts = if quoted.ts > 0 {
        quoted.ts as u64
    } else {
        now.saturating_sub(1000)
    };

    let qmsg_text = quote_preview_text(quoted);

    let quote_res: Result<Value> = async {
        if quoted.id.starts_with("local_") || qmsg_id_num == 0 {
            anyhow::bail!("quoted message has no server id");
        }
        if is_group {
            let clean_grid = to_id
                .trim_start_matches("group_")
                .trim_start_matches('g')
                .trim_start_matches('G');
            let (clean_text, text_props_opt) = extract_zalo_rtf_text_properties(text);
            let mut body = obj(&[
                ("grid", json!(clean_grid)),
                ("message", json!(clean_text)),
                ("clientId", json!(now)),
                ("language", json!("vi")),
                ("qmsgOwner", json!(qmsg_owner)),
                ("qmsgId", json!(qmsg_id_num)),
                ("qmsgCliId", json!(qmsg_cli_id)),
                ("qmsgType", json!(1)),
                ("qmsg", json!(qmsg_text)),
                ("qmsgTs", json!(qmsg_ts)),
                ("visibility", json!(0)),
                ("ttl", json!(0)),
                ("imei", json!(s.imei)),
                ("zsource", json!(101)),
            ]);
            if let (Some(tp), Some(map)) = (text_props_opt, body.as_object_mut()) {
                map.insert("textProperties".into(), json!(tp));
            }
            let params = s.encoded_params(&body)?;
            let url = format!("{}/api/group/quote?{}", s.domains.group, s.common_params());
            let resp = s.post_form(&url, format!("params={params}")).await?;
            let resp = s.maybe_decrypt(resp, s.session_key_bytes()).await;
            if let Some(code) = resp.get("error_code").and_then(Value::as_i64) {
                if code != 0 {
                    anyhow::bail!(
                        "quote error {code}: {}",
                        resp.get("error_message")
                            .and_then(Value::as_str)
                            .unwrap_or("")
                    );
                }
            }
            Ok(resp)
        } else {
            let host = if s.domains.chat.contains("-msg-chat.")
                || s.domains.chat.contains("tt-chat2-wpa")
                || s.domains.chat.is_empty()
            {
                "https://tt-chat3-wpa.api.zaloapp.com"
            } else {
                &s.domains.chat
            };
            let (clean_text, text_props_opt) = extract_zalo_rtf_text_properties(text);
            let mut body = obj(&[
                ("toid", json!(to_id)),
                ("message", json!(clean_text)),
                ("clientId", json!(now)),
                ("qmsgOwner", json!(qmsg_owner)),
                ("qmsgId", json!(qmsg_id_num)),
                ("qmsgCliId", json!(qmsg_cli_id)),
                ("qmsgType", json!(1)),
                ("qmsgTs", json!(qmsg_ts)),
                ("qmsg", json!(qmsg_text)),
                ("imei", json!(s.imei)),
                ("ttl", json!(0)),
                ("zsource", json!(101)),
            ]);
            if let (Some(tp), Some(map)) = (text_props_opt, body.as_object_mut()) {
                map.insert("textProperties".into(), json!(tp));
            }
            let params = s.encoded_params(&body)?;
            let url = format!("{host}/api/message/quote?{}", s.common_params());
            let resp = s.post_form(&url, format!("params={params}")).await?;
            let resp = s.maybe_decrypt(resp, s.session_key_bytes()).await;
            if let Some(code) = resp.get("error_code").and_then(Value::as_i64) {
                if code != 0 {
                    anyhow::bail!(
                        "quote error {code}: {}",
                        resp.get("error_message")
                            .and_then(Value::as_str)
                            .unwrap_or("")
                    );
                }
            }
            Ok(resp)
        }
    }
    .await;

    match quote_res {
        Ok(r) => Ok(r),
        Err(err) => {
            // The plain message still goes out, but the reply link is lost; leave a trace.
            crate::logger::log_error(
                "chat::send_quote",
                &format!("quote failed, sending without reply link: {err}"),
            );
            if is_group {
                send_group_message_with_client_id(s, to_id, text, now).await
            } else {
                send_message_with_client_id(s, to_id, text, now).await
            }
        }
    }
}

/// Forward a message or sticker to another conversation (`forwardMultiMessage` @ `compact-app-pc.js:14323900`).
pub async fn forward_message(
    s: &Session,
    target_id: &str,
    target_is_group: bool,
    msg: &ChatMessage,
) -> Result<Value> {
    forward_message_with_client_id(s, target_id, target_is_group, msg, next_client_message_id())
        .await
}

/// Filter thread candidates for forwarding by search keyword matching name or ID case-insensitively.
pub fn filter_forward_candidates<'a>(threads: &'a [Thread], query: &str) -> Vec<&'a Thread> {
    let q = query.trim().to_lowercase();
    threads
        .iter()
        .filter(|t| {
            if q.is_empty() {
                true
            } else {
                t.name.to_lowercase().contains(&q) || t.id.to_lowercase().contains(&q)
            }
        })
        .collect()
}

/// Resolve the forwarded payload text for a message.
///
/// For stickers, sticker ID takes precedence in dispatch.
/// For media messages (image/photo, file, web content/links) where `link_url` is present,
/// this ensures the media URL is preserved and forwarded rather than the placeholder
/// label string (e.g. `"[Hình ảnh]"` or `"[Photo]"` or `"[Tập tin]"`).
pub fn forward_content_for_message(msg: &ChatMessage) -> String {
    let raw_text = if let Some(stripped) = msg.text.strip_prefix("[Đã thu hồi] ") {
        stripped
    } else {
        &msg.text
    };

    if let Some(url) = msg.link_url.as_ref().filter(|u| !u.trim().is_empty()) {
        let trimmed = raw_text.trim();
        if trimmed.is_empty()
            || trimmed == "[Hình ảnh]"
            || trimmed == "[Photo]"
            || trimmed == "[Tập tin]"
            || trimmed == "[File]"
            || trimmed == "[Nhãn dán]"
            || trimmed == "[Chia sẻ]"
            || trimmed == "[Video]"
            || trimmed == "[Tin nhắn]"
        {
            url.clone()
        } else if let Some(caption) = trimmed
            .strip_prefix("[Hình ảnh] ")
            .or_else(|| trimmed.strip_prefix("[Photo] "))
        {
            if caption.trim().is_empty() {
                url.clone()
            } else if caption.contains(url) {
                caption.trim().to_string()
            } else {
                format!("{}\n{}", caption.trim(), url)
            }
        } else if let Some(caption) = trimmed
            .strip_prefix("[Tập tin] ")
            .or_else(|| trimmed.strip_prefix("[File] "))
        {
            if caption.trim().is_empty() {
                url.clone()
            } else if caption.contains(url) {
                caption.trim().to_string()
            } else {
                format!("{}\n{}", caption.trim(), url)
            }
        } else if !trimmed.contains(url) {
            format!("{trimmed}\n{url}")
        } else {
            trimmed.to_string()
        }
    } else {
        raw_text.to_string()
    }
}

/// Preserve the request client ID for reconciliation with a later server echo.
pub async fn forward_message_with_client_id(
    s: &Session,
    target_id: &str,
    target_is_group: bool,
    msg: &ChatMessage,
    now: u64,
) -> Result<Value> {
    if let Some(sid) = msg.sticker_id {
        if sid > 0 {
            let cid = msg.sticker_cate_id.unwrap_or(0);
            let stype = msg.sticker_type.unwrap_or(7);
            return send_sticker_with_client_id(
                s,
                target_id,
                sid,
                cid,
                stype,
                target_is_group,
                now,
            )
            .await;
        }
    }
    // Clean text if message was previously marked as recalled
    let clean_text = if let Some(stripped) = msg.text.strip_prefix("[Đã thu hồi] ") {
        stripped
    } else {
        &msg.text
    };

    // Prioritize rich-link card forwarding before photo CDN downloads
    let is_rich_link = (msg.kind == crate::message_model::MessageKind::Link
        || msg.kind == crate::message_model::MessageKind::WebContent)
        && msg.link_url.is_some()
        && (msg.link_title.is_some() || msg.link_thumb.is_some() || msg.link_desc.is_some());

    if is_rich_link {
        let url = msg.link_url.as_deref().unwrap_or_default();
        let meta = LinkMetadata {
            url: url.to_string(),
            title: msg.link_title.clone().unwrap_or_default(),
            desc: msg.link_desc.clone().unwrap_or_default(),
            thumb: msg.link_thumb.clone().unwrap_or_default(),
            src: msg.link_src.clone().unwrap_or_default(),
        };
        let text = if clean_text.trim().is_empty()
            || clean_text == "[Chia sẻ]"
            || clean_text == "[Tin nhắn]"
        {
            url
        } else {
            clean_text.trim()
        };
        if let Ok(resp) =
            send_link_message_with_client_id(s, target_id, text, &meta, target_is_group, now).await
        {
            return Ok(resp);
        }
    }

    // Forward photo if message contains a photo CDN URL
    if let Some(photo_url) = msg
        .link_url
        .as_deref()
        .filter(|u| is_zalo_photo_cdn_url(u) && is_safe_external_image_url(u))
    {
        if let Ok(resp) = s.http.get(photo_url).send().await {
            if let Ok(bytes) = resp.bytes().await {
                if !bytes.is_empty() {
                    let temp_path = std::env::temp_dir().join(format!("zalo_fwd_{now}.jpg"));
                    if std::fs::write(&temp_path, &bytes).is_ok() {
                        let caption = if clean_text == "[Hình ảnh]" {
                            ""
                        } else {
                            clean_text
                        };
                        let res =
                            send_attachment(s, target_id, target_is_group, &temp_path, caption)
                                .await;
                        let _ = std::fs::remove_file(&temp_path);
                        if let Ok((resp_v, _)) = res {
                            return Ok(resp_v);
                        }
                    }
                }
            }
        }
    }

    let forward_text = forward_content_for_message(msg);
    if target_is_group {
        send_group_message_with_client_id(s, target_id, &forward_text, now).await
    } else {
        send_message_with_client_id(s, target_id, &forward_text, now).await
    }
}

/// Convert Zalo reaction icon code to visual emoji.
pub fn icon_to_emoji(icon: &str) -> &'static str {
    match icon {
        "/-strong" => "👍",
        "/-heart" => "❤️",
        ":>" => "😆",
        ":o" => "😮",
        ":-((" | ":-(( " => "😢",
        ":-h" => "😠",
        _ => "❤️",
    }
}

/// Replace Zalo `/-code` emoticon tokens in message text with emoji for display
/// (e.g. the quick "like" button sends the text `/-strong`). Only whole whitespace-separated
/// tokens with a known code are replaced; unknown codes and all other text stay as typed.
pub fn text_emoticons_to_emoji(text: &str) -> std::borrow::Cow<'_, str> {
    fn emoji_for(token: &str) -> Option<&'static str> {
        Some(match token {
            "/-strong" => "👍",
            "/-weak" => "👎",
            "/-heart" => "❤️",
            "/-break" => "💔",
            "/-ok" => "👌",
            "/-v" => "✌️",
            "/-rose" => "🌹",
            _ => return None,
        })
    }
    if !text.contains("/-") {
        return std::borrow::Cow::Borrowed(text);
    }
    let mut out = String::with_capacity(text.len());
    let mut changed = false;
    let mut convert_words = |segment: &str, out: &mut String| {
        // `split_inclusive` keeps each separator so spacing and line breaks are preserved.
        for piece in segment.split_inclusive(char::is_whitespace) {
            let token = piece.trim_end_matches(char::is_whitespace);
            match emoji_for(token) {
                Some(emoji) => {
                    out.push_str(emoji);
                    out.push_str(&piece[token.len()..]);
                    changed = true;
                }
                None => out.push_str(piece),
            }
        }
    };
    // Code is left verbatim so a code block's Copy button returns the original text:
    // skip fenced ``` blocks and `inline code` spans.
    let mut in_fence = false;
    for line in text.split_inclusive('\n') {
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
            out.push_str(line);
            continue;
        }
        if in_fence {
            out.push_str(line);
            continue;
        }
        for (i, segment) in line.split('`').enumerate() {
            if i > 0 {
                out.push('`');
            }
            if i % 2 == 1 {
                out.push_str(segment);
            } else {
                convert_words(segment, &mut out);
            }
        }
    }
    if changed {
        std::borrow::Cow::Owned(out)
    } else {
        std::borrow::Cow::Borrowed(text)
    }
}


/// Special reaction code sent to clear all of our own reactions on a message (`rType = -1`, `rIcon = ""`).
pub const CLEAR_REACTION_CODE: &str = "/-remove";

/// Map a visual reaction emoji or Zalo reaction code to `(rIcon, rType)` (`compact-app-pc.js:10848900`).
pub fn reaction_code_and_type(emoji_or_code: &str) -> (&'static str, i32) {
    match emoji_or_code.trim() {
        "" | "/-remove" | "clear" | "none" => ("", -1),
        "👍" | "/-strong" => ("/-strong", 3),
        "❤️" | "❤" | "/-heart" => ("/-heart", 5),
        "😆" | ":>" => (":>", 0),
        "😮" | ":o" => (":o", 32),
        "😢" | ":-(( " | ":-((" => (":-((", 2),
        "😠" | ":-h" => (":-h", 20),
        _ => ("/-heart", 5),
    }
}

/// Returns `true` when `emoji_or_code` requests clearing/removing reactions (`rType == -1`).
pub fn is_clear_reaction(emoji_or_code: &str) -> bool {
    reaction_code_and_type(emoji_or_code).1 == -1
}

/// `rMsg` identity of the reacted message: (`gMsgID` = server msgId, `cMsgID` = the
/// sender's cliMsgId), taken from the wire identity. `ChatMessage::id` may hold either
/// id (re-notes/35) and `ts` is not an id; using them made the server accept the
/// reaction while other devices could not match it to a message (re-notes/39).
/// Without wire identity the fallbacks are best effort.
fn reaction_target_ids(msg: &ChatMessage, now: u64) -> (Value, u64) {
    let is_local_id =
        msg.id.is_empty() || msg.id.starts_with("out_") || msg.id.starts_with("local_");
    // When `id` is the client id (a merged row that kept its bare cliMsgId), use the wire
    // server id even if the wire client id is missing; `server_message_id` would return `id`.
    let wire_server = wire_identity(msg.wire_ref.server_id.as_ref());
    let id_is_client = !msg.client_message_id.is_empty() && msg.id == msg.client_message_id;
    let server = if id_is_client {
        wire_server.or_else(|| server_message_id(msg))
    } else {
        server_message_id(msg)
    }
    .filter(|id| !is_local_id || id != &msg.id);
    let g_msg_id = match server {
        Some(id) => json!(id),
        None if is_local_id => json!(""),
        None => json!(msg.id),
    };
    let c_msg_id = client_message_id(msg)
        .and_then(|id| id.parse::<u64>().ok())
        .or_else(|| (!is_local_id).then(|| msg.id.parse::<u64>().ok()).flatten())
        .or_else(|| (msg.ts > 0).then_some(msg.ts as u64))
        .unwrap_or(now);
    (g_msg_id, c_msg_id)
}

/// Send a message reaction (`sendReactionMessage` in `compact-app-pc.b7bec208c4c8f73e09f8.js:14317097`, cmd `12410` / `12411`).
pub async fn send_reaction(
    s: &Session,
    to_id: &str,
    is_group: bool,
    msg: &ChatMessage,
    reaction: &str,
) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);

    let (r_icon, r_type) = reaction_code_and_type(reaction);
    let (g_msg_id_val, c_msg_id) = reaction_target_ids(msg, now);

    let inner_msg = json!({
        "rMsg": [{
            "gMsgID": g_msg_id_val,
            "cMsgID": c_msg_id,
            "msgType": if msg.sticker_id.is_some() { 5 } else { 1 }
        }],
        "rIcon": r_icon,
        "rType": r_type,
        "source": 6
    })
    .to_string();

    let react_list = json!([{
        "message": inner_msg,
        "clientId": now
    }]);

    let (url, body) = if is_group {
        let clean_grid = to_id
            .trim_start_matches("group_")
            .trim_start_matches('g')
            .trim_start_matches('G');
        (
            format!(
                "https://reaction.api.zaloapp.com/api/group/reaction?{}",
                s.common_params()
            ),
            obj(&[
                ("react_list", react_list),
                ("grid", json!(clean_grid)),
                ("imei", json!(s.imei)),
            ]),
        )
    } else {
        (
            format!(
                "https://reaction.api.zaloapp.com/api/message/reaction?{}",
                s.common_params()
            ),
            obj(&[("react_list", react_list), ("toid", json!(to_id))]),
        )
    };

    let params = s.encoded_params(&body)?;
    let resp = s.post_form(&url, format!("params={params}")).await?;
    Ok(s.maybe_decrypt(resp, s.session_key_bytes()).await)
}

/// Delete a message for me (`deleteOneOneMessageV2` / `deleteGroupMessageV2` @ `compact-app-pc.js:14373319`, cmd `11779` / `11834`).
pub async fn delete_message_for_me(
    s: &Session,
    to_id: &str,
    is_group: bool,
    msg: &ChatMessage,
) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);
    let my_uid = s
        .login
        .as_ref()
        .map(|l| l.uin.clone())
        .unwrap_or_else(|| "0".into());
    let owner_id = if msg.is_outgoing {
        my_uid.as_str()
    } else if !is_group {
        to_id
    } else {
        &msg.sender
    };
    let cli_id = if msg.ts > 0 {
        msg.ts.to_string()
    } else {
        now.to_string()
    };

    let msgs_arr = json!([{
        "cliMsgId": cli_id,
        "globalMsgId": msg.id,
        "ownerId": owner_id,
        "destId": to_id
    }]);

    if is_group {
        let clean_grid = to_id
            .trim_start_matches("group_")
            .trim_start_matches('g')
            .trim_start_matches('G');
        let body = obj(&[
            ("grid", json!(clean_grid)),
            ("cliMsgId", json!(now)),
            ("msgs", msgs_arr),
            ("onlyMe", json!(1)),
        ]);
        let params = s.encoded_params(&body)?;
        let url = format!(
            "{}/api/group/deletemsg?{}",
            s.domains.group,
            s.common_params()
        );
        let resp = s.post_form(&url, format!("params={params}")).await?;
        Ok(s.maybe_decrypt(resp, s.session_key_bytes()).await)
    } else {
        let host = if s.domains.chat.contains("-msg-chat.")
            || s.domains.chat.contains("tt-chat2-wpa")
            || s.domains.chat.is_empty()
        {
            "https://tt-chat3-wpa.api.zaloapp.com"
        } else {
            &s.domains.chat
        };
        let body = obj(&[
            ("toid", json!(to_id)),
            ("cliMsgId", json!(now)),
            ("msgs", msgs_arr),
            ("onlyMe", json!(1)),
            ("imei", json!(s.imei)),
        ]);
        let params = s.encoded_params(&body)?;
        let url = format!("{host}/api/message/delete?{}", s.common_params());
        let resp = s.post_form(&url, format!("params={params}")).await?;
        Ok(s.maybe_decrypt(resp, s.session_key_bytes()).await)
    }
}

pub(crate) fn build_recall_payload(
    to_id: &str,
    is_group: bool,
    msg: &ChatMessage,
    client_id: u64,
    imei: &str,
) -> Result<Value> {
    let msg_id = msg.id.trim();
    if msg_id.is_empty() || msg_id.starts_with("out_") || msg_id.starts_with("local_") {
        anyhow::bail!("tin nhắn chưa có mã máy chủ");
    }
    let cli_id = if !msg.client_message_id.trim().is_empty() && msg.client_message_id.trim() != "0"
    {
        msg.client_message_id.trim().to_string()
    } else if let Some(h_cli) = msg
        .history_source
        .as_ref()
        .and_then(|v| pick_str(v, &["cliMsgId", "clientId", "client_message_id"]))
        .filter(|s| !s.trim().is_empty() && s.trim() != "0")
    {
        h_cli
    } else if msg.ts > 0 {
        msg.ts.to_string()
    } else {
        client_id.to_string()
    };

    if is_group {
        let clean_grid = to_id
            .trim_start_matches("group_")
            .trim_start_matches('g')
            .trim_start_matches('G');
        Ok(obj(&[
            ("msgId", json!(msg_id)),
            ("grid", json!(clean_grid)),
            ("clientId", json!(client_id)),
            ("visibility", json!(0)),
            ("cliMsgIdUndo", json!(cli_id)),
            ("imei", json!(imei)),
        ]))
    } else {
        Ok(obj(&[
            ("msgId", json!(msg_id)),
            ("toid", json!(to_id)),
            ("clientId", json!(client_id)),
            ("cliMsgIdUndo", json!(cli_id)),
            ("imei", json!(imei)),
        ]))
    }
}

/// Recall / undo an outgoing message for everyone (`undoMessage` @ `compact-app-pc.js:14376599`, cmd `11281` / `11192`).
pub async fn recall_message(
    s: &Session,
    to_id: &str,
    is_group: bool,
    msg: &ChatMessage,
) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);
    let body = build_recall_payload(to_id, is_group, msg, now, &s.imei)?;
    let params = s.encoded_params(&body)?;

    let url = if is_group {
        let group_host = if s.domains.group.is_empty() {
            "https://tt-group-wpa.chat.zalo.me"
        } else {
            s.domains.group.trim_end_matches('/')
        };
        format!("{group_host}/api/group/undomsg?{}", s.common_params())
    } else {
        let host = if s.domains.chat.contains("-msg-chat.")
            || s.domains.chat.contains("tt-chat2-wpa")
            || s.domains.chat.is_empty()
        {
            "https://tt-chat3-wpa.chat.zalo.me"
        } else {
            s.domains.chat.trim_end_matches('/')
        };
        format!("{host}/api/message/undo?{}", s.common_params())
    };
    let resp = s.post_form(&url, format!("params={params}")).await?;
    let dec = s.maybe_decrypt(resp, s.session_key_bytes()).await;
    if let Some(code) = dec.get("error_code").and_then(Value::as_i64) {
        if code != 0 {
            let emsg = dec
                .get("error_message")
                .and_then(Value::as_str)
                .unwrap_or("unknown");
            crate::logger::log_error(
                "chat::recall_message",
                &format!("msgId={} group={is_group} code={code} resp={dec}", msg.id),
            );
            return Err(crate::session::SessionError::new(code, emsg).into());
        }
    }
    if let Some(st) = dec
        .get("data")
        .and_then(|d| d.get("status"))
        .and_then(Value::as_i64)
    {
        if st < 0 {
            crate::logger::log_error(
                "chat::recall_message",
                &format!("rejected msgId={} group={is_group} status={st} resp={dec}", msg.id),
            );
            anyhow::bail!("máy chủ từ chối thu hồi (status={st})");
        }
    }
    Ok(dec)
}

/// `GET /api/preloadconvers/get-last-msgs` (cmd 10053).
///
/// [O] request: GET `{convers_api}/api/preloadconvers/get-last-msgs?{common}&params={encodeAES(json)}`
/// where payload is `{ threadIdLocalMsgId: map_object, imei }`.
pub async fn get_last_msgs(s: &Session, thread_ids: &[String]) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let mut map = serde_json::Map::new();
    // [O] `getLastMessagesForPreview(e)` in `compact-app-pc.js:14385933` passes `{}` on preload
    // to fetch all recent conversation previews without overflowing URL length.
    if thread_ids.len() <= 25 {
        for id in thread_ids {
            let clean = id.trim_start_matches("group_").trim_start_matches('g');
            let key = if id.starts_with("group_") || id.starts_with('g') {
                format!("{clean}_1")
            } else {
                format!("{clean}_0")
            };
            map.insert(key, json!("1"));
        }
    }
    let map_str = serde_json::to_string(&map).unwrap_or_else(|_| "{}".into());
    let body = obj(&[
        ("threadIdLocalMsgId", json!(map_str)),
        ("imei", json!(s.imei)),
    ]);
    let params = match s.encoded_params(&body) {
        Ok(p) => format!("params={p}"),
        Err(e) => {
            crate::logger::log_error("chat::get_last_msgs::encoded_params", &e.to_string());
            eprintln!("! get_last_msgs: no session key for encoded_params ({e}), falling back to plain_params");
            Session::plain_params(&body)
        }
    };

    let url = format!(
        "{}/api/preloadconvers/get-last-msgs?{}&{params}",
        s.domains.convers_api,
        s.common_params()
    );
    let resp = s.get(&url).await?;
    let resp = s.maybe_decrypt(resp, s.session_key_bytes()).await;
    Ok(resp)
}

/// `GET /api/pinconvers/list` (cmd 12120) — pinned threads.
pub async fn get_pinned(s: &Session) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let path = format!("/api/pinconvers/list?{}", s.common_params());
    let resp = match s.get_endpoint("conversation", &path).await {
        Ok(v) => v,
        Err(_) => {
            let url = format!(
                "{}/api/pinconvers/list?{}",
                s.domains.conversation,
                s.common_params()
            );
            s.get(&url).await?
        }
    };
    Ok(s.maybe_decrypt(resp, s.session_key_bytes()).await)
}

fn extract_pinned_id_from_value(item: &Value) -> Option<String> {
    if let Some(s) = item.as_str() {
        let trimmed = s.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }
    if let Some(n) = item.as_i64() {
        if n > 0 {
            return Some(n.to_string());
        }
    }
    if let Some(obj) = item.as_object() {
        for key in [
            "convId",
            "id",
            "conversId",
            "threadId",
            "grid",
            "uid",
            "targetId",
        ] {
            if let Some(v) = obj.get(key) {
                if let Some(s) = v.as_str() {
                    let trimmed = s.trim();
                    if !trimmed.is_empty() {
                        return Some(trimmed.to_string());
                    }
                }
                if let Some(n) = v.as_i64() {
                    if n > 0 {
                        return Some(n.to_string());
                    }
                }
            }
        }
    }
    None
}

/// Parse list of pinned conversation IDs from `/api/pinconvers/list` response (#87).
pub fn parse_pinned_conversations(resp: &Value) -> Vec<String> {
    let mut out = Vec::new();
    let data_node = if let Some(s) = resp.get("data").and_then(Value::as_str) {
        serde_json::from_str::<Value>(s.trim())
            .unwrap_or_else(|_| resp.get("data").cloned().unwrap_or_else(|| resp.clone()))
    } else {
        resp.get("data").cloned().unwrap_or_else(|| resp.clone())
    };

    let mut check_array = |arr: &[Value]| {
        for item in arr {
            if let Some(id) = extract_pinned_id_from_value(item) {
                if !out.contains(&id) {
                    out.push(id);
                }
            }
        }
    };

    if let Some(arr) = data_node.as_array() {
        check_array(arr);
    } else if let Some(obj) = data_node.as_object() {
        for key in [
            "conversations",
            "pins",
            "pinned",
            "convs",
            "data",
            "list",
            "items",
        ] {
            if let Some(arr) = obj.get(key).and_then(Value::as_array) {
                check_array(arr);
            }
        }
        if out.is_empty() {
            for (_, val) in obj {
                if let Some(id) = extract_pinned_id_from_value(val) {
                    if !out.contains(&id) {
                        out.push(id);
                    }
                }
            }
        }
    }
    out
}

/// Determine whether `thread_id` matches any pinned conversation identifier in `pinned_ids`.
///
/// Handles Zalo's prefix conventions:
/// - Group threads (`is_group: true`) match `g{id}`, `group_{id}`, or unadorned `{id}` (when not starting with `u`).
/// - 1-1 User threads (`is_group: false`) match `u{id}` or unadorned `{id}` (when not starting with `g`).
pub fn is_pinned_in_list(thread_id: &str, is_group: bool, pinned_ids: &[String]) -> bool {
    let clean = thread_id
        .trim_start_matches("group_")
        .trim_start_matches('g')
        .trim_start_matches('G')
        .trim_start_matches('u')
        .trim_start_matches('U')
        .trim();
    if clean.is_empty() {
        return false;
    }
    pinned_ids.iter().any(|pid| {
        let p_trimmed = pid.trim();
        if p_trimmed == thread_id {
            return true;
        }
        let p_clean = p_trimmed
            .trim_start_matches("group_")
            .trim_start_matches('g')
            .trim_start_matches('G')
            .trim_start_matches('u')
            .trim_start_matches('U')
            .trim();
        if p_clean == clean {
            if is_group {
                !p_trimmed.starts_with('u') && !p_trimmed.starts_with('U')
            } else {
                !p_trimmed.starts_with('g')
                    && !p_trimmed.starts_with('G')
                    && !p_trimmed.starts_with("group_")
            }
        } else {
            false
        }
    })
}

/// Apply server pinned status to threads.
pub fn apply_pinned_threads(threads: &mut [Thread], pinned_ids: &[String]) {
    for t in threads.iter_mut() {
        if is_pinned_in_list(&t.id, t.is_group, pinned_ids) {
            t.presentation.pinned = true;
        }
    }
}

/// `GET /api/group/history` (cmd 11240) — group message history.
pub async fn get_group_history(s: &Session, group_id: &str, count: u32) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let grid = group_id
        .trim_start_matches("group_")
        .trim_start_matches('g')
        .trim_start_matches('G');
    let body = obj(&[
        ("grid", json!(grid)),
        ("count", json!(count)),
        ("imei", json!(s.imei)),
    ]);
    let params = match s.encoded_params(&body) {
        Ok(p) => format!("params={p}"),
        Err(e) => {
            crate::logger::log_error("chat::get_group_history::encoded_params", &e.to_string());
            Session::plain_params(&body)
        }
    };
    let url = format!(
        "{}/api/group/history?{}&{params}",
        s.domains.group,
        s.common_params()
    );
    match s.get(&url).await {
        Ok(resp) => Ok(s.maybe_decrypt(resp, s.session_key_bytes()).await),
        Err(_) => get_group_history_raw(s, grid, count).await,
    }
}

/// Byte-exact replica of the shipped bundle's `getHistoryMessage(e, t)`.
///
/// [O] `compact-app-pc:14364564`:
/// `n = {grid: e, count: t}`
/// `getGroupDomain() + "/api/group/history?" + _getCommonParams() + "&params=" + encodeURIComponent(encodeAES(JSON.stringify(n)))`
/// Signing tag: **11240**.
///
/// Note: the bundle sends **only** `grid` and `count` — no `imei`, no `signkey` on this
/// path. Kept as a separate raw probe so the guessed `get_group_history` above can be
/// compared against it without breaking existing callers.
pub async fn get_group_history_raw(s: &Session, grid: &str, count: u32) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    // [O] getGroupIdFromConversationId: strip the group-id prefix if present.
    let grid = grid
        .trim_start_matches("group_")
        .trim_start_matches('g')
        .trim_start_matches('G');
    let body = obj(&[("grid", json!(grid)), ("count", json!(count))]);
    let params = match s.encoded_params(&body) {
        Ok(p) => format!("params={p}"),
        Err(e) => {
            crate::logger::log_error(
                "chat::get_group_history_raw::encoded_params",
                &e.to_string(),
            );
            Session::plain_params(&body)
        }
    };
    let url = format!(
        "{}/api/group/history?{}&{params}",
        s.domains.group,
        s.common_params()
    );
    let resp = s.get(&url).await?;
    let resp = s.maybe_decrypt(resp, s.session_key_bytes()).await;
    Ok(resp)
}

/// Fetch group metadata (`name`, `avt`/`fullAvt`, `totalMember`, `removedsGroup`) from
/// `POST /api/group/getmg-v2` by sending `gridVerMap: {"<grid>": 0}` so the server always
/// returns `gridInfoMap` instead of putting groups into `unchangedsGroup`.
pub async fn fetch_group_profiles(
    s: &Session,
    group_ids: &[String],
) -> Result<(
    serde_json::Map<String, Value>,
    std::collections::HashSet<String>,
)> {
    if s.login.is_none() || group_ids.is_empty() {
        return Ok((serde_json::Map::new(), std::collections::HashSet::new()));
    }
    let mut gv_zero = serde_json::Map::new();
    for gid in group_ids {
        let clean = gid
            .trim_start_matches("group_")
            .trim_start_matches('g')
            .trim_start_matches('G')
            .trim();
        if !clean.is_empty() {
            gv_zero.insert(clean.to_string(), json!(0));
        }
    }
    let gv_str = serde_json::to_string(&gv_zero)?;
    let body = obj(&[("gridVerMap", json!(gv_str)), ("imei", json!(s.imei))]);
    let params = s.encoded_params(&body)?;
    let mg_url = format!(
        "{}/api/group/getmg-v2?{}",
        s.domains.group,
        s.common_params()
    );
    let mg_resp = s.post_form(&mg_url, format!("params={params}")).await?;
    let mut mg_dec = s.maybe_decrypt(mg_resp, s.session_key_bytes()).await;
    if let Some(d_str) = mg_dec.get("data").and_then(Value::as_str) {
        if let Ok(parsed) = serde_json::from_str::<Value>(d_str.trim()) {
            mg_dec["data"] = parsed;
        }
    }
    let data_node = mg_dec
        .pointer("/data/data")
        .or_else(|| mg_dec.get("data"))
        .unwrap_or(&mg_dec);
    let info_map = data_node
        .get("gridInfoMap")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let removed: std::collections::HashSet<String> = data_node
        .get("removedsGroup")
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default();
    Ok((info_map, removed))
}

/// Extract up to 4 sorted member UIDs (`creatorId` first, then `adminIds`, then `memVerList`)
/// and total member count from a `getmg-v2` group info object.
fn extract_group_member_ids(info: &Value) -> (Vec<String>, usize) {
    let mut raw_ids: Vec<String> = Vec::new();
    if let Some(arr) = info.get("memVerList").and_then(Value::as_array) {
        for item in arr {
            if let Some(s) = item.as_str() {
                let uid = s.split('_').next().unwrap_or("").trim();
                if !uid.is_empty() && !raw_ids.iter().any(|x| x == uid) {
                    raw_ids.push(uid.to_string());
                }
            }
        }
    }
    let mut sorted: Vec<String> = Vec::new();
    if let Some(creator) = pick_str(info, &["creatorId"]) {
        if raw_ids.iter().any(|x| x == &creator) {
            sorted.push(creator);
        }
    }
    if let Some(admins) = info.get("adminIds").and_then(Value::as_array) {
        for a in admins {
            let aid = a
                .as_str()
                .map(str::to_string)
                .or_else(|| a.as_u64().map(|n| n.to_string()))
                .unwrap_or_default();
            if !aid.is_empty()
                && raw_ids.iter().any(|x| x == &aid)
                && !sorted.iter().any(|x| x == &aid)
            {
                sorted.push(aid);
            }
        }
    }
    for uid in &raw_ids {
        if !sorted.iter().any(|x| x == uid) {
            sorted.push(uid.clone());
        }
    }
    let total = info
        .get("totalMember")
        .and_then(Value::as_u64)
        .map(|n| n as usize)
        .unwrap_or_else(|| sorted.len())
        .max(sorted.len());
    sorted.truncate(4);
    (sorted, total)
}

/// Format a `multi:<total>;<name1>|<url1>;...` composite group avatar string.
fn format_multi_group_avatar(
    group_name: &str,
    mem_ids: &[String],
    total_members: usize,
    profiles: &serde_json::Map<String, Value>,
) -> Option<String> {
    let fallback_names: Vec<&str> = group_name
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    let mut slots: Vec<String> = Vec::new();
    for (idx, uid) in mem_ids.iter().take(4).enumerate() {
        let mut m_name = fallback_names
            .get(idx)
            .copied()
            .unwrap_or("Thành viên")
            .to_string();
        let mut m_ava = String::new();
        if let Some(p) = profiles.get(uid) {
            if let Some(n) = pick_str(p, &["displayName", "zaloName", "name", "display_name"]) {
                if !n.is_empty() {
                    m_name = n;
                }
            }
            if let Some(a) = pick_str(p, &["avatar", "avt", "profile_avatar"]) {
                if !a.is_empty() && !a.ends_with("/default") {
                    m_ava = a;
                }
            }
        }
        let clean_name = m_name.replace(';', " ").replace('|', " ");
        let clean_ava = m_ava.replace(';', "").replace('|', "");
        slots.push(format!("{clean_name}|{clean_ava}"));
    }
    if slots.is_empty() {
        return None;
    }
    // If the group name is a comma-separated list of member names (e.g. "Đỗ Thành, Đtc, Hoang"),
    // order the slots to match `fallback_names` so API-resolved and name-derived slots never swap positions.
    if fallback_names.len() >= 2 {
        slots.sort_by_key(|slot| {
            let s_name = slot.split('|').next().unwrap_or("").trim();
            let s_fold = strip_vn_for_sticker_kw(s_name);
            fallback_names
                .iter()
                .position(|cand| {
                    let c_fold = strip_vn_for_sticker_kw(cand);
                    !c_fold.is_empty()
                        && (c_fold == s_fold
                            || s_fold.contains(&c_fold)
                            || c_fold.contains(&s_fold))
                })
                .unwrap_or(99)
        });
    }
    let tot = total_members.max(slots.len());
    Some(format!("multi:{tot};{}", slots.join(";")))
}

fn group_posting_restricted(info: &Value, uid: &str) -> Option<bool> {
    let locked = info.pointer("/setting/lockSendMsg")?.as_i64()? != 0;
    if !locked {
        return Some(false);
    }
    let creator = info.get("creatorId")?.as_str()?;
    let admins = info.get("adminIds")?.as_array()?;
    if uid.is_empty() {
        return None;
    }
    Some(creator != uid && !admins.iter().any(|admin| admin.as_str() == Some(uid)))
}

/// `GET /api/group/getlg/v4` (cmd 12027) — list of all groups user belongs to.
pub async fn get_groups(s: &Session) -> Result<Vec<Thread>> {
    if s.login.is_none() {
        return Ok(Vec::new());
    }
    let url = format!(
        "{}/api/group/getlg/v4?{}",
        s.domains.group,
        s.common_params()
    );
    let resp = s.get(&url).await?;
    let mut resp = s.maybe_decrypt(resp, s.session_key_bytes()).await;
    if let Some(d_str) = resp.get("data").and_then(Value::as_str) {
        if let Ok(parsed) = serde_json::from_str::<Value>(d_str.trim()) {
            resp["data"] = parsed;
        }
    }
    let data_node = resp
        .pointer("/data/data")
        .or_else(|| resp.get("data"))
        .unwrap_or(&resp);

    let mut groups = Vec::new();
    if let Some(map) = data_node.get("gridVerMap").and_then(Value::as_object) {
        for (grid, _ver) in map {
            groups.push(Thread {
                presentation: Default::default(),
                unread: Default::default(),
                id: grid.clone(),
                name: format!("Nhóm {}", grid),
                preview: String::new(),
                time: crate::ui::clock(),
                avatar: None,
                is_group: true,
                messages: Vec::new(),
                vanish: None,
            });
        }
        if !groups.is_empty() {
            let gids: Vec<String> = groups.iter().map(|g| g.id.clone()).collect();
            if let Ok((info_map, rem_ids)) = fetch_group_profiles(s, &gids).await {
                let mut need_mem_uids: Vec<String> = Vec::new();
                for g in &mut groups {
                    if let Some(info) = info_map.get(&g.id) {
                        let uid = s
                            .login
                            .as_ref()
                            .map(|login| login.uin.as_str())
                            .unwrap_or("");
                        g.presentation.posting_restricted = group_posting_restricted(info, uid);
                        if let Some(name) = pick_str(info, &["name", "groupName", "displayName"]) {
                            g.name = name;
                        }
                        if let Some(avt) = pick_str(info, &["fullAvt", "avt", "avatar"])
                            .filter(|u| !u.is_empty() && !u.ends_with("/default"))
                        {
                            g.avatar = Some(avt);
                        } else {
                            let (mids, _) = extract_group_member_ids(info);
                            for uid in mids {
                                if !need_mem_uids.iter().any(|x| x == &uid) {
                                    need_mem_uids.push(uid);
                                }
                            }
                        }
                        if g.preview.is_empty() {
                            if let Some(n_mem) = info.get("totalMember").and_then(Value::as_u64) {
                                if n_mem > 0 {
                                    g.preview = format!("Nhóm · {n_mem} thành viên");
                                }
                            }
                        }
                    }
                }
                if !need_mem_uids.is_empty() {
                    let mut all_profiles = serde_json::Map::new();
                    for chunk in need_mem_uids.chunks(50) {
                        if let Ok(mp) = get_mini_profiles(s, chunk).await {
                            for (k, v) in mp {
                                all_profiles.insert(k, v);
                            }
                        }
                    }
                    for g in &mut groups {
                        if g.avatar.is_none() {
                            if let Some(info) = info_map.get(&g.id) {
                                let (mids, tot) = extract_group_member_ids(info);
                                g.avatar =
                                    format_multi_group_avatar(&g.name, &mids, tot, &all_profiles);
                            }
                        }
                    }
                }
                if !rem_ids.is_empty() {
                    groups.retain(|g| !rem_ids.contains(&g.id));
                }
            }
        }
    }
    Ok(groups)
}

/// `POST /api/social/group/members` (cmd 12030) — fetch member display names and avatars for user IDs.
///
/// Zalo's `/api/social/friend/getminiprofiles` returns `error_code = 114` for raw UID lists,
/// whereas `/api/social/group/members` with `friend_pversion_map: ["<uid>_0", ...]` returns
/// `data.profiles.<uid>.{displayName, zaloName, avatar}` with `error_code = 0`.
pub async fn get_mini_profiles(
    s: &Session,
    user_ids: &[String],
) -> Result<serde_json::Map<String, Value>> {
    if s.login.is_none() || user_ids.is_empty() {
        return Ok(serde_json::Map::new());
    }
    let versioned_ids: Vec<String> = user_ids
        .iter()
        .map(|uid| {
            let clean = uid.trim();
            if clean.contains('_') {
                clean.to_string()
            } else {
                format!("{clean}_0")
            }
        })
        .collect();
    let body = obj(&[("friend_pversion_map", json!(versioned_ids))]);
    let params = s.encoded_params(&body)?;
    let url = format!(
        "{}/api/social/group/members?{}",
        s.domains.profile,
        s.common_params()
    );
    let form = format!("params={params}");
    let resp = s.post_form(&url, form).await?;
    let mut resp = s.maybe_decrypt(resp, s.session_key_bytes()).await;
    if let Some(d_str) = resp.get("data").and_then(Value::as_str) {
        if let Ok(parsed) = serde_json::from_str::<Value>(d_str.trim()) {
            resp["data"] = parsed;
        }
    }
    if let Some(profiles) = resp
        .pointer("/data/profiles")
        .or_else(|| resp.pointer("/data/data/profiles"))
        .or_else(|| resp.get("data"))
        .and_then(Value::as_object)
    {
        return Ok(profiles.clone());
    }
    Ok(serde_json::Map::new())
}

/// Automatically resolve real display names and avatars for 1-1 and group threads that only have placeholder names.
pub async fn resolve_thread_names(s: &Session, threads: &mut [Thread]) {
    let unknown_ids: Vec<String> = threads
        .iter()
        .filter(|t| {
            !t.is_group
                && !t.name.contains("Cloud của tôi")
                && ((!t.presentation.contact_name
                    && (t.name.is_empty()
                        || is_placeholder_thread_name(&t.name, &t.id)
                        || is_generic_thread_name(&t.name)))
                    || t.avatar.as_deref().unwrap_or("").is_empty())
        })
        .map(|t| t.id.clone())
        .collect();

    if !unknown_ids.is_empty() {
        for chunk in unknown_ids.chunks(50) {
            if let Ok(profiles) = get_mini_profiles(s, chunk).await {
                for t in threads.iter_mut() {
                    if let Some(p) = profiles.get(&t.id) {
                        if !t.presentation.contact_name
                            && (t.name.is_empty()
                                || is_placeholder_thread_name(&t.name, &t.id)
                                || is_generic_thread_name(&t.name))
                        {
                            if let Some(name) =
                                pick_str(p, &["displayName", "zaloName", "name", "display_name"])
                            {
                                if !name.is_empty() {
                                    t.name = name;
                                }
                            }
                        }
                        if let Some(ava) = pick_str(p, &["avatar", "avt", "profile_avatar"]) {
                            if !ava.is_empty() {
                                t.avatar = Some(ava);
                            }
                        }
                    }
                }
            }
        }
    }

    let unknown_group_ids: Vec<String> = threads
        .iter()
        .filter(|t| {
            t.is_group
                && (is_generic_thread_name(&t.name)
                    || t.preview.is_empty()
                    || t.avatar.as_deref().unwrap_or("").is_empty())
        })
        .map(|t| t.id.clone())
        .collect();

    if !unknown_group_ids.is_empty() {
        for chunk in unknown_group_ids.chunks(50) {
            if let Ok((info_map, _)) = fetch_group_profiles(s, chunk).await {
                let mut need_mem_uids: Vec<String> = Vec::new();
                for t in threads.iter_mut() {
                    let clean_id =
                        t.id.trim_start_matches("group_")
                            .trim_start_matches('g')
                            .trim_start_matches('G');
                    if let Some(info) = info_map.get(&t.id).or_else(|| info_map.get(clean_id)) {
                        if let Some(name) = pick_str(info, &["name", "groupName", "displayName"]) {
                            if !name.is_empty() {
                                t.name = name;
                            }
                        }
                        if let Some(avt) = pick_str(info, &["fullAvt", "avt", "avatar"])
                            .filter(|u| !u.is_empty() && !u.ends_with("/default"))
                        {
                            t.avatar = Some(avt);
                        } else {
                            let (mids, _) = extract_group_member_ids(info);
                            for uid in mids {
                                if !need_mem_uids.iter().any(|x| x == &uid) {
                                    need_mem_uids.push(uid);
                                }
                            }
                        }
                        if t.preview.is_empty() {
                            if let Some(n_mem) = info.get("totalMember").and_then(Value::as_u64) {
                                if n_mem > 0 {
                                    t.preview = format!("Nhóm · {n_mem} thành viên");
                                }
                            }
                        }
                    }
                }
                if !need_mem_uids.is_empty() {
                    let mut all_profiles = serde_json::Map::new();
                    for mchunk in need_mem_uids.chunks(50) {
                        if let Ok(mp) = get_mini_profiles(s, mchunk).await {
                            for (k, v) in mp {
                                all_profiles.insert(k, v);
                            }
                        }
                    }
                    for t in threads.iter_mut() {
                        if !t.is_group {
                            continue;
                        }
                        let clean_id =
                            t.id.trim_start_matches("group_")
                                .trim_start_matches('g')
                                .trim_start_matches('G');
                        if let Some(info) = info_map.get(&t.id).or_else(|| info_map.get(clean_id)) {
                            let cur = t.avatar.as_deref().unwrap_or("");
                            if cur.is_empty() || cur.starts_with("multi:") {
                                let (mids, tot) = extract_group_member_ids(info);
                                if let Some(multi_s) =
                                    format_multi_group_avatar(&t.name, &mids, tot, &all_profiles)
                                {
                                    if should_replace_avatar(t.avatar.as_deref(), Some(&multi_s)) {
                                        t.avatar = Some(multi_s);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Offline / local contact fallback for composite group avatars: match comma-separated member names
    // against known 1-1 contact threads so group avatars display real member photos even without network.
    let known_contacts: Vec<(String, String)> = threads
        .iter()
        .filter(|t| {
            !t.is_group
                && !t.name.contains("Cloud của tôi")
                && !t.avatar.as_deref().unwrap_or("").is_empty()
                && !t.avatar.as_deref().unwrap_or("").ends_with("/default")
        })
        .map(|t| (t.name.clone(), t.avatar.clone().unwrap_or_default()))
        .collect();

    for t in threads.iter_mut() {
        if !t.is_group {
            continue;
        }
        let cur_ava = t.avatar.as_deref().unwrap_or("");
        if avatar_http_url_count(Some(cur_ava)) > 0 {
            continue;
        }
        let parts: Vec<&str> = t
            .name
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect();
        if parts.len() >= 2 {
            let mut slots: Vec<String> = Vec::new();
            for p in parts.iter().take(4) {
                let p_fold = strip_vn_for_sticker_kw(p);
                let matched_url = known_contacts
                    .iter()
                    .find(|(cname, _)| {
                        let c_fold = strip_vn_for_sticker_kw(cname);
                        !p_fold.is_empty() && c_fold == p_fold
                    })
                    .map(|(_, url)| url.as_str())
                    .unwrap_or("");
                slots.push(format!(
                    "{}|{}",
                    p.replace(';', " ").replace('|', " "),
                    matched_url
                ));
            }
            let new_multi = format!("multi:{};{}", parts.len(), slots.join(";"));
            if should_replace_avatar(t.avatar.as_deref(), Some(&new_multi)) {
                t.avatar = Some(new_multi);
            }
        }
    }
}

/// Per-queue cursor for `getChatFromPolling` (`MultiQueueId` in module `1iR4`, `sync-v2-sub-worker.js:1620000`).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct QueueCursor {
    #[serde(rename = "lastId")]
    pub last_id: String,
    pub ids: Vec<String>,
}

/// State machine for Zalo's native HTTP long-polling message receiver (`getChatFromPolling`, cmd `99100`).
///
/// [O] Observed in `sync-v2-sub-worker.b7bec208c4c8f73e09f8.js` (offsets `1620000`, `3498103`, `14557981`):
/// - `R = new y({ type: "MSG", qosId: 91021, queueIds: [1, 2, 3], ... })`
/// - `getLastActionIds()` builds:
///   `"{lastActionId},{lastActionIdOther}," + encodeURIComponent(btoa(JSON.stringify(queues)) + ",1")`
/// - `getChatFromPolling(params)`:
///   `GET {zpw_chts_new[0]}/?zpw_ver=694&zpw_type=30&params={params}&ts={unix_sec}`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PollState {
    pub last_action_id: String,
    pub last_action_id_other: String,
    pub queues: std::collections::BTreeMap<String, QueueCursor>,
}

impl Default for PollState {
    fn default() -> Self {
        let mut queues = std::collections::BTreeMap::new();
        for qid in ["1", "2", "3"] {
            queues.insert(
                qid.to_string(),
                QueueCursor {
                    last_id: "1".into(),
                    ids: Vec::new(),
                },
            );
        }
        Self {
            last_action_id: "1".into(),
            last_action_id_other: "1".into(),
            queues,
        }
    }
}

fn max_action_id(a: &str, b: &str) -> String {
    let na = a.parse::<u128>().unwrap_or(0);
    let nb = b.parse::<u128>().unwrap_or(0);
    if nb >= na && nb > 0 {
        b.to_string()
    } else if na > 0 {
        a.to_string()
    } else {
        "1".into()
    }
}

impl PollState {
    pub fn encode_params(&self) -> String {
        use base64::Engine;
        let q_json = serde_json::to_string(&self.queues).unwrap_or_else(|_| "{}".into());
        let b64 = base64::engine::general_purpose::STANDARD.encode(q_json.as_bytes());
        let tail = crate::session::urlencode(&format!("{b64},1"));
        format!(
            "{},{},{}",
            self.last_action_id, self.last_action_id_other, tail
        )
    }

    pub fn update_from_response(&mut self, resp: &Value) {
        let mut data_node = resp.get("data").unwrap_or(resp);
        if let Some(inner) = data_node.get("data").filter(|v| v.is_object()) {
            data_node = inner;
        }
        if let Some(reset_id) = pick_str(data_node, &["resetLastActionId"]) {
            if reset_id != "0" && reset_id.parse::<u128>().unwrap_or(0) > 0 {
                self.last_action_id = reset_id.clone();
                self.last_action_id_other = "1".into();
                for q in self.queues.values_mut() {
                    q.last_id = reset_id.clone();
                    q.ids.clear();
                }
                return;
            }
        }
        if let Some(lai) = pick_str(data_node, &["lastActionId"]) {
            if lai != "0" {
                self.last_action_id = max_action_id(&self.last_action_id, &lai);
            }
        }
        if let Some(laio) = pick_str(data_node, &["lastActionIdOther"]) {
            if laio != "0" {
                self.last_action_id_other = max_action_id(&self.last_action_id_other, &laio);
            }
        }
        if let Some(qs) = data_node.get("queueStatus").and_then(Value::as_object) {
            for (qid, qval) in qs {
                // Only update configured HTTP long-polling MSG queues ("1", "2", "3").
                // Adding socket-only queues ("15", "16", "7", "8") causes lp_chat to reject subsequent requests.
                let Some(entry) = self.queues.get_mut(qid) else {
                    continue;
                };
                if let Some(lid) = pick_str(qval, &["lastId"]) {
                    if lid != "0" {
                        entry.last_id = max_action_id(&entry.last_id, &lid);
                    }
                }
                if let Some(ids_arr) = qval.get("ids").and_then(Value::as_array) {
                    let max_lid = entry.last_id.parse::<u128>().unwrap_or(u128::MAX);
                    let mut new_ids: Vec<String> = ids_arr
                        .iter()
                        .filter_map(|v| match v {
                            Value::String(s) if !s.is_empty() => Some(s.clone()),
                            Value::Number(n) => Some(n.to_string()),
                            _ => None,
                        })
                        .filter(|s| s.parse::<u128>().map(|n| n <= max_lid).unwrap_or(true))
                        .collect();
                    if !new_ids.is_empty() {
                        // [O] `QueueControl.getLastActionIdsArray` (`sync-v2-sub-worker.js:1606950`):
                        // sort descending, take at most 10 <= lastId, then reverse so `ids` is ascending.
                        new_ids.sort_by_key(|s| std::cmp::Reverse(s.parse::<u128>().unwrap_or(0)));
                        new_ids.dedup();
                        new_ids.truncate(10);
                        new_ids.reverse();
                        entry.ids = new_ids;
                    }
                }
            }
        }
    }
}

/// Fetch cloud message history (`getCM` in `sync-v2-sub-worker.b7bec208c4c8f73e09f8.js:14595433`, cmd `12506` / `12507`).
///
/// [O] Observed in module `1EWQ` (`sync-v2-sub-worker.js:1545933`):
/// Cloud message history (`/api/cm/getrecentv2` and `/api/cm/getoldv2` on `group_cloud_message`)
/// is enabled for Group conversations (`g<groupId>`) and My Cloud (`sendToMeId`).
pub async fn get_cloud_messages(
    s: &Session,
    conv_id: &str,
    global_msg_id: &str,
    is_old: bool,
    count: u32,
) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let clean_id = conv_id
        .trim_start_matches("group_")
        .trim_start_matches('g')
        .trim_start_matches('G');
    let is_send2me = s
        .login
        .as_ref()
        .map(|l| !l.send2me_id.is_empty() && l.send2me_id == clean_id)
        .unwrap_or(false);
    let mut map = serde_json::Map::new();
    map.insert("groupId".into(), json!(clean_id));
    map.insert(
        "globalMsgId".into(),
        json!(if global_msg_id.is_empty() || global_msg_id == "0" {
            "9223372036854775808"
        } else {
            global_msg_id
        }),
    );
    map.insert("count".into(), json!(count));
    map.insert("msgIds".into(), json!([]));
    map.insert("imei".into(), json!(s.imei));
    if is_send2me {
        map.insert("isOA".into(), json!(1));
    }
    map.insert("src".into(), json!(if is_old { 3 } else { 1 }));

    let body = Value::Object(map);
    let params = s.encoded_params(&body)?;
    let ep = if is_old {
        "/api/cm/getoldv2"
    } else {
        "/api/cm/getrecentv2"
    };
    let host = if s.domains.cloud_msg.is_empty() {
        "https://tt-group-cm.api.zaloapp.com"
    } else {
        &s.domains.cloud_msg
    };
    let url = format!(
        "{}{ep}?{}&params={params}&nretry=0",
        host.trim_end_matches('/'),
        s.common_params()
    );
    let resp = s.get(&url).await?;
    let mut dec = s.maybe_decrypt(resp, s.session_key_bytes()).await;
    if let Some(d_str) = dec.get("data").and_then(Value::as_str) {
        let trimmed = d_str.trim();
        if (trimmed.starts_with('{') && trimmed.ends_with('}'))
            || (trimmed.starts_with('[') && trimmed.ends_with(']'))
        {
            if let Ok(parsed_data) = serde_json::from_str::<Value>(trimmed) {
                dec["data"] = parsed_data;
            }
        }
    }
    Ok(dec)
}

/// Extract `(has_more, is_old, last_msg_id)` from a `/api/cm/getrecentv2` or `/api/cm/getoldv2` response.
pub fn parse_cloud_page_meta(resp: &Value) -> (bool, bool, Option<String>) {
    let data_node = resp
        .pointer("/data/data")
        .filter(|v| v.is_object())
        .or_else(|| resp.get("data").filter(|v| v.is_object()))
        .unwrap_or(resp);
    let has_more = data_node
        .get("hasMore")
        .and_then(|v| v.as_i64().or_else(|| v.as_bool().map(i64::from)))
        .unwrap_or(0)
        != 0;
    let is_old = data_node
        .get("isOld")
        .and_then(|v| v.as_i64().or_else(|| v.as_bool().map(i64::from)))
        .unwrap_or(0)
        != 0;
    let last_msg_id = data_node
        .get("lastMsgId")
        .and_then(|v| {
            v.as_str()
                .map(str::to_string)
                .or_else(|| v.as_u64().map(|n| n.to_string()))
        })
        .filter(|s| !s.is_empty() && s != "0" && s.bytes().all(|b| b.is_ascii_digit()));
    (has_more, is_old, last_msg_id)
}

/// True when Zalo's cloud message endpoint returns `error_code = -69` (`STOP_RETRY`),
/// indicating there are no further older pages to fetch for this conversation.
pub fn is_cloud_stop_retry(err: &anyhow::Error) -> bool {
    err.downcast_ref::<crate::session::SessionError>()
        .map(|se| se.code == -69)
        .unwrap_or(false)
}

/// Call Zalo's native HTTP long-polling message endpoint (`getChatFromPolling`, cmd `99100`).
///
/// Uses a 12-second timeout matching `lp_chat` long-polling windows (`sync-v2-sub-worker.js:3498030`).
pub async fn get_chat_from_polling(s: &Session, state: &mut PollState) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let host = if s.domains.lp_chat.is_empty() {
        "https://p4-msg-chat.zaloapp.com"
    } else {
        &s.domains.lp_chat
    };
    let ts = crate::login::unix_now();
    let url = format!(
        "{}/?{}&params={}&ts={ts}",
        host.trim_end_matches('/'),
        s.common_params(),
        state.encode_params()
    );
    let fut = s.get(&url);
    match tokio::time::timeout(std::time::Duration::from_millis(12000), fut).await {
        Ok(Ok(resp)) => {
            let dec = s.maybe_decrypt(resp, s.session_key_bytes()).await;
            if let Some(code) = dec.get("error_code").and_then(Value::as_i64) {
                if code != 0 {
                    let msg = dec
                        .get("error_message")
                        .and_then(Value::as_str)
                        .unwrap_or("unknown");
                    return Err(crate::session::SessionError::new(code, msg).into());
                }
            }
            state.update_from_response(&dec);
            Ok(dec)
        }
        Ok(Err(e)) => Err(e),
        Err(_) => Ok(json!({"error_code": 0, "data": {}})),
    }
}

/// Poll both `getChatFromPolling` (`lp_chat`) and `get-last-msgs` (`convers_api`),
/// merge all incoming/outgoing/group messages into `threads`, resolve unknown names,
/// and persist to `zalo_history.db` and `threads_cache.json`.
pub async fn poll_new_messages(
    s: &Session,
    state: &mut PollState,
    threads: &mut Vec<Thread>,
) -> Result<()> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    // 1. Poll live/offline message queues via `getChatFromPolling` (follow `more == 1` / `resetLastActionId` up to 3 batches)
    for _ in 0..3 {
        let Ok(lp_resp) = get_chat_from_polling(s, state).await else {
            break;
        };
        merge_previews(threads, &lp_resp);
        let data_node = lp_resp.get("data").unwrap_or(&lp_resp);
        let more = data_node.get("more").and_then(Value::as_i64).unwrap_or(0);
        let has_reset = pick_str(data_node, &["resetLastActionId"])
            .map(|r| r != "0" && !r.is_empty())
            .unwrap_or(false);
        if more == 0 && !has_reset {
            break;
        }
    }
    // 2. Poll conversation previews via `get-last-msgs` (both empty map and populated known thread IDs)
    if let Ok(preview_resp) = get_last_msgs(s, &[]).await {
        merge_previews(threads, &preview_resp);
    }
    let known_ids: Vec<String> = threads
        .iter()
        .filter(|t| !t.id.is_empty())
        .map(|t| {
            if t.is_group && !t.id.starts_with('g') && !t.id.starts_with("group_") {
                format!("g{}", t.id)
            } else {
                t.id.clone()
            }
        })
        .collect();
    for chunk in known_ids.chunks(25) {
        if let Ok(preview_resp) = get_last_msgs(s, chunk).await {
            merge_previews(threads, &preview_resp);
        }
    }
    // 2b. Sync Cloud của tôi (`send2me_id`) and group conversations via `/api/cm/getrecentv2`
    let send2me_id = s
        .login
        .as_ref()
        .map(|l| l.send2me_id.clone())
        .unwrap_or_default();
    if !send2me_id.is_empty() {
        if let Ok(cm_resp) = get_cloud_messages(s, &send2me_id, "0", false, 50).await {
            if let Some(t) = threads.iter_mut().find(|t| t.id == send2me_id) {
                merge_group_history(t, &cm_resp);
            } else {
                merge_previews(threads, &cm_resp);
            }
        }
    }
    let group_ids: Vec<String> = threads
        .iter()
        .filter(|t| t.is_group && !t.id.is_empty())
        .map(|t| t.id.clone())
        .collect();
    for gid in group_ids {
        if let Ok(cm_resp) = get_cloud_messages(s, &gid, "0", false, 50).await {
            if let Some(t) = threads.iter_mut().find(|t| t.id == gid) {
                merge_group_history(t, &cm_resp);
            }
        }
    }
    resolve_thread_names(s, threads).await;
    threads.retain(|t| !t.messages.is_empty() || !t.name.starts_with("Người dùng "));
    deduplicate_threads(threads, None);
    let uin = s.login.as_ref().map(|l| l.uin.as_str()).unwrap_or_default();
    let acc_db = crate::store::db_path_for_account(uin);
    if let Ok(mut store) = crate::store::MessageStore::open(&acc_db) {
        let _ = crate::store::persist_threads(&mut store, threads);
    }
    save_threads_cache_for(uin, threads);
    Ok(())
}

fn extract_content_text(v: &Value) -> Option<String> {
    match v {
        Value::String(s) => {
            let trimmed = s.trim();
            if trimmed == "sendBubbleMessage" {
                return None;
            }
            if (trimmed.starts_with('{') && trimmed.ends_with('}'))
                || (trimmed.starts_with('[') && trimmed.ends_with(']'))
            {
                if let Ok(parsed) = serde_json::from_str::<Value>(trimmed) {
                    if let Some(inner) = extract_content_text(&parsed) {
                        if !inner.is_empty() {
                            return Some(inner);
                        }
                    }
                    return None;
                }
            }
            Some(s.clone())
        }
        Value::Array(arr) => {
            for item in arr {
                let is_header = item
                    .get("type")
                    .and_then(Value::as_str)
                    .is_some_and(|t| t.starts_with("l.a.header."));
                if is_header {
                    if let Some(title) = item
                        .get("title")
                        .and_then(Value::as_str)
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                    {
                        return Some(title.to_string());
                    }
                }
            }
            for item in arr {
                if let Some(inner) = extract_content_text(item) {
                    if !inner.trim().is_empty() {
                        return Some(inner);
                    }
                }
            }
            None
        }
        Value::Object(o) => {
            let href = o
                .get("href")
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|h| {
                    !h.is_empty() && (h.starts_with("http://") || h.starts_with("https://"))
                });
            if o.get("action").and_then(Value::as_str) == Some("rtf") {
                if let Some(title) = o.get("title").and_then(Value::as_str) {
                    if let Some(params_s) = o.get("params").and_then(Value::as_str) {
                        return Some(format_incoming_zalo_rtf(title, params_s));
                    }
                    return Some(title.to_string());
                }
            }
            for k in [
                "description",
                "caption",
                "text",
                "msg",
                "title",
                "content",
                "message",
                "body",
            ] {
                if let Some(val) = o.get(k) {
                    match val {
                        Value::String(s) => {
                            let t = s.trim();
                            if !t.is_empty() && t != "sendBubbleMessage" {
                                if t.starts_with('{') && t.ends_with('}') {
                                    if let Some(inner) = extract_content_text(val) {
                                        if !inner.is_empty() {
                                            return Some(inner);
                                        }
                                    }
                                } else {
                                    return Some(s.clone());
                                }
                            }
                        }
                        Value::Object(_) => {
                            if let Some(inner) = extract_content_text(val) {
                                if !inner.is_empty() {
                                    return Some(inner);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            for nested_key in ["data", "params", "zinstant_msg", "customMsg"] {
                if let Some(nested) = o.get(nested_key) {
                    match nested {
                        Value::Object(_) => {
                            if let Some(inner) = extract_content_text(nested) {
                                if !inner.is_empty() {
                                    return Some(inner);
                                }
                            }
                        }
                        Value::String(s) if s.trim().starts_with('{') || s.trim().starts_with('[') => {
                            if let Some(inner) = extract_content_text(nested) {
                                if !inner.is_empty() {
                                    return Some(inner);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            if let Some(vi_en) = o
                .get("vi")
                .or_else(|| o.get("en"))
                .or_else(|| o.get("previewTxt"))
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|s| !s.is_empty())
            {
                return Some(vi_en.to_string());
            }
            if let Some(Value::String(s)) = o.get("fileName").or_else(|| o.get("name")) {
                if !s.trim().is_empty() {
                    return Some(format!("[Tập tin] {s}"));
                }
            }
            if let Some(link) = href {
                return Some(link.to_string());
            }
            None
        }
        _ => None,
    }
}

fn normalize_msg_type(it: &Value) -> String {
    let raw = it
        .get("msgType")
        .or_else(|| it.get("messageType"))
        .or_else(|| it.get("type"));
    match raw {
        Some(Value::String(s)) => s.to_ascii_lowercase(),
        Some(Value::Number(n)) => match n.as_i64() {
            Some(1) => "chat.text".into(),
            Some(2) | Some(11) => "chat.photo".into(),
            Some(3) => "chat.voice".into(),
            Some(4) | Some(5) | Some(13) => "chat.sticker".into(),
            Some(6) => "chat.contact".into(),
            Some(7) => "chat.gif".into(),
            Some(15) | Some(21) => "chat.list.action".into(),
            Some(17) | Some(24) => "chat.location".into(),
            Some(18) => "chat.video".into(),
            Some(19) => "chat.file".into(),
            Some(20) | Some(31) => "chat.undo".into(),
            Some(25) => "chat.ecard".into(),
            Some(53) => "chat.webcontent.v2".into(),
            _ => n.to_string(),
        },
        _ => String::new(),
    }
}

fn extract_sticker_info(v: &Value) -> Option<(u64, u64, u32)> {
    match v {
        Value::String(s) => {
            let trimmed = s.trim();
            if trimmed.starts_with('{') && trimmed.ends_with('}') {
                if let Ok(parsed) = serde_json::from_str::<Value>(trimmed) {
                    return extract_sticker_info(&parsed);
                }
            }
            None
        }
        Value::Object(o) => {
            let parse_u64 = |val: &Value| -> Option<u64> {
                match val {
                    Value::Number(n) => n.as_u64(),
                    Value::String(s) => s.parse::<u64>().ok(),
                    _ => None,
                }
            };
            let cat_id = o
                .get("catId")
                .or_else(|| o.get("cateId"))
                .and_then(parse_u64);
            let stk_id = o
                .get("stickerId")
                .or_else(|| o.get("id"))
                .and_then(parse_u64);
            let stk_type = o
                .get("type")
                .and_then(parse_u64)
                .map(|t| t as u32)
                .unwrap_or(7);
            if let Some(sid) = stk_id {
                if cat_id.is_some()
                    || o.contains_key("stickerId")
                    || o.contains_key("catId")
                    || o.contains_key("cateId")
                {
                    return Some((sid, cat_id.unwrap_or(0), stk_type));
                }
            }
            None
        }
        _ => None,
    }
}

/// Check whether `url` points to an image asset (Zalo photo CDN, local photo, or direct image URL, #72).
pub fn is_zalo_photo_cdn_url(url: &str) -> bool {
    let u = url.trim().to_ascii_lowercase();
    if (!u.starts_with("http://") && !u.starts_with("https://") && !u.starts_with("local://"))
        || u.contains("/renewmedia")
    {
        return false;
    }
    let path_part = u.split('?').next().unwrap_or(&u);
    if [
        ".doc", ".docx", ".pdf", ".xls", ".xlsx", ".ppt", ".pptx", ".zip", ".rar", ".7z", ".tar",
        ".gz", ".txt", ".csv", ".mp4", ".mov", ".mkv", ".avi", ".mp3", ".m4a", ".wav", ".exe",
        ".dmg", ".apk",
    ]
    .iter()
    .any(|ext| path_part.ends_with(ext))
    {
        return false;
    }
    // Direct image file extensions anywhere (kể cả jxl/heic/avif Zalo dùng, #118)
    if [
        ".jpg", ".jpeg", ".png", ".webp", ".gif", ".bmp", ".ico", ".jfif", ".tif", ".tiff",
        ".tga", ".qoi", ".jxl", ".heic", ".heif", ".avif", ".svg",
    ]
    .iter()
    .any(|ext| path_part.ends_with(ext))
    {
        return true;
    }
    u.contains("photo-stal")
        || u.contains("photo-zmp")
        || u.contains("photo-aka")
        || u.contains("photo-link-talk")
        || u.contains("res-zalo")
        || u.contains("zpsocial")
        || ((u.contains("zdn.vn") || u.contains("zadn.vn"))
            && (path_part.contains("/jpg/")
                || path_part.contains("/png/")
                || path_part.contains("/webp/")
                || path_part.contains("/gif/")
                || path_part.contains("/gr/")
                || path_part.contains("/no/")
                || path_part.contains("/jxl/")
                || u.contains("zpcloud")
                || u.contains("zpg-r")
                || u.contains("zfcloud")))
}

/// Check that a remote URL does not target loopback, private RFC1918, or link-local destinations.
pub fn is_safe_external_image_url(url: &str) -> bool {
    let lower = url.trim().to_ascii_lowercase();
    let without_scheme = if let Some(rest) = lower.strip_prefix("https://") {
        rest
    } else if let Some(rest) = lower.strip_prefix("http://") {
        rest
    } else {
        return false;
    };
    let host = without_scheme
        .split(&['/', '?', ':', '#'][..])
        .next()
        .unwrap_or("")
        .trim();
    if host.is_empty()
        || host == "localhost"
        || host.ends_with(".localhost")
        || host.ends_with(".local")
        || host == "127.0.0.1"
        || host == "[::1]"
        || host == "::1"
        || host.starts_with("127.")
        || host.starts_with("10.")
        || host.starts_with("192.168.")
        || host.starts_with("169.254.")
        || (host.starts_with("172.") && {
            let parts: Vec<&str> = host.split('.').collect();
            parts
                .get(1)
                .and_then(|p| p.parse::<u8>().ok())
                .map(|b| (16..=31).contains(&b))
                .unwrap_or(false)
        })
    {
        return false;
    }
    true
}

/// Sanitize and resolve the original sender filename for a Zalo file attachment (#74),
/// ensuring `/renewMedia` URLs never overwrite the sender's real filename.
pub fn resolve_attachment_file_name(title: Option<&str>, text: &str, url: &str) -> String {
    let is_usable_name = |s: &str| -> bool {
        let t = s.trim();
        !t.is_empty()
            && !t.starts_with("http://")
            && !t.starts_with("https://")
            && !t.eq_ignore_ascii_case("renewmedia")
            && !t.starts_with("renewMedia.")
            && !t.starts_with("renewmedia.")
            && !matches!(
                t,
                "[Tập tin]" | "[Chia sẻ]" | "[Hình ảnh]" | "[Video]" | "[Tin nhắn]"
            )
    };

    fn strip_prefix_tag(s: &str) -> &str {
        let t = s.trim();
        t.strip_prefix("[Tập tin]")
            .or_else(|| t.strip_prefix("[Chia sẻ]"))
            .map(str::trim)
            .unwrap_or(t)
    }

    let raw_candidate = title
        .map(strip_prefix_tag)
        .filter(|s| is_usable_name(s))
        .or_else(|| {
            let cleaned = strip_prefix_tag(text);
            is_usable_name(cleaned).then_some(cleaned)
        })
        .unwrap_or_else(|| {
            let path_seg = url
                .split('?')
                .next()
                .unwrap_or(url)
                .rsplit('/')
                .next()
                .unwrap_or("tap_tin_zalo");
            if path_seg.is_empty() || path_seg.eq_ignore_ascii_case("renewmedia") {
                "tap_tin_zalo"
            } else {
                path_seg
            }
        });

    let sanitized: String = raw_candidate
        .chars()
        .map(|c| {
            if matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|') || c.is_control() {
                '_'
            } else {
                c
            }
        })
        .collect();
    let trimmed = sanitized.trim().trim_matches('.');
    if trimmed.is_empty() {
        "tap_tin_zalo".to_string()
    } else {
        trimmed.to_string()
    }
}

pub(crate) fn is_video_file_extension(ext: &str) -> bool {
    matches!(
        ext.trim().trim_start_matches('.').to_ascii_lowercase().as_str(),
        "mp4" | "mov" | "mkv" | "webm" | "avi"
    )
}

/// Extract rich link/media card metadata (`link_url`, `link_title`, `link_desc`, `link_thumb`, `link_src`)
/// from a Zalo `chat.recommended` (`recommened.link`) or rich-object `content`.
fn extract_link_info(
    v: &Value,
) -> (
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
) {
    match v {
        Value::String(s) => {
            let trimmed = s.trim();
            if trimmed.starts_with('{') && trimmed.ends_with('}') {
                if let Ok(parsed) = serde_json::from_str::<Value>(trimmed) {
                    return extract_link_info(&parsed);
                }
            }
            (None, None, None, None, None)
        }
        Value::Object(o) => {
            // href là chính, nhưng nhiều payload ảnh/video chỉ có normalUrl/hdUrl/oriUrl/url/videoUrl
            // hoặc mỗi thumb (#118) — fallback theo thứ tự để không mất URL.
            let href = [
                "href", "normalUrl", "hdUrl", "oriUrl", "url", "videoUrl", "playUrl", "thumb",
                "hd",
            ]
            .iter()
            .filter_map(|k| o.get(*k).and_then(Value::as_str))
            .map(str::trim)
            .find(|h| {
                !h.is_empty() && (h.starts_with("http://") || h.starts_with("https://"))
            })
            .map(str::to_string);
            let Some(url) = href else {
                return (None, None, None, None, None);
            };
            let thumb = o
                .get("thumb")
                .or_else(|| o.get("hd"))
                .or_else(|| o.get("thumbUrl"))
                .or_else(|| o.get("video_thumb"))
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|t| {
                    !t.is_empty() && (t.starts_with("http://") || t.starts_with("https://"))
                })
                .map(str::to_string);
            let mut title = o
                .get("fileName")
                .or_else(|| o.get("name"))
                .or_else(|| o.get("title"))
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|t| !t.is_empty())
                .map(str::to_string);
            let desc = o
                .get("description")
                .or_else(|| o.get("caption"))
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|d| !d.is_empty())
                .map(str::to_string);
            let mut src = None;
            let mut file_ext: Option<String> = o
                .get("fileExt")
                .and_then(Value::as_str)
                .map(|s| s.trim().trim_start_matches('.').to_string())
                .filter(|s| !s.is_empty());
            let mut has_file_params =
                o.get("fileName").is_some() || url.to_ascii_lowercase().contains("/renewmedia");
            let mut has_duration = o.get("duration").is_some();

            let mut is_video = o.get("videoUrl").is_some()
                || o.get("video_width").is_some()
                || o.get("video_height").is_some()
                || url.to_ascii_lowercase().contains("video-")
                || url.to_ascii_lowercase().contains(".dlmd.me")
                || url.to_ascii_lowercase().contains("/video/")
                || file_ext.as_deref().is_some_and(is_video_file_extension);

            if let Some(params_str) = o.get("params").and_then(Value::as_str) {
                if let Ok(p_json) = serde_json::from_str::<Value>(params_str.trim()) {
                    if title.is_none() {
                        title = p_json
                            .get("fileName")
                            .or_else(|| p_json.get("mediaTitle"))
                            .and_then(Value::as_str)
                            .map(str::trim)
                            .filter(|s| !s.is_empty())
                            .map(str::to_string);
                    }
                    if file_ext.is_none() {
                        file_ext = p_json
                            .get("fileExt")
                            .and_then(Value::as_str)
                            .map(|s| s.trim().trim_start_matches('.').to_string())
                            .filter(|s| !s.is_empty());
                    }
                    if p_json.get("video_width").is_some() || p_json.get("video_height").is_some() {
                        is_video = true;
                    }
                    has_duration |= p_json.get("duration").is_some();
                    if p_json.get("fileExt").is_some()
                        || p_json.get("fileSize").is_some()
                        || p_json.get("checksum").is_some()
                    {
                        has_file_params = true;
                    }
                    src = p_json
                        .get("src")
                        .and_then(Value::as_str)
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(str::to_string);
                }
            }
            let title_ext = title
                .as_deref()
                .filter(|name| !name.starts_with("http://") && !name.starts_with("https://"))
                .and_then(|name| name.rsplit_once('.'))
                .filter(|(base, ext)| {
                    !base.is_empty() && !ext.is_empty() && !ext.chars().any(char::is_whitespace)
                })
                .map(|(_, ext)| ext);
            let has_file_extension = file_ext.is_some() || title_ext.is_some();
            is_video |= file_ext.as_deref().is_some_and(is_video_file_extension)
                || title_ext.is_some_and(is_video_file_extension);
            is_video |= has_duration && !has_file_extension;
            if title.is_none() && file_ext.is_some() {
                title = Some(resolve_attachment_file_name(None, "", &url));
            }
            if let (Some(ref mut t), Some(ref ext)) = (&mut title, &file_ext) {
                let suffix = format!(".{ext}");
                if !t
                    .to_ascii_lowercase()
                    .ends_with(&suffix.to_ascii_lowercase())
                    && !t.starts_with("http://")
                    && !t.starts_with("https://")
                {
                    t.push_str(&suffix);
                }
            }
            if is_video {
                src = Some("Video Zalo".to_string());
                if file_ext.is_none() {
                    file_ext = Some("mp4".to_string());
                }
                if title.is_none()
                    || title
                        .as_deref()
                        .is_some_and(|t| t.len() > 20 && !t.contains(' ') && !t.contains('.'))
                {
                    title = Some("Video".to_string());
                }
            } else if src.is_none() && has_file_params {
                src = Some("Tập tin Zalo".to_string());
            }
            if src.is_none() && is_zalo_photo_cdn_url(&url) {
                src = Some("Hình ảnh Zalo".to_string());
            }
            if src.is_none() {
                src = url
                    .split("://")
                    .nth(1)
                    .and_then(|rest| rest.split('/').next())
                    .filter(|h| !h.is_empty())
                    .map(str::to_string);
            }
            (Some(url), title, desc, thumb, src)
        }
        _ => (None, None, None, None, None),
    }
}

fn parse_embedded_json(v: &Value) -> Option<Value> {
    match v {
        Value::Object(_) | Value::Array(_) => Some(v.clone()),
        Value::String(s) => {
            let t = s.trim();
            if (t.starts_with('{') && t.ends_with('}'))
                || (t.starts_with('[') && t.ends_with(']'))
            {
                serde_json::from_str::<Value>(t).ok()
            } else {
                None
            }
        }
        _ => None,
    }
}

fn pick_nonempty_str(o: &serde_json::Map<String, Value>, keys: &[&str]) -> Option<String> {
    keys.iter()
        .filter_map(|k| o.get(*k).and_then(Value::as_str))
        .map(str::trim)
        .find(|s| !s.is_empty() && *s != "sendBubbleMessage")
        .map(str::to_string)
}

fn pick_http_url(o: &serde_json::Map<String, Value>, keys: &[&str]) -> Option<String> {
    keys.iter()
        .filter_map(|k| o.get(*k).and_then(Value::as_str))
        .map(str::trim)
        .find(|s| s.starts_with("http://") || s.starts_with("https://"))
        .map(str::to_string)
}

fn pick_action_url(o: &serde_json::Map<String, Value>, keys: &[&str]) -> Option<String> {
    keys.iter()
        .filter_map(|k| o.get(*k).and_then(Value::as_str))
        .map(str::trim)
        .find(|s| {
            s.starts_with("http://") || s.starts_with("https://") || s.starts_with("zalo://")
        })
        .map(str::to_string)
}

fn extract_oa_card_items(arr: &[Value]) -> Vec<OaCardItem> {
    let mut out = Vec::new();
    for entry in arr {
        let Some(obj) = entry.as_object() else {
            continue;
        };
        let item_type = obj.get("type").and_then(Value::as_str).unwrap_or("");
        let is_header = item_type.starts_with("l.a.header.")
            || obj
                .get("header")
                .or_else(|| obj.get("isHeader"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
        let title = pick_nonempty_str(obj, &["title", "name", "text"]).unwrap_or_default();
        let description = pick_nonempty_str(
            obj,
            &["description", "desc", "subTitle", "subtitle", "caption"],
        )
        .unwrap_or_default();
        let thumb = pick_http_url(
            obj,
            &["thumb", "thumbUrl", "image", "cover", "photo", "icon"],
        );
        let mut url = pick_action_url(obj, &["href", "url", "link"]);
        if url.is_none() {
            if let Some(p) = obj.get("params").and_then(parse_embedded_json) {
                if let Some(po) = p.as_object() {
                    url = pick_action_url(po, &["href", "url", "link"]);
                }
            }
        }
        if !title.is_empty() || thumb.is_some() {
            out.push(OaCardItem {
                is_header,
                title,
                description,
                thumb,
                url,
            });
        }
    }
    out
}

fn extract_oa_buttons_from_array(arr: &[Value], out: &mut Vec<OaCardButton>) {
    for entry in arr {
        let Some(obj) = entry.as_object() else {
            continue;
        };
        let Some(label) = pick_nonempty_str(obj, &["title", "name", "label", "text"]) else {
            continue;
        };
        let action = pick_nonempty_str(obj, &["action", "actionId", "type"]);
        let mut url = pick_action_url(obj, &["href", "url", "link"]);
        if url.is_none() {
            if let Some(data_val) = obj.get("data").and_then(parse_embedded_json) {
                if let Some(data_obj) = data_val.as_object() {
                    url = pick_action_url(data_obj, &["href", "url", "link"]);
                }
            }
        }
        if url.is_none() {
            if let Some(p_val) = obj.get("params").and_then(parse_embedded_json) {
                if let Some(p_obj) = p_val.as_object() {
                    url = pick_action_url(p_obj, &["href", "url", "link"]);
                }
            }
        }
        if !out.iter().any(|b| b.label == label && b.url == url) {
            out.push(OaCardButton { label, url, action });
        }
    }
}

fn collect_oa_buttons(v: &Value, out: &mut Vec<OaCardButton>) {
    let Some(obj) = v.as_object() else {
        return;
    };
    for key in ["actions", "buttons"] {
        if let Some(arr) = obj.get(key).and_then(Value::as_array) {
            extract_oa_buttons_from_array(arr, out);
        }
    }
    for nested_key in ["customMsg", "data"] {
        if let Some(nested) = obj.get(nested_key).and_then(parse_embedded_json) {
            collect_oa_buttons(&nested, out);
        }
    }
}

fn collect_oa_nested_items(v: &Value) -> Vec<OaCardItem> {
    let Some(obj) = v.as_object() else {
        return Vec::new();
    };
    for key in ["elements", "list", "items", "articles"] {
        if let Some(arr) = obj.get(key).and_then(Value::as_array) {
            let parsed = extract_oa_card_items(arr);
            if !parsed.is_empty() {
                return parsed;
            }
        }
    }
    if let Some(custom) = obj.get("customMsg").and_then(parse_embedded_json) {
        let parsed = collect_oa_nested_items(&custom);
        if !parsed.is_empty() {
            return parsed;
        }
    }
    Vec::new()
}

fn extract_custom_msg_text(v: &Value) -> Option<String> {
    let obj = v.as_object()?;
    let custom = obj.get("customMsg").and_then(parse_embedded_json)?;
    let custom_obj = custom.as_object()?;
    if let Some(msg) = custom_obj.get("msg").and_then(Value::as_object) {
        if let Some(txt) = pick_nonempty_str(msg, &["vi", "en"]) {
            return Some(txt);
        }
    }
    pick_nonempty_str(custom_obj, &["msg", "description", "title", "text"])
}

fn build_oa_list_card(arr: &[Value]) -> Option<OaRichCard> {
    let parsed = extract_oa_card_items(arr);
    if parsed.is_empty() {
        return None;
    }
    let header_idx = parsed.iter().position(|it| it.is_header);
    let mut card = OaRichCard {
        card_type: "oa_list".to_string(),
        ..Default::default()
    };
    if let Some(idx) = header_idx {
        let header = &parsed[idx];
        card.banner_thumb = header.thumb.clone();
        if !header.title.is_empty() {
            card.title = Some(header.title.clone());
        }
        if !header.description.is_empty() {
            card.description = Some(header.description.clone());
        }
        card.url = header.url.clone();
        card.items = parsed
            .into_iter()
            .enumerate()
            .filter_map(|(i, it)| (i != idx).then_some(it))
            .collect();
    } else {
        card.items = parsed;
    }
    (!card.is_empty()).then_some(card)
}

/// Extract structured OA / MiniApp / ZInstant / ECard rich card metadata from a Zalo wire payload.
pub(crate) fn extract_oa_rich_card(msg_type: &str, raw_content_val: Option<&Value>) -> Option<OaRichCard> {
    let raw = raw_content_val?;
    let parsed = parse_embedded_json(raw)?;

    if let Some(arr) = parsed.as_array() {
        return build_oa_list_card(arr);
    }

    let obj = parsed.as_object()?;
    let action_str = obj
        .get("action")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_ascii_lowercase();

    if action_str.starts_with("recommened.misscall")
        || action_str.starts_with("recommened.calling")
        || action_str.starts_with("recommened.receivecall")
        || action_str.starts_with("recommened.calltime")
        || action_str == "recommened.user"
        || action_str == "recommened.contact"
        || action_str == "recommened.stickerset"
    {
        return None;
    }

    let params_val = obj.get("params").and_then(parse_embedded_json);
    let zinstant_val = obj
        .get("zinstant_msg")
        .and_then(parse_embedded_json)
        .or_else(|| {
            params_val
                .as_ref()
                .and_then(Value::as_object)
                .and_then(|p| p.get("zinstant_msg"))
                .and_then(parse_embedded_json)
        });
    let meta_val = zinstant_val.as_ref().or(params_val.as_ref());

    if msg_type == "chat.ecard" {
        let ecard_type = obj
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if ecard_type == "7" || ecard_type == "client.onboarding" {
            return None;
        }
        let mut buttons = Vec::new();
        collect_oa_buttons(&parsed, &mut buttons);
        if let Some(p) = &params_val {
            collect_oa_buttons(p, &mut buttons);
        }
        let banner_thumb = pick_http_url(obj, &["href", "thumb", "image", "banner", "cover"]);
        let title = pick_nonempty_str(obj, &["title"]);
        let description = pick_nonempty_str(obj, &["description"]).or_else(|| {
            params_val
                .as_ref()
                .and_then(Value::as_object)
                .and_then(|p| pick_nonempty_str(p, &["previewTxt"]))
        });
        let card = OaRichCard {
            card_type: "ecard".to_string(),
            banner_thumb,
            title,
            description,
            url: None,
            source_label: None,
            items: Vec::new(),
            buttons,
        };
        return (!card.is_empty()).then_some(card);
    }

    let is_webcontent = matches!(msg_type, "chat.webcontent" | "chat.webcontent.v2");
    let is_oa_action_type = msg_type == "chat.list.action"
        || action_str.contains("miniapp")
        || action_str.starts_with("oa.")
        || action_str.contains("zinstant");

    let mut buttons = Vec::new();
    collect_oa_buttons(&parsed, &mut buttons);
    if let Some(p) = &params_val {
        collect_oa_buttons(p, &mut buttons);
    }
    if let Some(z) = &zinstant_val {
        collect_oa_buttons(z, &mut buttons);
    }

    let mut items = collect_oa_nested_items(&parsed);
    if items.is_empty() {
        if let Some(m) = meta_val {
            items = collect_oa_nested_items(m);
        }
    }

    let has_http_href = pick_http_url(obj, &["href", "normalUrl", "hdUrl", "oriUrl", "url"]).is_some();
    let banner_thumb = pick_http_url(
        obj,
        &["thumb", "thumbUrl", "image", "banner", "cover", "icon"],
    )
    .or_else(|| {
        meta_val
            .and_then(Value::as_object)
            .and_then(|m| pick_http_url(m, &["thumb", "thumbUrl", "image", "banner", "cover"]))
    });
    let title = pick_nonempty_str(obj, &["title", "name"]).or_else(|| {
        meta_val
            .and_then(Value::as_object)
            .and_then(|m| pick_nonempty_str(m, &["title", "name"]))
    });
    let description = pick_nonempty_str(obj, &["description", "caption", "subTitle"])
        .or_else(|| meta_val.and_then(extract_custom_msg_text))
        .or_else(|| {
            meta_val
                .and_then(Value::as_object)
                .and_then(|m| pick_nonempty_str(m, &["description", "previewTxt"]))
        });
    let url = pick_action_url(obj, &["href", "url", "link"]).or_else(|| {
        meta_val
            .and_then(Value::as_object)
            .and_then(|m| pick_action_url(m, &["href", "url", "link"]))
    });
    let source_label = pick_nonempty_str(obj, &["src", "appName", "oaName"]).or_else(|| {
        meta_val
            .and_then(Value::as_object)
            .and_then(|m| pick_nonempty_str(m, &["src", "source", "appName", "oaName"]))
    });

    let is_banner_only_notice = !has_http_href
        && banner_thumb.is_some()
        && (title.is_some() || description.is_some())
        && obj.get("catId").is_none()
        && obj.get("stickerId").is_none()
        && obj.get("fileName").is_none();

    if !is_webcontent
        && !is_oa_action_type
        && items.is_empty()
        && buttons.is_empty()
        && !is_banner_only_notice
    {
        return None;
    }

    let is_miniapp = action_str.contains("miniapp")
        || url.as_deref().is_some_and(|u| u.starts_with("zalo://") || u.contains("mini.zalo.me"))
        || meta_val
            .and_then(Value::as_object)
            .is_some_and(|m| m.contains_key("appId") || m.contains_key("miniAppId"));

    let card_type = if !items.is_empty() && msg_type == "chat.list.action" {
        "oa_list"
    } else if is_miniapp {
        "miniapp"
    } else if is_webcontent {
        "webcontent"
    } else if !items.is_empty() {
        "oa_list"
    } else {
        "banner"
    };

    let mut card = OaRichCard {
        card_type: card_type.to_string(),
        banner_thumb,
        title,
        description,
        url,
        source_label,
        items,
        buttons,
    };
    if let (Some(t), Some(d)) = (card.title.as_deref(), card.description.as_deref()) {
        if t.trim() == d.trim() {
            card.description = None;
        }
    }
    (!card.is_empty()).then_some(card)
}

fn parse_chat_message(it: &Value) -> Option<ChatMessage> {
    let msg_type = normalize_msg_type(it);

    let raw_content_val = it
        .get("content")
        .or_else(|| {
            it.get("message")
                .and_then(|m| m.get("content"))
                .or_else(|| it.get("message"))
        })
        .or_else(|| it.get("msg"));

    let mut sticker_meta = raw_content_val.and_then(extract_sticker_info);
    if sticker_meta.is_none()
        && (msg_type == "chat.sticker" || msg_type == "sticker" || msg_type == "5")
    {
        sticker_meta = extract_sticker_info(it);
    }
    let (link_url, mut link_title, link_desc, mut link_thumb, mut link_src) = raw_content_val
        .map(extract_link_info)
        .unwrap_or((None, None, None, None, None));

    let mut content = raw_content_val
        .and_then(extract_content_text)
        .unwrap_or_default();

    // Voice & Video ưu tiên trước file: params voice/video có fileSize khiến extract_link_info
    // gán nhầm "Tập tin Zalo" (tin thoại/video hiện thành [Tập tin] token).
    let is_voice_msg = matches!(msg_type.as_str(), "chat.voice" | "voice" | "3");
    if is_voice_msg {
        link_src = Some("Thoại Zalo".to_string());
    }
    let is_video_msg = !is_voice_msg
        && (matches!(
            msg_type.as_str(),
            "chat.video" | "video" | "18" | "chat.video.msg"
        ) || link_src.as_deref() == Some("Video Zalo")
            || raw_content_val.and_then(|v| v.get("videoUrl")).is_some()
            || link_url.as_deref().is_some_and(|u| {
                let l = u.to_ascii_lowercase();
                l.ends_with(".mp4")
                    || l.ends_with(".mov")
                    || l.ends_with(".webm")
                    || l.contains("/video/")
                    || l.contains("videomedia")
                    || l.contains("video-")
                    || l.contains("video.")
                    || l.contains(".dlmd.me")
            }));
    if is_video_msg {
        link_src = Some("Video Zalo".to_string());
    }
    let is_undo_type = matches!(msg_type.as_str(), "chat.undo" | "undo" | "31");
    let oa_card = if !is_voice_msg && !is_video_msg && !is_undo_type && sticker_meta.is_none() {
        extract_oa_rich_card(&msg_type, raw_content_val)
    } else {
        None
    };
    let is_link_card_msg = !is_voice_msg
        && !is_video_msg
        && oa_card.is_none()
        && (matches!(
            msg_type.as_str(),
            "chat.link" | "link" | "6" | "share.link" | "chat.recommended"
        ) || (link_url.is_some()
            && (link_title.is_some() || link_desc.is_some())
            && link_src.as_deref() != Some("Hình ảnh Zalo")
            && link_src.as_deref() != Some("Tập tin Zalo")
            && link_src.as_deref() != Some("Video Zalo")
            && !matches!(
                msg_type.as_str(),
                "chat.photo" | "photo" | "2" | "11" | "chat.file" | "file" | "19" | "share.file"
            )));

    let is_photo_msg = !is_video_msg
        && !is_link_card_msg
        && oa_card.is_none()
        && (matches!(msg_type.as_str(), "chat.photo" | "photo" | "2" | "11")
            || link_src.as_deref() == Some("Hình ảnh Zalo")
            || link_url.as_deref().is_some_and(is_zalo_photo_cdn_url)
            || link_thumb.as_deref().is_some_and(is_zalo_photo_cdn_url));
    let is_file_msg = !is_voice_msg
        && !is_video_msg
        && oa_card.is_none()
        && (matches!(
            msg_type.as_str(),
            "chat.file" | "file" | "19" | "share.file"
        ) || link_src.as_deref() == Some("Tập tin Zalo")
            || link_url
                .as_deref()
                .is_some_and(|u| u.to_ascii_lowercase().contains("/renewmedia")));

    if is_video_msg && sticker_meta.is_none() {
        link_src = Some("Video Zalo".to_string());
        if link_thumb.is_none() {
            link_thumb = link_url.clone();
        }
        let is_raw_token_title = link_title.as_deref().is_some_and(|t| {
            t.len() > 20 && !t.contains(' ') && !t.contains('.')
        });
        if link_title.is_none() || is_raw_token_title {
            link_title = Some("Video".to_string());
        }
        let trimmed_c = content.trim();
        let is_raw_token_content = trimmed_c.len() > 20 && !trimmed_c.contains(' ');
        let is_raw_url_content =
            trimmed_c.starts_with("http://") || trimmed_c.starts_with("https://");
        if trimmed_c.is_empty()
            || is_raw_url_content
            || is_raw_token_content
            || trimmed_c.starts_with("[Tập tin]")
        {
            content = "[Video]".to_string();
        }
    } else if is_photo_msg && !is_file_msg && sticker_meta.is_none() {
        link_src = Some("Hình ảnh Zalo".to_string());
        if link_thumb.is_none() {
            link_thumb = link_url.clone();
        }
        let trimmed_c = content.trim();
        let is_raw_url_content =
            trimmed_c.starts_with("http://") || trimmed_c.starts_with("https://");
        if trimmed_c.is_empty() || is_raw_url_content {
            if let Some(cap) = link_desc.as_deref().map(str::trim).filter(|d| {
                !d.is_empty() && !d.starts_with("http://") && !d.starts_with("https://")
            }) {
                content = format!("[Hình ảnh] {cap}");
            } else {
                content = "[Hình ảnh]".to_string();
            }
        }
    } else if is_file_msg && sticker_meta.is_none() {
        link_src = Some("Tập tin Zalo".to_string());
        if link_url.is_some() || link_title.is_some() {
            let resolved_name = resolve_attachment_file_name(
                link_title.as_deref(),
                &content,
                link_url.as_deref().unwrap_or_default(),
            );
            if link_title.is_none()
                || link_title
                    .as_deref()
                    .is_some_and(|t| t.eq_ignore_ascii_case("renewmedia"))
            {
                link_title = Some(resolved_name.clone());
            }
            let final_title = link_title.as_deref().unwrap_or(&resolved_name);
            let trimmed_c = content.trim();
            if !trimmed_c.starts_with("[Tập tin]") {
                content = format!("[Tập tin] {final_title}");
            }
        }
    }
    if is_undo_type {
        content = "[Tin nhắn đã thu hồi]".to_string();
        sticker_meta = None;
    } else if sticker_meta.is_some() && (content.is_empty() || content.starts_with('{')) {
        content = "[Nhãn dán]".to_string();
    } else if content.is_empty() {
        if let Some(ref card) = oa_card {
            content = card
                .title
                .clone()
                .or_else(|| card.description.clone())
                .or_else(|| {
                    card.items
                        .iter()
                        .find(|it| !it.title.is_empty())
                        .map(|it| it.title.clone())
                })
                .unwrap_or_else(|| "[Tin nhắn OA]".to_string());
        }
    }
    if sticker_meta.is_none() {
        let parse_ami = |s: &str| -> Option<(u64, u64, u32)> {
            let pos = s.find("ami_fat_cat_")?;
            let num_str = s.get(pos + 12..pos + 14)?;
            let idx = num_str.parse::<u64>().ok()?;
            if (1..=15).contains(&idx) {
                Some((999000 + idx, 99901, 7))
            } else {
                None
            }
        };
        if let Some(m) = parse_ami(&content)
            .or_else(|| link_desc.as_deref().and_then(parse_ami))
            .or_else(|| link_title.as_deref().and_then(parse_ami))
            .or_else(|| link_url.as_deref().and_then(parse_ami))
        {
            sticker_meta = Some(m);
            content = "[Nhãn dán]".to_string();
        }
    }

    // Poll events ("X voted in the poll: …") carry an empty href/thumb and no text, and
    // used to fall through to the "[Hình ảnh]" label below.
    let is_poll_event = msg_type == "group.poll";
    if is_poll_event {
        let actor = it.get("dName").and_then(Value::as_str).unwrap_or_default();
        if let Some(notice) = raw_content_val.and_then(|c| poll_notice_text(c, actor)) {
            content = notice;
        }
    }

    if content.is_empty() {
        content = match msg_type.as_str() {
            "chat.sticker" | "sticker" | "4" | "5" | "13" => "[Nhãn dán]".to_string(),
            "chat.photo" | "photo" | "2" | "11" => "[Hình ảnh]".to_string(),
            "chat.voice" | "voice" | "3" => "[Tin nhắn thoại]".to_string(),
            "chat.video" | "video" | "18" => "[Video]".to_string(),
            "chat.file" | "file" | "19" => "[Tập tin]".to_string(),
            "chat.location" | "location" | "17" | "24" => "[Vị trí]".to_string(),
            "chat.contact" | "contact" | "6" => "[Danh thiếp]".to_string(),
            "chat.undo" | "undo" | "20" | "31" => "[Tin nhắn đã thu hồi]".to_string(),
            "chat.recommended" | "share.file" => "[Chia sẻ]".to_string(),
            _ => {
                let obj = it.get("content").or_else(|| it.get("message"));
                if let Some(o) = obj {
                    if o.get("catId").is_some()
                        || o.get("stickerId").is_some()
                        || o.get("id").is_some()
                    {
                        "[Nhãn dán]".to_string()
                    } else if [o.get("href"), o.get("thumb")]
                        .into_iter()
                        .flatten()
                        .any(|v| v.as_str().is_some_and(|s| !s.trim().is_empty()))
                    {
                        "[Hình ảnh]".to_string()
                    } else if o.is_object() {
                        "[Tin nhắn]".to_string()
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                }
            }
        };
    }

    let (undo_server_id, undo_client_id) = if is_undo_type {
        let pick_non_zero = |v: &Value, keys: &[&str]| -> Option<String> {
            keys.iter().find_map(|k| wire_identity(v.get(*k)))
        };
        let from_rv = raw_content_val.and_then(|rv| match rv {
            Value::Object(_) => Some((
                pick_non_zero(
                    rv,
                    &["globalMsgId", "msgId", "realMsgId", "undoMsgId", "srcMsgId"],
                ),
                pick_non_zero(rv, &["cliMsgId", "clientId", "client_message_id"]),
            )),
            Value::String(s) if s.trim().starts_with('{') => {
                serde_json::from_str::<Value>(s.trim()).ok().map(|parsed| {
                    (
                        pick_non_zero(
                            &parsed,
                            &["globalMsgId", "msgId", "realMsgId", "undoMsgId", "srcMsgId"],
                        ),
                        pick_non_zero(&parsed, &["cliMsgId", "clientId", "client_message_id"]),
                    )
                })
            }
            _ => None,
        });
        let srv = from_rv
            .as_ref()
            .and_then(|(s, _)| s.clone())
            .or_else(|| pick_non_zero(it, &["globalMsgId", "realMsgId", "undoMsgId", "srcMsgId"]));
        let cli = from_rv
            .and_then(|(_, c)| c)
            .or_else(|| pick_non_zero(it, &["cliMsgId", "clientId", "client_message_id"]));
        (srv, cli)
    } else {
        (None, None)
    };

    // Preserve the persisted canonical ID contract until a schema-aware migration
    // can replace legacy cliMsgId keys without duplicating cached messages.
    let msg_id = if is_undo_type {
        undo_server_id
            .clone()
            .or_else(|| undo_client_id.clone())
            .or_else(|| {
                ["msgId", "cliMsgId", "globalMsgId", "lastMsgId", "id"]
                    .iter()
                    .find_map(|key| wire_identity(it.get(*key)))
            })
            .unwrap_or_default()
    } else {
        ["msgId", "cliMsgId", "globalMsgId", "lastMsgId", "id"]
            .iter()
            .find_map(|key| wire_identity(it.get(*key)))
            .unwrap_or_default()
    };

    if content.is_empty() && msg_id.is_empty() {
        return None;
    }

    let uid_from = it
        .get("uidFrom")
        .or_else(|| it.get("fromUid"))
        .or_else(|| it.get("fromId"))
        .or_else(|| it.get("senderId"))
        .or_else(|| it.get("sender"))
        .and_then(|v| match v {
            Value::String(s) => Some(s.clone()),
            Value::Number(n) => Some(n.to_string()),
            _ => None,
        })
        .unwrap_or_default();

    let display_sender = it
        .get("dName")
        .or_else(|| it.get("displayName"))
        .or_else(|| it.get("fromName"))
        .or_else(|| it.get("senderName"))
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| uid_from.clone());

    let is_outgoing = uid_from == "0"
        || uid_from.is_empty()
        || it
            .get("isOutgoing")
            .and_then(Value::as_bool)
            .unwrap_or(false)
        || it
            .get("is_outgoing")
            .and_then(Value::as_i64)
            .map(|n| n == 1)
            .unwrap_or(false);

    let ts = it
        .get("ts")
        .or_else(|| it.get("cliMsgId"))
        .or_else(|| it.get("sendTime"))
        .or_else(|| it.get("serverTime"))
        .or_else(|| it.get("time"))
        .and_then(|v| match v {
            Value::Number(n) => n.as_i64(),
            Value::String(s) => s.parse::<i64>().ok(),
            _ => None,
        })
        .unwrap_or(0);

    let time = if ts > 0 {
        crate::clock::message_time(ts)
    } else {
        crate::ui::clock()
    };

    let (reply_to_id, reply_to_sender, reply_to_text) = it
        .get("quote")
        .and_then(parse_embedded_json)
        .filter(Value::is_object)
        .map(|q| extract_quote_reply_fields(&q))
        .unwrap_or((None, None, None));

    let (reactions, reaction_summaries) = {
        let mut summaries = crate::reactions::parse_reactions(it);
        if summaries.is_empty() {
            if let Some(c) = raw_content_val {
                summaries = crate::reactions::parse_reactions(c);
            }
        }
        let reacts = if !summaries.is_empty() {
            crate::reactions::legacy_emojis(&summaries)
        } else {
            let legacy = extract_reactions(it);
            if legacy.is_empty() {
                if let Some(c) = raw_content_val {
                    extract_reactions(c)
                } else {
                    Vec::new()
                }
            } else {
                legacy
            }
        };
        if summaries.is_empty() && !reacts.is_empty() {
            summaries = reacts
                .iter()
                .map(|e| crate::reactions::ReactionSummary {
                    emoji: e.clone(),
                    count: 1,
                    ..Default::default()
                })
                .collect();
        }
        (reacts, summaries)
    };

    // Voice: giữ params (duration/waveform/m4a) vào history_source để bubble
    // thoại hiển thị duration + waveform + nút chuyển thành chữ.
    let history_source = if is_voice_msg {
        raw_content_val.and_then(|rv| {
            let params_str = rv.get("params").and_then(Value::as_str)?;
            let mut params: Value = serde_json::from_str(params_str.trim()).ok()?;
            if let Some(obj) = params.as_object_mut() {
                // Giữ cả href để bubble biết URL gốc.
                if obj.get("href").is_none() {
                    if let Some(h) = rv.get("href").and_then(Value::as_str) {
                        obj.insert("href".into(), Value::String(h.to_string()));
                    }
                }
            }
            Some(params)
        })
    } else {
        None
    };

    let parsed_client_id = if is_undo_type {
        undo_client_id
            .clone()
            .or_else(|| {
                ["cliMsgId", "clientId", "client_message_id"]
                    .iter()
                    .find_map(|key| wire_identity(it.get(*key)))
            })
            .unwrap_or_default()
    } else {
        ["cliMsgId", "clientId", "client_message_id"]
            .iter()
            .find_map(|key| wire_identity(it.get(*key)))
            .unwrap_or_default()
    };

    let mut wire_ref = crate::message_model::WireMessageRef::from_raw(it);
    if is_undo_type && (undo_server_id.is_some() || undo_client_id.is_some()) {
        wire_ref.server_id = undo_server_id.map(Value::String);
        if let Some(cli) = undo_client_id {
            wire_ref.client_id = Some(Value::String(cli));
        }
    }

    let msg_kind = if is_poll_event {
        crate::message_model::MessageKind::System
    } else if sticker_meta.is_some() {
        crate::message_model::MessageKind::Sticker
    } else if is_voice_msg {
        crate::message_model::MessageKind::Voice
    } else if is_video_msg {
        crate::message_model::MessageKind::Video
    } else if is_file_msg {
        crate::message_model::MessageKind::File
    } else if msg_type == "chat.list.action"
        || oa_card
            .as_ref()
            .is_some_and(|c| c.card_type == "oa_list")
    {
        crate::message_model::MessageKind::Action
    } else if matches!(
        msg_type.as_str(),
        "chat.webcontent" | "chat.webcontent.v2" | "chat.ecard"
    ) || oa_card.is_some()
    {
        crate::message_model::MessageKind::WebContent
    } else if is_link_card_msg {
        crate::message_model::MessageKind::Link
    } else if is_photo_msg {
        crate::message_model::MessageKind::Photo
    } else if link_url.is_some()
        && (link_title.is_some() || link_thumb.is_some() || link_desc.is_some())
    {
        crate::message_model::MessageKind::Link
    } else {
        crate::message_model::MessageKind::Text
    };

    Some(ChatMessage {
        kind: msg_kind,
        id: msg_id,
        client_message_id: parsed_client_id,
        wire_ref,
        sender: display_sender,
        text: content,
        time,
        is_outgoing,
        ts,
        delivery: if is_outgoing {
            crate::message_model::DeliveryState::ServerAccepted
        } else {
            crate::message_model::DeliveryState::Unknown
        },
        sticker_id: sticker_meta.map(|(sid, _, _)| sid),
        sticker_cate_id: sticker_meta.map(|(_, cid, _)| cid),
        sticker_type: sticker_meta.map(|(_, _, st)| st),
        reply_to_id,
        reply_to_sender,
        reply_to_text,
        reactions,
        reaction_summaries,
        is_pinned: false,
        is_starred: false,
        link_url,
        link_title,
        link_desc,
        link_thumb,
        link_src,
        oa_card,
        media_group: if is_photo_msg {
            raw_content_val.and_then(MediaGroupLayout::from_content)
        } else {
            None
        },
        media_size: if is_photo_msg {
            raw_content_val.and_then(photo_size_from_content)
        } else if is_video_msg {
            raw_content_val.and_then(video_size_from_content)
        } else {
            None
        },
        history_source,

        ..Default::default()
    })
}

fn extract_reactions(it: &Value) -> Vec<String> {
    let mut res = Vec::new();
    if let Some(r) = it.get("reacts").or_else(|| it.get("reactions")) {
        let arr_opt = r
            .get("reactions")
            .and_then(Value::as_array)
            .or_else(|| r.as_array());
        if let Some(arr) = arr_opt {
            for item in arr {
                let icon = item
                    .as_str()
                    .or_else(|| item.get("rIcon").and_then(Value::as_str));
                if let Some(icon) = icon {
                    let em = icon_to_emoji(icon);
                    if !res.contains(&em.to_string()) {
                        res.push(em.to_string());
                    }
                }
            }
        }
    }
    res
}

fn is_group_flag(v: &Value) -> bool {
    v.get("isGroup")
        .map(|g| g.as_bool() == Some(true) || g.as_i64() == Some(1))
        .unwrap_or(false)
}

fn extract_message_thread_id_ext(
    entry: &Value,
    m_val: &Value,
    in_group_bucket: bool,
    cloud_thread_id: Option<&str>,
) -> Option<(String, bool)> {
    let is_group = in_group_bucket || is_group_flag(m_val) || is_group_flag(entry);

    // 1. Explicit `grid` field or `groupMsgs` bucket (`e.msgKey = GROUPID_PREFIX + e.idTo` in `sync-v2-sub-worker.js:3500340`)
    for obj in [m_val, entry] {
        if let Some(grid) = pick_str(obj, &["grid", "groupId"]) {
            if !grid.is_empty() && grid != "0" {
                let clean = grid.trim_start_matches("group_").to_string();
                return Some((clean, true));
            }
        }
    }
    if is_group {
        if let Some(gid) = pick_str(m_val, &["idTo", "toid", "toId", "threadId", "conversId"])
            .or_else(|| pick_str(entry, &["idTo", "toid", "toId", "threadId", "conversId"]))
        {
            if !gid.is_empty() && gid != "0" {
                let clean = gid.trim_start_matches("group_").to_string();
                return Some((clean, true));
            }
        }
    }

    // [O] `sync-v2-sub-worker.js:3500260`:
    //   `"0" == e.idTo && "0" == e.uidFrom && (e.idTo = "" + this.userId)`
    // [O] `sync-v2-sub-worker.js:829500` (`_filterDuplicatedMessages`):
    //   `e.isGroup ? e.msgKey : "0" == e.idTo ? e.uidFrom : e.idTo`
    let uid_from = pick_str(m_val, &["uidFrom", "fromUid", "fromId", "senderId"])
        .or_else(|| pick_str(entry, &["uidFrom", "fromUid", "fromId", "senderId"]))
        .unwrap_or_default();
    let id_to = pick_str(m_val, &["idTo", "toid", "toId"])
        .or_else(|| pick_str(entry, &["idTo", "toid", "toId"]))
        .unwrap_or_default();

    if let Some(cid) = cloud_thread_id.filter(|c| !c.is_empty()) {
        if ((id_to.is_empty() || id_to == "0") && (uid_from.is_empty() || uid_from == "0"))
            || id_to == cid
            || uid_from == cid
        {
            return Some((cid.to_string(), false));
        }
    }

    if !uid_from.is_empty() && uid_from != "0" && (id_to.is_empty() || id_to == "0") {
        // Incoming 1-1 message: peer sent it to us -> thread ID is `uid_from`
        Some((uid_from, false))
    } else if !id_to.is_empty() && id_to != "0" {
        // Outgoing 1-1 message or conversation wrapper with explicit `idTo` -> thread ID is `id_to`
        Some((id_to, false))
    } else if !uid_from.is_empty() && uid_from != "0" {
        Some((uid_from, false))
    } else {
        pick_str(
            m_val,
            &["threadId", "conversId", "userId", "friendId", "id"],
        )
        .or_else(|| {
            pick_str(
                entry,
                &["threadId", "conversId", "userId", "friendId", "id"],
            )
        })
        .filter(|s| s != "0" && !s.is_empty())
        .map(|s| (s, is_group))
    }
}

/// Fold a `get-last-msgs` or `getChatFromPolling` response into thread previews and conversation messages.
pub fn merge_previews(threads: &mut Vec<Thread>, resp: &Value) {
    let mut entries: Vec<(&Value, bool)> = Vec::new();
    let parsed_str_data: Option<Value> = resp
        .get("data")
        .and_then(Value::as_str)
        .and_then(|s| serde_json::from_str::<Value>(s.trim()).ok());
    let mut data_node = parsed_str_data
        .as_ref()
        .unwrap_or_else(|| resp.get("data").unwrap_or(resp));
    if let Some(inner_data) = data_node
        .get("data")
        .filter(|v| v.is_object() || v.is_array())
    {
        data_node = inner_data;
    }
    let cloud_tid: Option<String> = threads
        .iter()
        .find(|t| t.name.contains("Cloud của tôi"))
        .map(|t| t.id.clone());

    if let Some(arr) = data_node.as_array() {
        for item in arr {
            entries.push((item, is_group_flag(item)));
        }
    } else if let Some(obj) = data_node.as_object() {
        let mut found_list = false;
        for (key, is_grp) in [
            ("msgs", false),
            ("groupMsgs", true),
            ("pageMsgs", false),
            ("conversations", false),
            ("threads", false),
            ("allmsgs", false),
        ] {
            if let Some(arr) = obj.get(key).and_then(Value::as_array) {
                for item in arr {
                    entries.push((item, is_grp || is_group_flag(item)));
                }
                found_list = true;
            }
        }
        if let Some(cu_arr) = obj.get("clearUnreads").and_then(Value::as_array) {
            for cu in cu_arr {
                let Some(tid) = pick_str(cu, &["idTo", "threadId", "userId"]) else {
                    continue;
                };
                if tid.is_empty() || tid == "0" {
                    continue;
                }
                let is_grp = is_group_flag(cu);
                // Only auto-create 1-1 conversation threads from `clearUnreads`; disbanded groups
                // may linger in `clearUnreads` after removal, whereas active groups come from `get_groups`.
                if !is_grp && !threads.iter().any(|t| t.id == tid) {
                    threads.push(Thread {
                        presentation: Default::default(),
                        unread: Default::default(),
                        id: tid.clone(),
                        name: format!("Người dùng {tid}"),
                        preview: String::new(),
                        time: String::new(),
                        avatar: None,
                        is_group: false,
                        messages: Vec::new(),
                        vanish: None,
                    });
                }
            }
            found_list = true;
        }
        if !found_list {
            for (_, val) in obj {
                if val.is_object() {
                    entries.push((val, is_group_flag(val)));
                }
            }
        }
    }

    let mut message_indices: Vec<MessageIdentityIndex> = threads
        .iter()
        .map(|thread| {
            let mut index = MessageIdentityIndex::default();
            for (row, message) in thread.messages.iter().enumerate() {
                index.insert(message, row);
            }
            index
        })
        .collect();

    for (entry, is_grp_bucket) in entries {
        let mut raw_msgs: Vec<(&Value, bool)> = Vec::new();
        if let Some(arr) = entry.get("messages").and_then(Value::as_array) {
            for m in arr {
                raw_msgs.push((m, is_grp_bucket || is_group_flag(m)));
            }
        }
        if let Some(arr) = entry.get("msgs").and_then(Value::as_array) {
            for m in arr {
                raw_msgs.push((m, is_grp_bucket || is_group_flag(m)));
            }
        }
        if let Some(arr) = entry.get("groupMsgs").and_then(Value::as_array) {
            for m in arr {
                raw_msgs.push((m, true));
            }
        }
        if let Some(obj) = entry.get("lastMsg").filter(|v| v.is_object()) {
            raw_msgs.push((obj, is_grp_bucket || is_group_flag(obj)));
        }
        if let Some(obj) = entry.get("message").filter(|v| v.is_object()) {
            raw_msgs.push((obj, is_grp_bucket || is_group_flag(obj)));
        }
        if raw_msgs.is_empty()
            && (entry.get("content").is_some()
                || entry.get("msg").is_some()
                || entry.get("msgId").is_some())
        {
            raw_msgs.push((entry, is_grp_bucket));
        }

        for (m_val, is_grp) in raw_msgs {
            if m_val.get("msgType").and_then(Value::as_str) == Some("chat.reaction") {
                let events = crate::reactions::parse_reaction_events(m_val);
                for ev in events {
                    for t in threads.iter_mut() {
                        for m in t.messages.iter_mut() {
                            if (!ev.message_id.is_empty() && m.id == ev.message_id)
                                || (!ev.client_message_id.is_empty()
                                    && m.client_message_id == ev.client_message_id)
                            {
                                m.apply_reaction(&ev);
                            }
                        }
                    }
                }
                continue;
            }

            let Some(parsed) = parse_chat_message(m_val) else {
                continue;
            };

            let Some((tid, thread_is_group)) =
                extract_message_thread_id_ext(entry, m_val, is_grp, cloud_tid.as_deref())
            else {
                continue;
            };
            if parsed.oa_card.is_none() {
                let raw_mt = m_val.get("msgType").and_then(Value::as_str).unwrap_or("");
                let cval = m_val.get("content");
                let has_structured_content = cval.is_some_and(|v| {
                    v.is_object()
                        || v.is_array()
                        || v.as_str().is_some_and(|s| {
                            let t = s.trim_start();
                            t.starts_with('{') || t.starts_with('[')
                        })
                });
                if raw_mt.contains("webcontent")
                    || raw_mt.contains("list")
                    || raw_mt.contains("oa")
                    || raw_mt.contains("ecard")
                    || has_structured_content
                {
                    let c_keys: String = match cval {
                        Some(Value::Object(map)) => {
                            let mut ks: Vec<&str> = map.keys().map(String::as_str).collect();
                            ks.sort_unstable();
                            format!("obj[{}]", ks.join(","))
                        }
                        Some(Value::Array(arr)) => format!("arr({})", arr.len()),
                        Some(Value::String(s)) => {
                            if let Ok(pv) = serde_json::from_str::<Value>(s.trim()) {
                                match pv {
                                    Value::Object(map) => {
                                        let mut ks: Vec<&str> =
                                            map.keys().map(String::as_str).collect();
                                        ks.sort_unstable();
                                        format!("json_str_obj[{}]", ks.join(","))
                                    }
                                    Value::Array(arr) => format!("json_str_arr({})", arr.len()),
                                    _ => format!("str({})", s.len()),
                                }
                            } else {
                                format!("str({})", s.len())
                            }
                        }
                        Some(_) => "other".into(),
                        None => "none".into(),
                    };
                    let has_params = m_val.get("params").is_some();
                    crate::logger::log_sync(&format!(
                        "preview_diag tid={} mid={} mt={} content={} has_params={}",
                        tid, parsed.id, raw_mt, c_keys, has_params
                    ));
                }
            }

            if let Some(thread_index) = threads.iter().position(|thread| thread.id == tid) {
                let t = &mut threads[thread_index];
                let identities = &mut message_indices[thread_index];
                if thread_is_group {
                    t.is_group = true;
                }
                let is_incoming = !parsed.is_outgoing;
                let parsed_ts = parsed.ts;
                if let Some(row) = identities.find(&t.messages, &parsed) {
                    let existing = &mut t.messages[row];
                    identities.remove(existing, row);
                    existing.merge_reaction_metadata(&parsed);
                    merge_delivery_state(&mut existing.delivery, parsed.delivery);
                    merge_reply_metadata(existing, &parsed);
                    if existing.id.starts_with("out_")
                        && !parsed.id.is_empty()
                        && !parsed.id.starts_with("out_")
                    {
                        existing.id = parsed.id;
                    }
                    let is_undo_msg = parsed.text.trim() == "[Tin nhắn đã thu hồi]";
                    if parsed.ts > 0
                        && existing.ts != parsed.ts
                        && (!is_undo_msg || existing.ts == 0)
                    {
                        existing.ts = parsed.ts;
                        existing.time = parsed.time;
                    }
                    apply_recall_merge(existing, &parsed.text);
                    if existing.sticker_id.is_none() && parsed.sticker_id.is_some() {
                        existing.sticker_id = parsed.sticker_id;
                        existing.sticker_cate_id = parsed.sticker_cate_id;
                        existing.sticker_type = parsed.sticker_type;
                    }
                    if existing.link_url.is_none() && parsed.link_url.is_some() {
                        existing.link_url = parsed.link_url;
                        existing.link_title = parsed.link_title;
                        existing.link_desc = parsed.link_desc;
                        existing.link_thumb = parsed.link_thumb;
                        existing.link_src = parsed.link_src;
                        if !parsed.text.is_empty()
                            && !existing.text.starts_with("[Đã thu hồi]")
                            && existing.text.trim() != "[Tin nhắn đã thu hồi]"
                        {
                            existing.text = parsed.text;
                        }
                    }
                    if existing.oa_card.is_none() && parsed.oa_card.is_some() {
                        existing.oa_card = parsed.oa_card;
                        if existing.kind == crate::message_model::MessageKind::Text
                            && parsed.kind != crate::message_model::MessageKind::Text
                        {
                            existing.kind = parsed.kind;
                        }
                    }
                    if !parsed.reactions.is_empty() {
                        existing.reactions = parsed.reactions;
                    }
                    if !parsed.reaction_summaries.is_empty() {
                        crate::reactions::trace_count_change(
                            "merge_previews:overwrite_summaries",
                            &existing.id,
                            crate::reactions::total_count(&existing.reaction_summaries),
                            crate::reactions::total_count(&parsed.reaction_summaries),
                        );
                        existing.reaction_summaries = parsed.reaction_summaries;
                    }
                    existing.is_pinned = existing.is_pinned || parsed.is_pinned;
                    existing.is_starred = existing.is_starred || parsed.is_starred;
                    identities.insert(existing, row);
                } else {
                    identities.insert(&parsed, t.messages.len());
                    t.messages.push(parsed);
                }
                if is_incoming {
                    for em in t.messages.iter_mut() {
                        if em.is_outgoing && em.ts > 0 && (em.ts <= parsed_ts || parsed_ts == 0) {
                            em.delivery = crate::message_model::DeliveryState::Read;
                        }
                    }
                }
            } else {
                // New thread detected from incoming message!
                let default_name = if thread_is_group {
                    format!("Nhóm {}", tid)
                } else {
                    format!("Người dùng {}", tid)
                };
                let mut identities = MessageIdentityIndex::default();
                identities.insert(&parsed, 0);
                message_indices.push(identities);
                threads.push(Thread {
                    presentation: Default::default(),
                    unread: Default::default(),
                    id: tid,
                    name: default_name,
                    preview: parsed.text.clone(),
                    time: parsed.time.clone(),
                    avatar: None,
                    is_group: thread_is_group,
                    messages: vec![parsed],
                    vanish: None,
                });
            }
        }
    }

    for t in threads.iter_mut() {
        if !t.messages.is_empty() {
            t.messages.sort_by_key(|m| m.ts);
            repair_thread_quote_replies(&mut t.messages);
            if let Some(newest) = t.messages.last() {
                if !newest.text.is_empty() {
                    t.preview = newest.text.replace('\n', " ");
                    t.time = newest.time.clone();
                }
            }
        }
    }
}

/// Merge group history response into a thread's messages, sorted by timestamp and deduplicated by msg_id.
pub fn merge_group_history(thread: &mut Thread, resp: &Value) {
    let mut raw_msgs: Vec<&Value> = Vec::new();
    let parsed_str_data: Option<Value> = resp
        .get("data")
        .and_then(Value::as_str)
        .and_then(|s| serde_json::from_str::<Value>(s.trim()).ok());
    let mut data_node = parsed_str_data
        .as_ref()
        .unwrap_or_else(|| resp.get("data").unwrap_or(resp));
    if let Some(inner_data) = data_node
        .get("data")
        .filter(|v| v.is_object() || v.is_array())
    {
        data_node = inner_data;
    }

    if let Some(arr) = data_node.as_array() {
        raw_msgs.extend(arr);
    } else if let Some(obj) = data_node.as_object() {
        for key in ["msgs", "groupMsgs", "messages"] {
            if let Some(arr) = obj.get(key).and_then(Value::as_array) {
                raw_msgs.extend(arr);
            }
        }
    }

    let clean_thread_id = thread
        .id
        .trim_start_matches("group_")
        .trim_start_matches('g')
        .trim_start_matches('G')
        .to_string();

    let mut identities = MessageIdentityIndex::default();
    for (row, message) in thread.messages.iter().enumerate() {
        identities.insert(message, row);
    }

    for it in raw_msgs {
        if !clean_thread_id.is_empty() {
            if let Some(id_to) = pick_str(it, &["idTo", "toId", "grid", "groupId"]) {
                let clean_to = id_to
                    .trim_start_matches("group_")
                    .trim_start_matches('g')
                    .trim_start_matches('G');
                if !clean_to.is_empty() && clean_to != "0" && clean_to != clean_thread_id {
                    continue;
                }
            }
        }
        if let Some(m) = parse_chat_message(it) {
            if let Some(row) = identities.find(&thread.messages, &m) {
                let existing = &mut thread.messages[row];
                identities.remove(existing, row);
                existing.merge_reaction_metadata(&m);
                merge_delivery_state(&mut existing.delivery, m.delivery);
                merge_reply_metadata(existing, &m);
                if existing.id.starts_with("out_") && !m.id.is_empty() && !m.id.starts_with("out_")
                {
                    existing.id = m.id.clone();
                }
                let is_undo_msg = m.text.trim() == "[Tin nhắn đã thu hồi]";
                if m.ts > 0 && existing.ts != m.ts && (!is_undo_msg || existing.ts == 0) {
                    existing.ts = m.ts;
                    existing.time = m.time;
                }
                apply_recall_merge(existing, &m.text);
                if existing.sticker_id.is_none() && m.sticker_id.is_some() {
                    existing.sticker_id = m.sticker_id;
                    existing.sticker_cate_id = m.sticker_cate_id;
                    existing.sticker_type = m.sticker_type;
                }
                if existing.link_url.is_none() && m.link_url.is_some() {
                    existing.link_url = m.link_url;
                    existing.link_title = m.link_title;
                    existing.link_desc = m.link_desc;
                    existing.link_thumb = m.link_thumb;
                    existing.link_src = m.link_src;
                    if !m.text.is_empty()
                        && !existing.text.starts_with("[Đã thu hồi]")
                        && existing.text.trim() != "[Tin nhắn đã thu hồi]"
                    {
                        existing.text = m.text;
                    }
                }
                if existing.oa_card.is_none() && m.oa_card.is_some() {
                    existing.oa_card = m.oa_card;
                    if existing.kind == crate::message_model::MessageKind::Text
                        && m.kind != crate::message_model::MessageKind::Text
                    {
                        existing.kind = m.kind;
                    }
                }
                identities.insert(existing, row);
            } else {
                identities.insert(&m, thread.messages.len());
                thread.messages.push(m);
            }
        }
    }

    thread.messages.sort_by_key(|m| m.ts);
    repair_thread_quote_replies(&mut thread.messages);

    if let Some(newest) = thread.messages.last() {
        if !newest.text.is_empty() {
            thread.preview = newest.text.replace('\n', " ");
            thread.time = newest.time.clone();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum FriendshipStatus {
    #[default]
    Unknown,
    Friend,
    OutgoingPending,
    IncomingPending,
    Stranger,
}

/// Parse `/api/friend/reqstatus` (`cmd 11430`) JSON into `FriendshipStatus`
/// (`compact-app-pc.js:8351970`: `is_friend`, `is_requested` = I requested you, `is_requesting` = you requested me).
pub fn parse_friendship_status_response(resp: &Value) -> FriendshipStatus {
    let d = resp
        .get("data")
        .and_then(|x| x.get("data").or(Some(x)))
        .unwrap_or(resp);
    let is_friend = d
        .get("is_friend")
        .or_else(|| d.get("isFriend"))
        .and_then(|v| {
            v.as_i64()
                .or_else(|| v.as_bool().map(|b| if b { 1 } else { 0 }))
        })
        .unwrap_or(0);
    if is_friend == 1 {
        return FriendshipStatus::Friend;
    }
    let is_requesting = d
        .get("is_requesting")
        .or_else(|| d.get("isRequesting"))
        .and_then(|v| {
            v.as_i64()
                .or_else(|| v.as_bool().map(|b| if b { 1 } else { 0 }))
        })
        .unwrap_or(0);
    if is_requesting == 1 {
        return FriendshipStatus::IncomingPending;
    }
    let is_requested = d
        .get("is_requested")
        .or_else(|| d.get("isRequested"))
        .and_then(|v| {
            v.as_i64()
                .or_else(|| v.as_bool().map(|b| if b { 1 } else { 0 }))
        })
        .unwrap_or(0);
    if is_requested == 1 {
        return FriendshipStatus::OutgoingPending;
    }
    FriendshipStatus::Stranger
}

/// Returns `true` when a 1-1 thread is a Zalo system account or Official Account (OA)
/// that cannot be friended and must not display "Người lạ" / "Kết bạn" UI.
pub fn is_system_or_oa_thread(thread_id: &str, thread_name: &str) -> bool {
    let tid = thread_id.trim();
    if tid.starts_with("oa_") || tid.starts_with("OA_") {
        return true;
    }
    let norm = thread_name.trim().to_lowercase();
    if norm.is_empty() {
        return false;
    }
    const EXACT_NAMES: &[&str] = &[
        "zalo",
        "trợ lý kiki",
        "kiki",
        "thời tiết",
        "cloudotp",
        "zalopay",
        "zalo cloud",
        "zalo security",
        "zalo bảo mật",
        "zalo official",
        "zalo oa",
        "zalo video",
        "zalo mini app",
        "zalo stickers",
        "zing mp3",
        "zing news",
        "báo mới",
        "baomoi",
        "truyền thông số",
        "nạp điện thoại",
    ];
    if EXACT_NAMES.contains(&norm.as_str()) {
        return true;
    }
    const PREFIXES: &[&str] = &[
        "zalo ",
        "zalo-",
        "zalo –",
        "zstyle",
        "zalopay",
        "trợ lý ",
        "cloudotp",
        "tổng đài ",
        "cskh ",
        "thông báo ",
    ];
    PREFIXES.iter().any(|p| norm.starts_with(p))
}

/// Query `/api/friend/reqstatus` (`cmd 11430`) and return the typed `FriendshipStatus`.
pub async fn check_friendship_status(s: &Session, fid: &str) -> Result<FriendshipStatus> {
    let resp = get_friend_request_status(s, fid).await?;
    Ok(parse_friendship_status_response(&resp))
}
/// Check friend request status with a target user (`/api/friend/reqstatus`, cmd `11430`).
pub async fn get_friend_request_status(s: &Session, fid: &str) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let body = obj(&[("fid", json!(fid)), ("imei", json!(s.imei))]);
    let enc = s.encoded_params(&body)?;
    let host = if s.domains.friend.is_empty() {
        "https://tt-friend-wpa.chat.zalo.me"
    } else {
        &s.domains.friend
    };
    let url = format!(
        "{}/api/friend/reqstatus?{}&params={enc}",
        host.trim_end_matches('/'),
        s.common_params()
    );
    let resp = s.get(&url).await?;
    Ok(s.maybe_decrypt(resp, s.session_key_bytes()).await)
}

/// Query `/api/social/profile/lastOnline` (`cmd 12090`) and return `(is_online, label)`
/// matching Zalo PC's `getLastActiveText` (`user_active_time = 5 minutes`).
pub async fn get_user_online_status(s: &Session, uid: &str) -> Result<(bool, String)> {
    if s.login.is_none() || uid.trim().is_empty() {
        return Ok((false, String::new()));
    }
    let body = obj(&[
        ("uid", json!(uid.trim())),
        ("conv_type", json!(1)),
        ("imei", json!(s.imei)),
    ]);
    let enc = s.encoded_params(&body)?;
    let host = if s.domains.profile.is_empty() {
        "https://tt-profile-wpa.chat.zalo.me"
    } else {
        &s.domains.profile
    };
    let url = format!(
        "{}/api/social/profile/lastOnline?{}&params={enc}",
        host.trim_end_matches('/'),
        s.common_params()
    );
    let resp = s.get(&url).await?;
    let mut dec = s.maybe_decrypt(resp, s.session_key_bytes()).await;
    if let Some(d_str) = dec.get("data").and_then(Value::as_str) {
        if let Ok(parsed) = serde_json::from_str::<Value>(d_str.trim()) {
            dec["data"] = parsed;
        }
    }
    let d = dec
        .pointer("/data/data")
        .or_else(|| dec.get("data"))
        .unwrap_or(&dec);
    let show_online = d
        .pointer("/settings/show_online_status")
        .and_then(|v| v.as_bool().or_else(|| v.as_i64().map(|n| n != 0)))
        .unwrap_or(true);
    if !show_online {
        return Ok((false, String::new()));
    }
    let last_onl_raw = d
        .get("lastOnline")
        .and_then(|v| {
            v.as_i64()
                .or_else(|| v.as_str().and_then(|s| s.trim().parse::<i64>().ok()))
        })
        .unwrap_or(0);
    if last_onl_raw <= 0 {
        return Ok((false, String::new()));
    }
    let last_sec = if last_onl_raw > 10_000_000_000 {
        last_onl_raw / 1000
    } else {
        last_onl_raw
    };
    let now_sec = crate::login::unix_now() as i64;
    let diff_sec = (now_sec - last_sec).max(0);
    if diff_sec <= 300 {
        return Ok((true, "Vừa mới truy cập".into()));
    }
    let mins = diff_sec / 60;
    if mins < 60 {
        return Ok((false, format!("Truy cập {mins} phút trước")));
    }
    let hours = diff_sec / 3600;
    if hours < 24 {
        return Ok((false, format!("Truy cập {hours} giờ trước")));
    }
    let days = (diff_sec / 86400).min(7);
    Ok((false, format!("Truy cập {days} ngày trước")))
}

/// Send a friend request to a target user (`/api/friend/sendreq`, cmd `11424`).
pub async fn send_friend_request(s: &Session, to_id: &str, msg: &str) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let body = obj(&[
        ("toid", json!(to_id)),
        ("msg", json!(msg)),
        ("reqsrc", json!(30)),
        ("imei", json!(s.imei)),
        ("language", json!("vi")),
    ]);
    let enc = s.encoded_params(&body)?;
    let host = if s.domains.friend.is_empty() {
        "https://tt-friend-wpa.chat.zalo.me"
    } else {
        &s.domains.friend
    };
    let url = format!(
        "{}/api/friend/sendreq?{}",
        host.trim_end_matches('/'),
        s.common_params()
    );
    let resp = s.post_form(&url, format!("params={enc}")).await?;
    let dec = s.maybe_decrypt(resp, s.session_key_bytes()).await;
    if let Some(code) = dec.get("error_code").and_then(Value::as_i64) {
        if code != 0 {
            let err_msg = dec
                .get("error_message")
                .and_then(Value::as_str)
                .unwrap_or("unknown");
            anyhow::bail!("sendreq error {code}: {err_msg}");
        }
    }
    Ok(dec)
}

/// Accept a pending incoming friend request (`/api/friend/accept`, cmd `11427`).
pub async fn accept_friend_request(s: &Session, fid: &str) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let body = obj(&[("fid", json!(fid)), ("language", json!("vi"))]);
    let enc = s.encoded_params(&body)?;
    let host = if s.domains.friend.is_empty() {
        "https://tt-friend-wpa.chat.zalo.me"
    } else {
        &s.domains.friend
    };
    let url = format!(
        "{}/api/friend/accept?{}",
        host.trim_end_matches('/'),
        s.common_params()
    );
    let resp = s.post_form(&url, format!("params={enc}")).await?;
    let dec = s.maybe_decrypt(resp, s.session_key_bytes()).await;
    if let Some(code) = dec.get("error_code").and_then(Value::as_i64) {
        if code != 0 {
            let err_msg = dec
                .get("error_message")
                .and_then(Value::as_str)
                .unwrap_or("unknown");
            anyhow::bail!("accept error {code}: {err_msg}");
        }
    }
    Ok(dec)
}

/// Cancel / undo a pending outgoing friend request (`/api/friend/undo`, cmd `11441`).
pub async fn undo_friend_request(s: &Session, fid: &str) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let body = obj(&[("fid", json!(fid)), ("imei", json!(s.imei))]);
    let enc = s.encoded_params(&body)?;
    let host = if s.domains.friend.is_empty() {
        "https://tt-friend-wpa.chat.zalo.me"
    } else {
        &s.domains.friend
    };
    let url = format!(
        "{}/api/friend/undo?{}",
        host.trim_end_matches('/'),
        s.common_params()
    );
    let resp = s.post_form(&url, format!("params={enc}")).await?;
    Ok(s.maybe_decrypt(resp, s.session_key_bytes()).await)
}

/// Reject a pending incoming friend request (`/api/friend/reject`, cmd `11428`).
pub async fn reject_friend_request(s: &Session, fid: &str) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let body = obj(&[("fid", json!(fid)), ("imei", json!(s.imei))]);
    let enc = s.encoded_params(&body)?;
    let host = if s.domains.friend.is_empty() {
        "https://tt-friend-wpa.chat.zalo.me"
    } else {
        &s.domains.friend
    };
    let url = format!(
        "{}/api/friend/reject?{}",
        host.trim_end_matches('/'),
        s.common_params()
    );
    let resp = s.post_form(&url, format!("params={enc}")).await?;
    Ok(s.maybe_decrypt(resp, s.session_key_bytes()).await)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhoneLookupResult {
    pub user_id: String,
    pub display_name: String,
    pub zalo_name: String,
    pub phone: String,
    pub avatar: String,
    pub status: String,
    #[serde(default)]
    pub friendship: FriendshipStatus,
}

/// Normalize a Vietnamese or international phone string for Zalo's `/api/friend/profile/get`.
///
/// Converts leading `0` to `84` (e.g. `0912345678` -> `84912345678`), strips `+`, spaces, dots, and dashes.
pub fn normalize_phone_for_lookup(raw: &str) -> String {
    let digits: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
    if let Some(rest) = digits.strip_prefix('0') {
        format!("84{rest}")
    } else {
        digits
    }
}

/// Look up a Zalo user profile by phone number (`getUserByPhone` in `compact-app-pc.js:14306443`, cmd `11420`).
///
/// Endpoint: `GET {domains.friend}/api/friend/profile/get?{common}&params={encodeAES({ phone, avatar_size: 240, language: "vi", imei, reqSrc: 40 })}`
pub async fn find_user_by_phone(s: &Session, raw_phone: &str) -> Result<PhoneLookupResult> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let normalized = normalize_phone_for_lookup(raw_phone);
    if normalized.len() < 8 {
        anyhow::bail!("Số điện thoại không hợp lệ (cần ít nhất 9 chữ số)");
    }
    let body = obj(&[
        ("phone", json!(normalized)),
        ("avatar_size", json!(240)),
        ("language", json!("vi")),
        ("imei", json!(s.imei)),
        ("reqSrc", json!(40)),
    ]);
    let enc = s.encoded_params(&body)?;
    let host = if s.domains.friend.is_empty() {
        "https://tt-friend-wpa.chat.zalo.me"
    } else {
        s.domains.friend.trim_end_matches('/')
    };
    let url = format!(
        "{host}/api/friend/profile/get?{}&params={enc}",
        s.common_params()
    );
    let resp = s.get(&url).await?;
    let dec = s.maybe_decrypt(resp, s.session_key_bytes()).await;
    if let Some(code) = dec.get("error_code").and_then(Value::as_i64) {
        if code != 0 {
            if code == 216 {
                anyhow::bail!(
                    "Số điện thoại chưa đăng ký Zalo hoặc người dùng đã chặn tìm kiếm qua SĐT (mã 216)"
                );
            }
            let err_msg = dec
                .get("error_message")
                .and_then(Value::as_str)
                .unwrap_or("Không tìm thấy tài khoản");
            anyhow::bail!("{err_msg} (mã {code})");
        }
    }

    let data = dec.get("data").unwrap_or(&dec);
    let Some(uid) = pick_str(data, &["uid", "userId", "id", "globalId"]) else {
        anyhow::bail!("Không tìm thấy tài khoản Zalo ứng với số điện thoại này");
    };
    let display_name = pick_str(
        data,
        &["display_name", "displayName", "zalo_name", "zaloName"],
    )
    .unwrap_or_else(|| raw_phone.trim().to_string());
    let zalo_name = pick_str(
        data,
        &["zalo_name", "zaloName", "display_name", "displayName"],
    )
    .unwrap_or_else(|| display_name.clone());
    let avatar = pick_str(data, &["avatar", "avt"]).unwrap_or_default();
    let status = pick_str(data, &["status"]).unwrap_or_default();

    let friendship = check_friendship_status(s, &uid)
        .await
        .unwrap_or(FriendshipStatus::Unknown);

    Ok(PhoneLookupResult {
        user_id: uid,
        display_name,
        zalo_name,
        phone: raw_phone.trim().to_string(),
        avatar,
        status,
        friendship,
    })
}
/// Fetch a group poll (`GET group_poll /api/poll/detail`, cmd `12081`).
///
/// # Errors
/// Fails when logged out, on transport errors, or when the server rejects the poll
/// (see [`crate::poll::parse_poll_detail`]).
pub async fn get_poll_detail(s: &Session, poll_id: &str) -> Result<crate::poll::PollDetail> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let params = s.encoded_params(&crate::poll::detail_params(poll_id, &s.imei))?;
    let path = format!("/api/poll/detail?{}&params={params}", s.common_params());
    let resp = s.get_endpoint("group_poll", &path).await?;
    let resp = s.maybe_decrypt(resp, s.session_key_bytes()).await;
    crate::poll::parse_poll_detail(&resp)
}

/// Vote, change or clear a vote in a group poll (`GET group_poll /api/poll/vote`, cmd `12084`).
/// `option_ids` is the full new selection. Returns the options from the reply.
///
/// # Errors
/// Fails when logged out, on transport errors, or when the server rejects the vote
/// (code `166`: poll closed or locked).
pub async fn vote_poll(
    s: &Session,
    poll_id: &str,
    option_ids: &[i64],
) -> Result<Vec<crate::poll::PollOption>> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let params = s.encoded_params(&crate::poll::vote_params(poll_id, option_ids, &s.imei))?;
    let path = format!("/api/poll/vote?{}&params={params}", s.common_params());
    let resp = s.get_endpoint("group_poll", &path).await?;
    let resp = s.maybe_decrypt(resp, s.session_key_bytes()).await;
    let data = crate::poll::poll_response_data(&resp)?;
    Ok(crate::poll::parse_options(&data).unwrap_or_default())
}

/// Board topic type for a pinned chat message (`MSG_TOPIC` = 2 in Zalo's topic enum:
/// `TEXT_TOPIC=0`, `LINK_TOPIC=1`, `MSG_TOPIC=2`, `POLL_TOPIC=3`, `REMINDER_TOPIC=4`).
pub const MSG_TOPIC: i32 = 2;

/// Group id as the board API expects it: no `g`/`group_` prefix.
fn board_grid(thread_id: &str) -> String {
    thread_id
        .trim_start_matches("group_")
        .trim_start_matches('g')
        .trim_start_matches('G')
        .trim()
        .to_string()
}

/// Zalo answers HTTP 200 with `error_code` in the body; non-zero means the board call was refused
/// (permissions, stale grid, unsupported thread) and nothing changed server-side.
fn board_reject_nonzero(resp: Value) -> Result<Value> {
    let code = resp.get("error_code").and_then(Value::as_i64).unwrap_or(0);
    if code != 0 {
        let msg = resp
            .get("error_message")
            .and_then(Value::as_str)
            .filter(|m| !m.is_empty())
            .unwrap_or("board request rejected");
        anyhow::bail!("mã lỗi {code}: {msg}");
    }
    Ok(resp)
}

/// Topic `params` object for a pinned message, using the field names Zalo reads back
/// (`params.global_msg_id`, `params.client_msg_id`, `params.senderUid`, `params.msg_type`,
/// `params.title` — `getTopicDataFromMessage` in `compact-app-pc.js`).
fn message_topic_params(msg: &ChatMessage, thread_id: &str) -> Value {
    let title = if msg.sticker_id.is_some() {
        "[Nhãn dán]".to_string()
    } else {
        let trimmed = msg.text.trim();
        let mut s: String = trimmed.chars().take(120).collect();
        if trimmed.chars().count() > 120 {
            s.push('…');
        }
        if s.is_empty() {
            "[Tin nhắn]".to_string()
        } else {
            s
        }
    };
    let sender = if msg.is_outgoing {
        "0".to_string()
    } else {
        sidebar_sender_id(msg).unwrap_or_else(|| thread_id.to_string())
    };
    json!({
        "global_msg_id": msg.id,
        "client_msg_id": msg.client_message_id,
        "senderUid": sender,
        "title": title,
        "createTime": msg.ts,
    })
}

fn board_topic_body(grid: &str, thread_id: &str, msg: &ChatMessage, imei: &str) -> Value {
    json!({
        "grid": grid,
        "type": MSG_TOPIC,
        "color": -16777216,
        "emoji": "",
        "startTime": -1,
        "duration": -1,
        "params": message_topic_params(msg, thread_id).to_string(),
        "needPin": true,
        "pinAct": 1,
        "repeat": 0,
        "src": 0,
        "imei": imei,
    })
}

/// Pin one chat message.
///
/// Zalo stores a pinned message as a *board topic* of type `MSG_TOPIC` and pins it in the same
/// call. Groups: POST `groupboard-wpa` `/api/board/topic/createv2` (cmd `12007`). 1-1 chats:
/// GET `friendboard-wpa` `/api/friendboard/create` (cmd `12067`, confirmed by a live web capture).
pub async fn pin_message(
    s: &Session,
    thread_id: &str,
    is_group: bool,
    msg: &ChatMessage,
) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let grid = board_grid(thread_id);
    let body = if is_group {
        board_topic_body(&grid, thread_id, msg, &s.imei)
    } else {
        // 1-1 uses the friend board: same topic fields, keyed by the peer uid.
        let mut v = board_topic_body(&grid, thread_id, msg, &s.imei);
        if let Some(obj) = v.as_object_mut() {
            obj.remove("grid");
            obj.insert("uid".into(), json!(grid));
        }
        v
    };
    let params = s.encoded_params(&body)?;
    if is_group {
        let url = format!(
            "{}/api/board/topic/createv2?{}",
            crate::endpoints::GROUP_BOARD,
            s.common_params()
        );
        let resp = s.post_form(&url, format!("params={params}")).await?;
        board_reject_nonzero(s.maybe_decrypt(resp, s.session_key_bytes()).await)
    } else {
        let url = format!(
            "{}/api/friendboard/create?{}&params={params}",
            crate::endpoints::FRIEND_BOARD,
            s.common_params()
        );
        let resp = s.get(&url).await?;
        board_reject_nonzero(s.maybe_decrypt(resp, s.session_key_bytes()).await)
    }
}

/// Unpin a chat message previously pinned with [`pin_message`].
pub async fn unpin_message(
    s: &Session,
    thread_id: &str,
    is_group: bool,
    topic_id: &str,
) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    if topic_id.trim().is_empty() {
        // Without the board topic id the server has nothing to address; say so instead of letting
        // it answer with the opaque 114 "invalid parameter".
        anyhow::bail!("chưa có mã bảng tin (topic id) của tin nhắn này");
    }
    let grid = board_grid(thread_id);
    // Zalo unpins topics through the multi_unpin GET endpoint. Its `topics` array carries
    // `{topicId, topicType}` objects (module `L()` in `compact-app-pc.js`), not bare id strings —
    // sending strings is rejected with error 114 "invalid parameter".
    let topic_entry = json!({ "topicId": topic_id, "topicType": MSG_TOPIC });
    let body = if is_group {
        json!({ "grid": grid, "topics": [topic_entry], "boardVersion": 0 })
    } else {
        json!({ "uid": grid, "topics": [topic_entry], "boardVersion": 0 })
    };
    let params = s.encoded_params(&body)?;
    let url = if is_group {
        format!(
            "{}/api/board/multi_unpin?{}&params={params}",
            crate::endpoints::GROUP_BOARD,
            s.common_params()
        )
    } else {
        format!(
            "{}/api/friendboard/multi_unpin?{}&params={params}",
            crate::endpoints::FRIEND_BOARD,
            s.common_params()
        )
    };
    let resp = s.get(&url).await?;
    board_reject_nonzero(s.maybe_decrypt(resp, s.session_key_bytes()).await)
}

/// One pinned board item as the server reports it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BoardTopic {
    pub id: String,
    /// Message ids from `topic.params` (`global_msg_id` / `client_msg_id`), used to match the
    /// pinned message back to a loaded bubble.
    pub msg_ids: Vec<String>,
}

fn collect_msg_ids(params: &Value, out: &mut Vec<String>) {
    for key in ["global_msg_id", "client_msg_id", "msgId", "cliMsgId"] {
        if let Some(v) = params.get(key) {
            let s = match v {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                _ => String::new(),
            };
            if !s.is_empty() && s != "0" && !out.contains(&s) {
                out.push(s);
            }
        }
    }
}

/// Parse a board list response into topics. `data` is normally a JSON object, but some endpoints
/// hand it back as a JSON string.
pub fn parse_board_topics(resp: &Value) -> Vec<BoardTopic> {
    let mut data = resp.get("data").cloned().unwrap_or(Value::Null);
    if let Value::String(txt) = &data {
        data = serde_json::from_str::<Value>(txt).unwrap_or(Value::Null);
    }
    let mut raw: Vec<Value> = Vec::new();
    match &data {
        Value::Array(items) => raw.extend(items.iter().cloned()),
        Value::Object(map) => {
            for key in ["topics", "topic", "items", "data", "boards"] {
                match map.get(key) {
                    Some(Value::Array(items)) => raw.extend(items.iter().cloned()),
                    Some(v @ Value::Object(_)) => raw.push(v.clone()),
                    _ => {}
                }
            }
        }
        _ => {}
    }

    let mut out: Vec<BoardTopic> = Vec::new();
    for item in raw {
        let id = ["id", "topicId"]
            .iter()
            .find_map(|k| item.get(*k))
            .map(|v| match v {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                _ => String::new(),
            })
            .unwrap_or_default();
        if id.is_empty() {
            continue;
        }
        let mut msg_ids: Vec<String> = Vec::new();
        if let Some(params) = item.get("params") {
            let params = match params {
                Value::String(txt) => serde_json::from_str::<Value>(txt).unwrap_or(Value::Null),
                other => other.clone(),
            };
            collect_msg_ids(&params, &mut msg_ids);
        }
        out.push(BoardTopic { id, msg_ids });
    }
    out
}

fn board_trace(label: &str, detail: &str) {
    // Opt-in diagnostics: `CTDOZLITE_BOARD_TRACE=1` appends the board calls to `board_trace.log`
    // (gitignored). Off by default: payloads carry conversation content.
    if std::env::var("CTDOZLITE_BOARD_TRACE").ok().as_deref() != Some("1") {
        return;
    }
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("board_trace.log")
    {
        let _ = writeln!(f, "{label} | {detail}");
    }
}

async fn board_get_json(s: &Session, url: &str) -> Result<Value> {
    let resp = s.get(url).await?;
    let dec = s.maybe_decrypt(resp, s.session_key_bytes()).await;
    board_reject_nonzero(dec)
}

#[allow(dead_code)]
async fn board_post_json(s: &Session, url: &str, body: &Value) -> Result<Value> {
    let params = s.encoded_params(body)?;
    let resp = s.post_form(url, format!("params={params}")).await?;
    let dec = s.maybe_decrypt(resp, s.session_key_bytes()).await;
    board_reject_nonzero(dec)
}

/// Fetch the pinned board items of a conversation, so pins made elsewhere show up here too.
///
/// Zalo has two generations of this API and which one answers depends on the account/session, so
/// both are tried in order and the first accepted reply wins:
/// - 1-1: `GET /api/friendboard/list` (cmd `12065`), then `POST /api/board/oneone/list` (cmd `12432`).
/// - group: `GET /api/board/pin/list` (cmd `12001`), then `GET /api/board/list` (cmd `12140`).
pub async fn list_board_topics(
    s: &Session,
    thread_id: &str,
    is_group: bool,
) -> Result<Vec<BoardTopic>> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let grid = board_grid(thread_id);
    let mut errors: Vec<String> = Vec::new();

    if is_group {
        let body = json!({ "groupId": grid, "boardVersion": 0, "imei": s.imei });
        if let Ok(params) = s.encoded_params(&body) {
            let url = format!(
                "{}/api/board/pin/list?{}&params={params}",
                crate::endpoints::GROUP_BOARD,
                s.common_params()
            );
            match board_get_json(s, &url).await {
                Ok(v) => {
                    let topics = parse_board_topics(&v);
                    board_trace("pin/list", &format!("ok topics={}", topics.len()));
                    return Ok(topics);
                }
                Err(e) => {
                    board_trace("pin/list", &format!("err {e}"));
                    errors.push(format!("pin/list: {e}"));
                }
            }
        }
        let body = json!({
            "group_id": grid, "board_type": 0, "page": 0, "count": 50,
            "last_id": 0, "last_type": 0, "imei": s.imei,
        });
        if let Ok(params) = s.encoded_params(&body) {
            let url = format!(
                "{}/api/board/list?{}&params={params}",
                crate::endpoints::GROUP_BOARD,
                s.common_params()
            );
            match board_get_json(s, &url).await {
                Ok(v) => {
                    let topics = parse_board_topics(&v);
                    board_trace("board/list", &format!("ok topics={}", topics.len()));
                    return Ok(topics);
                }
                Err(e) => {
                    board_trace("board/list", &format!("err {e}"));
                    errors.push(format!("board/list: {e}"));
                }
            }
        }
    } else {
        let body = json!({
            "uid": grid, "page": 0, "count": 50, "boardVersion": 0, "imei": s.imei,
        });
        if let Ok(params) = s.encoded_params(&body) {
            let url = format!(
                "{}/api/friendboard/list?{}&params={params}",
                crate::endpoints::FRIEND_BOARD,
                s.common_params()
            );
            match board_get_json(s, &url).await {
                Ok(v) => {
                    let topics = parse_board_topics(&v);
                    board_trace("friendboard/list", &format!("ok topics={}", topics.len()));
                    return Ok(topics);
                }
                Err(e) => {
                    board_trace("friendboard/list", &format!("err {e}"));
                    errors.push(format!("friendboard/list: {e}"));
                }
            }
        }
        // `getListBoardOneOne`: GET, and everything except `objectData` sits inside that JSON
        // string — sending it as a POST body is what returned a generic -1.
        let object = json!({
            "uid": grid, "board_type": 0, "page": 0, "count": 50,
            "last_id": 0, "last_type": 0,
        });
        let body = json!({ "objectData": object.to_string() });
        if let Ok(params) = s.encoded_params(&body) {
            let url = format!(
                "{}/api/board/oneone/list?{}&params={params}",
                crate::endpoints::GROUP_BOARD,
                s.common_params()
            );
            match board_get_json(s, &url).await {
                Ok(v) => {
                    let topics = parse_board_topics(&v);
                    board_trace("oneone/list", &format!("ok topics={}", topics.len()));
                    return Ok(topics);
                }
                Err(e) => {
                    board_trace("oneone/list", &format!("err {e}"));
                    errors.push(format!("oneone/list: {e}"));
                }
            }
        }
    }

    anyhow::bail!("{}", errors.join(" · "))
}

/// Pin or unpin a conversation (`updatePinnedConversationsV2` @ `compact-app-pc.js:14354215`, cmd `12122`).
pub async fn update_pinned_conversation(
    s: &Session,
    thread_id: &str,
    is_group: bool,
    pin: bool,
) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let clean = thread_id
        .trim_start_matches("group_")
        .trim_start_matches('g')
        .trim_start_matches('G')
        .trim_start_matches('u')
        .trim();
    let prefixed = if is_group {
        format!("g{clean}")
    } else {
        format!("u{clean}")
    };
    let body = json!({
        "actionType": if pin { 1 } else { 2 },
        "conversations": [prefixed],
        "tab": 0
    });
    let params = s.encoded_params(&body)?;
    let url = format!(
        "{}/api/pinconvers/updatev2?{}",
        s.domains.conversation,
        s.common_params()
    );
    let resp = s.post_form(&url, format!("params={params}")).await?;
    Ok(s.maybe_decrypt(resp, s.session_key_bytes()).await)
}

/// Mark a conversation as unread or read (`addMarkAsUnread` / `removeMarkAsUnread` @ `compact-app-pc.js:14299958`, cmd `12013` / `12014`).
pub async fn set_unread_mark(
    s: &Session,
    thread_id: &str,
    is_group: bool,
    unread: bool,
) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let clean = thread_id
        .trim_start_matches("group_")
        .trim_start_matches('g')
        .trim_start_matches('G')
        .trim();
    let (convs_group, convs_user) = if is_group {
        (vec![clean], Vec::<&str>::new())
    } else {
        (Vec::<&str>::new(), vec![clean])
    };
    let (path, inner) = if unread {
        (
            "/api/conv/addUnreadMark",
            json!({
                "convsGroup": convs_group,
                "convsUser": convs_user,
                "imei": s.imei
            }),
        )
    } else {
        (
            "/api/conv/removeUnreadMark",
            json!({
                "convsGroup": convs_group,
                "convsUser": convs_user,
                "convsGroupData": [],
                "convsUserData": []
            }),
        )
    };
    let outer = json!({ "param": inner.to_string() });
    let params = s.encoded_params(&outer)?;
    let url = format!("{}{path}?{}", s.domains.conversation, s.common_params());
    let resp = s.post_form(&url, format!("params={params}")).await?;
    Ok(s.maybe_decrypt(resp, s.session_key_bytes()).await)
}

/// Wire action code for muting a conversation (`SET_MUTE` in Official Zalo PC JS).
pub const MUTE_ACTION_SET: u8 = 1;
/// Wire action code for unmuting a conversation (`UNSET_MUTE` in Official Zalo PC JS).
pub const MUTE_ACTION_UNSET: u8 = 3;

/// Build the decrypted JSON payload for `/api/social/profile/setmute`.
pub fn build_set_mute_payload(
    thread_id: &str,
    is_group: bool,
    duration_sec: i64,
    mute: bool,
    start_time: i64,
    imei: &str,
) -> Value {
    let clean = thread_id
        .trim_start_matches("group_")
        .trim_start_matches('g')
        .trim_start_matches('G')
        .trim();
    json!({
        "toid": clean,
        "duration": if mute { duration_sec } else { -1 },
        "action": if mute { MUTE_ACTION_SET } else { MUTE_ACTION_UNSET },
        "startTime": start_time,
        "muteType": if is_group { 2 } else { 1 },
        "imei": imei
    })
}

/// Mute or unmute notifications for a conversation (`setMuteConversation` @ `compact-app-pc.js`, cmd `11288`).
/// `duration_sec`: `3600` (1h), `14400` (4h), seconds until 8:00 AM, or `-1` (until reopened).
/// `mute`: `true` -> `action = 1` (`MUTE_ACTION_SET`), `false` -> `action = 3` (`MUTE_ACTION_UNSET`).
pub async fn set_mute_conversation(
    s: &Session,
    thread_id: &str,
    is_group: bool,
    duration_sec: i64,
    mute: bool,
) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let body = build_set_mute_payload(
        thread_id,
        is_group,
        duration_sec,
        mute,
        crate::login::unix_now(),
        &s.imei,
    );
    let params = s.encoded_params(&body)?;
    let path = format!("/api/social/profile/setmute?{}", s.common_params());
    let resp = s
        .post_endpoint_form("profile", &path, format!("params={params}"))
        .await?;
    Ok(s.maybe_decrypt(resp, s.session_key_bytes()).await)
}

/// Fetch the full mute snapshot (`chatEntries` + `groupChatEntries`) from Official Zalo PC
/// (`getMuteConversations` @ `default-login-main-startup-shared-worker.js`, cmd `11280`, #67).
pub async fn get_mute_conversations(s: &Session) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let body = json!({
        "imei": s.imei
    });
    let params = s.encoded_params(&body)?;
    let path = format!(
        "/api/social/profile/getmute?{}&params={params}",
        s.common_params()
    );
    let resp = s.get_endpoint("profile", &path).await?;
    Ok(s.maybe_decrypt(resp, s.session_key_bytes()).await)
}

/// Real-time mute/unmute control notification (`act_type: "mute"`, `act: "set" | "unset"`, #67).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MuteControlEvent {
    /// Canonical thread ID (`g<id>` for groups, `<id>` for 1-1 peers).
    pub thread_id: String,
    pub is_group: bool,
    pub muted: bool,
    /// Expiry Unix timestamp (`Some(ts)`), or `None` when indefinite (`duration == -1`) or unmuted.
    pub muted_until: Option<i64>,
    /// Value for `AppSettings::muted_threads` (`Some(-1)` for indefinite, `Some(until)` for timed, `None` when unmuted).
    pub settings_until: Option<i64>,
}

/// Parse `act_type: "mute"` items inside WebSocket `cmd 601`/`602`/`603` or `lp_control`
/// payloads (`_processMuteConversations` in Zalo PC `app.asar`, #67).
pub fn extract_mute_controls(data_node: &Value, now_unix: i64) -> Vec<MuteControlEvent> {
    let mut out = Vec::new();
    let mut candidates: Vec<&Value> = Vec::new();
    if let Some(arr) = data_node.get("controls").and_then(Value::as_array) {
        candidates.extend(arr.iter());
    } else if let Some(arr) = data_node.as_array() {
        for item in arr {
            if let Some(sub) = item.get("controls").and_then(Value::as_array) {
                candidates.extend(sub.iter());
            } else {
                candidates.push(item);
            }
        }
    } else {
        candidates.push(data_node);
    }

    for item in candidates {
        let c = item.get("content").unwrap_or(item);
        let act_type = c
            .get("act_type")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if act_type != "mute" {
            continue;
        }
        let act = c.get("act").and_then(Value::as_str).unwrap_or_default();
        let raw_data = c.get("data").unwrap_or(c);
        let parsed_data: Value;
        let d = if let Some(s) = raw_data.as_str() {
            if let Ok(v) = serde_json::from_str::<Value>(s) {
                parsed_data = v;
                &parsed_data
            } else {
                continue;
            }
        } else {
            raw_data
        };

        let raw_id = d
            .get("toId")
            .or_else(|| d.get("toid"))
            .or_else(|| d.get("id"))
            .and_then(|v| {
                v.as_str()
                    .map(str::to_owned)
                    .or_else(|| v.as_u64().map(|n| n.to_string()))
                    .or_else(|| v.as_i64().map(|n| n.to_string()))
            })
            .unwrap_or_default();
        let clean = raw_id
            .trim()
            .trim_start_matches("group_")
            .trim_start_matches('g')
            .trim_start_matches('G');
        if clean.is_empty() || clean == "0" {
            continue;
        }

        let mute_type = d
            .get("muteType")
            .and_then(|v| v.as_i64().or_else(|| v.as_str()?.parse().ok()))
            .unwrap_or(1);
        let is_group = mute_type == 2 || raw_id.starts_with('g') || raw_id.starts_with("group_");
        let thread_id = if is_group {
            format!("g{clean}")
        } else {
            clean.to_string()
        };

        let duration = d
            .get("duration")
            .and_then(|v| v.as_i64().or_else(|| v.as_str()?.parse().ok()))
            .unwrap_or(-1);
        let mute_mode = d
            .get("muteMode")
            .and_then(|v| v.as_i64().or_else(|| v.as_str()?.parse().ok()))
            .unwrap_or(1);
        let start_time = d
            .get("startTime")
            .or_else(|| d.get("systemTime"))
            .and_then(|v| v.as_i64().or_else(|| v.as_str()?.parse().ok()))
            .unwrap_or(now_unix);

        let is_set = match act {
            "set" => mute_mode == 1 && duration != 0,
            "unset" => false,
            _ => {
                let action_code = d.get("action").and_then(Value::as_i64).unwrap_or(1);
                action_code == 1 && mute_mode == 1 && duration != 0
            }
        };

        if !is_set {
            out.push(MuteControlEvent {
                thread_id,
                is_group,
                muted: false,
                muted_until: None,
                settings_until: None,
            });
            continue;
        }

        if duration < 0 {
            out.push(MuteControlEvent {
                thread_id,
                is_group,
                muted: true,
                muted_until: None,
                settings_until: Some(-1),
            });
        } else {
            let until = start_time.saturating_add(duration);
            if until > now_unix {
                out.push(MuteControlEvent {
                    thread_id,
                    is_group,
                    muted: true,
                    muted_until: Some(until),
                    settings_until: Some(until),
                });
            } else {
                out.push(MuteControlEvent {
                    thread_id,
                    is_group,
                    muted: false,
                    muted_until: None,
                    settings_until: None,
                });
            }
        }
    }
    out
}

/// Real-time typing notification event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypingEvent {
    pub thread_id: String,
    pub uid: String,
    pub is_typing: bool,
}

/// Real-time message read receipt / seen event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeenEvent {
    pub thread_id: String,
    pub uid: String,
    pub seen_ts: i64,
}

/// Parse `act_type: "typing"` and `act_type: "seen"` controls from WebSocket `ev.payload` or `lp_control`.
pub fn extract_typing_and_seen_controls(data_node: &Value) -> (Vec<TypingEvent>, Vec<SeenEvent>) {
    let mut typing_events = Vec::new();
    let mut seen_events = Vec::new();
    let mut candidates: Vec<&Value> = Vec::new();

    if let Some(arr) = data_node.get("controls").and_then(Value::as_array) {
        candidates.extend(arr.iter());
    } else if let Some(arr) = data_node.as_array() {
        for item in arr {
            if let Some(sub) = item.get("controls").and_then(Value::as_array) {
                candidates.extend(sub.iter());
            } else {
                candidates.push(item);
            }
        }
    } else {
        candidates.push(data_node);
    }

    for item in candidates {
        let c = item.get("content").unwrap_or(item);
        let act_type = c
            .get("act_type")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let act = c.get("act").and_then(Value::as_str).unwrap_or_default();

        let raw_data = c.get("data").unwrap_or(c);
        let parsed_data: Value;
        let d = if let Some(s) = raw_data.as_str() {
            if let Ok(v) = serde_json::from_str::<Value>(s) {
                parsed_data = v;
                &parsed_data
            } else {
                continue;
            }
        } else {
            raw_data
        };

        if act_type == "typing" || act == "typing" {
            let from_uid = d
                .get("fromUid")
                .or_else(|| d.get("uid"))
                .or_else(|| d.get("from"))
                .and_then(|v| {
                    v.as_str()
                        .map(str::to_owned)
                        .or_else(|| v.as_i64().map(|n| n.to_string()))
                })
                .unwrap_or_default();

            let grid = d.get("grid").and_then(|v| {
                v.as_str()
                    .map(str::to_owned)
                    .or_else(|| v.as_i64().map(|n| n.to_string()))
            });

            let to_id = d.get("toid").or_else(|| d.get("toId")).and_then(|v| {
                v.as_str()
                    .map(str::to_owned)
                    .or_else(|| v.as_i64().map(|n| n.to_string()))
            });

            let thread_id = if let Some(g) = grid {
                if g.starts_with('g') || g.starts_with('G') {
                    g
                } else {
                    format!("g{g}")
                }
            } else if !from_uid.is_empty() {
                from_uid.clone()
            } else if let Some(t) = to_id {
                t
            } else {
                continue;
            };

            let is_typing = d
                .get("status")
                .and_then(Value::as_i64)
                .map(|s| s != 0)
                .unwrap_or(true);

            typing_events.push(TypingEvent {
                thread_id,
                uid: from_uid,
                is_typing,
            });
        } else if act_type == "seen" || act_type == "read" || act == "seen" {
            let from_uid = d
                .get("fromUid")
                .or_else(|| d.get("uid"))
                .or_else(|| d.get("from"))
                .and_then(|v| {
                    v.as_str()
                        .map(str::to_owned)
                        .or_else(|| v.as_i64().map(|n| n.to_string()))
                })
                .unwrap_or_default();

            let grid = d.get("grid").and_then(|v| {
                v.as_str()
                    .map(str::to_owned)
                    .or_else(|| v.as_i64().map(|n| n.to_string()))
            });

            let to_id = d.get("toid").or_else(|| d.get("toId")).and_then(|v| {
                v.as_str()
                    .map(str::to_owned)
                    .or_else(|| v.as_i64().map(|n| n.to_string()))
            });

            let thread_id = if let Some(g) = grid {
                if g.starts_with('g') || g.starts_with('G') {
                    g
                } else {
                    format!("g{g}")
                }
            } else if !from_uid.is_empty() {
                from_uid.clone()
            } else if let Some(t) = to_id {
                t
            } else {
                continue;
            };

            let seen_ts = d
                .get("seenTime")
                .or_else(|| d.get("ts"))
                .and_then(|v| v.as_i64().or_else(|| v.as_str()?.parse().ok()))
                .unwrap_or(0);

            seen_events.push(SeenEvent {
                thread_id,
                uid: from_uid,
                seen_ts,
            });
        }
    }

    (typing_events, seen_events)
}

/// Send typing indicator signal to recipient or group (`/api/message/typing` or `/api/group/typing`).
pub async fn send_typing(s: &Session, to_id: &str, is_group: bool) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let host = if s.domains.conversation.is_empty() {
        "https://tt-conversation-wpa.zaloapp.com"
    } else {
        s.domains.conversation.trim_end_matches('/')
    };
    let clean_target = if is_group {
        to_id
            .trim_start_matches("group_")
            .trim_start_matches('g')
            .trim_start_matches('G')
    } else {
        to_id
    };
    let ep = if is_group {
        "/api/group/typing"
    } else {
        "/api/message/typing"
    };
    let mut m = serde_json::Map::new();
    m.insert("status".into(), json!(1));
    m.insert("imei".into(), json!(s.imei));
    if is_group {
        m.insert("grid".into(), json!(clean_target));
    } else {
        m.insert("toid".into(), json!(clean_target));
    }
    let params = s.encoded_params(&Value::Object(m))?;
    let url = format!("{host}{ep}?{}", s.common_params());
    let r = s.post_form(&url, format!("params={params}")).await?;
    Ok(s.maybe_decrypt(r, s.session_key_bytes()).await)
}

/// Send a real-time "is typing" indicator to a 1-1 contact (`cmd 11144`, `/api/message/typing`)
/// or a group (`cmd 11196`, `/api/group/typing`), unless blocked by `block_typing`.
pub async fn send_typing_indicator(s: &Session, thread_id: &str, is_group: bool) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    if crate::session::privacy_blocker_state().0 {
        return Ok(json!({"error_code": 0, "data": {}}));
    }
    let clean = thread_id
        .trim_start_matches("group_")
        .trim_start_matches('g')
        .trim_start_matches('G')
        .trim_start_matches('u')
        .trim();
    if clean.is_empty() {
        return Ok(json!({"error_code": 0, "data": {}}));
    }
    let (endpoint, path_and_q, body) = if is_group {
        (
            "group",
            format!("/api/group/typing?{}", s.common_params()),
            json!({
                "grid": clean,
                "imei": s.imei,
            }),
        )
    } else {
        (
            "chat",
            format!("/api/message/typing?{}", s.common_params()),
            json!({
                "toid": clean,
                "destType": 0,
                "imei": s.imei,
            }),
        )
    };
    let params = s.encoded_params(&body)?;
    let resp = s
        .post_endpoint_form(endpoint, &path_and_q, format!("params={params}"))
        .await?;
    Ok(s.maybe_decrypt(resp, s.session_key_bytes()).await)
}

/// Send a "seen" receipt (`cmd 12118` `/api/message/seenv2` or `cmd 12124` `/api/group/seenv2`),
/// unless blocked by `block_seen`.
pub async fn send_seen_v2(
    s: &Session,
    thread_id: &str,
    is_group: bool,
    last_msg_id: &str,
) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    if crate::session::privacy_blocker_state().2 {
        return Ok(json!({"error_code": 0, "data": {}}));
    }
    let clean = thread_id
        .trim_start_matches("group_")
        .trim_start_matches('g')
        .trim_start_matches('G')
        .trim_start_matches('u')
        .trim();
    if clean.is_empty() || last_msg_id.trim().is_empty() {
        return Ok(json!({"error_code": 0, "data": {}}));
    }
    let (endpoint, path_and_q, body) = if is_group {
        (
            "group",
            format!("/api/group/seenv2?{}", s.common_params()),
            json!({
                "msgInfos": json!({
                    "data": [{ "gmi": last_msg_id, "cmi": last_msg_id }],
                    "grid": clean,
                }).to_string(),
                "imei": s.imei,
            }),
        )
    } else {
        (
            "chat",
            format!("/api/message/seenv2?{}", s.common_params()),
            json!({
                "msgInfos": json!({
                    "data": [{ "gmi": last_msg_id, "cmi": last_msg_id }],
                    "senderId": clean,
                }).to_string(),
                "imei": s.imei,
            }),
        )
    };
    let params = s.encoded_params(&body)?;
    let resp = s
        .post_endpoint_form(endpoint, &path_and_q, format!("params={params}"))
        .await?;
    Ok(s.maybe_decrypt(resp, s.session_key_bytes()).await)
}

/// Request mobile message sync (`pullMobileMsg` @ `app.asar`, cmd `12000`, `file` endpoint, #68, #73).
/// Sends a wake-up/sync signal with a 2048-bit RSA SPKI DER public key to the user's phone Zalo client.
pub async fn request_phone_sync(s: &Session, public_key: &str) -> Result<Value> {
    let temp_key = crate::history_sync::latest_mobile_temp_key();
    request_phone_sync_with_temp_key(s, public_key, &temp_key).await
}

/// Request mobile message sync (`pullMobileMsg`, cmd `12000`) with an explicit `temp_key`
/// (received from `transfer_after_login` control event or empty for manual request).
pub async fn request_phone_sync_with_temp_key(
    s: &Session,
    public_key: &str,
    temp_key: &str,
) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let body = json!({
        "pc_name": s.computer_name,
        "public_key": public_key,
        "from_seq_id": 0,
        "is_retry": 0,
        "min_seq_id": 0,
        "temp_key": temp_key,
        "imei": s.imei
    });
    let params = s.encoded_params(&body)?;
    let path = format!(
        "/api/message/pull_mobile_msg?{}&params={params}",
        s.common_params()
    );
    if let Ok(resp) = s.get_endpoint("file", &path).await {
        let dec = s.maybe_decrypt(resp, s.session_key_bytes()).await;
        if dec.get("error_code").and_then(Value::as_i64) == Some(0) {
            return Ok(dec);
        }
    }
    let post_url = format!(
        "{}/api/message/pull_mobile_msg?{}",
        s.domains.file,
        s.common_params()
    );
    let resp = s.post_form(&post_url, format!("params={params}")).await?;
    Ok(s.maybe_decrypt(resp, s.session_key_bytes()).await)
}

/// Wake-up điện thoại cho Sync V2 (`cmd 592`). Zalo PC gửi wake-up TRƯỚC, chờ
/// user xác nhận trên điện thoại, RỒI mới gửi `cmd 590` (xem sync state machine
/// `waiting_for_mobile_open` → `USER_CONFIRM` → sync trong sync-v2-sub-worker.js).
/// Gửi 590 quá sớm khiến phone bỏ qua, chỉ hiện "đang đồng bộ" mà không nén.
/// Trả về `sync_id` để dùng cho [`send_sync_v2_request`].
pub async fn wake_phone_sync_v2(s: &Session) -> Result<String> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    // Đúng Zalo `generateRandomString(32)`: 32 ký tự base62 (A-Za-z0-9),
    // KHÔNG phải hex. Phone/server validate format, hex bị bỏ qua.
    let uin = s.login.as_ref().map(|l| l.uin.as_str()).unwrap_or_default();
    let sync_id = crate::param_cipher::random_base62(32);
    crate::history_sync::set_active_v2_sync_id(Some(sync_id.clone()));
    // Sync V2 BẮT BUỘC có keypair thật từ WASM libzproto (host bằng wasmtime,
    // không cần Node.js). Fail rõ ở đây thay vì gửi request rác.
    // The loader's error already says how to fix a missing Zalo PC / WASM.
    crate::history_sync::generate_sync_v2_keys_for(uin)
        .map_err(|e| anyhow::anyhow!("Không sinh được khóa Sync V2: {e}"))?;
    crate::logger::log_sync(&format!(
        "V2 session mở: sync={}",
        &sync_id.chars().take(8).collect::<String>()
    ));
    // KHÔNG gửi 592 ở đây: Zalo chỉ dùng 592 wake-up cho session ĐÃ tồn tại
    // (shared-worker.js `session.wakeup()`). Lệnh mở màn là 590.
    Ok(sync_id)
}

/// Tạo payload cho SyncFull v2 request theo đúng Zalo PC (`shared-worker.js:282247` & `314736`).
/// Khi `partitions` rỗng: gửi `type: "conversation"` với partition `"conversations"`.
/// Khi `partitions` có phần tử: gửi `type: "message"` với queries tương ứng.
pub fn build_sync_v2_payload(
    s: &Session,
    sync_id: &str,
    partitions: &[String],
) -> Result<serde_json::Value> {
    let temp_key = crate::history_sync::latest_mobile_temp_key();
    let keys = crate::history_sync::current_v2_keys()
        .map_err(|_| anyhow::anyhow!("chưa có khóa Sync V2 — gọi wake_phone_sync_v2 trước"))?;
    let (ik_pub, ek_pub) = (keys.ik_pub, keys.ek_pub);

    let (req_type, queries) = if partitions.is_empty() {
        (
            "conversation",
            vec![json!({
                "partition": "",
                "from": 0,
                "to": 9_007_199_254_740_991_i64,
                "limit": 9_007_199_254_740_991_i64
            })],
        )
    } else {
        let q: Vec<serde_json::Value> = partitions
            .iter()
            .map(|p| {
                let partition_str = if p.starts_with("oneone/") || p.starts_with("group/") {
                    p.clone()
                } else {
                    format!("oneone/{p}")
                };
                json!({
                    "partition": partition_str,
                    "from": 0,
                    "to": 9_007_199_254_740_991_i64,
                    "limit": 9_007_199_254_740_991_i64
                })
            })
            .collect();
        ("message", q)
    };

    Ok(json!({
        "syncId": sync_id,
        "syncType": 0, // No.d.SyncFull = 0
        "ek": ek_pub,
        "ik": ik_pub,
        "toDevice": 0, // No.c.Mobile = 0
        "tempKey": temp_key,
        "deviceName": s.computer_name,
        "req": {
            "type": req_type,
            "priority": 1,
            "queries": queries,
            "batchSize": 2000
        },
        "ver": 1,
        "ussidx": 0
    }))
}

/// Gửi yêu cầu đồng bộ tin nhắn qua HTTP endpoint `/api/transfer-sync-v2/request-sync` (cmd `12888`, file domain).
/// Zalo PC gọi hàm này (`ge.requestSyncMessage`) song song hoặc trước WebSocket để đăng ký phiên sync với server.
pub async fn request_phone_sync_v2_http(
    s: &Session,
    req_id: &str,
    sync_data: &Value,
) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let body = json!({
        "reqId": req_id,
        "data": sync_data.to_string()
    });
    let params = s.encoded_params(&body)?;
    let path = format!(
        "/api/transfer-sync-v2/request-sync?{}&params={params}",
        s.common_params()
    );
    if let Ok(resp) = s.get_endpoint("file", &path).await {
        let dec = s.maybe_decrypt(resp, s.session_key_bytes()).await;
        if dec.get("error_code").and_then(Value::as_i64) == Some(0) {
            return Ok(dec);
        }
    }
    let get_url = format!(
        "{}/api/transfer-sync-v2/request-sync?{}&params={params}",
        s.domains.file,
        s.common_params()
    );
    let resp = match s.get(&get_url).await {
        Ok(v) => v,
        Err(_) => s.post_form(&get_url, format!("params={params}")).await?,
    };
    Ok(s.maybe_decrypt(resp, s.session_key_bytes()).await)
}

/// Tra cứu danh sách partition (`oneone/<globalId>` và `group/<globalId>`) đúng chuẩn Zalo PC
/// (`GetGlobalNoisedIdsHandler` @ `app.asar`: `getprofiles/v2` → `profile.globalId` cho 1-1,
/// `getmg-v2` → `groupInfo.globalId` cho nhóm; KHÔNG dùng `/api/znoise` vì `znoise` trả ID khác
/// khiến iPhone báo `transfer_error` `err=999` cho mọi partition).
pub async fn resolve_thread_partitions(s: &Session, threads: &[Thread]) -> Vec<String> {
    let own_uin = s
        .login
        .as_ref()
        .map(|l| l.uin.trim().to_string())
        .unwrap_or_default();
    let send2me_id = s
        .login
        .as_ref()
        .map(|l| l.send2me_id.trim().to_string())
        .unwrap_or_default();

    let mut fids: Vec<String> = Vec::new();
    // (clean_numeric_gid, original_thread_id)
    let mut gids: Vec<(String, String)> = Vec::new();

    for t in threads {
        let tid = t.id.trim();
        if tid.is_empty() || (!send2me_id.is_empty() && tid == send2me_id) {
            continue;
        }
        if t.is_group || tid.starts_with('g') || tid.starts_with("group_") {
            let clean = tid
                .trim_start_matches("group_")
                .trim_start_matches('g')
                .trim_start_matches('G')
                .trim();
            if !clean.is_empty()
                && clean.bytes().all(|b| b.is_ascii_digit())
                && !gids.iter().any(|(c, _)| c == clean)
            {
                gids.push((clean.to_string(), tid.to_string()));
            }
        } else if tid.bytes().all(|b| b.is_ascii_digit()) && !fids.iter().any(|u| u == tid) {
            fids.push(tid.to_string());
        }
    }

    let extract_gid = |v: &Value| -> Option<String> {
        v.get("globalId")
            .and_then(|g| {
                g.as_str()
                    .map(str::trim)
                    .map(str::to_string)
                    .or_else(|| g.as_i64().map(|n| n.to_string()))
                    .or_else(|| g.as_u64().map(|n| n.to_string()))
            })
            .filter(|id| !id.is_empty() && id != "0" && id != "-1")
    };

    let mut lookup_uids = fids.clone();
    if !own_uin.is_empty() && !lookup_uids.iter().any(|u| u == &own_uin) {
        lookup_uids.push(own_uin.clone());
    }

    let mut resolved_users: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();
    for chunk in lookup_uids.chunks(40) {
        let pversion_map: Vec<String> = chunk.iter().map(|u| format!("{u}_0")).collect();
        let body = json!({
            "phonebook_version": 0,
            "friend_pversion_map": pversion_map,
            "avatar_size": 240,
            "language": "vi",
            "show_online_status": 1,
            "imei": s.imei,
        });
        let Ok(enc) = s.encoded_params(&body) else {
            continue;
        };
        let url = format!(
            "{}/api/social/friend/getprofiles/v2?{}",
            s.domains.profile,
            s.common_params()
        );
        let Ok(resp) = s.post_form(&url, format!("params={enc}")).await else {
            continue;
        };
        let dec = s.maybe_decrypt(resp, s.session_key_bytes()).await;
        if let Some(changed) = dec
            .get("data")
            .and_then(|d| d.get("changed_profiles").or(Some(d)))
            .and_then(Value::as_object)
        {
            for (uid, pval) in changed {
                let clean_uid = uid.split('_').next().unwrap_or(uid).trim();
                if let Some(gid) = extract_gid(pval) {
                    resolved_users.insert(clean_uid.to_string(), gid);
                }
            }
        }
    }

    // Fallback cho hội thoại 1-1 người lạ chưa nằm trong `changed_profiles` (tối đa 12 hội thoại).
    let missing_uids: Vec<String> = fids
        .iter()
        .filter(|u| !resolved_users.contains_key(*u))
        .take(12)
        .cloned()
        .collect();
    for uid in &missing_uids {
        let body = json!({
            "phonebook_version": 0,
            "friend_pversion_map": [format!("{uid}_0")],
            "avatar_size": 240,
            "language": "vi",
            "show_online_status": 1,
            "imei": s.imei,
        });
        let Ok(enc) = s.encoded_params(&body) else {
            continue;
        };
        let url = format!(
            "{}/api/social/friend/getprofiles?{}&params={enc}&os=8",
            s.domains.profile,
            s.common_params()
        );
        if let Ok(resp) = s.get(&url).await {
            let dec = s.maybe_decrypt(resp, s.session_key_bytes()).await;
            if let Some(prof_obj) = dec.get("data").and_then(|d| d.get(uid).or(Some(d))) {
                if let Some(gid) = extract_gid(prof_obj) {
                    resolved_users.insert(uid.clone(), gid);
                }
            }
        }
    }

    let mut partitions = Vec::new();
    let mut noised_to_real: Vec<(String, String, bool)> = Vec::new();

    if !own_uin.is_empty() {
        if let Some(my_gid) = resolved_users.get(&own_uin) {
            noised_to_real.push((my_gid.clone(), own_uin.clone(), false));
        }
    }

    for fid in &fids {
        if !own_uin.is_empty() && fid == &own_uin {
            continue;
        }
        if let Some(gid) = resolved_users.get(fid) {
            partitions.push(format!("oneone/{gid}"));
            noised_to_real.push((gid.clone(), fid.clone(), false));
        }
    }

    if !gids.is_empty() {
        let raw_gids: Vec<String> = gids.iter().map(|(c, _)| c.clone()).collect();
        if let Ok((info_map, _)) = fetch_group_profiles(s, &raw_gids).await {
            for (clean_gid, orig_tid) in &gids {
                if let Some(gval) = info_map
                    .get(clean_gid)
                    .or_else(|| info_map.get(&format!("g{clean_gid}")))
                {
                    if let Some(gid) = extract_gid(gval) {
                        partitions.push(format!("group/{gid}"));
                        noised_to_real.push((gid, orig_tid.clone(), true));
                    }
                }
            }
        }
    }

    if !noised_to_real.is_empty() {
        crate::history_sync::record_sync_v2_partition_map(&noised_to_real);
    }
    partitions
}

/// Gửi `cmd 590` (SYNC_MESSAGE.REQUEST) SAU KHI user đã xác nhận trên điện
/// thoại. Keys phải đã sinh bởi [`wake_phone_sync_v2`] (giữ trong RAM).
pub fn send_sync_v2_request(s: &Session, sync_id: &str) -> Result<bool> {
    send_sync_v2_request_with_queries(s, sync_id, &[])
}

/// Gửi `cmd 590` với queries partition thật (`oneone/<globalNoisedId>` hoặc `group/<globalNoisedId>`).
/// Chỉ gửi đúng `cmd 590` để mở phiên Sync V2 (không kèm `cmd 592` hay HTTP `/request-sync`).
pub fn send_sync_v2_request_with_queries(
    s: &Session,
    sync_id: &str,
    noised_ids: &[String],
) -> Result<bool> {
    let sync_data = build_sync_v2_payload(s, sync_id, noised_ids)?;
    crate::logger::log_sync(&format!(
        "590 payload: {}",
        sync_data.to_string().chars().take(600).collect::<String>()
    ));
    let sent_ws = crate::socket::send_sync_message_ws_request(&sync_data);
    crate::logger::log_sync(&format!(
        "590 gửi: sync={} ws_590={}",
        &sync_id.chars().take(8).collect::<String>(),
        sent_ws
    ));
    Ok(sent_ws)
}

/// Cancel an active mobile message sync request (`cancelTransferMessage` @ `app.asar:14372356`, cmd `12700`, #68).
pub async fn cancel_phone_sync(s: &Session, public_key: &str) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let body = json!({
        "pc_name": s.computer_name,
        "public_key": public_key,
        "imei": s.imei
    });
    let params = s.encoded_params(&body)?;
    let path = format!(
        "/api/message/cancel_pull_mobile_msg?{}&params={params}",
        s.common_params()
    );
    let resp = s.get_endpoint("file", &path).await?;
    Ok(s.maybe_decrypt(resp, s.session_key_bytes()).await)
}

/// Notify Zalo server and mobile device that sync snapshot download is completed
/// (`cleanSync` @ `app.asar`, cmd `12003`), dismissing the mobile sync spinner.
pub async fn clean_sync(s: &Session, public_key: &str) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let body = json!({
        "public_key": public_key,
        "imei": s.imei
    });
    let params = s.encoded_params(&body)?;
    let path = format!(
        "/api/message/delete_snapshot_mobile_msg?{}&params={params}",
        s.common_params()
    );
    let resp = s.get_endpoint("file", &path).await?;
    Ok(s.maybe_decrypt(resp, s.session_key_bytes()).await)
}

/// Mobile sync control event received via WebSocket (`cmd 601`/`602`/`603`) or `lp_control`
/// (`act_type: "transfer_msg" | "syncmsgmb" | "transfer_sync2"`, `main-startup.js:647494`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MobileSyncControlEvent {
    /// `act == "transfer_after_login"` — carries `temp_key` issued right after QR scan.
    TransferAfterLogin { temp_key: String },
    /// `act == "transfer_status"` (`status == 3`) — phone displayed confirmation prompt and is waiting for user tap!
    WaitingConfirmOnPhone { sync_id: String },
    /// `act == "user_confirm"` (`user_action == 1 | 3`) or `transfer_status` (`status == 4 | 5`) — user tapped "Đồng bộ" on their phone!
    UserConfirmed,
    /// `act == "user_confirm"` (`user_action == 0`) or `transfer_status` (`status == 6 | 7`) — user declined sync on their phone.
    UserRejected,
    /// `act == "user_confirm"` (`user_action == 2`) or `transfer_status` (`status == 8`) — phone is busy backing up/restoring (`BUSY_RESTORING`).
    MobileBusyRestoring,
    /// `act == "syncmsg_info"` or `act == "upload_batch"` — phone finished uploading backup package (`url`, `file_name`, `encrypted_key`).
    BackupReady {
        url: String,
        file_name: String,
        encrypted_key: String,
        checksum: String,
    },
    /// `act == "upload_batch"` (`act_type == "transfer_sync2"`) — Sync v2 chunk (used by iOS).
    SyncV2Batch {
        url: String,
        sync_id: String,
        batch_type: u8,
        is_last: bool,
    },
    /// `act == "upload_batch"` with `err == 0`, empty `msgUrl`, and `isLast == 1`.
    SyncV2EmptyDone { sync_id: String },
    /// `act == "transfer_status"` (`status == 2`) — phone went idle/backgrounded.
    SyncV2MobileIdle { sync_id: String },
    /// `act == "transfer_error"`, `upload_batch` with `err != 0`, or fatal mobile status (9/10).
    SyncV2Error {
        sync_id: String,
        err: i64,
        message: String,
        is_last: bool,
    },
}

/// Extract `transfer_msg` / `syncmsgmb` / `transfer_sync2` events from a control payload.
pub fn extract_mobile_sync_controls(data_node: &Value) -> Vec<MobileSyncControlEvent> {
    let mut out = Vec::new();
    let mut candidates: Vec<&Value> = Vec::new();
    if let Some(arr) = data_node.get("controls").and_then(Value::as_array) {
        candidates.extend(arr.iter());
    } else if let Some(arr) = data_node.as_array() {
        for item in arr {
            if let Some(sub) = item.get("controls").and_then(Value::as_array) {
                candidates.extend(sub.iter());
            } else {
                candidates.push(item);
            }
        }
    } else {
        candidates.push(data_node);
    }

    for item in candidates {
        let c = item.get("content").unwrap_or(item);
        let act_type = c
            .get("act_type")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if !matches!(act_type, "transfer_msg" | "syncmsgmb" | "transfer_sync2") {
            continue;
        }
        let act = c.get("act").and_then(Value::as_str).unwrap_or_default();
        let parsed_data: Value = match c.get("data") {
            Some(Value::String(s)) => serde_json::from_str(s).unwrap_or(Value::Null),
            Some(v) => v.clone(),
            None => Value::Null,
        };
        if act_type == "transfer_sync2" && act != "transfer_after_login" {
            let sid_log = parsed_data
                .get("syncId")
                .and_then(Value::as_str)
                .unwrap_or("-");
            let st_log = parsed_data
                .get("status")
                .and_then(|v| v.as_i64().or_else(|| v.as_str()?.parse().ok()));
            let bt_log = parsed_data
                .get("batchType")
                .and_then(|v| v.as_i64().or_else(|| v.as_str()?.parse().ok()));
            let err_log = parsed_data
                .get("err")
                .or_else(|| parsed_data.get("errorCode"))
                .and_then(|v| v.as_i64().or_else(|| v.as_str()?.parse().ok()));
            let last_log = parsed_data.get("isLast").and_then(|v| {
                v.as_i64()
                    .or_else(|| v.as_str()?.parse().ok())
                    .or_else(|| v.as_bool().map(|b| if b { 1 } else { 0 }))
            });
            let has_url = parsed_data
                .get("msgUrl")
                .or_else(|| parsed_data.get("url"))
                .and_then(Value::as_str)
                .is_some_and(|u| !u.is_empty());
            crate::logger::log_sync(&format!(
                "ctrl transfer_sync2: act={act} sync={sid_log} status={st_log:?} batchType={bt_log:?} err={err_log:?} isLast={last_log:?} has_url={has_url}"
            ));
        } else if matches!(act_type, "transfer_msg" | "syncmsgmb") && act != "transfer_after_login" {
            let ua_log = parsed_data
                .get("user_action")
                .and_then(|v| v.as_i64().or_else(|| v.as_str()?.parse().ok()));
            let has_url = parsed_data
                .get("url")
                .and_then(Value::as_str)
                .is_some_and(|u| !u.is_empty());
            crate::logger::log_sync(&format!(
                "ctrl {act_type}: act={act} user_action={ua_log:?} has_url={has_url}"
            ));
        }
        match act {
            "transfer_after_login" => {
                if let Some(tk) = parsed_data
                    .get("temp_key")
                    .and_then(Value::as_str)
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                {
                    out.push(MobileSyncControlEvent::TransferAfterLogin {
                        temp_key: tk.to_string(),
                    });
                }
            }
            "user_confirm" => {
                let ua = parsed_data
                    .get("user_action")
                    .and_then(|v| v.as_i64().or_else(|| v.as_str()?.parse().ok()))
                    .unwrap_or(-1);
                if ua == 1 || ua == 3 {
                    out.push(MobileSyncControlEvent::UserConfirmed);
                } else if ua == 2 {
                    out.push(MobileSyncControlEvent::MobileBusyRestoring);
                } else if ua == 0 {
                    out.push(MobileSyncControlEvent::UserRejected);
                }
            }
            "transfer_status" => {
                let st = parsed_data
                    .get("status")
                    .and_then(|v| v.as_i64().or_else(|| v.as_str()?.parse().ok()))
                    .unwrap_or(-1);
                let sid = parsed_data
                    .get("syncId")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string();
                match st {
                    2 => out.push(MobileSyncControlEvent::SyncV2MobileIdle { sync_id: sid }),
                    3 => out.push(MobileSyncControlEvent::WaitingConfirmOnPhone { sync_id: sid }),
                    4 | 5 => out.push(MobileSyncControlEvent::UserConfirmed),
                    6 | 7 => out.push(MobileSyncControlEvent::UserRejected),
                    8 => out.push(MobileSyncControlEvent::MobileBusyRestoring),
                    9 => out.push(MobileSyncControlEvent::SyncV2Error {
                        sync_id: sid,
                        err: 1105,
                        message: "Điện thoại hết dung lượng trống".to_string(),
                        is_last: true,
                    }),
                    10 => out.push(MobileSyncControlEvent::SyncV2Error {
                        sync_id: sid,
                        err: 1107,
                        message: "Zalo trên điện thoại đã đăng xuất".to_string(),
                        is_last: true,
                    }),
                    _ => {}
                }
            }
            "upload_batch" => {
                let err = parsed_data
                    .get("err")
                    .or_else(|| parsed_data.get("errorCode"))
                    .and_then(|v| v.as_i64().or_else(|| v.as_str()?.parse().ok()))
                    .unwrap_or(0);
                let sid = parsed_data
                    .get("syncId")
                    .and_then(Value::as_str)
                    .unwrap_or("sync_v2_batch")
                    .to_string();
                let batch_type = parsed_data
                    .get("batchType")
                    .and_then(|v| v.as_i64().or_else(|| v.as_str()?.parse().ok()))
                    .unwrap_or(0) as u8;
                let is_last = parsed_data
                    .get("isLast")
                    .and_then(|v| {
                        v.as_i64()
                            .or_else(|| v.as_str()?.parse().ok())
                            .or_else(|| v.as_bool().map(|b| if b { 1 } else { 0 }))
                    })
                    .unwrap_or(0)
                    == 1;
                if err != 0 {
                    let message = parsed_data
                        .get("errorMessage")
                        .or_else(|| parsed_data.get("message"))
                        .and_then(Value::as_str)
                        .unwrap_or("Điện thoại báo lỗi upload batch")
                        .to_string();
                    out.push(MobileSyncControlEvent::SyncV2Error {
                        sync_id: sid,
                        err,
                        message,
                        is_last: is_last || batch_type == 2,
                    });
                    continue;
                }
                let url = parsed_data
                    .get("msgUrl")
                    .or_else(|| parsed_data.get("url"))
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string();
                if !url.is_empty() {
                    out.push(MobileSyncControlEvent::SyncV2Batch {
                        url,
                        sync_id: sid,
                        batch_type,
                        is_last,
                    });
                } else if is_last && act_type == "transfer_sync2" {
                    out.push(MobileSyncControlEvent::SyncV2EmptyDone { sync_id: sid });
                } else {
                    out.push(MobileSyncControlEvent::MobileBusyRestoring);
                }
            }
            "transfer_error" => {
                let sid = parsed_data
                    .get("syncId")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string();
                let err = parsed_data
                    .get("errorCode")
                    .or_else(|| parsed_data.get("err"))
                    .and_then(|v| v.as_i64().or_else(|| v.as_str()?.parse().ok()))
                    .unwrap_or(-1);
                let batch_type = parsed_data
                    .get("batchType")
                    .and_then(|v| v.as_i64().or_else(|| v.as_str()?.parse().ok()))
                    .unwrap_or(0) as u8;
                let is_last = parsed_data
                    .get("isLast")
                    .and_then(|v| {
                        v.as_i64()
                            .or_else(|| v.as_str()?.parse().ok())
                            .or_else(|| v.as_bool().map(|b| if b { 1 } else { 0 }))
                    })
                    .unwrap_or(0)
                    == 1;
                let message = parsed_data
                    .get("errorMessage")
                    .or_else(|| parsed_data.get("message"))
                    .and_then(Value::as_str)
                    .unwrap_or("Điện thoại báo lỗi đồng bộ")
                    .to_string();
                out.push(MobileSyncControlEvent::SyncV2Error {
                    sync_id: sid,
                    err,
                    message,
                    is_last: is_last || batch_type == 2,
                });
            }
            "syncmsg_info" => {
                let url = parsed_data
                    .get("url")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string();
                let file_name = parsed_data
                    .get("file_name")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string();
                let encrypted_key = parsed_data
                    .get("encrypted_key")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string();
                let checksum = parsed_data
                    .get("checksum_code")
                    .or_else(|| parsed_data.get("checksum"))
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string();
                if !url.is_empty() {
                    out.push(MobileSyncControlEvent::BackupReady {
                        url,
                        file_name,
                        encrypted_key,
                        checksum,
                    });
                }
            }
            _ => {}
        }
    }
    out
}

/// Hide or unhide a conversation (`updateItemHiddenChat` @ `compact-app-pc.js:14384423`, cmd `10049`).
pub async fn set_hidden_conversation(
    s: &Session,
    thread_id: &str,
    is_group: bool,
    hide: bool,
) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let clean = thread_id
        .trim_start_matches("group_")
        .trim_start_matches('g')
        .trim_start_matches('G')
        .trim();
    let entry = json!([{
        "thread_id": clean,
        "is_group": if is_group { 1 } else { 0 }
    }]);
    let (add_str, del_str) = if hide {
        (entry.to_string(), "[]".to_string())
    } else {
        ("[]".to_string(), entry.to_string())
    };
    let body = json!({
        "del_threads": del_str,
        "add_threads": add_str,
        "imei": s.imei
    });
    let params = s.encoded_params(&body)?;
    let url = format!(
        "{}/api/hiddenconvers/add-remove?{}",
        s.domains.conversation,
        s.common_params()
    );
    let resp = s.post_form(&url, format!("params={params}")).await?;
    Ok(s.maybe_decrypt(resp, s.session_key_bytes()).await)
}

/// Delete conversation history for me (`deleteOneOneConversationV2` / `deleteGroupConversationV2` @ `compact-app-pc.js:14374881`, cmd `11778` / `11835`).
pub async fn delete_conversation(
    s: &Session,
    thread_id: &str,
    is_group: bool,
    last_cli_msg_id: &str,
) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let clean = thread_id
        .trim_start_matches("group_")
        .trim_start_matches('g')
        .trim_start_matches('G')
        .trim();
    let cli_id = if last_cli_msg_id.is_empty() {
        format!("{}", crate::login::unix_now() * 1000)
    } else {
        last_cli_msg_id.to_string()
    };
    if is_group {
        let body = json!({
            "grid": clean,
            "cliMsgId": cli_id,
            "conver": {},
            "onlyMe": 1,
            "imei": s.imei
        });
        let params = s.encoded_params(&body)?;
        let url = format!(
            "{}/api/group/deleteconver?{}",
            s.domains.group,
            s.common_params()
        );
        let resp = s.post_form(&url, format!("params={params}")).await?;
        Ok(s.maybe_decrypt(resp, s.session_key_bytes()).await)
    } else {
        let body = json!({
            "toid": clean,
            "cliMsgId": cli_id,
            "conver": {},
            "onlyMe": 1,
            "imei": s.imei
        });
        let params = s.encoded_params(&body)?;
        let url = format!(
            "{}/api/message/deleteconver?{}",
            s.domains.chat,
            s.common_params()
        );
        let resp = s.post_form(&url, format!("params={params}")).await?;
        Ok(s.maybe_decrypt(resp, s.session_key_bytes()).await)
    }
}

/// Leave a Zalo group: `POST {domains.group}/api/group/leave?{common}` (commandId 11189).
/// Payload (AES encrypted): `{ "grids": [clean_grid], "imei": s.imei, "silent": 0|1, "language": "vi" }`
pub async fn leave_group(s: &Session, grid: &str, silent: bool) -> Result<Value> {
    if s.login.is_none() {
        anyhow::bail!("chưa đăng nhập");
    }
    let clean = grid
        .trim()
        .trim_start_matches("g_")
        .trim_start_matches("group_")
        .trim();
    if clean.is_empty() {
        anyhow::bail!("group ID không hợp lệ");
    }
    let body = json!({
        "grids": [clean],
        "imei": s.imei,
        "silent": if silent { 1 } else { 0 },
        "language": "vi"
    });
    let params = s.encoded_params(&body)?;
    let url = format!(
        "{}/api/group/leave?{}",
        s.domains.group,
        s.common_params()
    );
    let resp = s.post_form(&url, format!("params={params}")).await?;
    let resp = s.maybe_decrypt(resp, s.session_key_bytes()).await;
    if let Some(code) = resp.get("error_code").and_then(Value::as_i64) {
        if code != 0 {
            let msg = resp
                .get("error_message")
                .and_then(Value::as_str)
                .unwrap_or("Không thể rời nhóm");
            anyhow::bail!("{msg} (mã {code})");
        }
    }
    Ok(resp)
}
