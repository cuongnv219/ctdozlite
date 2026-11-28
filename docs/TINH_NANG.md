# Tính năng & giới hạn

Trang này cho biết **cái gì đang chạy, cái gì chạy một phần, cái gì chưa có** — để bạn không mất thời gian báo lại những lỗi đã biết.

> **Kiểm chứng trên nhánh `main` (v0.1.4).** Bảng này được cập nhật mỗi lần release.
> Nếu bạn đang dùng bản build cũ hơn, một số dòng có thể đã khác.

## Cách đọc

| Ký hiệu | Nghĩa |
|---|---|
| ✅ | Hoạt động bình thường |
| 🟡 | Hoạt động một phần — có giới hạn được ghi rõ |
| 🐞 | Đã biết lỗi — đang có issue theo dõi |
| ⛔ | Chưa hỗ trợ |
| 🚫 | Không nằm trong kế hoạch |

Cột **Issue** trỏ tới nơi đang theo dõi. Nếu issue đã đóng, tính năng đã được sửa.

---

## Đăng nhập & phiên làm việc

| Tính năng | Trạng thái | Ghi chú | Issue |
|---|---|---|---|
| Đăng nhập bằng mã QR | ✅ | Quét bằng app Zalo trên điện thoại | |
| Đăng nhập bằng số điện thoại + mật khẩu | ✅ | `--phone`, `--pass` | |
| Tự khôi phục phiên từ `session.json` | ✅ | Giữ nguyên phiên khi gặp lỗi mạng / HTTP 404 tạm thời; tự failover qua nhiều host ứng viên | [#21](https://github.com/ctdoteam1110/ctdozlite/issues/21), [#27](https://github.com/ctdoteam1110/ctdozlite/issues/27) |
| Đăng xuất | ✅ | Chỉ xoá phiên xác thực, giữ nguyên lịch sử cục bộ `zalo_history.db` | [#21](https://github.com/ctdoteam1110/ctdozlite/issues/21) |
| Nhiều tài khoản / thư mục dữ liệu riêng | ✅ | Hỗ trợ chuyển đổi đa tài khoản trong GUI, `--data-dir <PATH>`, `--profile <NAME>` hoặc `CTDOZLITE_DATA_DIR` | [#30](https://github.com/ctdoteam1110/ctdozlite/issues/30) |
| Khóa ứng dụng PIN 4 số (App Lock) | ✅ | Khóa nhanh `Ctrl+L`, modal thiết lập nhanh, tự khóa idle/sleep, ẩn thông báo & cách ly cuộc gọi khi khóa | [#110](https://github.com/ctdoteam1110/ctdozlite/issues/110), [#211](https://github.com/ctdoteam1110/ctdozlite/issues/211) |
| Chặn mở trùng cửa sổ (Single-Instance) | 🟡 | Đã hỗ trợ theo thư mục dữ liệu trên Linux/macOS (`PR #182`); Windows đang lên kế hoạch | [#182](https://github.com/ctdoteam1110/ctdozlite/pull/182) |

---

## Nhắn tin & Hẹn giờ

| Tính năng | Trạng thái | Ghi chú | Issue |
|---|---|---|---|
| Gửi/nhận tin nhắn văn bản (1-1 và nhóm) | ✅ | | |
| Trả lời trích dẫn | ✅ | Chuột phải vào bong bóng tin nhắn | |
| Chuyển tiếp tin nhắn | ✅ | | |
| Thả cảm xúc (reaction) | ✅ | Nút thả tim nhanh `❤️` trực tiếp trên thanh hover + bảng emoji đầy đủ | [#174](https://github.com/ctdoteam1110/ctdozlite/pull/174) |
| Nhắc tên (`@mention`) chính xác theo UTF-16 | ✅ | Đọc `pos`/`len` UTF-16 từ metadata + kiểm tra ranh giới từ (`@An` != `@Anh`), giữ fallback 5 từ tiếng Việt | [#174](https://github.com/ctdoteam1110/ctdozlite/pull/174) |
| Bình chọn nhóm (Group Polls) | ✅ | Hiển thị thẻ bình chọn và bầu chọn trực tiếp trong nhóm | [#193](https://github.com/ctdoteam1110/ctdozlite/pull/193) |
| Thu hồi tin nhắn | ✅ | Thu hồi với mọi người & chống thu hồi cục bộ (Anti-Recall) | |
| Xoá tin nhắn phía tôi & Tin nhắn tự xóa | ✅ | Gia cố `VANISH_TOMBSTONE_JSON` không để lộ lại media/text đã hết hạn | [#192](https://github.com/ctdoteam1110/ctdozlite/pull/192) |
| Ghim / đánh dấu sao | ✅ | Lưu trữ bền vững trong SQLite, sắp xếp mới nhất trước | [#9](https://github.com/ctdoteam1110/ctdozlite/issues/9) |
| Hẹn giờ gửi tin nhắn & Nhắc hẹn | ✅ | Bấm nút `🕒` cạnh ô nhập tin nhắn; hỗ trợ mốc nhanh (`+15m`, `+1h`, `18:00`, `08:00 mai`) và giờ tùy chỉnh | [#38](https://github.com/ctdoteam1110/ctdozlite/issues/38), [#24](https://github.com/ctdoteam1110/ctdozlite/issues/24) |
| Gửi nhiều ảnh / Album lưới & Giữ tỷ lệ khi tải | ✅ | Gom album 2–3 cột; lưu `media_size` vào SQLite giữ khung đúng tỷ lệ tránh nhảy cuộn | [#127](https://github.com/ctdoteam1110/ctdozlite/pull/127), [#192](https://github.com/ctdoteam1110/ctdozlite/pull/192) |
| Xem trước tài liệu & Lưu media về `Downloads` | ✅ | Xem trực tiếp PDF/DOCX/XLSX/PPTX/CSV (`doc_preview.rs`), duyệt ảnh hội thoại (`ImageViewerModal`) & lưu nhanh vào `Downloads` | [#167](https://github.com/ctdoteam1110/ctdozlite/pull/167), [#177](https://github.com/ctdoteam1110/ctdozlite/pull/177) |
| Gửi sticker, Twemoji & Open Sans | ✅ | Có kho sticker + `custom_stickers.json` + emoji màu Twemoji (`twemoji72.pack`) & font Open Sans | [#7](https://github.com/ctdoteam1110/ctdozlite/issues/7), [#189](https://github.com/ctdoteam1110/ctdozlite/pull/189) |
| Dán ảnh trực tiếp (`Ctrl+V` / `Cmd+V`) | ✅ | Đã hỗ trợ trên Windows, macOS và Linux (Clipboard DIB / `vendor/egui-winit`) | [#165](https://github.com/ctdoteam1110/ctdozlite/pull/165) |
| Xem trước liên kết (OpenGraph) | ✅ | | |
| Tin nhắn thoại | ⛔ | Chưa ghi âm và gửi được tin nhắn thoại | |
| Chỉnh sửa tin nhắn đã gửi | ⛔ | | |

---

## Hội thoại & lịch sử

| Tính năng | Trạng thái | Ghi chú | Issue |
|---|---|---|---|
| Danh sách hội thoại (bạn bè, nhóm, Cloud của tôi) | ✅ | Tự động thử qua danh sách host dự phòng khi một domain trả 404/lỗi mạng | [#22](https://github.com/ctdoteam1110/ctdozlite/issues/22), [#27](https://github.com/ctdoteam1110/ctdozlite/issues/27) |
| Tên nhóm và ảnh đại diện nhóm | ✅ | Đồng bộ đầy đủ qua `getAllGroups` + `getGroupInfo` | [#8](https://github.com/ctdoteam1110/ctdozlite/issues/8) |
| Quản lý hội thoại (Ghim / Tắt thông báo / Ẩn / Xoá lịch sử) | ✅ | Chuột phải vào hội thoại ở danh sách bên trái | [#5](https://github.com/ctdoteam1110/ctdozlite/issues/5), [#24](https://github.com/ctdoteam1110/ctdozlite/issues/24) |
| Nhận tin nhắn mới theo thời gian thực | ✅ | Polling trong worker + WebSocket | [#28](https://github.com/ctdoteam1110/ctdozlite/issues/28) |
| Tìm kiếm hội thoại & nội dung tin nhắn | ✅ | Tìm không dấu theo tên hội thoại và nội dung tin nhắn trong `zalo_history.db` | [#5](https://github.com/ctdoteam1110/ctdozlite/issues/5) |
| Lưu lịch sử cục bộ (SQLite WAL) & Quản lý bộ nhớ 3 tầng | ✅ | `accounts/<uin>_history.db` + quét/dọn/di chuyển root dữ liệu an toàn (`storage_manager.rs`) | |
| Đồng bộ lịch sử cũ (Dual Sync V2/V1) | ✅ | Thử iOS Sync V2 (`cmd 590` via Wasmtime `libzproto_wasm`) trước 5,5s, tự động fallback sang Android Sync V1 (`cmd 12000`), hỗ trợ nhập lịch sử từ Zalo PC | [#155](https://github.com/ctdoteam1110/ctdozlite/issues/155), [#200](https://github.com/ctdoteam1110/ctdozlite/issues/200) |
| Nút "Làm mới hội thoại" | ✅ | Làm mới danh sách bạn bè, nhóm, tin nhắn mới và hiển thị rõ trạng thái / lỗi nếu có | [#23](https://github.com/ctdoteam1110/ctdozlite/issues/23) |
| Nhập (Migrate) lịch sử từ máy/thư mục khác | ✅ | **Cài đặt → Đồng bộ tin nhắn → Chọn file hoặc thư mục để nhập** (hỗ trợ `.db`/`.sqlite` và `.json`/`.jsonl`) | [#9](https://github.com/ctdoteam1110/ctdozlite/issues/9) |

---

## Tự động hoá, REST API & MCP

| Tính năng | Trạng thái | Ghi chú | Issue |
|---|---|---|---|
| Авто-Reply (Tự động trả lời theo từ khóa) | ✅ | Cấu hình trong **Cài đặt → Tiện ích (`autoreply_rules.json`)** | [#35](https://github.com/ctdoteam1110/ctdozlite/issues/35) |
| Local HTTP REST API (`127.0.0.1`) | ✅ | Chạy với `--api-port <PORT>` hoặc `CTDOZLITE_API_PORT` (`/api/v1/status`, `/threads`, `/messages`, `/send`, `/schedule`, `/autoreply`) | [#35](https://github.com/ctdoteam1110/ctdozlite/issues/35) |
| Model Context Protocol (MCP Server) | ✅ | Chạy `--mcp` (stdio JSON-RPC 2.0) hoặc gọi `POST /api/v1/mcp` để tích hợp Claude Desktop / Cursor / AI Agent | [#35](https://github.com/ctdoteam1110/ctdozlite/issues/35) |
| Kịch bản tự động Rhai (`ThreadScript`) & LLM Agent | ✅ | Chạy script Rhai theo từng hội thoại (`src/thread_script.rs`) & TUI LLM Agent loop (`src/agent.rs`, `src/llm.rs`) | [#137](https://github.com/ctdoteam1110/ctdozlite/pull/137), [#144](https://github.com/ctdoteam1110/ctdozlite/pull/144) |
| Cầu nối 2 chiều Telegram ↔ Zalo | ✅ | Đồng bộ tin nhắn hai chiều giữa Telegram bot và Zalo kèm LRU echo-guard (`src/tg_bridge.rs`) | [#143](https://github.com/ctdoteam1110/ctdozlite/pull/143) |

---

## Bạn bè

| Tính năng | Trạng thái | Ghi chú | Issue |
|---|---|---|---|
| Tìm người dùng bằng số điện thoại | ✅ | | |
| Gửi lời mời kết bạn | ✅ | | |
| Chấp nhận / từ chối lời mời | ✅ | | |
| Huỷ lời mời đã gửi | ✅ | | |

---

## Gọi thoại & video

| Tính năng | Trạng thái | Ghi chú | Issue |
|---|---|---|---|
| Gọi thoại 1-1 | 🟡 | Đã có luồng gọi; chất lượng phụ thuộc mạng | |
| Gọi video | 🟡 | Cần `ffmpeg` trong `PATH` để thu và giải mã hình | |
| Gọi nhóm | 🟡 | | |

---

## Giao diện & nền tảng

| Tính năng | Trạng thái | Ghi chú | Issue |
|---|---|---|---|
| Giao diện đồ họa (GUI) | ✅ | `--gui`, mặc định; hỗ trợ kéo co giãn xuống `480×360` và Info Overlay `< 880px` | |
| Giao diện terminal (TUI) | ✅ | `--tui`; tương thích terminal `80×24`, tìm kiếm cho phép nhập `j`/`k`, cuộn Help | |
| Hiệu ứng chuyển động (Animations) | ✅ | Bật/tắt và chỉnh tốc độ (`0.5x–2.0x`) trong **Cài đặt → Giao diện → Hiệu ứng chuyển động** | |
| Chủ đề sáng / tối & Theme JSON tùy chỉnh | ✅ | Chuẩn tương phản WCAG 2.1 AA; hỗ trợ ghi đè bảng màu qua `theme.json` (**Cài đặt → Giao diện**) | [#7](https://github.com/ctdoteam1110/ctdozlite/issues/7) |
| Nhập mật khẩu CLI ẩn ký tự (`*`) | ✅ | Ẩn ký tự khi nhập mật khẩu trực tiếp trên terminal (hỗ trợ `--pass-stdin`) | |
| Hiển thị tiếng Việt và tiếng Trung (CJK) | ✅ | Tự nạp font hệ thống đa ngôn ngữ + fallback Noto Sans CJK | [#6](https://github.com/ctdoteam1110/ctdozlite/issues/6), [#5](https://github.com/ctdoteam1110/ctdozlite/issues/5) |
| Khay hệ thống (System Tray & Close-to-Tray) | 🟡 | Đầy đủ trên Windows (`PR #213`) & Linux X11 (`ksni`, Wayland tự thoát khi đóng `PR #207`); macOS chưa có icon menu bar riêng | [#213](https://github.com/ctdoteam1110/ctdozlite/pull/213), [#207](https://github.com/ctdoteam1110/ctdozlite/pull/207) |
| Thông báo hệ điều hành & Corner Popup | ✅ | Corner popup tùy chỉnh chỉ bật trên Windows (`PR #208`); Linux/macOS dùng thông báo gốc OS, ẩn nội dung khi khóa PIN4 | [#208](https://github.com/ctdoteam1110/ctdozlite/pull/208) |
| Tự động kiểm tra & cập nhật phiên bản mới | ✅ | Kiểm tra qua GitHub Releases & `latest.json` kèm xác thực SHA-256 trước khi cài đặt | [#12](https://github.com/ctdoteam1110/ctdozlite/issues/12), [#29](https://github.com/ctdoteam1110/ctdozlite/issues/29) |
| Đóng gói đa nền tảng Desktop (Windows / macOS / Linux) | ✅ | `.exe` (setup + portable), `.dmg` (Apple Silicon + Intel + Universal), `.deb`/`.rpm`/`.AppImage`/`.tar.gz` + Legacy `glibc 2.31` | |
| iOS (Sắp có) | 🟡 | Hoạt động dưới dạng phiên phụ (`zpw_type=30`), sẽ phát hành gói `.ipa` trong bản cập nhật tới | |

---

## Báo một lỗi mới

Trước khi mở issue, kiểm tra [trang Đã biết](DA_BIET.md). Nếu chưa có, mở issue kèm:

1. Hệ điều hành và phiên bản (Windows 11, macOS 15, Fedora 41…).
2. Phiên bản app (góc trên bên trái, `Native v…`).
3. Chế độ đang chạy: GUI hay TUI.
4. Bước tái hiện, càng ngắn càng tốt.
5. Log: chạy lại với `--dev` và dán phần lỗi.

Xem [Hướng dẫn sử dụng](HUONG_DAN_SU_DUNG.md) để biết chi tiết các tham số.

