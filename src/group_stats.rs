//! Thống kê nhóm cho Group Workspace: tính từ DB (SQLite) + RAM (thread đang mở).
//!
//! Rẻ: 1 query COUNT + GROUP BY cho DB, còn lại duyệt RAM. Không đoán —
//! thiếu dữ liệu thì số 0 / chuỗi rỗng.

use std::collections::HashMap;

use crate::chat::Thread;
use crate::message_model::MessageKind;
use crate::store::MessageStore;

/// Thống kê 1 thread nhóm.
#[derive(Debug, Clone, Default)]
pub struct GroupStats {
    /// Tổng số tin (DB + RAM, khử trùng theo id).
    pub total: usize,
    /// Số tin 7 ngày gần nhất.
    pub last_7d: usize,
    /// Phân phối 7 ngày (cũ → mới, 7 bucket).
    pub daily: [usize; 7],
    /// Top 5 người gửi (tên, số tin).
    pub top_senders: Vec<(String, usize)>,
    /// Đếm media: ảnh, file, link, voice.
    pub photos: usize,
    pub files: usize,
    pub links: usize,
    pub voices: usize,
    /// Giờ sôi động nhất 0–23h (giờ địa phương). `None` khi chưa có tin nào.
    pub peak_hour: Option<u8>,
    /// Số tin trong giờ sôi động nhất.
    pub peak_hour_count: usize,
}

fn classify(kind: MessageKind, link_src: Option<&str>, text: &str) -> &'static str {
    match kind {
        MessageKind::Photo => "photo",
        MessageKind::Video => "photo",
        MessageKind::File => "file",
        MessageKind::Voice => "voice",
        MessageKind::Link | MessageKind::WebContent => "link",
        _ => {
            let src = link_src.unwrap_or("");
            let t = text.trim();
            if src == "Hình ảnh Zalo"
                || t == "[Hình ảnh]"
                || t.starts_with("[Hình ảnh] ")
            {
                "photo"
            } else if src == "Tập tin Zalo" || t.starts_with("[Tập tin]") {
                "file"
            } else if src == "Thoại Zalo"
                || t == "[Tin nhắn thoại]"
                || t.starts_with("[Tin nhắn thoại]")
            {
                "voice"
            } else if t.starts_with("http://") || t.starts_with("https://") {
                "link"
            } else {
                "text"
            }
        }
    }
}

fn hour_of(ts_ms: i64) -> Option<u8> {
    crate::clock::local_datetime(ts_ms).map(|d| {
        use chrono::Timelike;
        d.hour() as u8
    })
}

/// Bucket 1 tin vào 1 trong 7 ngày địa phương gần nhất (0 = 6 ngày trước, 6 = hôm nay).
/// So sánh theo ngày lịch địa phương, không theo 24h rolling — DST không làm mất ngày.
/// Tin tương lai (> hôm nay) và tin quá 6 ngày trước → `None`.
fn day_bucket(ts_ms: i64, today_day: i64) -> Option<usize> {
    let day = crate::clock::local_day(ts_ms)?;
    if day > today_day || today_day - day > 6 {
        return None;
    }
    Some((6 - (today_day - day)) as usize)
}

