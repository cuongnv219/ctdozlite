//! Native Rust protocol, cryptography, multi-account store, scheduler, auto-reply,
//! and GitHub Releases updater core for `ctdozlite` iOS / iPadOS / iPhone Duo (`min iOS 12.0+`).

use aes::Aes128;
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use cbc::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use md5::{Digest as Md5Digest, Md5};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::ffi::{CStr, CString};
use std::fs;
use std::os::raw::c_char;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

type Aes128CbcEnc = cbc::Encryptor<Aes128>;
type Aes128CbcDec = cbc::Decryptor<Aes128>;

pub const ZPW_VER: u32 = 671;
pub const ZPW_TYPE: u32 = 30;
const ZERO_IV: [u8; 16] = [0u8; 16];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccountProfile {
    pub slot: i32,
    pub uid: String,
    pub display_name: String,
    pub phone_masked: String,
    pub status_text: String,
    pub avatar_initials: String,
    pub is_online: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatMessage {
    pub id: String,
    pub thread_id: String,
    pub sender_id: String,
    pub sender_name: String,
    pub content: String,
    pub timestamp_ms: i64,
    pub time_label: String,
    pub is_self: bool,
    pub reply_to_id: Option<String>,
    pub reply_preview: Option<String>,
    pub reactions: Vec<String>,
    pub msg_type: String, // "text", "sticker", "image", "voice", "scheduled_sent", "autoreply"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatThread {
    pub id: String,
    pub name: String,
    pub subtitle_tag: String,
    pub avatar_initials: String,
    pub avatar_color_hex: String,
    pub last_message: String,
    pub last_time_label: String,
    pub last_timestamp_ms: i64,
    pub unread_count: u32,
    pub is_group: bool,
    pub is_pinned: bool,
    pub is_muted: bool,
    pub is_hidden: bool,
    pub is_online: bool,
    pub member_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScheduledMessageItem {
    pub id: i64,
    pub slot: i32,
    pub thread_id: String,
    pub thread_name: String,
    pub content: String,
    pub deliver_at_ms: i64,
    pub deliver_time_label: String,
    pub status: String, // "pending", "sent", "cancelled"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AutoReplyRule {
    pub id: String,
    pub pattern: String,
    pub response: String,
    pub match_mode: String, // "contains", "exact", "all"
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSlotData {
    pub profile: AccountProfile,
    pub threads: Vec<ChatThread>,
    pub messages_by_thread: BTreeMap<String, Vec<ChatMessage>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedState {
    pub active_slot: i32,
    pub slots: BTreeMap<i32, AccountSlotData>,
    pub scheduled: Vec<ScheduledMessageItem>,
    pub next_sched_id: i64,
    pub autoreply_rules: Vec<AutoReplyRule>,
    pub msg_counter: u64,
}

struct CoreRuntime {
    data_dir: PathBuf,
    state: PersistedState,
}

static GLOBAL_CORE: OnceLock<Mutex<CoreRuntime>> = OnceLock::new();

fn now_epoch_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(1_762_060_000_000)
}

fn format_hhmm_from_ms(ts_ms: i64) -> String {
    let total_secs = (ts_ms / 1000).max(0);
    // Vietnam / ICT (+07:00) display helper
    let local_secs = total_secs + 7 * 3600;
    let hours = (local_secs / 3600) % 24;
    let mins = (local_secs / 60) % 60;
    format!("{hours:02}:{mins:02}")
}

/// Multi-host Zalo service failover table (#27).
pub fn service_failover_hosts(service_kind: &str) -> Vec<&'static str> {
    match service_kind {
        "chat" | "msg" => vec![
            "https://tt-chat1-wpa.chat.zalo.me",
            "https://tt-chat2-wpa.chat.zalo.me",
            "https://tt-chat3-wpa.chat.zalo.me",
            "https://chat-wpa.chat.zalo.me",
        ],
        "group" | "group_poll" => vec![
            "https://tt-group-wpa.chat.zalo.me",
            "https://group-wpa.chat.zalo.me",
            "https://tt-group-poll-wpa.chat.zalo.me",
        ],
        "friend" | "profile" => vec![
            "https://tt-friend-wpa.chat.zalo.me",
            "https://friend-wpa.chat.zalo.me",
            "https://tt-profile-wpa.chat.zalo.me",
        ],
        "conversation" => vec![
            "https://tt-convers-wpa.chat.zalo.me",
            "https://convers-wpa.chat.zalo.me",
        ],
        _ => vec![
            "https://tt-chat1-wpa.chat.zalo.me",
            "https://wpa.chat.zalo.me",
        ],
    }
}

/// Encrypt plaintext JSON using AES-128-CBC (PKCS#7, zero IV) for Zalo `ParamCipher v2`.
pub fn zcrypto_encrypt_cbc(plain_utf8: &str, key_utf8: &str) -> Result<String, String> {
    let mut key_bytes = [0u8; 16];
    let src = key_utf8.as_bytes();
    let copy_len = src.len().min(16);
    key_bytes[..copy_len].copy_from_slice(&src[..copy_len]);

    let pt = plain_utf8.as_bytes();
    let mut buf = vec![0u8; pt.len() + 16];
    buf[..pt.len()].copy_from_slice(pt);
    let ct = Aes128CbcEnc::new_from_slices(&key_bytes, &ZERO_IV)
        .map_err(|e| format!("cipher init err: {e}"))?
        .encrypt_padded_mut::<Pkcs7>(&mut buf, pt.len())
        .map_err(|e| format!("encrypt pad err: {e}"))?;
    Ok(BASE64.encode(ct))
}

/// Decrypt base64 ciphertext using AES-128-CBC (PKCS#7, zero IV).
pub fn zcrypto_decrypt_cbc(b64_cipher: &str, key_utf8: &str) -> Result<String, String> {
    let mut key_bytes = [0u8; 16];
    let src = key_utf8.as_bytes();
    let copy_len = src.len().min(16);
    key_bytes[..copy_len].copy_from_slice(&src[..copy_len]);

    let mut raw = BASE64
        .decode(b64_cipher.trim())
        .map_err(|e| format!("base64 decode err: {e}"))?;
    let pt = Aes128CbcDec::new_from_slices(&key_bytes, &ZERO_IV)
        .map_err(|e| format!("cipher init err: {e}"))?
        .decrypt_padded_mut::<Pkcs7>(&mut raw)
        .map_err(|e| format!("decrypt pad err: {e}"))?;
    String::from_utf8(pt.to_vec()).map_err(|e| format!("utf8 err: {e}"))
}

/// Decrypt a Zalo WebSocket AES-256-GCM frame (`12-byte nonce + ciphertext+tag`) (#19, #16).
pub fn zcrypto_decrypt_ws_gcm(key_32: &[u8; 32], nonce_12: &[u8; 12], ciphertext: &[u8]) -> Result<Vec<u8>, String> {
    let cipher = Aes256Gcm::new_from_slice(key_32).map_err(|e| format!("gcm key err: {e}"))?;
    let nonce = Nonce::from_slice(nonce_12);
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| format!("gcm decrypt err: {e}"))
}

/// Compute Zalo MD5 `signkey` (`md5("zsecure" + method + zpw_type + sorted_param_values)`).
pub fn compute_zalo_signkey(method_name: &str, params: &BTreeMap<String, String>) -> String {
    let mut hasher = Md5::new();
    hasher.update(b"zsecure");
    hasher.update(method_name.as_bytes());
    for val in params.values() {
        hasher.update(val.as_bytes());
    }
    let digest = hasher.finalize();
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

/// Check if two threads represent the same underlying entity (#63).
/// Strict rule: two threads with distinct non-empty IDs NEVER merge just because their display names match,
/// except for the singleton `"Cloud của tôi"` self-cloud thread.
pub fn threads_represent_same_contact(a: &ChatThread, b: &ChatThread) -> bool {
    if !a.id.is_empty() && !b.id.is_empty() {
        if a.id == b.id {
            return true;
        }
        let a_cloud = a.name.trim().eq_ignore_ascii_case("Cloud của tôi") || a.id == "cloud_my";
        let b_cloud = b.name.trim().eq_ignore_ascii_case("Cloud của tôi") || b.id == "cloud_my";
        if a_cloud && b_cloud {
            return true;
        }
        return false;
    }
    false
}

/// Compare semantic versions (`"v0.0.5"` > `"0.0.4"`).
pub fn is_newer_semver(candidate: &str, current: &str) -> bool {
    fn parse_ver(s: &str) -> (u64, u64, u64, bool) {
        let clean = s.trim().trim_start_matches(['v', 'V']);
        let (core, prerelease) = match clean.split_once('-') {
            Some((c, pre)) => (c, !pre.is_empty()),
            None => (clean, false),
        };
        let mut parts = core.split('.').map(|p| p.parse::<u64>().unwrap_or(0));
        let major = parts.next().unwrap_or(0);
        let minor = parts.next().unwrap_or(0);
        let patch = parts.next().unwrap_or(0);
        (major, minor, patch, prerelease)
    }
    let (c_maj, c_min, c_pat, c_pre) = parse_ver(candidate);
    let (o_maj, o_min, o_pat, o_pre) = parse_ver(current);
    match (c_maj.cmp(&o_maj), c_min.cmp(&o_min), c_pat.cmp(&o_pat)) {
        (std::cmp::Ordering::Greater, _, _) => true,
        (std::cmp::Ordering::Less, _, _) => false,
        (_, std::cmp::Ordering::Greater, _) => true,
        (_, std::cmp::Ordering::Less, _) => false,
        (_, _, std::cmp::Ordering::Greater) => true,
        (_, _, std::cmp::Ordering::Less) => false,
        _ => !c_pre && o_pre,
    }
}

/// Extract clean Vietnamese release notes bullets from a GitHub Release markdown `body`.
pub fn extract_release_bullets(markdown: &str) -> Vec<String> {
    let mut bullets = Vec::new();
    let mut in_code = false;
    for line in markdown.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            in_code = !in_code;
            continue;
        }
        if in_code || trimmed.is_empty() {
            continue;
        }
        if let Some(item) = trimmed
            .strip_prefix("- ")
            .or_else(|| trimmed.strip_prefix("* "))
        {
            let clean = item
                .replace("**", "")
                .replace('`', "")
                .trim()
                .to_string();
            if !clean.is_empty() {
                bullets.push(clean);
            }
        }
    }
    bullets
}

