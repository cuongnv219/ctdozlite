//! Deterministic property-based fuzz tests for core UI layout, album grouping,
//! settings sanitization, and QR protocol parsers.

use crate::chat::{
    apply_recall_merge, build_native_video_forward_params, build_recall_payload,
    build_set_mute_payload, probe_mp4_metadata, ChatMessage, MediaGroupLayout, MUTE_ACTION_SET,
    MUTE_ACTION_UNSET,
};
use crate::gui::attachments::is_video_path;
use crate::gui::lucide;
use crate::gui::messages::{album_run_lengths, compute_video_thumb_size};
use crate::gui::workspace::{
    compute_chat_header_actions_width_exact, compute_thread_row_reserved_right, format_sidebar_time,
};
use crate::gui::{AppSettings, IncomingPopupState};
use crate::qr::parse_rate_limit_wait_secs;

struct Xorshift64(u64);

impl Xorshift64 {
    fn new(seed: u64) -> Self {
        Self(if seed == 0 { 0x9E37_79B9_7F4A_7C15 } else { seed })
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    fn next_f32_adversarial(&mut self) -> f32 {
        match self.next_u64() % 16 {
            0 => 0.0,
            1 => -0.0,
            2 => -150.0,
            3 => 1.0,
            4 => 119.9,
            5 => 140.0,
            6 => 10_000.0,
            7 => f32::NAN,
            8 => f32::INFINITY,
            9 => f32::NEG_INFINITY,
            _ => ((self.next_u64() % 2000) as f32) * 0.5 - 50.0,
        }
    }

    fn next_usize_adversarial(&mut self) -> usize {
        match self.next_u64() % 10 {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 320,
            4 => 1920,
            5 => u32::MAX as usize,
            _ => (self.next_u64() % 4096) as usize,
        }
    }
}

#[test]
fn prop_video_thumb_size_never_panics_and_fits_bounds() {
    let mut rng = Xorshift64::new(0xC7D0_0001);
    for _ in 0..2000 {
        let tw = rng.next_usize_adversarial();
        let th = rng.next_usize_adversarial();
        let max_w = rng.next_f32_adversarial();
        let (w, h) = compute_video_thumb_size(tw, th, max_w);
        assert!(w.is_finite() && w >= 0.0, "invalid w={w} for ({tw},{th},{max_w})");
        assert!(h.is_finite() && h >= 0.0, "invalid h={h} for ({tw},{th},{max_w})");
    }
}

#[test]
fn prop_header_actions_width_monotonic_and_finite() {
    for is_group in [false, true] {
        for is_narrow in [false, true] {
            let no_calls = compute_chat_header_actions_width_exact(is_group, is_narrow, false);
            let with_calls = compute_chat_header_actions_width_exact(is_group, is_narrow, true);
            assert!(no_calls.is_finite() && no_calls > 0.0);
            assert!(with_calls.is_finite() && with_calls > no_calls);
        }
        let narrow = compute_chat_header_actions_width_exact(is_group, true, true);
        let wide = compute_chat_header_actions_width_exact(is_group, false, true);
        assert!(wide > narrow);
    }
}

#[test]
fn prop_album_run_lengths_partitions_all_messages() {
    let mut rng = Xorshift64::new(0xC7D0_0002);
    let senders = ["Hoang", "Đỗ Thành", "Lan", "Minh"];
    let group_ids = ["grp_a", "grp_b", "grp_c"];

    for _ in 0..500 {
        let len = (rng.next_u64() % 28) as usize;
        let mut msgs = Vec::with_capacity(len);
        let mut ts: i64 = 1_795_000_000_000;

        for idx in 0..len {
            let is_photo = (rng.next_u64() % 5) != 0;
            let recalled = (rng.next_u64() % 12) == 0;
            let is_outgoing = (rng.next_u64() % 2) == 0;
            let sender = senders[(rng.next_u64() as usize) % senders.len()].to_string();
            ts += match rng.next_u64() % 4 {
                0 => 1_000,
                1 => 8_000,
                2 => 30_000,
                _ => 0,
            };
            let media_group = match rng.next_u64() % 4 {
                0 => Some(MediaGroupLayout::new(
                    group_ids[(rng.next_u64() as usize) % group_ids.len()],
                    idx as u32,
                    4,
                )),
                _ => None,
            };

            msgs.push(ChatMessage {
                id: format!("m_{idx}"),
                sender,
                text: if recalled {
                    "[Tin nhắn đã thu hồi]".to_string()
                } else if is_photo {
                    "[Hình ảnh]".to_string()
                } else {
                    "tin nhắn thường".to_string()
                },
                is_outgoing,
                ts,
                link_src: is_photo.then(|| "Hình ảnh Zalo".to_string()),
                link_url: is_photo.then(|| format!("https://b-f10-zpg-r.zdn.vn/photo_{idx}.jpg")),
                media_group,
                ..Default::default()
            });
        }

        let refs: Vec<&ChatMessage> = msgs.iter().collect();
        let runs = album_run_lengths(&refs);
        assert_eq!(runs.len(), msgs.len());
        assert_eq!(runs.iter().sum::<usize>(), msgs.len());

        let mut i = 0;
        while i < runs.len() {
            let r = runs[i];
            assert!(r >= 1);
            for slot in &runs[i + 1..i + r] {
                assert_eq!(*slot, 0);
            }
            if r >= 2 && msgs[i].media_group.is_none() {
                assert!(r <= 9, "burst album without media_group exceeded cap: {r}");
            }
            i += r;
        }
    }

    let drift_outgoing = vec![
        ChatMessage {
            id: "d1".into(),
            sender: "Hoang".into(),
            text: "[Hình ảnh]".into(),
            is_outgoing: true,
            ts: 1000,
            link_src: Some("Hình ảnh Zalo".into()),
            media_group: Some(MediaGroupLayout::new("shared_123", 0, 2)),
            ..Default::default()
        },
        ChatMessage {
            id: "d2".into(),
            sender: "Đỗ Thành".into(),
            text: "[Hình ảnh]".into(),
            is_outgoing: true,
            ts: 1001,
            link_src: Some("Hình ảnh Zalo".into()),
            media_group: Some(MediaGroupLayout::new("shared_123", 1, 2)),
            ..Default::default()
        },
    ];
    let drift_refs: Vec<&ChatMessage> = drift_outgoing.iter().collect();
    assert_eq!(album_run_lengths(&drift_refs), vec![2, 0]);
}

#[test]
fn prop_app_settings_clamp_bounds_always_valid() {
    let mut rng = Xorshift64::new(0xC7D0_0003);
    for _ in 0..500 {
        let mut s = AppSettings::default();
        s.font_scale = rng.next_f32_adversarial();
        s.animation_speed = rng.next_f32_adversarial();
        s.scroll_speed = rng.next_f32_adversarial();

        let fs = s.normalized_font_scale();
        let an = s.normalized_animation_speed();
        let sc = s.normalized_scroll_speed();

        assert!(fs.is_finite() && (0.5..=3.0).contains(&fs), "font_scale={fs}");
        assert!(an.is_finite() && (0.5..=2.0).contains(&an), "animation_speed={an}");
        assert!(sc.is_finite() && (0.5..=4.0).contains(&sc), "scroll_speed={sc}");
    }
}

#[test]
fn prop_qr_rate_limit_parser_never_panics() {
    let mut rng = Xorshift64::new(0xC7D0_0004);
    let fragments = [
        "",
        "(",
        "(chờ ",
        "(chờ 0s)",
        "(chờ -5s)",
        "(chờ 99999999999999999999999999s)",
        "(chờ 42s)",
        "s)",
        "😀(chờ 15s)🔥",
        "18012",
    ];
    for _ in 0..500 {
        let a = fragments[(rng.next_u64() as usize) % fragments.len()];
        let b = fragments[(rng.next_u64() as usize) % fragments.len()];
        let sample = format!("{a}{b}");
        let parsed = parse_rate_limit_wait_secs(&sample);
        if let Some(secs) = parsed {
            assert!(secs > 0);
        }
    }
}

#[test]
fn test_set_mute_payload_uses_zalo_action_1_and_3() {
    assert_eq!(MUTE_ACTION_SET, 1);
    assert_eq!(MUTE_ACTION_UNSET, 3);

    let muted_dm = build_set_mute_payload("7602591761395115371", false, 3600, true, 1_795_365_460, "pc-imei");
    assert_eq!(muted_dm["toid"], "7602591761395115371");
    assert_eq!(muted_dm["action"], 1);
    assert_eq!(muted_dm["duration"], 3600);
    assert_eq!(muted_dm["muteType"], 1);

    let unmuted_dm = build_set_mute_payload("7602591761395115371", false, 0, false, 1_795_365_600, "pc-imei");
    assert_eq!(unmuted_dm["toid"], "7602591761395115371");
    assert_eq!(unmuted_dm["action"], 3);
    assert_ne!(unmuted_dm["action"], 2);
    assert_eq!(unmuted_dm["duration"], -1);
    assert_eq!(unmuted_dm["muteType"], 1);

    let unmuted_grp = build_set_mute_payload("group_3247153953139077370", true, -1, false, 1_795_365_600, "pc-imei");
    assert_eq!(unmuted_grp["toid"], "3247153953139077370");
    assert_eq!(unmuted_grp["action"], 3);
    assert_eq!(unmuted_grp["duration"], -1);
    assert_eq!(unmuted_grp["muteType"], 2);
}

#[test]
fn test_incoming_popup_state_unread_floor_and_draft_reset() {
    let p = IncomingPopupState::new(
        "7602591761395115371".into(),
        "Đtc".into(),
        "U".into(),
        None,
        false,
        0,
    );
    assert_eq!(p.unread_total, 1);
    assert!(p.reply_draft.is_empty());
}

#[test]
fn prop_thread_row_reserved_right_prevents_time_and_icon_overlap() {
    for &tw in &[-5.0_f32, 0.0, 28.0, 42.0, f32::NAN] {
        let base = compute_thread_row_reserved_right(tw, false, false);
        let pinned = compute_thread_row_reserved_right(tw, true, false);
        let muted = compute_thread_row_reserved_right(tw, false, true);
        let both = compute_thread_row_reserved_right(tw, true, true);
        assert!(base.is_finite() && base >= 14.0);
        if tw.is_finite() && tw > 0.0 {
            assert!(base > tw + 9.0);
            assert!(pinned > base);
            assert!(muted > base);
            assert!(both > pinned && both > muted);
        }
    }
}

#[test]
fn test_format_sidebar_time_today_same_year_and_older() {
    use chrono::TimeZone;
    let now_local = chrono::Local
        .with_ymd_and_hms(2026, 11, 22, 14, 30, 0)
        .single()
        .expect("valid local noon timestamp");
    let now_secs = now_local.timestamp();

    assert_eq!(format_sidebar_time(0, now_secs, "fallback"), "fallback");
    assert_eq!(format_sidebar_time(0, now_secs, "14:40:27"), "14:40");
    assert_eq!(format_sidebar_time(0, now_secs, "22/11"), "22/11");

    let today_secs = chrono::Local
        .with_ymd_and_hms(2026, 11, 22, 9, 5, 0)
        .single()
        .expect("valid today timestamp")
        .timestamp();
    assert_eq!(format_sidebar_time(today_secs, now_secs, "fb"), "09:05");
    assert_eq!(
        format_sidebar_time(today_secs * 1000, now_secs, "fb"),
        "09:05"
    );

    let earlier_same_year_secs = chrono::Local
        .with_ymd_and_hms(2026, 11, 15, 7, 10, 0)
        .single()
        .expect("valid same-year timestamp")
        .timestamp();
    assert_eq!(
        format_sidebar_time(earlier_same_year_secs * 1000, now_secs, "fb"),
        "15/11"
    );

    let older_year_secs = chrono::Local
        .with_ymd_and_hms(2025, 10, 18, 20, 0, 0)
        .single()
        .expect("valid older-year timestamp")
        .timestamp();
    assert_eq!(
        format_sidebar_time(older_year_secs * 1000, now_secs, "fb"),
        "18/10/25"
    );
}

#[test]
fn test_probe_mp4_metadata_extracts_duration_and_dimensions() {
    fn mk_box(tag: &[u8; 4], body: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(8 + body.len());
        out.extend_from_slice(&((8 + body.len()) as u32).to_be_bytes());
        out.extend_from_slice(tag);
        out.extend_from_slice(body);
        out
    }
    let mut mvhd_body = vec![0u8; 100];
    mvhd_body[12..16].copy_from_slice(&1000u32.to_be_bytes());
    mvhd_body[16..20].copy_from_slice(&5500u32.to_be_bytes());
    let mut tkhd_body = vec![0u8; 84];
    tkhd_body[76..80].copy_from_slice(&(1920u32 << 16).to_be_bytes());
    tkhd_body[80..84].copy_from_slice(&(1080u32 << 16).to_be_bytes());

    let mut moov_body = mk_box(b"mvhd", &mvhd_body);
    let tkhd_box = mk_box(b"tkhd", &tkhd_body);
    moov_body.extend_from_slice(&mk_box(b"trak", &tkhd_box));
    let mp4 = mk_box(b"moov", &moov_body);

    assert_eq!(probe_mp4_metadata(&mp4), Some((5500, 1920, 1080)));
    assert_eq!(probe_mp4_metadata(&[]), None);
}

#[test]
fn test_build_native_video_forward_params_dm_and_group() {
    let dm = build_native_video_forward_params(
        "7602591761395115371",
        false,
        1_795_375_000,
        "pc-imei",
        "https://f10.zdn.vn/vid.mp4",
        "https://f10.zdn.vn/thumb.jpg",
        5500,
        1920,
        1080,
        12_345,
        "demo video",
    );
    assert_eq!(dm["toId"], "7602591761395115371");
    assert_eq!(dm["msgType"], 5);
    assert_eq!(dm["zsource"], 704);
    let info: serde_json::Value =
        serde_json::from_str(dm["msgInfo"].as_str().expect("msgInfo JSON")).expect("valid msgInfo");
    assert_eq!(info["videoUrl"], "https://f10.zdn.vn/vid.mp4");
    assert_eq!(info["thumbUrl"], "https://f10.zdn.vn/thumb.jpg");
    assert_eq!(info["duration"], 5500);
    assert_eq!(info["width"], 1920);
    assert_eq!(info["height"], 1080);
    assert_eq!(info["fileSize"], 12_345);
    assert_eq!(info["properties"]["type"], 1003);
    assert_eq!(info["title"], "demo video");

    let grp = build_native_video_forward_params(
        "3247153953139077370",
        true,
        1_795_375_001,
        "pc-imei",
        "https://f10.zdn.vn/vid.mp4",
        "",
        0,
        1280,
        720,
        999,
        "",
    );
    assert_eq!(grp["grid"], "3247153953139077370");
    assert_eq!(grp["visibility"], 0);
    assert_eq!(grp["msgType"], 5);
}

#[test]
fn test_is_video_path_extensions() {
    use std::path::Path;
    for ext in ["mp4", "MOV", "mkv", "webm", "avi", "m4v"] {
        assert!(is_video_path(Path::new(&format!("clip.{ext}"))));
    }
    for ext in ["png", "jpg", "pdf", "zip"] {
        assert!(!is_video_path(Path::new(&format!("doc.{ext}"))));
    }
}

#[test]
fn test_build_recall_payload_prefers_client_message_id_over_ts() {
    let mut dm_msg = ChatMessage {
        id: "8518773491790".into(),
        client_message_id: "1795376379001".into(),
        ts: 1_795_376_380_456,
        is_outgoing: true,
        ..Default::default()
    };
    let dm = build_recall_payload("7602591761395115371", false, &dm_msg, 1_795_400_000_000, "pc-imei")
        .expect("valid dm recall payload");
    assert_eq!(dm["msgId"], "8518773491790");
    assert_eq!(dm["toid"], "7602591761395115371");
    assert_eq!(dm["cliMsgIdUndo"], "1795376379001");
    assert_eq!(dm["imei"], "pc-imei");

    dm_msg.client_message_id.clear();
    dm_msg.history_source = Some(serde_json::json!({ "cliMsgId": "1795376379555" }));
    let grp = build_recall_payload("group_3247153953139077370", true, &dm_msg, 1_795_400_000_000, "pc-imei")
        .expect("valid group recall payload");
    assert_eq!(grp["grid"], "3247153953139077370");
    assert_eq!(grp["visibility"], 0);
    assert_eq!(grp["cliMsgIdUndo"], "1795376379555");

    let local_msg = ChatMessage {
        id: "out_12345".into(),
        ..Default::default()
    };
    assert!(build_recall_payload("7602591761395115371", false, &local_msg, 1_795_400_000_000, "pc-imei").is_err());
}

#[test]
fn test_lucide_shield_check_codepoint_matches_ttf() {
    assert_eq!(lucide::SHIELD_CHECK, "\u{e1ff}");
}

#[test]
fn test_outgoing_recall_merge_clears_media_and_avoids_anti_recall() {
    let mut outgoing = ChatMessage {
        id: "m_out".into(),
        text: "[Hình ảnh]".into(),
        is_outgoing: true,
        link_url: Some("https://cdn.example/photo.jpg".into()),
        link_thumb: Some("https://cdn.example/thumb.jpg".into()),
        ..Default::default()
    };
    apply_recall_merge(&mut outgoing, "[Tin nhắn đã thu hồi]");
    assert_eq!(outgoing.text, "[Tin nhắn đã thu hồi]");
    assert!(outgoing.link_url.is_none());
    assert!(outgoing.link_thumb.is_none());

    let mut incoming = ChatMessage {
        id: "m_in".into(),
        text: "Bí mật".into(),
        is_outgoing: false,
        link_url: Some("https://cdn.example/photo.jpg".into()),
        ..Default::default()
    };
    apply_recall_merge(&mut incoming, "[Tin nhắn đã thu hồi]");
    assert_eq!(incoming.text, "[Đã thu hồi] Bí mật");
    assert_eq!(
        incoming.link_url.as_deref(),
        Some("https://cdn.example/photo.jpg")
    );
}

#[test]
fn test_target_decode_max_dim_bounds_avatars_and_hd_photos() {
    assert_eq!(crate::gui::worker::target_decode_max_dim("avt_abc123"), 128);
    assert_eq!(crate::gui::worker::target_decode_max_dim("ava_abc123"), 128);
    assert_eq!(crate::gui::worker::target_decode_max_dim("hd_abc123"), 2048);
    assert_eq!(crate::gui::worker::target_decode_max_dim("thumb_abc123"), 720);
    assert_eq!(crate::gui::worker::target_decode_max_dim("stk_123_456"), 720);
}
