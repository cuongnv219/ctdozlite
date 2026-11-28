# Danh sách tính năng chuẩn & kiểm chứng (Inner Loop Checklist)

Mọi thay đổi trước khi merge phải pass các kiểm tra tự động và không làm hỏng bất kỳ mục cốt lõi nào dưới đây.

## 1. Cổng tự động bắt buộc (Automated Gates)
- [ ] `python scripts/check_invariants.py` — 0 vi phạm (`R1`..`R8`, không nới ratchet)
- [ ] Không chèn module `#[cfg(test)]` hoặc mock data trong `src/` — mọi test đặt trong `tests/`
- [ ] `cargo clippy --locked --lib --bin ctdozlite -j 1` — 0 lỗi (`[lints.clippy]` deny `dbg_macro`, `todo`, `unimplemented`)
- [ ] Chạy toàn bộ integration & lib test suites (`cargo test --locked -j 1`) — 0 thất bại

## 2. Đăng nhập, Session, App Lock & Đa tài khoản
- [ ] QR Login xử lý sạch race condition (`600` / `18012` cooldown), fallback PNG (`base64_png`)
- [ ] Khởi động lại app giữ nguyên session, không bắt quét QR lại khi session còn hạn
- [ ] Chuyển đổi / thêm tài khoản cô lập hoàn toàn DB, `threads.json`, avatar, `active_uin`
- [ ] Đồng bộ tin nhắn từ điện thoại (iOS V2 Wasmtime & Android V1 fallback) không đè mất dữ liệu acc khác
- [ ] App Lock PIN 4 số (`Ctrl+L`, auto-lock idle/sleep): không rewind con trỏ ô PIN, ẩn thông báo và cách ly cuộc gọi khi khóa

## 3. Nhắn tin, Album, Media & Bộ gõ / Cuộn chuột
- [ ] Bộ gõ tiếng Việt trên Windows (`EVKey` / `UniKey`): nhấn `Enter` 1 lần gửi ngay (không nuốt Enter như CJK IME, không bôi đen khó chịu)
- [ ] Tốc độ cuộn chuột mượt, chỉnh được trong Cài đặt (`scroll_speed`: `0.5x`–`4.0x`) và lưu xuống đĩa
- [ ] Gửi nhiều ảnh đồng thời hoặc cùng `media_group` gộp đúng dạng Album grid (kể cả khi `sender` display name lệch ở tin outgoing)
- [ ] Ảnh/video/sticker/link giữ nguyên kích thước & tỷ lệ khung theo `media_size` trong lúc tải (`#192`), tin tự xóa ghi `VANISH_TOMBSTONE_JSON`

## 4. Tương tác tin nhắn & Phân loại Thread
- [ ] Hover lên tin nhắn hiện 4 nút gọn (`❤️` thả tim nhanh, `Reply`, `Forward`, `...`) — không tự bật dải emoji gây rối; `...` vẽ vector rõ nét không lỗi glyph (`#174`)
- [ ] Highlight `@mention` khớp chính xác UTF-16 `pos`/`len` + ranh giới từ (`@An` != `@Anh`), giữ fallback 5 từ tiếng Việt khi thiếu wire metadata (`#174`)
- [ ] Double-click vào tin nhắn reply đúng tin nhắn (theo cài đặt `double_click_to_reply`)
- [ ] Thread hệ thống / OA (`Trợ lý Kiki`, `Zalo`, `Thời Tiết`, `CloudOTP`…) hiển thị nhãn `"Hệ thống"`, không hiện nút Gọi / Kết bạn / `"Người lạ"`

## 5. Responsive UI, RAM, Storage & Tích hợp OS
- [ ] Thu nhỏ cửa sổ ở mọi kích thước (`< 620px` đến `1920px+`) không chồng chữ ở Chat Header, thanh sidebar có nút thu/mở `«` / `»`
- [ ] Thu nhỏ / minimize xuống taskbar hoặc treo máy 15+ phút không crash / không panic (`clamp` an toàn)
- [ ] Mở/đóng xem ảnh HD giải phóng `hd_*` textures, bộ nhớ sticker & media giới hạn bởi `ResourceLru` (`64 MB` bảo vệ `FRAME_TOUCHED_TEXTURES`)
- [ ] Windows Close-to-Tray + lưu `app_settings.json` trước đăng nhập (`#213`); Linux Wayland thoát sạch khi đóng (`#207`); corner popup chỉ bật trên Windows (`#208`)
- [ ] Quản lý bộ nhớ 3 tầng (`storage_manager.rs`) quét và di chuyển thư mục lưu trữ không làm mất session/DB