fn default_seed_state() -> PersistedState {
    let base_ts = now_epoch_ms();

    // Slot 0: Primary Zalo Account (includes #63 duplicate display name test pair "Nguyễn Văn An")
    let mut slot0_threads = Vec::new();
    let mut slot0_msgs: BTreeMap<String, Vec<ChatMessage>> = BTreeMap::new();

    slot0_threads.push(ChatThread {
        id: "cloud_my".to_string(),
        name: "Cloud của tôi".to_string(),
        subtitle_tag: "Lưu trữ cá nhân · Đồng bộ tức thì".to_string(),
        avatar_initials: "☁️".to_string(),
        avatar_color_hex: "#0068FF".to_string(),
        last_message: "Đã đồng bộ bản phát hành ctdozlite v0.0.5 cho iOS 12+ & iPhone Duo".to_string(),
        last_time_label: "Vừa xong".to_string(),
        last_timestamp_ms: base_ts - 30_000,
        unread_count: 0,
        is_group: false,
        is_pinned: true,
        is_muted: false,
        is_hidden: false,
        is_online: true,
        member_count: 1,
    });
    slot0_msgs.insert(
        "cloud_my".to_string(),
        vec![
            ChatMessage {
                id: "m_cloud_1".to_string(),
                thread_id: "cloud_my".to_string(),
                sender_id: "u_self_0".to_string(),
                sender_name: "Tôi".to_string(),
                content: "📌 Ghi chú nhanh: Bản ctdozlite iOS hỗ trợ từ iOS 12.0+, iPhone, iPad Split-View và đặc biệt tối ưu cho iPhone Duo (màn hình gập đôi / 2 khung chat song song).".to_string(),
                timestamp_ms: base_ts - 180_000,
                time_label: format_hhmm_from_ms(base_ts - 180_000),
                is_self: true,
                reply_to_id: None,
                reply_preview: None,
                reactions: vec!["❤️".to_string(), "👍".to_string()],
                msg_type: "text".to_string(),
            },
            ChatMessage {
                id: "m_cloud_2".to_string(),
                thread_id: "cloud_my".to_string(),
                sender_id: "u_self_0".to_string(),
                sender_name: "Tôi".to_string(),
                content: "Đã đồng bộ bản phát hành ctdozlite v0.0.5 cho iOS 12+ & iPhone Duo".to_string(),
                timestamp_ms: base_ts - 30_000,
                time_label: format_hhmm_from_ms(base_ts - 30_000),
                is_self: true,
                reply_to_id: None,
                reply_preview: None,
                reactions: vec![],
                msg_type: "text".to_string(),
            },
        ],
    );

    // Contact 1 of #63 duplicate-name verification: Nguyễn Văn An (UID 10001 - Phòng Kỹ Thuật)
    slot0_threads.push(ChatThread {
        id: "uid_10001_an_kythuat".to_string(),
        name: "Nguyễn Văn An".to_string(),
        subtitle_tag: "ID: 10001 · Kỹ thuật Hệ thống (#63)".to_string(),
        avatar_initials: "A1".to_string(),
        avatar_color_hex: "#10B981".to_string(),
        last_message: "Bản build iOS 12.0 arm64 + iPhone Duo chạy mượt 120fps luôn anh!".to_string(),
        last_time_label: "2 phút".to_string(),
        last_timestamp_ms: base_ts - 120_000,
        unread_count: 2,
        is_group: false,
        is_pinned: true,
        is_muted: false,
        is_hidden: false,
        is_online: true,
        member_count: 2,
    });
    slot0_msgs.insert(
        "uid_10001_an_kythuat".to_string(),
        vec![
            ChatMessage {
                id: "m_an1_1".to_string(),
                thread_id: "uid_10001_an_kythuat".to_string(),
                sender_id: "uid_10001_an_kythuat".to_string(),
                sender_name: "Nguyễn Văn An (Kỹ thuật)".to_string(),
                content: "Em là Nguyễn Văn An bên đội Kỹ thuật (ID: uid_10001_an_kythuat). Đã kiểm tra fix #63 không còn bị nhảy nhầm sang anh An bên Kinh doanh nữa!".to_string(),
                timestamp_ms: base_ts - 240_000,
                time_label: format_hhmm_from_ms(base_ts - 240_000),
                is_self: false,
                reply_to_id: None,
                reply_preview: None,
                reactions: vec!["👍".to_string()],
                msg_type: "text".to_string(),
            },
            ChatMessage {
                id: "m_an1_2".to_string(),
                thread_id: "uid_10001_an_kythuat".to_string(),
                sender_id: "uid_10001_an_kythuat".to_string(),
                sender_name: "Nguyễn Văn An (Kỹ thuật)".to_string(),
                content: "Bản build iOS 12.0 arm64 + iPhone Duo chạy mượt 120fps luôn anh!".to_string(),
                timestamp_ms: base_ts - 120_000,
                time_label: format_hhmm_from_ms(base_ts - 120_000),
                is_self: false,
                reply_to_id: None,
                reply_preview: None,
                reactions: vec!["❤️".to_string()],
                msg_type: "text".to_string(),
            },
        ],
    );

    // Contact 2 of #63 duplicate-name verification: Nguyễn Văn An (UID 20002 - Phòng Kinh Doanh)
    slot0_threads.push(ChatThread {
        id: "uid_20002_an_kinhdoanh".to_string(),
        name: "Nguyễn Văn An".to_string(),
        subtitle_tag: "ID: 20002 · Phòng Kinh Doanh (#63)".to_string(),
        avatar_initials: "A2".to_string(),
        avatar_color_hex: "#F59E0B".to_string(),
        last_message: "Em gửi báo giá hợp đồng Q4 của bộ phận Kinh doanh nhé.".to_string(),
        last_time_label: "5 phút".to_string(),
        last_timestamp_ms: base_ts - 300_000,
        unread_count: 1,
        is_group: false,
        is_pinned: false,
        is_muted: false,
        is_hidden: false,
        is_online: true,
        member_count: 2,
    });
    slot0_msgs.insert(
        "uid_20002_an_kinhdoanh".to_string(),
        vec![
            ChatMessage {
                id: "m_an2_1".to_string(),
                thread_id: "uid_20002_an_kinhdoanh".to_string(),
                sender_id: "uid_20002_an_kinhdoanh".to_string(),
                sender_name: "Nguyễn Văn An (Kinh doanh)".to_string(),
                content: "Chào anh, em là Nguyễn Văn An bên Kinh Doanh (ID: uid_20002_an_kinhdoanh). Tuy trùng tên hiển thị với bạn An Kỹ thuật nhưng lịch sử chat tách biệt 100%.".to_string(),
                timestamp_ms: base_ts - 360_000,
                time_label: format_hhmm_from_ms(base_ts - 360_000),
                is_self: false,
                reply_to_id: None,
                reply_preview: None,
                reactions: vec![],
                msg_type: "text".to_string(),
            },
            ChatMessage {
                id: "m_an2_2".to_string(),
                thread_id: "uid_20002_an_kinhdoanh".to_string(),
                sender_id: "uid_20002_an_kinhdoanh".to_string(),
                sender_name: "Nguyễn Văn An (Kinh doanh)".to_string(),
                content: "Em gửi báo giá hợp đồng Q4 của bộ phận Kinh doanh nhé.".to_string(),
                timestamp_ms: base_ts - 300_000,
                time_label: format_hhmm_from_ms(base_ts - 300_000),
                is_self: false,
                reply_to_id: None,
                reply_preview: None,
                reactions: vec![],
                msg_type: "text".to_string(),
            },
        ],
    );

    // Group Thread: Đội ngũ Phát triển ctdozlite
    slot0_threads.push(ChatThread {
        id: "grp_ctdozlite_dev".to_string(),
        name: "Nhóm Phát triển ctdozlite (Rust & iOS)".to_string(),
        subtitle_tag: "Nhóm · 18 thành viên · Đang hoạt động".to_string(),
        avatar_initials: "CT".to_string(),
        avatar_color_hex: "#6366F1".to_string(),
        last_message: "TIPC1110: Đã mở API cập nhật tự động v0.0.5 trên GitHub Releases!".to_string(),
        last_time_label: "10 phút".to_string(),
        last_timestamp_ms: base_ts - 600_000,
        unread_count: 0,
        is_group: true,
        is_pinned: true,
        is_muted: false,
        is_hidden: false,
        is_online: true,
        member_count: 18,
    });
    slot0_msgs.insert(
        "grp_ctdozlite_dev".to_string(),
        vec![
            ChatMessage {
                id: "m_grp_1".to_string(),
                thread_id: "grp_ctdozlite_dev".to_string(),
                sender_id: "u_tipc".to_string(),
                sender_name: "TIPC1110".to_string(),
                content: "Anh em kiểm tra chế độ iPhone Duo (Dual-Screen Split Chat) mở 2 hội thoại cùng lúc nhé!".to_string(),
                timestamp_ms: base_ts - 900_000,
                time_label: format_hhmm_from_ms(base_ts - 900_000),
                is_self: false,
                reply_to_id: None,
                reply_preview: None,
                reactions: vec!["❤️".to_string(), "🔥".to_string()],
                msg_type: "text".to_string(),
            },
            ChatMessage {
                id: "m_grp_2".to_string(),
                thread_id: "grp_ctdozlite_dev".to_string(),
                sender_id: "u_self_0".to_string(),
                sender_name: "Tôi".to_string(),
                content: "Đã tích hợp đầy đủ: Hẹn giờ gửi tin (#38), Auto-Reply (#35), Failover đa máy chủ (#27) và kiểm tra bản mới qua GitHub Releases API.".to_string(),
                timestamp_ms: base_ts - 720_000,
                time_label: format_hhmm_from_ms(base_ts - 720_000),
                is_self: true,
                reply_to_id: Some("m_grp_1".to_string()),
                reply_preview: Some("TIPC1110: Anh em kiểm tra chế độ iPhone Duo...".to_string()),
                reactions: vec!["👍".to_string()],
                msg_type: "text".to_string(),
            },
            ChatMessage {
                id: "m_grp_3".to_string(),
                thread_id: "grp_ctdozlite_dev".to_string(),
                sender_id: "u_tipc".to_string(),
                sender_name: "TIPC1110".to_string(),
                content: "TIPC1110: Đã mở API cập nhật tự động v0.0.5 trên GitHub Releases!".to_string(),
                timestamp_ms: base_ts - 600_000,
                time_label: format_hhmm_from_ms(base_ts - 600_000),
                is_self: false,
                reply_to_id: None,
                reply_preview: None,
                reactions: vec!["❤️".to_string()],
                msg_type: "text".to_string(),
            },
        ],
    );

    // Contact 4: Trần Minh Tâm (Thiết kế UI/UX iPad & iPhone Duo)
    slot0_threads.push(ChatThread {
        id: "uid_30003_minhtam".to_string(),
        name: "Trần Minh Tâm (UI/UX)".to_string(),
        subtitle_tag: "Chuyên gia Giao diện iOS/iPadOS".to_string(),
        avatar_initials: "MT".to_string(),
        avatar_color_hex: "#EC4899".to_string(),
        last_message: "Giao diện iPhone Duo tự động chia 2 màn hình qua bản lề cực chuẩn!".to_string(),
        last_time_label: "25 phút".to_string(),
        last_timestamp_ms: base_ts - 1_500_000,
        unread_count: 0,
        is_group: false,
        is_pinned: false,
        is_muted: false,
        is_hidden: false,
        is_online: true,
        member_count: 2,
    });
    slot0_msgs.insert(
        "uid_30003_minhtam".to_string(),
        vec![ChatMessage {
            id: "m_mt_1".to_string(),
            thread_id: "uid_30003_minhtam".to_string(),
            sender_id: "uid_30003_minhtam".to_string(),
            sender_name: "Trần Minh Tâm (UI/UX)".to_string(),
            content: "Giao diện iPhone Duo tự động chia 2 màn hình qua bản lề cực chuẩn!".to_string(),
            timestamp_ms: base_ts - 1_500_000,
            time_label: format_hhmm_from_ms(base_ts - 1_500_000),
            is_self: false,
            reply_to_id: None,
            reply_preview: None,
            reactions: vec!["❤️".to_string()],
            msg_type: "text".to_string(),
        }],
    );

    // Slot 1: Secondary Zalo Account ("Tài khoản 2 - Duo Work")
    let mut slot1_threads = Vec::new();
    let mut slot1_msgs: BTreeMap<String, Vec<ChatMessage>> = BTreeMap::new();

    slot1_threads.push(ChatThread {
        id: "cloud_my".to_string(),
        name: "Cloud của tôi (Tài khoản 2)".to_string(),
        subtitle_tag: "Tài khoản Công việc (Duo Slot #2)".to_string(),
        avatar_initials: "☁️".to_string(),
        avatar_color_hex: "#0EA5E9".to_string(),
        last_message: "Kho lưu trữ riêng biệt của Tài khoản Zalo số 2 (Chế độ Dual Account)".to_string(),
        last_time_label: "1 phút".to_string(),
        last_timestamp_ms: base_ts - 60_000,
        unread_count: 0,
        is_group: false,
        is_pinned: true,
        is_muted: false,
        is_hidden: false,
        is_online: true,
        member_count: 1,
    });
    slot1_msgs.insert(
        "cloud_my".to_string(),
        vec![ChatMessage {
            id: "m_s1_1".to_string(),
            thread_id: "cloud_my".to_string(),
            sender_id: "u_self_1".to_string(),
            sender_name: "Tôi (TK 2)".to_string(),
            content: "Kho lưu trữ riêng biệt của Tài khoản Zalo số 2 (Chế độ Dual Account)".to_string(),
            timestamp_ms: base_ts - 60_000,
            time_label: format_hhmm_from_ms(base_ts - 60_000),
            is_self: true,
            reply_to_id: None,
            reply_preview: None,
            reactions: vec![],
            msg_type: "text".to_string(),
        }],
    );

    slot1_threads.push(ChatThread {
        id: "uid_90001_khachhang_vip".to_string(),
        name: "Đối tác Doanh nghiệp VIP".to_string(),
        subtitle_tag: "Tài khoản 2 · Kênh CSKH 24/7".to_string(),
        avatar_initials: "VP".to_string(),
        avatar_color_hex: "#8B5CF6".to_string(),
        last_message: "Cảm ơn hệ thống Auto-Reply của ctdozlite đã phản hồi tức thì!".to_string(),
        last_time_label: "3 phút".to_string(),
        last_timestamp_ms: base_ts - 180_000,
        unread_count: 1,
        is_group: false,
        is_pinned: true,
        is_muted: false,
        is_hidden: false,
        is_online: true,
        member_count: 2,
    });
    slot1_msgs.insert(
        "uid_90001_khachhang_vip".to_string(),
        vec![ChatMessage {
            id: "m_s1_2".to_string(),
            thread_id: "uid_90001_khachhang_vip".to_string(),
            sender_id: "uid_90001_khachhang_vip".to_string(),
            sender_name: "Đối tác Doanh nghiệp VIP".to_string(),
            content: "Cảm ơn hệ thống Auto-Reply của ctdozlite đã phản hồi tức thì!".to_string(),
            timestamp_ms: base_ts - 180_000,
            time_label: format_hhmm_from_ms(base_ts - 180_000),
            is_self: false,
            reply_to_id: None,
            reply_preview: None,
            reactions: vec!["👍".to_string()],
            msg_type: "text".to_string(),
        }],
    );

    let mut slots = BTreeMap::new();
    slots.insert(
        0,
        AccountSlotData {
            profile: AccountProfile {
                slot: 0,
                uid: "zalo_uid_primary_01".to_string(),
                display_name: "Đỗ Trung Thành (TK Chính)".to_string(),
                phone_masked: "(+84) 098***1110".to_string(),
                status_text: "Trực tuyến · ctdozlite iOS v0.0.5".to_string(),
                avatar_initials: "ĐT".to_string(),
                is_online: true,
            },
            threads: slot0_threads,
            messages_by_thread: slot0_msgs,
        },
    );
    slots.insert(
        1,
        AccountSlotData {
            profile: AccountProfile {
                slot: 1,
                uid: "zalo_uid_duo_02".to_string(),
                display_name: "CTDO Team Work (TK Duo #2)".to_string(),
                phone_masked: "(+84) 091***2220".to_string(),
                status_text: "Song song 2 tài khoản (iPhone Duo)".to_string(),
                avatar_initials: "CW".to_string(),
                is_online: true,
            },
            threads: slot1_threads,
            messages_by_thread: slot1_msgs,
        },
    );

    let scheduled = vec![ScheduledMessageItem {
        id: 1,
        slot: 0,
        thread_id: "grp_ctdozlite_dev".to_string(),
        thread_name: "Nhóm Phát triển ctdozlite (Rust & iOS)".to_string(),
        content: "⏰ [Nhắc hẹn tự động #38]: Họp tổng kết phát hành bản iOS 12+ & iPhone Duo lúc 18:00!".to_string(),
        deliver_at_ms: base_ts + 900_000,
        deliver_time_label: format_hhmm_from_ms(base_ts + 900_000),
        status: "pending".to_string(),
    }];

    let autoreply_rules = vec![
        AutoReplyRule {
            id: "rule_1".to_string(),
            pattern: "báo giá".to_string(),
            response: "🤖 [Auto-Reply ctdozlite #35]: Mình đã nhận được yêu cầu báo giá, mình sẽ phản hồi chi tiết trong ít phút nhé!".to_string(),
            match_mode: "contains".to_string(),
            enabled: true,
        },
        AutoReplyRule {
            id: "rule_2".to_string(),
            pattern: "bận".to_string(),
            response: "🤖 [Auto-Reply ctdozlite #35]: Hiện mình đang trong cuộc họp, bạn để lại lời nhắn nhé!".to_string(),
            match_mode: "contains".to_string(),
            enabled: true,
        },
    ];

    PersistedState {
        active_slot: 0,
        slots,
        scheduled,
        next_sched_id: 2,
        autoreply_rules,
        msg_counter: 100,
    }
}

