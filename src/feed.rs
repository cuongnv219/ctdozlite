//! Social Feed (Nhật ký) & Social Profile module for Zalo PC / Android
//! -------------------------------------------------------------------
//! Cung cấp cấu trúc dữ liệu và các API tương tác mạng xã hội thời gian thực:
//! - Lấy hồ sơ chi tiết bạn bè & chính mình (Ảnh bìa Cover HD, Avatar, Ngày sinh, Bio, lastActionTime)
//! - Lấy kho ảnh Nhật ký cá nhân (CDN zpsocial) & hợp nhất media chia sẻ thực tế
//! - Đóng gói binary RequestPacket chuẩn bytecode Zalo (`pw0.e`: cmd=1230/subcmd=6, `mw0.c`: cmd=2030/subcmd=0, `o41.g`: cmd=1234/2045)
//! - Nạp và hợp nhất bài viết Nhật ký thực tế + trạng thái like/comment thời gian thực từ SQLite (`timeline_feed_1`, `user_feed`, `table_feed_real_time_like_comment`)

use anyhow::{anyhow, Result};
use chrono::{FixedOffset, TimeZone};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedAuthor {
    pub uid: String,
    pub display_name: String,
    pub avatar_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedMedia {
    pub photo_id: String,
    pub media_type: String, // "image" | "video"
    pub url: String,
    pub thumbnail: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedReaction {
    pub like_count: u32,
    pub comment_count: u32,
    pub is_liked: bool,
    pub top_reactions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedComment {
    pub comment_id: String,
    pub author_name: String,
    pub author_uid: String,
    pub content: String,
    pub timestamp_str: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedItem {
    pub feed_id: String,
    pub author: FeedAuthor,
    pub date_str: String,
    pub content: String,
    pub status_badge: Option<String>,
    pub media_list: Vec<FeedMedia>,
    pub reaction: FeedReaction,
    pub comments: Vec<FeedComment>,
    #[serde(default)]
    pub timestamp_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialUserProfile {
    pub uid: String,
    pub username: String,
    pub display_name: String,
    pub zalo_name: String,
    pub avatar_url: String,
    pub cover_url: String,
    pub gender: i32,
    pub sdob: String,
    pub status: String,
    pub is_friend: bool,
    pub last_action_time: Option<u64>,
}

fn parse_u64_any(val: Option<&Value>) -> Option<u64> {
    let v = val?;
    v.as_u64()
        .or_else(|| v.as_i64().and_then(|n| u64::try_from(n).ok()))
        .or_else(|| v.as_str().and_then(|s| s.trim().parse::<u64>().ok()))
}

pub fn format_vn_timestamp_ms(ts_ms: i64) -> String {
    if ts_ms <= 0 {
        return String::new();
    }
    let normalized_ms = if ts_ms < 100_000_000_000 {
        ts_ms * 1000
    } else {
        ts_ms
    };
    let tz = FixedOffset::east_opt(7 * 3600).unwrap();
    tz.timestamp_millis_opt(normalized_ms)
        .single()
        .map(|dt| dt.format("%d/%m/%Y lúc %H:%M:%S").to_string())
        .unwrap_or_default()
}

pub fn normalize_social_media_url(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.ends_with(".jxl") && trimmed.len() > 4 {
        format!("{}.jpg", &trimmed[..trimmed.len() - 4])
    } else {
        trimmed.to_string()
    }
}

pub fn reaction_id_to_label(reaction_id: i64) -> &'static str {
    match reaction_id {
        1 => "Thích",
        2 => "Yêu thích",
        3 => "Haha",
        4 => "Wow",
        5 => "Buồn",
        6 => "Phẫn nộ",
        _ => "Thích",
    }
}

/// Normalize a Zalo UID by trimming whitespace and stripping any `_<version>` suffix.
pub(crate) fn clean_feed_uid(raw: &str) -> String {
    raw.split('_').next().unwrap_or(raw).trim().to_string()
}

/// Return `true` when `name` is empty, a raw numeric UIN placeholder, or matches `uid`.
pub(crate) fn is_placeholder_author_name(name: &str, uid: &str) -> bool {
    let t = name.trim();
    if t.is_empty() {
        return true;
    }
    let clean_uid = clean_feed_uid(uid);
    if !clean_uid.is_empty() && (t == clean_uid || clean_feed_uid(t) == clean_uid) {
        return true;
    }
    if t.len() >= 6 && t.chars().all(|c| c.is_ascii_digit()) {
        return true;
    }
    crate::chat::is_placeholder_thread_name(t, &clean_uid)
}

fn extract_json_id(val: &Value, keys: &[&str]) -> Option<String> {
    for k in keys {
        let Some(v) = val.get(*k) else {
            continue;
        };
        let candidate = match v {
            Value::String(s) => s.trim().to_string(),
            Value::Number(n) => {
                if let Some(u) = n.as_u64() {
                    u.to_string()
                } else if let Some(i) = n.as_i64() {
                    i.to_string()
                } else {
                    String::new()
                }
            }
            _ => String::new(),
        };
        if !candidate.is_empty() && candidate != "0" && candidate != "-1" {
            return Some(candidate);
        }
    }
    None
}

fn extract_json_text(val: &Value, keys: &[&str]) -> Option<String> {
    for k in keys {
        if let Some(s) = val.get(*k).and_then(Value::as_str) {
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
    }
    None
}

fn pick_best_display_name(val: &Value, keys: &[&str], uid: &str) -> String {
    let mut first_non_empty: Option<String> = None;
    for k in keys {
        let Some(s) = val.get(*k).and_then(Value::as_str) else {
            continue;
        };
        let trimmed = s.trim();
        if trimmed.is_empty() {
            continue;
        }
        if first_non_empty.is_none() {
            first_non_empty = Some(trimmed.to_string());
        }
        if !is_placeholder_author_name(trimmed, uid) {
            return trimmed.to_string();
        }
    }
    first_non_empty.unwrap_or_else(|| uid.to_string())
}

fn unwrap_data_value<'a>(dec: &'a Value, scratch: &'a mut Option<Value>) -> &'a Value {
    match dec.get("data") {
        Some(Value::String(s)) => {
            if let Ok(parsed) = serde_json::from_str::<Value>(s.trim()) {
                *scratch = Some(parsed);
                scratch.as_ref().unwrap_or(dec)
            } else {
                dec
            }
        }
        Some(v) => v,
        None => dec,
    }
}

fn merge_profile_into(dst: &mut SocialUserProfile, src: SocialUserProfile) {
    if is_placeholder_author_name(&dst.display_name, &dst.uid)
        && !is_placeholder_author_name(&src.display_name, &src.uid)
    {
        dst.display_name = src.display_name;
    }
    if is_placeholder_author_name(&dst.zalo_name, &dst.uid)
        && !is_placeholder_author_name(&src.zalo_name, &src.uid)
    {
        dst.zalo_name = src.zalo_name;
    }
    if dst.username.is_empty() && !src.username.is_empty() {
        dst.username = src.username;
    }
    if (dst.avatar_url.trim().is_empty() || dst.avatar_url.ends_with("/default"))
        && !src.avatar_url.trim().is_empty()
        && !src.avatar_url.ends_with("/default")
    {
        dst.avatar_url = src.avatar_url;
    }
    if (dst.cover_url.trim().is_empty() || dst.cover_url.ends_with("/default"))
        && !src.cover_url.trim().is_empty()
        && !src.cover_url.ends_with("/default")
    {
        dst.cover_url = src.cover_url;
    }
    if dst.sdob.is_empty() && !src.sdob.is_empty() {
        dst.sdob = src.sdob;
    }
    if dst.status.is_empty() && !src.status.is_empty() {
        dst.status = src.status;
    }
    if dst.last_action_time.is_none() {
        dst.last_action_time = src.last_action_time;
    }
}

/// Merge resolved profile display name and avatar onto a feed item without crossing distinct UIDs.
pub(crate) fn enrich_feed_item_author_from_profiles(
    item: &mut FeedItem,
    profiles: &[SocialUserProfile],
) {
    let item_uid = clean_feed_uid(&item.author.uid);
    let matched = if !item_uid.is_empty() {
        profiles.iter().find(|p| clean_feed_uid(&p.uid) == item_uid)
    } else if !item.author.display_name.trim().is_empty() {
        profiles.iter().find(|p| {
            p.display_name
                .trim()
                .eq_ignore_ascii_case(item.author.display_name.trim())
                || p.zalo_name
                    .trim()
                    .eq_ignore_ascii_case(item.author.display_name.trim())
        })
    } else {
        None
    };

    if let Some(prof) = matched {
        let prof_uid = clean_feed_uid(&prof.uid);
        if item.author.uid.trim().is_empty() && !prof_uid.is_empty() {
            item.author.uid = prof_uid.clone();
        } else if !item_uid.is_empty() {
            item.author.uid = item_uid;
        }
        let target_uid = &item.author.uid;
        if is_placeholder_author_name(&item.author.display_name, target_uid) {
            if !is_placeholder_author_name(&prof.display_name, &prof.uid) {
                item.author.display_name = prof.display_name.clone();
            } else if !is_placeholder_author_name(&prof.zalo_name, &prof.uid) {
                item.author.display_name = prof.zalo_name.clone();
            }
        }
        if (item.author.avatar_url.trim().is_empty() || item.author.avatar_url.ends_with("/default"))
            && !prof.avatar_url.trim().is_empty()
            && !prof.avatar_url.ends_with("/default")
        {
            item.author.avatar_url = prof.avatar_url.clone();
        }
    }
}

impl SocialUserProfile {
    pub fn from_json(uid: &str, val: &Value) -> Option<Self> {
        let obj = val.as_object()?;
        let fallback_uid = clean_feed_uid(uid);
        let resolved_uid = extract_json_id(
            val,
            &["userId", "uid", "id", "ownerId", "owner_id", "creatorId"],
        )
        .map(|s| clean_feed_uid(&s))
        .filter(|s| !s.is_empty())
        .unwrap_or(fallback_uid);
        let display_name = pick_best_display_name(
            val,
            &[
                "displayName",
                "dpn",
                "zaloName",
                "display_name",
                "name",
                "alias",
                "username",
            ],
            &resolved_uid,
        );
        let zalo_name = pick_best_display_name(
            val,
            &["zaloName", "displayName", "dpn", "name", "username"],
            &resolved_uid,
        );
        let zalo_name = if is_placeholder_author_name(&zalo_name, &resolved_uid)
            && !is_placeholder_author_name(&display_name, &resolved_uid)
        {
            display_name.clone()
        } else {
            zalo_name
        };
        let username = extract_json_text(val, &["username"]).unwrap_or_default();
        let avatar_url = extract_json_text(
            val,
            &["avatar", "avt", "avatarUrl", "fullAvt", "profile_avatar"],
        )
        .map(|u| normalize_social_media_url(&u))
        .unwrap_or_default();
        let cover_url = obj
            .get("cover")
            .or_else(|| obj.get("bgavatar"))
            .or_else(|| obj.get("coverUrl"))
            .and_then(Value::as_str)
            .or_else(|| {
                obj.get("cover")
                    .and_then(|c| c.get("coverUrl").or_else(|| c.get("url")))
                    .and_then(Value::as_str)
            })
            .map(normalize_social_media_url)
            .unwrap_or_default();
        let gender = obj.get("gender").and_then(Value::as_i64).unwrap_or(0) as i32;
        let sdob = obj
            .get("sdob")
            .or_else(|| obj.get("dob"))
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let status = obj
            .get("status")
            .or_else(|| obj.get("statusMsg"))
            .or_else(|| obj.get("bio"))
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let is_friend = obj
            .get("isFr")
            .and_then(Value::as_i64)
            .map(|v| v == 1)
            .unwrap_or(true);
        let last_action_time = parse_u64_any(obj.get("lastActionTime"))
            .or_else(|| parse_u64_any(obj.get("lastUpdateTime")))
            .or_else(|| parse_u64_any(obj.get("lastUpdate")));

        Some(Self {
            uid: resolved_uid,
            username,
            display_name,
            zalo_name,
            avatar_url,
            cover_url,
            gender,
            sdob,
            status,
            is_friend,
            last_action_time,
        })
    }
}

/// Parse `/api/social/profile/me-v2` JSON payloads across `data.profile`, `data.info`, and flat variants.
pub(crate) fn parse_my_social_profile_response(
    my_uid: &str,
    dec: &Value,
) -> Option<SocialUserProfile> {
    let mut scratch = None;
    let data = unwrap_data_value(dec, &mut scratch);
    let versioned = format!("{}_0", clean_feed_uid(my_uid));
    let target_obj = data
        .get("profile")
        .or_else(|| data.get("info"))
        .or_else(|| data.get("user"))
        .or_else(|| data.get(my_uid))
        .or_else(|| data.get(&versioned))
        .unwrap_or(data);
    SocialUserProfile::from_json(my_uid, target_obj)
}

/// Parse `/api/social/friend/getprofiles` JSON payloads across `data.<uid>`, `changed_profiles`, and flat maps.
pub(crate) fn parse_friend_social_profile_response(
    target_uid: &str,
    dec: &Value,
) -> Option<SocialUserProfile> {
    let mut scratch = None;
    let data = unwrap_data_value(dec, &mut scratch);
    let clean = clean_feed_uid(target_uid);
    let versioned = format!("{clean}_0");
    let prof_obj = data
        .get(target_uid)
        .or_else(|| data.get(&clean))
        .or_else(|| data.get(&versioned))
        .or_else(|| {
            data.get("changed_profiles").and_then(|m| {
                m.get(target_uid)
                    .or_else(|| m.get(&clean))
                    .or_else(|| m.get(&versioned))
            })
        })
        .or_else(|| {
            data.get("unchanged_profiles").and_then(|m| {
                m.get(target_uid)
                    .or_else(|| m.get(&clean))
                    .or_else(|| m.get(&versioned))
            })
        })
        .or_else(|| {
            data.get("profiles").and_then(|m| {
                m.get(target_uid)
                    .or_else(|| m.get(&clean))
                    .or_else(|| m.get(&versioned))
            })
        })
        .unwrap_or(data);
    SocialUserProfile::from_json(&clean, prof_obj)
}

/// Parse `/api/social/friend/getprofiles/v2` across `changed_profiles`, `unchanged_profiles`, and `profiles`.
pub(crate) fn parse_social_profiles_batch_response(dec: &Value) -> Vec<SocialUserProfile> {
    let mut scratch = None;
    let data = unwrap_data_value(dec, &mut scratch);
    let root = data.get("data").filter(|v| v.is_object()).unwrap_or(data);

    let mut maps: Vec<&serde_json::Map<String, Value>> = Vec::new();
    for key in ["changed_profiles", "unchanged_profiles", "profiles"] {
        if let Some(m) = root.get(key).and_then(Value::as_object) {
            maps.push(m);
        }
    }
    if maps.is_empty() {
        if let Some(m) = root.as_object() {
            maps.push(m);
        }
    }

    let mut all_profiles: Vec<SocialUserProfile> = Vec::new();
    let mut idx_by_uid: HashMap<String, usize> = HashMap::new();
    for map in maps {
        for (raw_uid, pval) in map {
            let clean_uid = clean_feed_uid(raw_uid);
            if clean_uid.is_empty() || clean_uid == "0" {
                continue;
            }
            if let Some(p) = SocialUserProfile::from_json(&clean_uid, pval) {
                if p.uid.is_empty() || p.uid == "0" {
                    continue;
                }
                if let Some(&existing_idx) = idx_by_uid.get(&p.uid) {
                    merge_profile_into(&mut all_profiles[existing_idx], p);
                } else {
                    idx_by_uid.insert(p.uid.clone(), all_profiles.len());
                    all_profiles.push(p);
                }
            }
        }
    }
    all_profiles
}

/// Đóng gói binary `RequestPacket` chuẩn giao thức Zalo Social Feed (reverse từ `pw0.e`, `mw0.c`, `o41.g`, `nu0.v`)
pub mod packet {
    /// `pw0.e.d`: Get Timeline (`cmd = 1230` / `0x04CE`, `subcmd = 6`, `cmd_type = 3`)
    pub const CMD_FEED_TIMELINE: u16 = 1230;
    pub const SUBCMD_GET_FEED: u8 = 6;

    /// `mw0.c.k`: Get Friend User Timeline (`cmd = 2030` / `0x07EE`, `subcmd = 0`, `cmd_type = 3`)
    pub const CMD_USER_FEED: u16 = 2030;
    pub const SUBCMD_USER_FEED: u8 = 0;

    /// `o41.g`: Post Feed (`cmd = 1234` / `0x04D2`, `subcmd = 8`) & Edit Feed (`cmd = 2045` / `0x07FD`, `subcmd = 0`)
    pub const CMD_POST_FEED: u16 = 1234;
    pub const SUBCMD_POST_FEED: u8 = 8;
    pub const CMD_EDIT_FEED: u16 = 2045;
    pub const SUBCMD_EDIT_FEED: u8 = 0;

    /// Mã lỗi client trả về khi mất mạng (`!b2.M(false)`) trong `pw0.e`, `mw0.c`, `o41.g`
    pub const ERR_FEED_NO_NETWORK: i32 = 50001;

    /// Phiên bản client Zalo (`AbstractC27691d.f126612a` trong APK 26.10.02)
    pub const CLIENT_VERSION: i32 = 26_10_02_903;
    pub const CMD_TYPE_REQUEST: u8 = 3;

    /// Tạo payload nhị phân chuẩn `pw0.e.d` (52 bytes):
    /// `u8(1) + i32(client_version) + i32(lang_platform) + u8(tab) + i32(page) + i32(count)`
    /// `+ i64(last_feed_id_server) + i64(last_feed_id_local) + i64(max_feed_id) + i64(feed_version) + i16(source_enum)`
    pub fn build_timeline_payload_with_cursors(
        tab: u8,
        page: i32,
        count: i32,
        last_feed_id_server: i64,
        last_feed_id_local: i64,
        max_feed_id: i64,
        feed_version: i64,
        source_enum: i16,
    ) -> Vec<u8> {
        let mut buf = Vec::with_capacity(52);
        buf.push(0x01);
        buf.extend_from_slice(&CLIENT_VERSION.to_be_bytes());
        buf.extend_from_slice(&1i32.to_be_bytes()); // AbstractC21220r.l() = 1 (vi/android)
        buf.push(tab);
        buf.extend_from_slice(&page.to_be_bytes());
        buf.extend_from_slice(&count.to_be_bytes());
        buf.extend_from_slice(&last_feed_id_server.to_be_bytes());
        buf.extend_from_slice(&last_feed_id_local.to_be_bytes());
        buf.extend_from_slice(&max_feed_id.to_be_bytes());
        buf.extend_from_slice(&feed_version.to_be_bytes());
        buf.extend_from_slice(&source_enum.to_be_bytes());
        buf
    }

    /// Tạo payload nhị phân chuẩn `mw0.c.k` (18 bytes) khi xem dòng thời gian của 1 người dùng cụ thể (`cmd = 2030`)
    pub fn build_user_timeline_payload(owner_id: u32, flag: u8) -> Vec<u8> {
        let mut buf = Vec::with_capacity(18);
        buf.extend_from_slice(&0u16.to_be_bytes());
        buf.push(0x01);
        buf.extend_from_slice(&CLIENT_VERSION.to_be_bytes());
        buf.extend_from_slice(&0u16.to_be_bytes());
        buf.extend_from_slice(&(owner_id as i32).to_be_bytes());
        buf.extend_from_slice(&1i32.to_be_bytes());
        buf.push(flag);
        buf
    }

    /// Tạo payload cho lệnh GET_FEED (tương thích chữ ký caller `modals.rs`, xuất đúng 49 bytes theo `pw0.e.d`)
    pub fn build_get_feed_payload(
        owner_id: u64,
        viewer_id: u64,
        tab: u32,
        page: u32,
        count: u32,
    ) -> Vec<u8> {
        let _ = viewer_id;
        build_timeline_payload_with_cursors(
            tab as u8,
            page as i32,
            count as i32,
            0,
            0,
            owner_id as i64,
            2, // C26856y.y() feed_version = 2
            0,
        )
    }

    /// Đóng gói payload vào khung `RequestPacket` chuẩn (`nu0.v`):
    /// `[total_len: u32 BE][version=1: u8][encrypt=0: u8][uin: u32 BE][cmd_type=3: u8][cmd: u16 BE][subcmd: u8][seq_id: u16 BE][params_len: u32 BE][payload]`
    pub fn build_request_packet_with_uin(
        uin: u32,
        cmd: u16,
        subcmd: u8,
        seq_id: u16,
        payload: &[u8],
    ) -> Vec<u8> {
        let header_len = 1 + 1 + 4 + 1 + 2 + 1 + 2 + 4; // 16 bytes
        let body_len = header_len + payload.len();
        let mut packet = Vec::with_capacity(4 + body_len);
        packet.extend_from_slice(&(body_len as u32).to_be_bytes());
        packet.push(0x01); // version = 1 (n(1))
        packet.push(0x00); // encrypt = 0 (g(0))
        packet.extend_from_slice(&uin.to_be_bytes()); // m(uin)
        packet.push(CMD_TYPE_REQUEST); // f((byte) 3)
        packet.extend_from_slice(&cmd.to_be_bytes()); // d((short) cmd)
        packet.push(subcmd); // l((byte) subcmd)
        packet.extend_from_slice(&seq_id.to_be_bytes()); // k(seqId)
        packet.extend_from_slice(&(payload.len() as u32).to_be_bytes()); // i(payload.len)
        packet.extend_from_slice(payload);
        packet
    }

    /// Đóng gói `RequestPacket` mặc định (tương thích chữ ký caller `modals.rs`)
    pub fn build_request_packet(cmd: u16, subcmd: u8, payload: &[u8]) -> Vec<u8> {
        build_request_packet_with_uin(0, cmd, subcmd, 1, payload)
    }
}

/// Mở rộng các phương thức mạng xã hội cho Session
impl crate::session::Session {
    /// Lấy hồ sơ mạng xã hội của chính mình (`/api/social/profile/me-v2` hoặc fallback `getprofiles`)
    pub async fn get_my_social_profile(&self) -> Result<SocialUserProfile> {
        let my_uid = self
            .login
            .as_ref()
            .map(|l| l.uin.clone())
            .unwrap_or_default();
        if my_uid.is_empty() {
            return Err(anyhow!("Missing logged-in UIN"));
        }

        let body = json!({
            "avatar_size": 240,
            "imei": self.imei,
        });
        if let Ok(enc) = self.encoded_params(&body) {
            let url = format!(
                "{}/api/social/profile/me-v2?{}&params={enc}&os=8",
                self.domains.profile,
                self.common_params()
            );
            if let Ok(resp) = self.get(&url).await {
                let dec = self.maybe_decrypt(resp, self.session_key_bytes()).await;
                if dec.get("error_code").and_then(Value::as_i64) == Some(0) {
                    if let Some(p) = parse_my_social_profile_response(&my_uid, &dec) {
                        return Ok(p);
                    }
                }
            }
        }

        self.get_friend_social_profile(&my_uid).await
    }

    /// Lấy hồ sơ chi tiết bạn bè (Ảnh bìa Cover HD, Avatar, Ngày sinh, Giới tính, Bio)
    pub async fn get_friend_social_profile(&self, target_uid: &str) -> Result<SocialUserProfile> {
        let clean_target = clean_feed_uid(target_uid);
        let body = json!({
            "avatar_size": 240,
            "friend_ids": [&clean_target],
            "imei": self.imei,
        });

        let enc = self.encoded_params(&body)?;
        let url = format!(
            "{}/api/social/friend/getprofiles?{}&params={enc}&os=8",
            self.domains.profile,
            self.common_params()
        );

        let resp = self.get(&url).await?;
        let dec = self.maybe_decrypt(resp, self.session_key_bytes()).await;

        if dec.get("error_code").and_then(Value::as_i64) == Some(0) {
            if let Some(p) = parse_friend_social_profile_response(&clean_target, &dec) {
                return Ok(p);
            }
        }

        // Fallback sang batch v2 cho 1 UID nếu endpoint v1 trả lỗi
        if let Ok(mut list) = self
            .get_friend_social_profiles_batch(&[clean_target.clone()])
            .await
        {
            if let Some(p) = list.pop() {
                return Ok(p);
            }
        }

        Err(anyhow!(
            "Server returned error or missing data for uid {target_uid}: {dec}"
        ))
    }

    /// Lấy kho ảnh Nhật ký cá nhân (CDN zpsocial)
    pub async fn get_my_social_avatar_list(&self, page: u32, count: u32) -> Result<Vec<FeedMedia>> {
        let body = json!({
            "page": page,
            "count": count,
            "imei": self.imei,
        });

        let enc = self.encoded_params(&body)?;
        let url = format!(
            "{}/api/social/avatar-list?{}&params={enc}&os=8",
            self.domains.profile,
            self.common_params()
        );

        let resp = self.get(&url).await?;
        let dec = self.maybe_decrypt(resp, self.session_key_bytes()).await;

        let mut photos = Vec::new();
        if let Some(photo_arr) = dec
            .get("data")
            .and_then(|d| d.get("photos"))
            .and_then(Value::as_array)
        {
            for p in photo_arr {
                let photo_id = p
                    .get("photoId")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string();
                let raw_url = p.get("url").and_then(Value::as_str).unwrap_or("");
                let url = normalize_social_media_url(raw_url);
                let thumbnail = p
                    .get("thumbnail")
                    .and_then(Value::as_str)
                    .map(normalize_social_media_url)
                    .filter(|s| !s.is_empty())
                    .unwrap_or_else(|| url.clone());
                let width = p.get("width").and_then(Value::as_u64).unwrap_or(0) as u32;
                let height = p.get("height").and_then(Value::as_u64).unwrap_or(0) as u32;
                if !url.is_empty() {
                    photos.push(FeedMedia {
                        photo_id,
                        media_type: "image".to_string(),
                        url,
                        thumbnail,
                        width,
                        height,
                    });
                }
            }
        }

        Ok(photos)
    }

    /// Lấy hàng loạt hồ sơ mạng xã hội của nhiều bạn bè (`POST /api/social/friend/getprofiles/v2`)
    pub async fn get_friend_social_profiles_batch(
        &self,
        target_uids: &[String],
    ) -> Result<Vec<SocialUserProfile>> {
        if target_uids.is_empty() {
            return Ok(Vec::new());
        }
        let mut all_profiles: Vec<SocialUserProfile> = Vec::new();
        let mut idx_by_uid: HashMap<String, usize> = HashMap::new();

        for chunk in target_uids.chunks(40) {
            let pversion_map: Vec<String> = chunk
                .iter()
                .map(|u| clean_feed_uid(u))
                .filter(|u| !u.is_empty() && u != "0")
                .map(|u| format!("{u}_0"))
                .collect();
            if pversion_map.is_empty() {
                continue;
            }
            let body = json!({
                "phonebook_version": 0,
                "friend_pversion_map": pversion_map,
                "avatar_size": 240,
                "language": "vi",
                "show_online_status": 1,
                "imei": self.imei,
            });
            let enc = self.encoded_params(&body)?;
            let url = format!(
                "{}/api/social/friend/getprofiles/v2?{}",
                self.domains.profile,
                self.common_params()
            );
            let form = format!("params={enc}");
            let resp = self.post_form(&url, form).await?;
            let dec = self.maybe_decrypt(resp, self.session_key_bytes()).await;

            for p in parse_social_profiles_batch_response(&dec) {
                if let Some(&existing_idx) = idx_by_uid.get(&p.uid) {
                    merge_profile_into(&mut all_profiles[existing_idx], p);
                } else {
                    idx_by_uid.insert(p.uid.clone(), all_profiles.len());
                    all_profiles.push(p);
                }
            }
        }

        Ok(all_profiles)
    }

    /// Tổng hợp Dòng thời gian (Nhật ký) thời gian thực từ:
    /// 1. Hồ sơ cá nhân & kho ảnh đại diện/nhật ký (`/api/social/avatar-list`, `/api/social/profile/me-v2`)
    /// 2. Hồ sơ mạng xã hội thời gian thực của toàn bộ bạn bè (`/api/social/friend/getprofiles/v2`)
    /// 3. Cơ sở dữ liệu Nhật ký thực tế (`timeline_feed_1`, `user_feed`, `table_feed_real_time_like_comment`)
    /// 4. Media thực tế chia sẻ trong phiên (`shared_media_by_uid`) và cập nhật trạng thái/ảnh bìa thực tế của bạn bè
    pub async fn fetch_timeline_feed(
        &self,
        friend_uids: &[String],
        shared_media_by_uid: &HashMap<String, Vec<FeedMedia>>,
    ) -> Result<(Vec<SocialUserProfile>, Vec<FeedItem>)> {
        let mut items: Vec<FeedItem> = Vec::new();
        let mut seen_feed_ids: HashSet<String> = HashSet::new();
        let now_ms = chrono::Utc::now().timestamp_millis();

        let my_uid = self
            .login
            .as_ref()
            .map(|l| clean_feed_uid(&l.uin))
            .unwrap_or_default();
        let mut my_name = self
            .login
            .as_ref()
            .map(|l| l.display_name.clone())
            .unwrap_or_else(|| "Tôi".to_string());
        let mut my_ava = self
            .login
            .as_ref()
            .map(|l| l.avatar.clone())
            .unwrap_or_default();

        let my_profile_opt = self.get_my_social_profile().await.ok();
        if let Some(ref mp) = my_profile_opt {
            if !is_placeholder_author_name(&mp.display_name, &my_uid)
                || is_placeholder_author_name(&my_name, &my_uid)
            {
                if !mp.display_name.trim().is_empty() {
                    my_name = mp.display_name.clone();
                }
            }
            if !mp.avatar_url.is_empty() {
                my_ava = mp.avatar_url.clone();
            }
        }

        if let Ok(my_photos) = self.get_my_social_avatar_list(1, 20).await {
            if !my_photos.is_empty() {
                let my_ts_ms = my_profile_opt
                    .as_ref()
                    .and_then(|p| p.last_action_time)
                    .map(|t| if t < 100_000_000_000 { (t * 1000) as i64 } else { t as i64 })
                    .unwrap_or(now_ms);
                let feed_id = format!("my_album_{my_uid}");
                seen_feed_ids.insert(feed_id.clone());
                items.push(FeedItem {
                    feed_id,
                    author: FeedAuthor {
                        uid: my_uid.clone(),
                        display_name: my_name.clone(),
                        avatar_url: my_ava.clone(),
                    },
                    date_str: if my_ts_ms > 0 {
                        format_vn_timestamp_ms(my_ts_ms)
                    } else {
                        "Kho ảnh cá nhân".to_string()
                    },
                    content: my_profile_opt
                        .as_ref()
                        .filter(|p| !p.status.trim().is_empty())
                        .map(|p| p.status.clone())
                        .unwrap_or_else(|| {
                            format!("Album ảnh đại diện mạng xã hội ({} ảnh)", my_photos.len())
                        }),
                    status_badge: Some("Album ảnh".to_string()),
                    media_list: my_photos,
                    reaction: FeedReaction {
                        like_count: 0,
                        comment_count: 0,
                        is_liked: false,
                        top_reactions: Vec::new(),
                    },
                    comments: Vec::new(),
                    timestamp_ms: my_ts_ms,
                });
            }
        }

        let mut ordered_uids: Vec<String> = Vec::new();
        let mut uid_set: HashSet<String> = HashSet::new();
        let mut priority_uids: Vec<String> = Vec::new();
        for u in shared_media_by_uid.keys() {
            let trimmed = clean_feed_uid(u);
            if !trimmed.is_empty()
                && trimmed != "0"
                && trimmed != my_uid
                && !trimmed.starts_with("group_")
                && uid_set.insert(trimmed.clone())
            {
                ordered_uids.push(trimmed.clone());
                priority_uids.push(trimmed);
            }
        }
        for u in friend_uids {
            let trimmed = clean_feed_uid(u);
            if !trimmed.is_empty()
                && trimmed != "0"
                && trimmed != my_uid
                && !trimmed.starts_with("group_")
                && uid_set.insert(trimmed.clone())
            {
                ordered_uids.push(trimmed);
            }
        }

        let batch_ids: Vec<String> = ordered_uids.into_iter().take(400).collect();
        let mut profiles = self
            .get_friend_social_profiles_batch(&batch_ids)
            .await
            .unwrap_or_default();

        let mut missing_or_incomplete: Vec<String> = Vec::new();
        let mut missing_seen: HashSet<String> = HashSet::new();
        let is_profile_incomplete = |p: &SocialUserProfile| {
            is_placeholder_author_name(&p.display_name, &p.uid) || p.avatar_url.trim().is_empty()
        };
        for uid in &priority_uids {
            let needs_lookup = profiles
                .iter()
                .find(|p| clean_feed_uid(&p.uid) == *uid)
                .map(is_profile_incomplete)
                .unwrap_or(true);
            if needs_lookup && missing_seen.insert(uid.clone()) {
                missing_or_incomplete.push(uid.clone());
            }
        }
        for uid in batch_ids.iter().take(60) {
            if !profiles.iter().any(|p| clean_feed_uid(&p.uid) == *uid)
                && missing_seen.insert(uid.clone())
            {
                missing_or_incomplete.push(uid.clone());
            }
        }

        if !missing_or_incomplete.is_empty() {
            for chunk in missing_or_incomplete.chunks(50) {
                if let Ok(mini_map) = crate::chat::get_mini_profiles(self, chunk).await {
                    for (raw_k, pval) in &mini_map {
                        upsert_mini_profile_into_profiles(&mut profiles, raw_k, pval);
                    }
                }
            }
            for uid in priority_uids.iter().take(15) {
                let still_missing = profiles
                    .iter()
                    .find(|p| clean_feed_uid(&p.uid) == *uid)
                    .map(is_profile_incomplete)
                    .unwrap_or(true);
                if still_missing {
                    if let Ok(p) = self.get_friend_social_profile(uid).await {
                        if let Some(existing) = profiles
                            .iter_mut()
                            .find(|ep| clean_feed_uid(&ep.uid) == clean_feed_uid(&p.uid))
                        {
                            merge_profile_into(existing, p);
                        } else {
                            profiles.push(p);
                        }
                    }
                }
            }
        }

        if let Some(mp) = my_profile_opt {
            if !profiles
                .iter()
                .any(|p| clean_feed_uid(&p.uid) == clean_feed_uid(&mp.uid))
            {
                profiles.push(mp);
            }
        }

        profiles.sort_by(|a, b| {
            b.last_action_time
                .unwrap_or(0)
                .cmp(&a.last_action_time.unwrap_or(0))
        });

        let mut db_candidates = Vec::new();
        if let Ok(env_db) = std::env::var("ZALO_TIMELINE_DB") {
            if !env_db.trim().is_empty() {
                db_candidates.push(env_db);
            }
        }
        db_candidates.push("zalo_history.db".to_string());
        db_candidates.push("databases/zalo".to_string());

        for db_path in db_candidates {
            if std::path::Path::new(&db_path).exists() {
                if let Ok(real_feeds) = load_real_timeline_from_sqlite(&db_path, "") {
                    for mut rf in real_feeds {
                        if !seen_feed_ids.insert(rf.feed_id.clone()) {
                            continue;
                        }
                        enrich_feed_item_author_from_profiles(&mut rf, &profiles);
                        items.push(rf);
                    }
                }
            }
        }

        let profile_map: HashMap<String, &SocialUserProfile> = profiles
            .iter()
            .map(|p| (clean_feed_uid(&p.uid), p))
            .collect();

        for (uid, media_vec) in shared_media_by_uid {
            if media_vec.is_empty() {
                continue;
            }
            let clean_uid = clean_feed_uid(uid);
            if clean_uid.is_empty() || clean_uid == my_uid {
                continue;
            }
            let feed_id = format!("shared_media_{clean_uid}");
            if !seen_feed_ids.insert(feed_id.clone()) {
                continue;
            }
            let (display_name, avatar_url, status_text, ts_ms) =
                if let Some(prof) = profile_map.get(&clean_uid) {
                    let ts = prof
                        .last_action_time
                        .map(|t| if t < 100_000_000_000 { (t * 1000) as i64 } else { t as i64 })
                        .unwrap_or(now_ms - 1_000);
                    let best_name = if !is_placeholder_author_name(&prof.display_name, &clean_uid) {
                        prof.display_name.clone()
                    } else if !is_placeholder_author_name(&prof.zalo_name, &clean_uid) {
                        prof.zalo_name.clone()
                    } else {
                        prof.display_name.clone()
                    };
                    (best_name, prof.avatar_url.clone(), prof.status.clone(), ts)
                } else {
                    (
                        clean_uid.clone(),
                        String::new(),
                        String::new(),
                        now_ms - 1_000,
                    )
                };

            let date_str = format_vn_timestamp_ms(ts_ms);
            let content = if !status_text.trim().is_empty() {
                status_text
            } else {
                format!("Đã chia sẻ {} ảnh/video", media_vec.len())
            };

            items.push(FeedItem {
                feed_id,
                author: FeedAuthor {
                    uid: clean_uid,
                    display_name,
                    avatar_url,
                },
                date_str,
                content,
                status_badge: Some("Ảnh chia sẻ".to_string()),
                media_list: media_vec.clone(),
                reaction: FeedReaction {
                    like_count: 0,
                    comment_count: 0,
                    is_liked: false,
                    top_reactions: Vec::new(),
                },
                comments: Vec::new(),
                timestamp_ms: ts_ms,
            });
        }

        // Với bạn bè có trạng thái (status) hoặc ảnh bìa (cover_url) thực tế từ `/api/social/friend/getprofiles/v2`
        // mà chưa có bài đăng nào trong danh sách, hiển thị thẻ hoạt động hồ sơ thực tế của họ
        for prof in &profiles {
            if prof.uid == my_uid {
                continue;
            }
            let has_status = !prof.status.trim().is_empty();
            let has_cover = !prof.cover_url.trim().is_empty();
            if !has_status && !has_cover {
                continue;
            }
            if items.iter().any(|it| it.author.uid == prof.uid) {
                continue;
            }

            let feed_id = format!("profile_activity_{}", prof.uid);
            if !seen_feed_ids.insert(feed_id.clone()) {
                continue;
            }

            let ts_ms = prof
                .last_action_time
                .map(|t| if t < 100_000_000_000 { (t * 1000) as i64 } else { t as i64 })
                .unwrap_or(0);
            let date_str = if ts_ms > 0 {
                format_vn_timestamp_ms(ts_ms)
            } else {
                "Hoạt động gần đây".to_string()
            };

            let mut media_list = Vec::new();
            if has_cover {
                let norm_cover = normalize_social_media_url(&prof.cover_url);
                media_list.push(FeedMedia {
                    photo_id: format!("cover_{}", prof.uid),
                    media_type: "image".to_string(),
                    url: norm_cover.clone(),
                    thumbnail: norm_cover,
                    width: 0,
                    height: 0,
                });
            }

            let badge = if has_status && has_cover {
                Some("Tiểu sử & Ảnh bìa".to_string())
            } else if has_status {
                Some("Trạng thái".to_string())
            } else {
                Some("Ảnh bìa".to_string())
            };

            items.push(FeedItem {
                feed_id,
                author: FeedAuthor {
                    uid: prof.uid.clone(),
                    display_name: prof.display_name.clone(),
                    avatar_url: prof.avatar_url.clone(),
                },
                date_str,
                content: prof.status.clone(),
                status_badge: badge,
                media_list,
                reaction: FeedReaction {
                    like_count: 0,
                    comment_count: 0,
                    is_liked: false,
                    top_reactions: Vec::new(),
                },
                comments: Vec::new(),
                timestamp_ms: ts_ms,
            });
        }

        // Sắp xếp toàn bộ dòng thời gian theo timestamp thực tế mới nhất lên đầu
        items.sort_by(|a, b| {
            b.timestamp_ms
                .cmp(&a.timestamp_ms)
                .then_with(|| b.feed_id.cmp(&a.feed_id))
        });

        Ok((profiles, items))
    }
}

#[derive(Debug, Clone, Default)]
struct RealtimeFeedStat {
    like_count: Option<u32>,
    comment_count: Option<u32>,
    is_liked: Option<bool>,
}

fn load_realtime_like_comment_map(conn: &rusqlite::Connection) -> HashMap<String, RealtimeFeedStat> {
    let mut map = HashMap::new();
    let exists: bool = conn
        .query_row(
            "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='table_feed_real_time_like_comment'",
            [],
            |row| row.get(0),
        )
        .map(|count: i64| count > 0)
        .unwrap_or(false);

    if !exists {
        return map;
    }

    if let Ok(mut stmt) = conn.prepare(
        "SELECT fid, tot, is_like, tot_cmt FROM table_feed_real_time_like_comment",
    ) {
        if let Ok(rows) = stmt.query_map([], |row| {
            let fid: String = row.get(0)?;
            let tot: i64 = row.get(1).unwrap_or(-1);
            let is_like: i64 = row.get(2).unwrap_or(-1);
            let tot_cmt: i64 = row.get(3).unwrap_or(-1);
            Ok((fid, tot, is_like, tot_cmt))
        }) {
            for r in rows.flatten() {
                let (fid, tot, is_like, tot_cmt) = r;
                if fid.is_empty() {
                    continue;
                }
                map.insert(
                    fid,
                    RealtimeFeedStat {
                        like_count: if tot >= 0 { Some(tot as u32) } else { None },
                        comment_count: if tot_cmt >= 0 {
                            Some(tot_cmt as u32)
                        } else {
                            None
                        },
                        is_liked: if is_like >= 0 {
                            Some(is_like > 0)
                        } else {
                            None
                        },
                    },
                );
            }
        }
    }

    map
}

fn extract_media_from_photo_val(photo: &Value, default_dim: Option<&Value>) -> Option<FeedMedia> {
    let photoid = photo
        .get("photoid")
        .or_else(|| photo.get("photoId"))
        .or_else(|| photo.get("id"))
        .and_then(|v| {
            v.as_str()
                .map(|s| s.to_string())
                .or_else(|| v.as_i64().map(|n| n.to_string()))
        })
        .unwrap_or_default();

    let raw_origin = photo
        .get("origin")
        .or_else(|| photo.get("url"))
        .or_else(|| photo.get("hd"))
        .or_else(|| photo.get("large"))
        .or_else(|| photo.get("thumb"))
        .and_then(Value::as_str)
        .unwrap_or("");

    let raw_video = photo
        .get("videourl")
        .or_else(|| photo.get("videoUrl"))
        .or_else(|| photo.get("streamingUrl"))
        .and_then(Value::as_str)
        .unwrap_or("");

    let (media_type, primary_url) = if !raw_video.is_empty() {
        ("video".to_string(), normalize_social_media_url(raw_video))
    } else if !raw_origin.is_empty() {
        ("image".to_string(), normalize_social_media_url(raw_origin))
    } else {
        return None;
    };

    let thumbnail = photo
        .get("thumb")
        .or_else(|| photo.get("thumbnail"))
        .and_then(Value::as_str)
        .map(normalize_social_media_url)
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| primary_url.clone());

    let dim = photo.get("photo_dimension").or(default_dim);
    let width = dim
        .and_then(|d| d.get("width"))
        .or_else(|| photo.get("width"))
        .and_then(Value::as_u64)
        .unwrap_or(0) as u32;
    let height = dim
        .and_then(|d| d.get("height"))
        .or_else(|| photo.get("height"))
        .and_then(Value::as_u64)
        .unwrap_or(0) as u32;

    Some(FeedMedia {
        photo_id: photoid,
        media_type,
        url: primary_url,
        thumbnail,
        width,
        height,
    })
}

/// Upsert a profile object from `/api/social/group/members` (`get_mini_profiles`) into `profiles`.
pub(crate) fn upsert_mini_profile_into_profiles(
    profiles: &mut Vec<SocialUserProfile>,
    raw_k: &str,
    pval: &Value,
) {
    let clean_k = clean_feed_uid(raw_k);
    if clean_k.is_empty() || clean_k == "0" {
        return;
    }
    if let Some(p) = SocialUserProfile::from_json(&clean_k, pval) {
        if let Some(existing) = profiles
            .iter_mut()
            .find(|ep| clean_feed_uid(&ep.uid) == clean_feed_uid(&p.uid))
        {
            merge_profile_into(existing, p);
        } else {
            profiles.push(p);
        }
    }
}

fn extract_feed_author(item: &Value) -> Option<FeedAuthor> {
    let header = item.get("header");
    let nested_owner = item
        .get("owner")
        .or_else(|| item.get("author"))
        .or_else(|| item.get("creator"))
        .or_else(|| item.get("user"));

    if header.is_none() && nested_owner.is_none() {
        let has_top_id = extract_json_id(
            item,
            &[
                "owner_id",
                "ownerId",
                "creator_id",
                "creatorId",
                "author_uid",
                "uid",
                "user_id",
                "userId",
            ],
        )
        .is_some();
        if !has_top_id {
            return None;
        }
    }

    let uid = header
        .and_then(|h| {
            extract_json_id(
                h,
                &[
                    "uid",
                    "ownerId",
                    "owner_id",
                    "creatorId",
                    "creator_id",
                    "userId",
                    "user_id",
                    "id",
                ],
            )
        })
        .or_else(|| {
            nested_owner.and_then(|o| {
                extract_json_id(
                    o,
                    &[
                        "uid",
                        "ownerId",
                        "owner_id",
                        "creatorId",
                        "userId",
                        "user_id",
                        "id",
                    ],
                )
            })
        })
        .or_else(|| {
            extract_json_id(
                item,
                &[
                    "owner_id",
                    "ownerId",
                    "creator_id",
                    "creatorId",
                    "author_uid",
                    "uid",
                    "user_id",
                    "userId",
                ],
            )
        })
        .map(|s| clean_feed_uid(&s))
        .unwrap_or_default();

    let name_keys = [
        "dpn",
        "displayName",
        "zaloName",
        "display_name",
        "author_name",
        "name",
    ];
    let display_name = [header, nested_owner, Some(item)]
        .into_iter()
        .flatten()
        .find_map(|src| {
            let picked = pick_best_display_name(src, &name_keys, &uid);
            (!picked.is_empty() && picked != uid).then_some(picked)
        })
        .or_else(|| {
            [header, nested_owner, Some(item)]
                .into_iter()
                .flatten()
                .find_map(|src| extract_json_text(src, &name_keys))
        })
        .unwrap_or_default();

    let ava_keys = ["avt", "avatar", "avatarUrl", "fullAvt", "author_avatar"];
    let avatar_url = [header, nested_owner, Some(item)]
        .into_iter()
        .flatten()
        .find_map(|src| extract_json_text(src, &ava_keys))
        .map(|u| normalize_social_media_url(&u))
        .unwrap_or_default();

    Some(FeedAuthor {
        uid,
        display_name,
        avatar_url,
    })
}

fn parse_feed_item_json(
    val: &Value,
    target_uid: &str,
    rt_stats: &HashMap<String, RealtimeFeedStat>,
) -> Option<FeedItem> {
    let item = val.get("item").unwrap_or(val);
    let author = extract_feed_author(item)?;
    let clean_target = clean_feed_uid(target_uid);

    let matches_target = clean_target.is_empty()
        || author.uid == clean_target
        || author
            .display_name
            .trim()
            .eq_ignore_ascii_case(target_uid.trim());
    if !matches_target {
        return None;
    }

    let fid = extract_json_id(item, &["fid", "feedId", "id"]).unwrap_or_default();
    if fid.is_empty() {
        return None;
    }

    let cts = item.get("cts").and_then(Value::as_i64).unwrap_or(0);
    let date_str = format_vn_timestamp_ms(cts);

    let content_obj = item.get("content").cloned().unwrap_or(json!({}));
    let caption = content_obj
        .get("caption")
        .or_else(|| content_obj.get("msg"))
        .or_else(|| content_obj.get("description"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();

    let default_dim = content_obj.get("photo_dimension");
    let mut media_list = Vec::new();
    let mut seen_urls = HashSet::new();

    if let Some(photo) = content_obj.get("photo") {
        if let Some(arr) = photo.as_array() {
            for p in arr {
                if let Some(m) = extract_media_from_photo_val(p, default_dim) {
                    if seen_urls.insert(m.url.clone()) {
                        media_list.push(m);
                    }
                }
            }
        } else if let Some(m) = extract_media_from_photo_val(photo, default_dim) {
            if seen_urls.insert(m.url.clone()) {
                media_list.push(m);
            }
        }
    }

    for key in ["photos", "items", "media_items", "medias"] {
        if let Some(arr) = content_obj.get(key).and_then(Value::as_array) {
            for p in arr {
                let inner = p.get("photo").unwrap_or(p);
                if let Some(m) = extract_media_from_photo_val(inner, default_dim) {
                    if seen_urls.insert(m.url.clone()) {
                        media_list.push(m);
                    }
                }
            }
        }
    }

    if let Some(video) = content_obj.get("video") {
        if let Some(m) = extract_media_from_photo_val(video, default_dim) {
            if seen_urls.insert(m.url.clone()) {
                media_list.push(m);
            }
        }
    }

    let footer = item.get("footer").cloned().unwrap_or(json!({}));
    let mut numlike = footer.get("numlike").and_then(Value::as_u64).unwrap_or(0) as u32;
    let mut numcmt = footer.get("numcmt").and_then(Value::as_u64).unwrap_or(0) as u32;
    let mut is_liked = item.get("islike").and_then(Value::as_i64).unwrap_or(0) > 0;

    let mut top_reactions = Vec::new();
    if let Some(rinfo) = footer.get("reaction_info") {
        if rinfo
            .get("current_reaction_id")
            .and_then(Value::as_i64)
            .unwrap_or(0)
            > 0
        {
            is_liked = true;
        }
        if let Some(top_arr) = rinfo.get("top_reactions").and_then(Value::as_array) {
            for r in top_arr {
                if let Some(rid) = r.as_i64() {
                    let label = reaction_id_to_label(rid).to_string();
                    if !top_reactions.contains(&label) {
                        top_reactions.push(label);
                    }
                }
            }
        }
    }
    if top_reactions.is_empty() && numlike > 0 {
        top_reactions.push("Thích".to_string());
    }

    // Ghi đè bằng số liệu real-time trong table_feed_real_time_like_comment nếu có
    if let Some(rt) = rt_stats.get(&fid) {
        if let Some(lk) = rt.like_count {
            numlike = lk;
        }
        if let Some(cm) = rt.comment_count {
            numcmt = cm;
        }
        if let Some(il) = rt.is_liked {
            is_liked = il;
        }
    }

    let privacy = item.get("privacy").and_then(Value::as_i64).unwrap_or(0);
    let status_badge = match privacy {
        2 => Some("Chỉ mình tôi".to_string()),
        1 => Some("Bạn bè".to_string()),
        _ => None,
    };

    Some(FeedItem {
        feed_id: fid,
        author,
        date_str,
        content: caption,
        status_badge,
        media_list,
        reaction: FeedReaction {
            like_count: numlike,
            comment_count: numcmt,
            is_liked,
            top_reactions,
        },
        comments: Vec::new(),
        timestamp_ms: cts,
    })
}

/// Nạp dữ liệu bài viết thật từ bảng `timeline_feed_1`, `user_feed` và hợp nhất số liệu real-time từ `table_feed_real_time_like_comment`
pub fn load_real_timeline_from_sqlite(db_path: &str, target_uid: &str) -> Result<Vec<FeedItem>> {
    let conn = rusqlite::Connection::open(db_path)?;
    let rt_stats = load_realtime_like_comment_map(&conn);

    let mut feeds = Vec::new();
    let mut seen_fids = HashSet::new();

    for table_name in ["timeline_feed_1", "user_feed"] {
        let table_exists: bool = conn
            .query_row(
                "SELECT count(*) FROM sqlite_master WHERE type='table' AND name=?1",
                [table_name],
                |row| row.get(0),
            )
            .map(|count: i64| count > 0)
            .unwrap_or(false);

        if !table_exists {
            continue;
        }

        let query = format!("SELECT content FROM {table_name}");
        let mut stmt = conn.prepare(&query)?;
        let rows = stmt.query_map([], |row| {
            let content_str: String = row.get(0)?;
            Ok(content_str)
        })?;

        for r in rows {
            let content_str = r?;
            if let Ok(val) = serde_json::from_str::<Value>(&content_str) {
                if let Some(feed_item) = parse_feed_item_json(&val, target_uid, &rt_stats) {
                    if seen_fids.insert(feed_item.feed_id.clone()) {
                        feeds.push(feed_item);
                    }
                }
            }
        }
    }

    // Sắp xếp bài đăng mới nhất (timestamp_ms lớn nhất) lên đầu danh sách
    feeds.sort_by(|a, b| {
        b.timestamp_ms
            .cmp(&a.timestamp_ms)
            .then_with(|| b.feed_id.cmp(&a.feed_id))
    });

    Ok(feeds)
}
