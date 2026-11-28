use eframe::egui::Vec2;
use serde_json::json;
use zalo_rust::chat::{
    apply_vanish_expiry, photo_size_from_content, video_size_from_content, ChatMessage,
    VANISH_EXPIRED_TEXT,
};
use zalo_rust::gui::{
    compute_video_thumb_size, cover_uv_for, image_file_preview_size, media_source_size,
    parse_inline_md_runs, parse_inline_md_runs_for_message, photo_display_size, InlineMdRun,
};
use zalo_rust::store::{MessageStore, StoredMessage, StoredThread, VANISH_TOMBSTONE_JSON};

fn photo_msg(id: &str, media_size: Option<[u32; 2]>) -> ChatMessage {
    ChatMessage {
        id: id.into(),
        sender: "Alice".into(),
        text: "[Hình ảnh] https://example.com/photo.jpg".into(),
        ts: 1_795_000_000_000,
        media_size,
        ..Default::default()
    }
}

#[test]
fn pr192_content_dimensions_parse_photo_and_video_sizes() {
    assert_eq!(
        photo_size_from_content(&json!({ "params": { "width": 1080, "height": 1920 } })),
        Some([1080, 1920])
    );
    assert_eq!(
        photo_size_from_content(&json!({ "params": r#"{"width":"800","height":600}"# })),
        Some([800, 600])
    );
    assert_eq!(
        photo_size_from_content(&json!({ "params": { "width": 0, "height": 600 } })),
        None
    );

    assert_eq!(
        video_size_from_content(&json!({ "params": { "video_width": "1920", "video_height": "1080" } })),
        Some([1920, 1080])
    );
    assert_eq!(
        video_size_from_content(&json!({ "params": r#"{"video_width":720,"video_height":1280}"# })),
        Some([720, 1280])
    );
    assert_eq!(
        video_size_from_content(&json!({ "params": { "video_width": 0, "video_height": 1080 } })),
        None
    );
}

#[test]
fn pr192_media_source_size_keeps_declared_and_rejects_mismatched_aspect() {
    let msg = photo_msg("m1", Some([800, 1600]));
    assert_eq!(media_source_size(&msg, None), Some(Vec2::new(800.0, 1600.0)));
    assert_eq!(
        media_source_size(&msg, Some([400, 800])),
        Some(Vec2::new(800.0, 1600.0))
    );
    // >3% aspect ratio mismatch falls back to actual decoded texture dimensions.
    assert_eq!(
        media_source_size(&msg, Some([1600, 800])),
        Some(Vec2::new(1600.0, 800.0))
    );

    let undeclared = photo_msg("m2", None);
    assert_eq!(media_source_size(&undeclared, None), None);
    assert_eq!(
        media_source_size(&undeclared, Some([640, 480])),
        Some(Vec2::new(640.0, 480.0))
    );
}

#[test]
fn pr192_display_sizes_and_cover_uv_are_aspect_safe_and_narrow_clamped() {
    let tall = photo_display_size(Vec2::new(900.0, 1800.0), 300.0);
    assert!(tall.x <= 300.0 && tall.y <= 340.0);

    let (vw, vh) = compute_video_thumb_size(1920, 1080, 90.0);
    assert!(vw <= 90.0 && vh <= 200.0);

    let (iw, ih) = image_file_preview_size(400.0);
    assert!(iw > ih);

    // Wide 2:1 texture cropped into a 1:1 square trims horizontally.
    let uv_wide = cover_uv_for(200.0, 100.0, 1.0);
    assert!((uv_wide.min.x - 0.25).abs() < 1e-4 && (uv_wide.max.x - 0.75).abs() < 1e-4);
    assert_eq!(uv_wide.min.y, 0.0);
    assert_eq!(uv_wide.max.y, 1.0);

    // Zero or non-finite inputs fall back to full UV instead of NaN.
    let uv_zero = cover_uv_for(0.0, 100.0, 1.0);
    assert_eq!(uv_zero.min.x, 0.0);
    assert_eq!(uv_zero.max.x, 1.0);
}

#[test]
fn pr192_sqlite_roundtrip_media_size_and_vanish_tombstone() {
    let mut store = MessageStore::open(":memory:").expect("open in-memory store");
    store
        .upsert_thread(&StoredThread {
            thread_id: "t-media".into(),
            name: "Media Thread".into(),
            is_group: false,
            last_ts: 1_795_000_000_000,
            ..Default::default()
        })
        .expect("upsert thread");

    let mut m = photo_msg("m-persist", Some([1200, 900]));
    m.reactions.push("❤️".into());
    let mut stored = StoredMessage::from(&m);
    stored.thread_id = "t-media".into();
    assert!(stored.raw_json.contains("\"media_size\":[1200,900]"));
    store.upsert_message(&stored).expect("upsert photo");

    let rows = store.messages_for("t-media", 10).expect("load rows");
    assert_eq!(rows.len(), 1);
    assert!(rows[0].raw_json.contains("\"media_size\":[1200,900]"));

    assert!(apply_vanish_expiry(&mut m));
    assert_eq!(m.text, VANISH_EXPIRED_TEXT);
    assert_eq!(m.media_size, None);
    let mut vanished = StoredMessage::from(&m);
    vanished.thread_id = "t-media".into();
    assert_eq!(vanished.raw_json, "");

    // Overwriting an existing row that previously held metadata writes VANISH_TOMBSTONE_JSON.
    store.upsert_message(&vanished).expect("upsert tombstone");
    let rows_after = store.messages_for("t-media", 10).expect("reload rows");
    assert_eq!(rows_after[0].text, VANISH_EXPIRED_TEXT);
    assert_eq!(rows_after[0].raw_json, VANISH_TOMBSTONE_JSON);

    // Fresh vanish tombstone row directly inserted also receives VANISH_TOMBSTONE_JSON.
    let mut fresh_vanished = photo_msg("m-vanish-fresh", Some([600, 600]));
    assert!(apply_vanish_expiry(&mut fresh_vanished));
    let mut fresh_stored = StoredMessage::from(&fresh_vanished);
    fresh_stored.thread_id = "t-media".into();
    store
        .upsert_message(&fresh_stored)
        .expect("insert fresh vanish tombstone");
    let all = store.messages_for("t-media", 10).expect("reload all");
    let fresh_row = all.iter().find(|r| r.msg_id == "m-vanish-fresh").expect("fresh row");
    assert_eq!(fresh_row.raw_json, VANISH_TOMBSTONE_JSON);
}

#[test]
fn pr174_exact_mention_uses_utf16_wire_offsets_and_word_boundary() {
    let text = "😀 @An đi ăn cơm chưa";
    // "😀 " is 2 UTF-16 code units + 1 space = offset 3; "@An" length = 3 UTF-16 units.
    let mut msg = ChatMessage {
        id: "m-mention-live".into(),
        text: text.into(),
        ..Default::default()
    };
    msg.wire_ref.fields.insert(
        "mentions".into(),
        json!([{"pos": 3, "len": 3, "uid": "u1"}]),
    );
    let runs = parse_inline_md_runs_for_message(&msg, text, false);
    assert_eq!(
        runs,
        vec![
            InlineMdRun::Plain("😀 ".into()),
            InlineMdRun::Mention("@An".into()),
            InlineMdRun::Plain(" đi ăn cơm chưa".into()),
        ]
    );

    // Must not match "@An" as a prefix of "@Anh".
    let mut msg_prefix = ChatMessage {
        id: "m-mention-boundary".into(),
        text: "@Anh ơi".into(),
        ..Default::default()
    };
    msg_prefix.wire_ref.fields.insert(
        "mentions".into(),
        json!([{"pos": 0, "len": 3}]),
    );
    let runs_prefix = parse_inline_md_runs_for_message(&msg_prefix, "@Anh ơi", false);
    // UTF-16 slice [0..3] on "@Anh ơi" is "@An", but after[3..] starts with 'h' (alnum),
    // so word-boundary guard prevents partial highlight of "@Anh" and max_words=0 drops guess.
    assert_eq!(runs_prefix, vec![InlineMdRun::Plain("@Anh ơi".into())]);
}

#[test]
fn pr174_mention_fallback_keeps_5_word_vietnamese_names_without_wire_data() {
    let runs = parse_inline_md_runs("Chào @Nguyễn Văn An nhé", false);
    assert_eq!(
        runs,
        vec![
            InlineMdRun::Plain("Chào ".into()),
            InlineMdRun::Mention("@Nguyễn Văn An".into()),
            InlineMdRun::Plain(" nhé".into()),
        ]
    );
}

#[cfg(windows)]
#[test]
fn pr213_tray_masking_and_windows_close_action() {
    use zalo_rust::gui::{tray_notification_body, windows_close_action, WindowsCloseAction};

    assert_eq!(
        tray_notification_body(true, "Nội dung bí mật"),
        "Bạn có thông báo mới"
    );
    assert_eq!(
        tray_notification_body(false, "Nội dung bình thường"),
        "Nội dung bình thường"
    );
    assert_eq!(
        windows_close_action(true, true, false),
        WindowsCloseAction::Quit
    );
    assert_eq!(
        windows_close_action(false, false, true),
        WindowsCloseAction::Quit
    );
    assert_eq!(
        windows_close_action(false, true, true),
        WindowsCloseAction::Minimize
    );
    assert_eq!(
        windows_close_action(false, true, false),
        WindowsCloseAction::Park
    );
}

#[cfg(windows)]
#[test]
fn pr213_settings_flush_persists_latest_snapshot_without_session_runtime() {
    use zalo_rust::gui::{write_app_settings, GuiEvent, GuiTx, SettingsRequest};

    let path = std::env::temp_dir().join(format!(
        "ctdozlite-settings-writer-{}.json",
        std::process::id(),
    ));
    let (requests_tx, requests_rx) = std::sync::mpsc::channel();
    let (events_tx, events_rx) = std::sync::mpsc::channel();
    assert!(requests_tx.send(SettingsRequest::Save("{}".into())).is_ok());
    let final_snapshot = r#"{"close_to_tray":true}"#;
    assert!(requests_tx
        .send(SettingsRequest::Save(final_snapshot.into()))
        .is_ok());
    assert!(requests_tx.send(SettingsRequest::Flush).is_ok());
    write_app_settings(
        requests_rx,
        GuiTx::new(events_tx, eframe::egui::Context::default()),
        &path,
    );
    let saved = std::fs::read_to_string(&path);
    assert!(std::fs::remove_file(path).is_ok());
    assert_eq!(saved.as_deref().ok(), Some(final_snapshot));
    assert!(matches!(
        events_rx.try_recv(),
        Ok(GuiEvent::SettingsFlushed)
    ));
    assert!(requests_tx.send(SettingsRequest::Save("{}".into())).is_err());
}

#[cfg(windows)]
#[test]
fn pr213_save_app_settings_persists_before_writer_start_and_is_idempotent_across_restarts() {
    use zalo_rust::gui::{
        flush_app_settings_with, save_app_settings_with, start_settings_writer_with, GuiEvent,
        GuiTx, SettingsRequest,
    };

    let path = std::env::temp_dir().join(format!(
        "ctdozlite-settings-prelogin-{}.json",
        std::process::id(),
    ));
    let _ = std::fs::remove_file(&path);
    let slot: std::sync::OnceLock<std::sync::mpsc::Sender<SettingsRequest>> =
        std::sync::OnceLock::new();

    let pre_login_snapshot = r#"{"close_to_tray":true,"stage":"pre_login"}"#;
    save_app_settings_with(&slot, pre_login_snapshot.into(), &path);
    assert_eq!(
        std::fs::read_to_string(&path).as_deref().ok(),
        Some(pre_login_snapshot)
    );

    let (events_tx, events_rx) = std::sync::mpsc::channel();
    let gui_tx = GuiTx::new(events_tx, eframe::egui::Context::default());
    start_settings_writer_with(&slot, &gui_tx, path.clone());
    let Some(first_sender) = slot.get().cloned() else {
        panic!("writer sender initialized");
    };
    start_settings_writer_with(&slot, &gui_tx, path.clone());
    let Some(second_sender) = slot.get().cloned() else {
        panic!("writer sender kept");
    };
    assert!(first_sender
        .send(SettingsRequest::Save(r#"{"stage":"active"}"#.into()))
        .is_ok());

    let queued_snapshot = r#"{"close_to_tray":false,"stage":"relogin"}"#;
    save_app_settings_with(&slot, queued_snapshot.into(), &path);
    assert!(flush_app_settings_with(&slot));
    assert!(matches!(
        events_rx.recv_timeout(std::time::Duration::from_secs(2)),
        Ok(GuiEvent::SettingsFlushed)
    ));
    assert_eq!(
        std::fs::read_to_string(&path).as_deref().ok(),
        Some(queued_snapshot)
    );
    assert!(second_sender.send(SettingsRequest::Flush).is_err());

    let post_flush_snapshot = r#"{"close_to_tray":true,"stage":"post_flush"}"#;
    save_app_settings_with(&slot, post_flush_snapshot.into(), &path);
    assert_eq!(
        std::fs::read_to_string(&path).as_deref().ok(),
        Some(post_flush_snapshot)
    );
    let _ = std::fs::remove_file(&path);
}