impl CoreRuntime {
    fn state_file_path(&self) -> PathBuf {
        self.data_dir.join("ctdozlite_ios_state_v5.json")
    }

    fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.state) {
            let _ = fs::create_dir_all(&self.data_dir);
            let tmp = self.data_dir.join("ctdozlite_ios_state_v5.json.tmp");
            if fs::write(&tmp, json.as_bytes()).is_ok() {
                let _ = fs::rename(&tmp, self.state_file_path());
            }
        }
    }

    fn load_or_default(data_dir: PathBuf) -> Self {
        let _ = fs::create_dir_all(&data_dir);
        let path = data_dir.join("ctdozlite_ios_state_v5.json");
        if let Ok(raw) = fs::read_to_string(&path) {
            if let Ok(state) = serde_json::from_str::<PersistedState>(&raw) {
                return Self { data_dir, state };
            }
        }
        let rt = Self {
            data_dir,
            state: default_seed_state(),
        };
        rt.save();
        rt
    }

    fn sort_threads(threads: &mut [ChatThread]) {
        threads.sort_by(|a, b| {
            b.is_pinned
                .cmp(&a.is_pinned)
                .then_with(|| b.last_timestamp_ms.cmp(&a.last_timestamp_ms))
                .then_with(|| a.id.cmp(&b.id))
        });
    }
}

