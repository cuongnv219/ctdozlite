# Kiến trúc hệ thống ctdozlite

Tài liệu mô tả cách phân tách luồng xử lý, 13 module giao diện `src/gui/`, cơ chế lưu trữ 3 tầng và các cổng chất lượng tự động của `ctdozlite` (`v0.1.4`).

---

## 1. Sơ đồ tổng thể

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│                              TẦNG GIAO DIỆN (UI)                             │
│                                                                              │
│  ┌────────────────────────────────────┐  ┌────────────────────────────────┐  │
│  │ GUI (src/gui.rs + 13 src/gui/*)    │  │ TUI (src/tui.rs)               │  │
│  │ • eframe / egui + ResourceLru 64MB │  │ • ratatui / crossterm          │  │
│  │ • App Lock PIN4 + Tray Win/Linux   │  │ • LLM Agent Loop (agent/llm)   │  │
│  └──────────────────┬─────────────────┘  └───────────────┬────────────────┘  │
└─────────────────────┼────────────────────────────────────┼───────────────────┘
                      │ WorkerCmd (mpsc)                   │
                      ▼                                    ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│                 LUỒNG NỀN DUY NHẤT (SHARED TOKIO RUNTIME)                    │
│                                                                              │
│  • WebSocket (`zpw_ws` + pre-auth buffer) & HTTP Poller (`get_last_msgs`)    │
│  • Dual History Sync: iOS V2 (Wasmtime `zproto_host`) ⇄ Android V1 fallback  │
│  • CallBridge (`src/call.rs`): VoIP ZRTP + RTP/RTCP H.264/Opus               │
│  • Automation: Local REST/MCP (`api_mcp`), Rhai (`thread_script`), Telegram  │
└─────────────────────┬────────────────────────────────────┬───────────────────┘
                      │                                    │
                      ▼                                    ▼
┌──────────────────────────────────────┐  ┌────────────────────────────────────┐
│     LƯU TRỮ & QUẢN LÝ BỘ NHỚ 3 TẦNG   │  │      MẬT MÃ, KHÓA & ĐA NỀN TẢNG    │
│                                      │  │                                    │
│ • threads_cache.json (snapshot UI)   │  │ • ParamCipher v2 / SignKey MD5     │
│ • accounts/<uin>_history.db (WAL)    │  │ • App Lock PIN4 (app_lock.rs)      │
│ • StorageManager (root/media/cache)  │  │ • single_instance (Unix) + os_notify│
└──────────────────────────────────────┘  └────────────────────────────────────┘
```

---

## 2. Tổ chức module GUI (`src/gui/`)

Phần giao diện đồ họa gồm `src/gui.rs` và **13 module con** phân tách rõ trách nhiệm:

- `src/gui.rs`: Khởi tạo `ZaloGuiApp`, vòng lặp sự kiện `GuiEvent`/`GuiTx`, thanh tiêu đề cửa sổ, đăng nhập và điều phối close-to-tray.
- `src/gui/workspace.rs`: Bố cục 3 cột gồm thanh điều hướng, danh sách hội thoại, vùng khung chat chính và ngăn thông tin chi tiết bên phải.
- `src/gui/messages.rs`: Render bong bóng tin nhắn, kích thước media giữ tỷ lệ (`photo_display_size`, `media_source_size`), nút thả tim nhanh `❤️`, highlight `@mention` theo UTF-16 `pos`/`len`, trích dẫn, reaction và menu chuột phải.
- `src/gui/modals.rs`: Các màn hình & modal phụ: Danh bạ, Nhật ký Social, Hẹn giờ, Cài đặt, Trình quản lý bộ nhớ 3 tầng, Quản lý tài khoản, Cuộc gọi, `ImageViewerModal` và Dev Inspector.
- `src/gui/worker.rs`: Định nghĩa `WorkerCmd`, luồng nền Tokio thống nhất (`spawn_session_worker`), ghi `app_settings.json` bất đồng bộ trên Windows và tải trước sticker/thumbnail.
- `src/gui/app_lock_ui.rs`: Màn hình khóa mã PIN 4 số, modal thiết lập nhanh PIN4, khóa nhanh `Ctrl+L` và theo dõi idle/sleep.
- `src/gui/stickers.rs`: Bảng chọn nhãn dán (`StickerPicker`), gợi ý nhãn dán theo thời gian thực và bộ lọc sticker.
- `src/gui/theme.rs`: Bảng màu `ThemePalette` (Dark/Light), bộ icon vector `lucide`, khử răng cưa avatar và nạp font tiếng Việt/CJK/`Open Sans`.
- `src/gui/attachments.rs`: Xử lý đính kèm nhiều tệp, dán ảnh clipboard (`CF_DIB` / `vendor/egui-winit`) và kéo thả tệp.
- `src/gui/poll_card.rs`: Thẻ hiển thị và tương tác bình chọn nhóm (Group Polls).
- `src/gui/windows_tray.rs`: Icon & menu khay hệ thống thuần Win32 (`windows-sys`), khôi phục `WINDOWPLACEMENT` khi ẩn/hiện cửa sổ.
- `src/gui/color_emoji.rs`: Giải mã và render bộ emoji màu Twemoji (`twemoji72.pack`).
- `src/gui/tray.rs`: Tích hợp khay hệ thống Linux qua giao thức D-Bus StatusNotifierItem (`ksni`).
- `src/gui/reactions_dialog.rs`: Hộp thoại chi tiết danh sách người thả cảm xúc.

---

## 3. Cơ chế đồng bộ, Bộ nhớ & Tích hợp hệ thống

### 3.1. Tách biệt luồng UI và Shared Tokio Runtime
- Luồng giao diện (UI thread) không bao giờ gọi trực tiếp hàm mạng chặn hoặc `std::thread::sleep`. Mọi lệnh được gửi qua `tokio::sync::mpsc::unbounded_channel` (`WorkerCmd`).
- Chế độ GUI sử dụng một Tokio runtime dùng chung (giảm số thread nền), xử lý WebSocket `zpw_ws` (kèm bộ đệm gói trước khi `authed`), HTTP fallback, Dual Sync và giải mã ảnh nền. Kết quả trả về qua `GuiTx` tự động gọi `egui_ctx.request_repaint()`.

### 3.2. Dual Sync di động (iOS V2 Wasmtime + Android V1 fallback)
- `src/zproto_host.rs` nhúng runtime **Wasmtime** chạy module `libzproto_wasm` giải mã Sync V2 (Curve25519/X25519).
- `src/history_sync.rs` thử Sync V2 (`cmd 590`) trong 5,5 giây; nếu điện thoại Android không phản hồi sẽ tự dọn phiên (`cmd 591`) và chuyển sang Sync V1 (`pull_mobile_msg` `cmd 12000`).

### 3.3. ResourceLru 64 MB, Media Sizing & Vanish Tombstone
- `src/media_resources.rs` quản lý `ResourceLru` ngân sách **64 MB**, bảo vệ toàn bộ texture đang dùng trong khung hình (`FRAME_TOUCHED_TEXTURES`), kết hợp giới hạn độ phân giải giải mã ảnh (`128px` avatar, `720px` thường, `2048px` HD) và `Semaphore(4)` cho sticker.
- `ChatMessage::media_size` được lưu vào SQLite để giữ đúng tỷ lệ ảnh/video trong lúc chờ tải; `apply_vanish_expiry` kết hợp `VANISH_TOMBSTONE_JSON` ngăn phục hồi dữ liệu tin nhắn tự xóa.

### 3.4. Lưu trữ 3 tầng & Cổng kiểm tra chất lượng
1. **Snapshot JSON (`threads_cache.json`, `accounts/<uin>/...`)**: Phục hồi nhanh danh sách hội thoại và tách biệt snapshot theo từng tài khoản.
2. **SQLite WAL (`accounts/<uin>_history.db`)**: Lưu lịch sử, khử trùng theo `msg_id`/identity, lưu `media_size` và tombstone.
3. **Storage Manager (`src/storage_manager.rs`)**: Thống kê dung lượng root / media / cache / system, hỗ trợ dọn dẹp và di chuyển thư mục dữ liệu (`relocate_storage_root`).
4. **Invariants & Testing (`scripts/check_invariants.py`, `tests/`)**: Toàn bộ 8 rule tự động (`R1`–`R8`: panic-free GUI, safe clamp, ranh giới GUI, no sleep/hack, hygiene, comment/timer ratchets, popup Windows) và 7 suite integration tests (`72 tests`) được đặt tập trung trong `tests/`, không chèn test/mock vào `src/`.