/// 1 bản ghi nhẹ đã gộp DB+RAM: (identity, tên hiển thị, text, ts_ms, media label).
/// identity = sender uid khi có (DB `sender_id` / RAM `sidebar_sender_id`),
/// fallback tên hiển thị — 2 người trùng tên không gộp nhầm.
type LiteMsg = (String, String, String, i64, &'static str);

fn text_hash(text: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    text.hash(&mut h);
    h.finish()
}

/// Gộp DB (toàn bộ lịch sử đã lưu) + RAM (tin chưa persist) theo id.
/// DB trước, RAM đè (RAM mới hơn / đủ kind). Tin id rỗng vẫn giữ riêng lẻ
/// bằng key `(ts, hash text)` thay vì ghi đè nhau.
fn merge_lite(
    store: Option<&MessageStore>,
    thread: Option<&Thread>,
) -> Vec<LiteMsg> {
    let mut by_id: HashMap<String, LiteMsg> = HashMap::new();
    if let (Some(s), Some(t)) = (store, thread) {
        if let Ok(rows) = s.messages_for_all(&t.id) {
            for r in rows {
                let kind = MessageKind::from_wire_name(&r.msg_type);
                let link_src: Option<String> =
                    serde_json::from_str::<serde_json::Value>(&r.raw_json)
                        .ok()
                        .and_then(|v| {
                            v.get("link_src")
                                .and_then(|x| x.as_str())
                                .map(|x| x.to_string())
                        });
                // raw_json giữ link_src đã persist (ảnh/file/voice) để classify đúng.
                let label = classify(kind, link_src.as_deref(), &r.text);
                let ident = if r.sender_id.trim().is_empty() {
                    r.sender_name.clone()
                } else {
                    r.sender_id.clone()
                };
                if r.msg_id.is_empty() {
                    by_id.insert(
                        format!("db_noid_{}_{}", r.ts, text_hash(&r.text)),
                        (ident, r.sender_name.clone(), r.text.clone(), r.ts, label),
                    );
                } else {
                    by_id
                        .entry(r.msg_id.clone())
                        .or_insert((ident, r.sender_name.clone(), r.text.clone(), r.ts, ""));
                    if let Some(e) = by_id.get_mut(&r.msg_id) {
                        e.4 = label;
                    }
                }
            }
        }
    }
    if let Some(t) = thread {
        for m in &t.messages {
            let label = classify(m.kind, m.link_src.as_deref(), &m.text);
            let ident = crate::chat::sidebar_sender_id(m).unwrap_or_else(|| m.sender.clone());
            if m.id.is_empty() {
                by_id.insert(
                    format!("ram_noid_{}_{}_{}", t.id, m.ts, text_hash(&m.text)),
                    (ident, m.sender.clone(), m.text.clone(), m.ts, label),
                );
            } else {
                by_id.insert(
                    m.id.clone(),
                    (ident, m.sender.clone(), m.text.clone(), m.ts, label),
                );
            }
        }
    }
    by_id.into_values().collect()
}

/// Tính thống kê từ DB (toàn bộ lịch sử đã lưu) + RAM (tin chưa persist).
/// DB thiếu (mở lỗi) → chỉ dùng RAM; RAM thiếu → chỉ dùng DB.
pub fn compute_group_stats(store: Option<&MessageStore>, thread: Option<&Thread>) -> GroupStats {
    let now_ms = crate::clock::normalize_ts_ms(chrono::Local::now().timestamp_millis());
    let today_day = crate::clock::local_day(now_ms).unwrap_or(0);
    let week_ago = now_ms - 7 * 86_400_000;

    let mut st = GroupStats::default();
    let msgs = merge_lite(store, thread);
    st.total = msgs.len();
    // Hiển thị theo identity (uid), tên chỉ để trình bày — trùng tên không gộp.
    let mut sender_counts: HashMap<String, (String, usize)> = HashMap::new();
    let mut hour_counts = [0usize; 24];

    for (ident, sender, _text, ts, label) in &msgs {
        let ts_ms = crate::clock::normalize_ts_ms(*ts);
        match *label {
            "photo" => st.photos += 1,
            "file" => st.files += 1,
            "link" => st.links += 1,
            "voice" => st.voices += 1,
            _ => {}
        }
        let key = if ident.trim().is_empty() {
            format!("name:{}", sender)
        } else {
            format!("id:{ident}")
        };
        let disp = if sender.trim().is_empty() {
            "Ẩn danh".to_string()
        } else {
            sender.clone()
        };
        sender_counts
            .entry(key)
            .and_modify(|e| e.1 += 1)
            .or_insert((disp, 1));
        // last_7d chặn trên now_ms: tin tương lai không lọt vào rolling count
        // (daily bucket đã loại tin tương lai riêng).
        if ts_ms >= week_ago && ts_ms <= now_ms {
            st.last_7d += 1;
        }
        if let Some(b) = day_bucket(ts_ms, today_day) {
            st.daily[b] += 1;
        }
        if let Some(h) = hour_of(ts_ms) {
            hour_counts[h as usize] += 1;
        }
    }

    let mut senders: Vec<(String, usize)> = sender_counts.into_values().collect();
    senders.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    senders.truncate(5);
    st.top_senders = senders;

    let (peak_h, peak_c) = hour_counts
        .iter()
        .enumerate()
        .max_by_key(|(_, c)| **c)
        .map(|(h, c)| (h as u8, *c))
        .unwrap_or((0, 0));
    if peak_c > 0 {
        st.peak_hour = Some(peak_h);
        st.peak_hour_count = peak_c;
    }
    st
}

/// Tóm tắt 1 dòng cho LLM + UI ("120 tin · 7 ngày: 34 · top: An (40) · ảnh 5 …").
pub fn summarize_thread(thread: Option<&Thread>, store: &MessageStore) -> String {
    let st = compute_group_stats(Some(store), thread);
    let top = st
        .top_senders
        .iter()
        .map(|(n, c)| format!("{n} ({c})"))
        .collect::<Vec<_>>()
        .join(", ");
    let peak = match st.peak_hour {
        Some(h) => format!("{h:02}h ({} tin)", st.peak_hour_count),
        None => "—".to_string(),
    };
    format!(
        "{} tin · 7 ngày: {} · top: {} · ảnh {} · file {} · link {} · voice {} · giờ sôi động: {}",
        st.total,
        st.last_7d,
        if top.is_empty() { "—".into() } else { top },
        st.photos,
        st.files,
        st.links,
        st.voices,
        peak,
    )
}