fn with_core<F, R>(f: F) -> R
where
    F: FnOnce(&mut CoreRuntime) -> R,
{
    let lock = GLOBAL_CORE.get_or_init(|| {
        let fallback = std::env::temp_dir().join("ctdozlite_ios_default");
        Mutex::new(CoreRuntime::load_or_default(fallback))
    });
    let mut guard = lock.lock().expect("core mutex poisoned");
    f(&mut guard)
}

fn cstr_to_string(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
}

fn string_to_c_char(s: String) -> *mut c_char {
    let sanitized = s.replace('\0', "");
    CString::new(sanitized)
        .unwrap_or_else(|_| CString::new("{}").unwrap())
        .into_raw()
}

// ============================================================================
// C FFI Exports for Objective-C / UIKit (iOS 12.0+)
// ============================================================================

#[no_mangle]
pub extern "C" fn ctdoz_ios_init(data_dir_utf8: *const c_char) -> i32 {
    let dir_str = cstr_to_string(data_dir_utf8);
    let path = if dir_str.trim().is_empty() {
        std::env::temp_dir().join("ctdozlite_ios_default")
    } else {
        PathBuf::from(dir_str)
    };
    if let Some(lock) = GLOBAL_CORE.get() {
        if let Ok(mut guard) = lock.lock() {
            *guard = CoreRuntime::load_or_default(path);
            return 0;
        }
        return -1;
    }
    let _ = GLOBAL_CORE.set(Mutex::new(CoreRuntime::load_or_default(path)));
    0
}

#[no_mangle]
pub extern "C" fn ctdoz_ios_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

#[no_mangle]
pub extern "C" fn ctdoz_ios_version() -> *mut c_char {
    string_to_c_char(env!("CARGO_PKG_VERSION").to_string())
}

#[no_mangle]
pub extern "C" fn ctdoz_ios_switch_account(slot: i32) -> *mut c_char {
    with_core(|rt| {
        let target = if rt.state.slots.contains_key(&slot) {
            slot
        } else {
            0
        };
        rt.state.active_slot = target;
        rt.save();
        let profile = &rt.state.slots.get(&target).unwrap().profile;
        string_to_c_char(serde_json::to_string(profile).unwrap_or_else(|_| "{}".to_string()))
    })
}

#[no_mangle]
pub extern "C" fn ctdoz_ios_list_threads_json(
    slot: i32,
    filter_mode: i32,
    search_query: *const c_char,
) -> *mut c_char {
    let query = cstr_to_string(search_query).trim().to_lowercase();
    with_core(|rt| {
        let Some(slot_data) = rt.state.slots.get_mut(&slot) else {
            return string_to_c_char("[]".to_string());
        };
        CoreRuntime::sort_threads(&mut slot_data.threads);
        let filtered: Vec<&ChatThread> = slot_data
            .threads
            .iter()
            .filter(|t| {
                let mode_ok = match filter_mode {
                    1 => !t.is_hidden && t.unread_count > 0,
                    2 => !t.is_hidden && t.is_pinned,
                    3 => !t.is_hidden && t.is_group,
                    4 => t.is_hidden,
                    _ => !t.is_hidden,
                };
                if !mode_ok {
                    return false;
                }
                if query.is_empty() {
                    return true;
                }
                t.name.to_lowercase().contains(&query)
                    || t.subtitle_tag.to_lowercase().contains(&query)
                    || t.last_message.to_lowercase().contains(&query)
                    || t.id.to_lowercase().contains(&query)
            })
            .collect();
        string_to_c_char(serde_json::to_string(&filtered).unwrap_or_else(|_| "[]".to_string()))
    })
}

#[no_mangle]
pub extern "C" fn ctdoz_ios_get_messages_json(slot: i32, thread_id: *const c_char) -> *mut c_char {
    let tid = cstr_to_string(thread_id);
    with_core(|rt| {
        let Some(slot_data) = rt.state.slots.get_mut(&slot) else {
            return string_to_c_char("[]".to_string());
        };
        if let Some(t) = slot_data.threads.iter_mut().find(|t| t.id == tid) {
            t.unread_count = 0;
        }
        let msgs = slot_data
            .messages_by_thread
            .get(&tid)
            .cloned()
            .unwrap_or_default();
        string_to_c_char(serde_json::to_string(&msgs).unwrap_or_else(|_| "[]".to_string()))
    })
}

#[no_mangle]
pub extern "C" fn ctdoz_ios_send_message_json(
    slot: i32,
    thread_id: *const c_char,
    content: *const c_char,
    reply_to_msg_id: *const c_char,
) -> *mut c_char {
    let tid = cstr_to_string(thread_id);
    let text = cstr_to_string(content).trim().to_string();
    let reply_id = {
        let s = cstr_to_string(reply_to_msg_id).trim().to_string();
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    };
    with_core(|rt| {
        if text.is_empty() {
            let msgs = rt
                .state
                .slots
                .get(&slot)
                .and_then(|s| s.messages_by_thread.get(&tid))
                .cloned()
                .unwrap_or_default();
            return string_to_c_char(
                serde_json::to_string(&msgs).unwrap_or_else(|_| "[]".to_string()),
            );
        }
        rt.state.msg_counter += 1;
        let msg_id = format!("msg_{}", rt.state.msg_counter);
        let now_ms = now_epoch_ms();
        let time_lbl = format_hhmm_from_ms(now_ms);

        let Some(slot_data) = rt.state.slots.get_mut(&slot) else {
            return string_to_c_char("[]".to_string());
        };

        let reply_preview = reply_id.as_ref().and_then(|rid| {
            slot_data
                .messages_by_thread
                .get(&tid)
                .and_then(|list| list.iter().find(|m| &m.id == rid))
                .map(|m| format!("{}: {}", m.sender_name, m.content))
        });

        let msg_type = if text.starts_with("[Sticker]") {
            "sticker"
        } else if text.starts_with("[Ảnh]") {
            "image"
        } else if text.starts_with("[Thoại]") {
            "voice"
        } else {
            "text"
        };

        let new_msg = ChatMessage {
            id: msg_id,
            thread_id: tid.clone(),
            sender_id: slot_data.profile.uid.clone(),
            sender_name: "Tôi".to_string(),
            content: text.clone(),
            timestamp_ms: now_ms,
            time_label: time_lbl.clone(),
            is_self: true,
            reply_to_id: reply_id,
            reply_preview,
            reactions: Vec::new(),
            msg_type: msg_type.to_string(),
        };

        let list = slot_data.messages_by_thread.entry(tid.clone()).or_default();
        list.push(new_msg);

        if let Some(t) = slot_data.threads.iter_mut().find(|t| t.id == tid) {
            t.last_message = format!("Bạn: {text}");
            t.last_time_label = "Vừa xong".to_string();
            t.last_timestamp_ms = now_ms;
            t.unread_count = 0;
        }
        let out = list.clone();
        rt.save();
        string_to_c_char(serde_json::to_string(&out).unwrap_or_else(|_| "[]".to_string()))
    })
}

#[no_mangle]
pub extern "C" fn ctdoz_ios_receive_message_json(
    slot: i32,
    thread_id: *const c_char,
    sender_name: *const c_char,
    content: *const c_char,
) -> *mut c_char {
    let tid = cstr_to_string(thread_id);
    let sender = cstr_to_string(sender_name);
    let text = cstr_to_string(content).trim().to_string();

    with_core(|rt| {
        if text.is_empty() {
            return string_to_c_char("[]".to_string());
        }
        let now_ms = now_epoch_ms();
        rt.state.msg_counter += 1;
        let inc_id = format!("msg_inc_{}", rt.state.msg_counter);

        // Check if any enabled Auto-Reply rule matches (#35)
        let lower_in = text.to_lowercase();
        let matched_reply: Option<String> = rt
            .state
            .autoreply_rules
            .iter()
            .find(|r| {
                if !r.enabled {
                    return false;
                }
                let pat = r.pattern.trim().to_lowercase();
                match r.match_mode.as_str() {
                    "exact" => lower_in == pat,
                    "all" => true,
                    _ => !pat.is_empty() && lower_in.contains(&pat),
                }
            })
            .map(|r| r.response.clone());

        let mut auto_id = String::new();
        if matched_reply.is_some() {
            rt.state.msg_counter += 1;
            auto_id = format!("msg_auto_{}", rt.state.msg_counter);
        }

        let Some(slot_data) = rt.state.slots.get_mut(&slot) else {
            return string_to_c_char("[]".to_string());
        };

        let resolved_sender = if sender.trim().is_empty() {
            slot_data
                .threads
                .iter()
                .find(|t| t.id == tid)
                .map(|t| t.name.clone())
                .unwrap_or_else(|| "Đối tác".to_string())
        } else {
            sender
        };

        let inc_msg = ChatMessage {
            id: inc_id.clone(),
            thread_id: tid.clone(),
            sender_id: tid.clone(),
            sender_name: resolved_sender.clone(),
            content: text.clone(),
            timestamp_ms: now_ms,
            time_label: format_hhmm_from_ms(now_ms),
            is_self: false,
            reply_to_id: None,
            reply_preview: None,
            reactions: Vec::new(),
            msg_type: "text".to_string(),
        };

        let list = slot_data.messages_by_thread.entry(tid.clone()).or_default();
        list.push(inc_msg);

        let mut final_preview = text;
        if let Some(reply_txt) = matched_reply {
            let auto_msg = ChatMessage {
                id: auto_id,
                thread_id: tid.clone(),
                sender_id: slot_data.profile.uid.clone(),
                sender_name: "Tôi (Auto-Reply)".to_string(),
                content: reply_txt.clone(),
                timestamp_ms: now_ms + 50,
                time_label: format_hhmm_from_ms(now_ms + 50),
                is_self: true,
                reply_to_id: Some(inc_id),
                reply_preview: Some(format!("{resolved_sender}: {final_preview}")),
                reactions: Vec::new(),
                msg_type: "autoreply".to_string(),
            };
            list.push(auto_msg);
            final_preview = reply_txt;
        }

        if let Some(t) = slot_data.threads.iter_mut().find(|t| t.id == tid) {
            t.last_message = final_preview;
            t.last_time_label = "Vừa xong".to_string();
            t.last_timestamp_ms = now_ms + 50;
            t.unread_count += 1;
        }

        let out = list.clone();
        rt.save();
        string_to_c_char(serde_json::to_string(&out).unwrap_or_else(|_| "[]".to_string()))
    })
}

#[no_mangle]
pub extern "C" fn ctdoz_ios_react_message_json(
    slot: i32,
    thread_id: *const c_char,
    msg_id: *const c_char,
    reaction_emoji: *const c_char,
) -> *mut c_char {
    let tid = cstr_to_string(thread_id);
    let mid = cstr_to_string(msg_id);
    let emoji = cstr_to_string(reaction_emoji);

    with_core(|rt| {
        let Some(slot_data) = rt.state.slots.get_mut(&slot) else {
            return string_to_c_char("[]".to_string());
        };
        let list = slot_data.messages_by_thread.entry(tid).or_default();
        if let Some(m) = list.iter_mut().find(|m| m.id == mid) {
            if let Some(pos) = m.reactions.iter().position(|r| r == &emoji) {
                m.reactions.remove(pos);
            } else if !emoji.trim().is_empty() {
                m.reactions.push(emoji);
            }
        }
        let out = list.clone();
        rt.save();
        string_to_c_char(serde_json::to_string(&out).unwrap_or_else(|_| "[]".to_string()))
    })
}

#[no_mangle]
pub extern "C" fn ctdoz_ios_thread_action_json(
    slot: i32,
    thread_id: *const c_char,
    action: i32,
) -> *mut c_char {
    let tid = cstr_to_string(thread_id);
    with_core(|rt| {
        let Some(slot_data) = rt.state.slots.get_mut(&slot) else {
            return string_to_c_char("[]".to_string());
        };
        if let Some(t) = slot_data.threads.iter_mut().find(|t| t.id == tid) {
            match action {
                1 => t.is_pinned = !t.is_pinned,
                2 => t.is_muted = !t.is_muted,
                3 => t.is_hidden = !t.is_hidden,
                4 => {
                    t.last_message = "Đã xoá lịch sử cuộc trò chuyện".to_string();
                    t.unread_count = 0;
                    slot_data.messages_by_thread.insert(tid.clone(), Vec::new());
                }
                5 => t.unread_count = 0,
                _ => {}
            }
        }
        CoreRuntime::sort_threads(&mut slot_data.threads);
        let visible: Vec<&ChatThread> = slot_data
            .threads
            .iter()
            .filter(|t| !t.is_hidden)
            .collect();
        let json = serde_json::to_string(&visible).unwrap_or_else(|_| "[]".to_string());
        rt.save();
        string_to_c_char(json)
    })
}

#[no_mangle]
pub extern "C" fn ctdoz_ios_schedule_message_json(
    slot: i32,
    thread_id: *const c_char,
    content: *const c_char,
    deliver_at_ms: i64,
) -> *mut c_char {
    let tid = cstr_to_string(thread_id);
    let text = cstr_to_string(content).trim().to_string();
    with_core(|rt| {
        if !text.is_empty() {
            let t_name = rt
                .state
                .slots
                .get(&slot)
                .and_then(|s| s.threads.iter().find(|t| t.id == tid))
                .map(|t| t.name.clone())
                .unwrap_or_else(|| tid.clone());
            let id = rt.state.next_sched_id;
            rt.state.next_sched_id += 1;
            rt.state.scheduled.push(ScheduledMessageItem {
                id,
                slot,
                thread_id: tid,
                thread_name: t_name,
                content: text,
                deliver_at_ms,
                deliver_time_label: format_hhmm_from_ms(deliver_at_ms),
                status: "pending".to_string(),
            });
            rt.save();
        }
        let items: Vec<&ScheduledMessageItem> = rt
            .state
            .scheduled
            .iter()
            .filter(|s| s.slot == slot && s.status == "pending")
            .collect();
        string_to_c_char(serde_json::to_string(&items).unwrap_or_else(|_| "[]".to_string()))
    })
}

#[no_mangle]
pub extern "C" fn ctdoz_ios_list_scheduled_json(slot: i32) -> *mut c_char {
    with_core(|rt| {
        let items: Vec<&ScheduledMessageItem> = rt
            .state
            .scheduled
            .iter()
            .filter(|s| s.slot == slot && s.status == "pending")
            .collect();
        string_to_c_char(serde_json::to_string(&items).unwrap_or_else(|_| "[]".to_string()))
    })
}

#[no_mangle]
pub extern "C" fn ctdoz_ios_cancel_scheduled_json(slot: i32, sched_id: i64) -> *mut c_char {
    with_core(|rt| {
        if let Some(item) = rt.state.scheduled.iter_mut().find(|s| s.id == sched_id) {
            item.status = "cancelled".to_string();
            rt.save();
        }
        let items: Vec<&ScheduledMessageItem> = rt
            .state
            .scheduled
            .iter()
            .filter(|s| s.slot == slot && s.status == "pending")
            .collect();
        string_to_c_char(serde_json::to_string(&items).unwrap_or_else(|_| "[]".to_string()))
    })
}

#[no_mangle]
pub extern "C" fn ctdoz_ios_poll_due_scheduled_json(now_ms: i64) -> *mut c_char {
    let effective_now = if now_ms <= 0 { now_epoch_ms() } else { now_ms };
    with_core(|rt| {
        let mut dispatched = Vec::new();
        for item in rt.state.scheduled.iter_mut() {
            if item.status == "pending" && item.deliver_at_ms <= effective_now {
                item.status = "sent".to_string();
                dispatched.push(item.clone());
            }
        }
        if !dispatched.is_empty() {
            for item in &dispatched {
                rt.state.msg_counter += 1;
                let mid = format!("msg_sched_{}", rt.state.msg_counter);
                if let Some(slot_data) = rt.state.slots.get_mut(&item.slot) {
                    let msg = ChatMessage {
                        id: mid,
                        thread_id: item.thread_id.clone(),
                        sender_id: slot_data.profile.uid.clone(),
                        sender_name: "Tôi (Hẹn giờ 🕒)".to_string(),
                        content: item.content.clone(),
                        timestamp_ms: effective_now,
                        time_label: format_hhmm_from_ms(effective_now),
                        is_self: true,
                        reply_to_id: None,
                        reply_preview: None,
                        reactions: Vec::new(),
                        msg_type: "scheduled_sent".to_string(),
                    };
                    slot_data
                        .messages_by_thread
                        .entry(item.thread_id.clone())
                        .or_default()
                        .push(msg);
                    if let Some(t) = slot_data
                        .threads
                        .iter_mut()
                        .find(|t| t.id == item.thread_id)
                    {
                        t.last_message = format!("🕒 {}", item.content);
                        t.last_time_label = "Vừa xong".to_string();
                        t.last_timestamp_ms = effective_now;
                    }
                }
            }
            rt.save();
        }
        string_to_c_char(serde_json::to_string(&dispatched).unwrap_or_else(|_| "[]".to_string()))
    })
}

#[no_mangle]
pub extern "C" fn ctdoz_ios_list_autoreply_json() -> *mut c_char {
    with_core(|rt| {
        string_to_c_char(
            serde_json::to_string(&rt.state.autoreply_rules).unwrap_or_else(|_| "[]".to_string()),
        )
    })
}

#[no_mangle]
pub extern "C" fn ctdoz_ios_mutate_autoreply_json(
    op: i32,
    pattern_or_id: *const c_char,
    response: *const c_char,
    match_mode: *const c_char,
) -> *mut c_char {
    let key = cstr_to_string(pattern_or_id).trim().to_string();
    let resp = cstr_to_string(response).trim().to_string();
    let mode = {
        let m = cstr_to_string(match_mode).trim().to_lowercase();
        if m == "exact" || m == "all" {
            m
        } else {
            "contains".to_string()
        }
    };
    with_core(|rt| {
        match op {
            1 => {
                if !key.is_empty() && !resp.is_empty() {
                    rt.state.msg_counter += 1;
                    let rid = format!("rule_{}", rt.state.msg_counter);
                    rt.state.autoreply_rules.push(AutoReplyRule {
                        id: rid,
                        pattern: key,
                        response: resp,
                        match_mode: mode,
                        enabled: true,
                    });
                    rt.save();
                }
            }
            2 => {
                if let Some(r) = rt.state.autoreply_rules.iter_mut().find(|r| r.id == key) {
                    r.enabled = !r.enabled;
                    rt.save();
                }
            }
            3 => {
                let before = rt.state.autoreply_rules.len();
                rt.state.autoreply_rules.retain(|r| r.id != key);
                if rt.state.autoreply_rules.len() != before {
                    rt.save();
                }
            }
            _ => {}
        }
        string_to_c_char(
            serde_json::to_string(&rt.state.autoreply_rules).unwrap_or_else(|_| "[]".to_string()),
        )
    })
}

#[no_mangle]
pub extern "C" fn ctdoz_ios_build_signed_request_json(
    service_kind: *const c_char,
    method_name: *const c_char,
    plain_json_params: *const c_char,
    zpw_enk_utf8: *const c_char,
) -> *mut c_char {
    let svc = cstr_to_string(service_kind);
    let method = cstr_to_string(method_name);
    let plain = cstr_to_string(plain_json_params);
    let enk = {
        let k = cstr_to_string(zpw_enk_utf8);
        if k.trim().is_empty() {
            "ctdozlite_ios12_default_key".to_string()
        } else {
            k
        }
    };

    let encrypted = zcrypto_encrypt_cbc(&plain, &enk).unwrap_or_default();
    let mut sign_map = BTreeMap::new();
    sign_map.insert("params".to_string(), encrypted.clone());
    sign_map.insert("zpw_type".to_string(), ZPW_TYPE.to_string());
    sign_map.insert("zpw_ver".to_string(), ZPW_VER.to_string());
    let signkey = compute_zalo_signkey(&method, &sign_map);
    let hosts = service_failover_hosts(&svc);
    let primary = hosts.first().copied().unwrap_or("https://wpa.chat.zalo.me");

    let out = serde_json::json!({
        "zpw_ver": ZPW_VER,
        "zpw_type": ZPW_TYPE,
        "encrypted_params": encrypted,
        "signkey": signkey,
        "endpoint": format!("{primary}/api/{svc}/{method}?zpw_ver={ZPW_VER}&zpw_type={ZPW_TYPE}&signkey={signkey}"),
        "failover_hosts": hosts,
    });
    string_to_c_char(out.to_string())
}

#[no_mangle]
pub extern "C" fn ctdoz_ios_parse_github_release_json(
    raw_github_json: *const c_char,
    current_version_override: *const c_char,
) -> *mut c_char {
    let raw = cstr_to_string(raw_github_json);
    let current_ver = {
        let v = cstr_to_string(current_version_override);
        if v.trim().is_empty() {
            env!("CARGO_PKG_VERSION").to_string()
        } else {
            v.trim().trim_start_matches(['v', 'V']).to_string()
        }
    };

    let parsed: serde_json::Value = match serde_json::from_str(&raw) {
        Ok(v) => v,
        Err(e) => {
            return string_to_c_char(
                serde_json::json!({
                    "error": format!("JSON parse error: {e}"),
                    "has_update": false,
                    "current_version": current_ver,
                })
                .to_string(),
            );
        }
    };

    let tag_name = parsed
        .get("tag_name")
        .and_then(|v| v.as_str())
        .unwrap_or("v0.0.5");
    let latest_ver = tag_name.trim().trim_start_matches(['v', 'V']);
    let title = parsed
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or(tag_name);
    let html_url = parsed
        .get("html_url")
        .and_then(|v| v.as_str())
        .unwrap_or("https://github.com/ctdoteam1110/ctdozlite/releases");
    let published_at = parsed
        .get("published_at")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let body = parsed.get("body").and_then(|v| v.as_str()).unwrap_or("");
    let bullets = extract_release_bullets(body);
    let has_update = is_newer_semver(latest_ver, &current_ver);

    let resp = serde_json::json!({
        "has_update": has_update,
        "current_version": current_ver,
        "latest_version": latest_ver,
        "tag_name": tag_name,
        "title": title,
        "published_at": published_at,
        "html_url": html_url,
        "bullets": bullets,
    });
    string_to_c_char(resp.to_string())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PortableHistoryBundle {
    threads: Vec<ChatThread>,
    messages_by_thread: BTreeMap<String, Vec<ChatMessage>>,
}

#[no_mangle]
pub extern "C" fn ctdoz_ios_export_history_json(slot: i32) -> *mut c_char {
    with_core(|rt| {
        let Some(slot_data) = rt.state.slots.get(&slot) else {
            return string_to_c_char("{}".to_string());
        };
        let bundle = PortableHistoryBundle {
            threads: slot_data.threads.clone(),
            messages_by_thread: slot_data.messages_by_thread.clone(),
        };
        string_to_c_char(
            serde_json::to_string_pretty(&bundle).unwrap_or_else(|_| "{}".to_string()),
        )
    })
}

#[no_mangle]
pub extern "C" fn ctdoz_ios_import_history_json(
    slot: i32,
    external_json: *const c_char,
) -> *mut c_char {
    let raw = cstr_to_string(external_json);
    with_core(|rt| {
        let Ok(bundle) = serde_json::from_str::<PortableHistoryBundle>(&raw) else {
            return string_to_c_char(
                serde_json::json!({
                    "imported_threads": 0,
                    "imported_messages": 0,
                    "skipped_duplicates": 0,
                    "error": "Định dạng JSON lịch sử không hợp lệ"
                })
                .to_string(),
            );
        };
        let Some(slot_data) = rt.state.slots.get_mut(&slot) else {
            return string_to_c_char("{}".to_string());
        };
        let mut imported_threads = 0usize;
        let mut imported_messages = 0usize;
        let mut skipped_duplicates = 0usize;

        for ext_t in bundle.threads {
            let exists = slot_data
                .threads
                .iter()
                .any(|cur| threads_represent_same_contact(cur, &ext_t));
            if !exists {
                slot_data.threads.push(ext_t);
                imported_threads += 1;
            }
        }
        for (tid, msgs) in bundle.messages_by_thread {
            let dest = slot_data.messages_by_thread.entry(tid).or_default();
            for m in msgs {
                if dest.iter().any(|existing| existing.id == m.id) {
                    skipped_duplicates += 1;
                } else {
                    dest.push(m);
                    imported_messages += 1;
                }
            }
            dest.sort_by_key(|m| m.timestamp_ms);
        }
        CoreRuntime::sort_threads(&mut slot_data.threads);
        rt.save();
        string_to_c_char(
            serde_json::json!({
                "imported_threads": imported_threads,
                "imported_messages": imported_messages,
                "skipped_duplicates": skipped_duplicates
            })
            .to_string(),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn param_cipher_v2_roundtrip_and_signkey() {
        let key = "zpw_enk_secret16";
        let plain = r#"{"thread_id":"uid_10001","msg":"Xin chào iOS 12 & iPhone Duo"}"#;
        let enc = zcrypto_encrypt_cbc(plain, key).expect("encrypt ok");
        let dec = zcrypto_decrypt_cbc(&enc, key).expect("decrypt ok");
        assert_eq!(dec, plain);

        let mut map = BTreeMap::new();
        map.insert("params".to_string(), enc);
        map.insert("zpw_type".to_string(), "30".to_string());
        map.insert("zpw_ver".to_string(), "671".to_string());
        let sk = compute_zalo_signkey("sendmsg", &map);
        assert_eq!(sk.len(), 32);
    }

    #[test]
    fn duplicate_display_names_with_different_ids_never_merge_issue_63() {
        let state = default_seed_state();
        let slot0 = state.slots.get(&0).unwrap();
        let an_threads: Vec<&ChatThread> = slot0
            .threads
            .iter()
            .filter(|t| t.name == "Nguyễn Văn An")
            .collect();
        assert_eq!(an_threads.len(), 2);
        assert_ne!(an_threads[0].id, an_threads[1].id);
        assert!(!threads_represent_same_contact(an_threads[0], an_threads[1]));
    }

    #[test]
    fn scheduled_message_and_autoreply_and_migration_work_end_to_end() {
        let tmp = std::env::temp_dir().join(format!("ctdoz_ios_test_{}", now_epoch_ms()));
        let dir_c = CString::new(tmp.to_string_lossy().as_bytes()).unwrap();
        assert_eq!(ctdoz_ios_init(dir_c.as_ptr()), 0);

        // 1) Auto-reply triggers on incoming "báo giá"
        let tid = CString::new("uid_10001_an_kythuat").unwrap();
        let sender = CString::new("Nguyễn Văn An").unwrap();
        let inc = CString::new("Anh gửi giúp em báo giá bản iOS nhé").unwrap();
        let msgs_ptr = ctdoz_ios_receive_message_json(0, tid.as_ptr(), sender.as_ptr(), inc.as_ptr());
        let msgs_json = cstr_to_string(msgs_ptr);
        ctdoz_ios_free_string(msgs_ptr);
        assert!(msgs_json.contains("Auto-Reply ctdozlite #35"));

        // 2) Schedule message & poll due
        let content = CString::new("Tin nhắn hẹn giờ tự động").unwrap();
        let sched_ptr = ctdoz_ios_schedule_message_json(0, tid.as_ptr(), content.as_ptr(), 1000);
        ctdoz_ios_free_string(sched_ptr);
        let due_ptr = ctdoz_ios_poll_due_scheduled_json(2000);
        let due_json = cstr_to_string(due_ptr);
        ctdoz_ios_free_string(due_ptr);
        assert!(due_json.contains("Tin nhắn hẹn giờ tự động"));

        // 3) History export & idempotent import (#9)
        let exp_ptr = ctdoz_ios_export_history_json(0);
        let exp_json = cstr_to_string(exp_ptr);
        ctdoz_ios_free_string(exp_ptr);
        let exp_c = CString::new(exp_json).unwrap();
        let imp_ptr = ctdoz_ios_import_history_json(0, exp_c.as_ptr());
        let imp_json = cstr_to_string(imp_ptr);
        ctdoz_ios_free_string(imp_ptr);
        assert!(imp_json.contains("\"imported_messages\":0"));

        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn github_release_parser_detects_v0_0_5_and_extracts_bullets() {
        let sample = r####"{
            "tag_name": "v0.0.5",
            "name": "ctdozlite v0.0.5",
            "html_url": "https://github.com/ctdoteam1110/ctdozlite/releases/tag/v0.0.5",
            "published_at": "2026-11-02T03:15:00Z",
            "body": "### 🐛 Đã sửa lỗi\n- **Sửa lỗi trùng tên Zalo (#63):** Tách biệt theo `thread_id`.\n- **Hẹn giờ tin nhắn (#38):** Thêm bảng `scheduled_messages`."
        }"####;
        let sample_c = CString::new(sample).unwrap();
        let cur_c = CString::new("0.0.4").unwrap();
        let out_ptr = ctdoz_ios_parse_github_release_json(sample_c.as_ptr(), cur_c.as_ptr());
        let out_json = cstr_to_string(out_ptr);
        ctdoz_ios_free_string(out_ptr);
        assert!(out_json.contains("\"has_update\":true"));
        assert!(out_json.contains("Sửa lỗi trùng tên Zalo (#63)"));
    }

    #[test]
    fn websocket_aes_256_gcm_frame_roundtrip_issue_19_16() {
        let key = [0x42u8; 32];
        let nonce_bytes = [0x24u8; 12];
        let cipher = Aes256Gcm::new_from_slice(&key).unwrap();
        let plain = b"{\"cmd\":501,\"thread_id\":\"uid_10001_an_kythuat\",\"msg\":\"Realtime WS frame\"}";
        let ct = cipher
            .encrypt(Nonce::from_slice(&nonce_bytes), plain.as_ref())
            .unwrap();
        let dec = zcrypto_decrypt_ws_gcm(&key, &nonce_bytes, &ct).expect("gcm decrypt ok");
        assert_eq!(dec, plain);
    }

    #[test]
    fn multi_host_failover_and_dual_account_isolation() {
        let chat_hosts = service_failover_hosts("chat");
        assert!(chat_hosts.len() >= 3);
        let group_hosts = service_failover_hosts("group");
        assert!(group_hosts.len() >= 2);

        let state = default_seed_state();
        let s0 = state.slots.get(&0).unwrap();
        let s1 = state.slots.get(&1).unwrap();
        assert_ne!(s0.profile.uid, s1.profile.uid);
        assert!(s1.threads.iter().any(|t| t.id == "uid_90001_khachhang_vip"));
        assert!(!s0.threads.iter().any(|t| t.id == "uid_90001_khachhang_vip"));
    }
}
