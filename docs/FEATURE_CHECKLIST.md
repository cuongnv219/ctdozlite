# Danh sách tính năng chuẩn & kiểm chứng (Inner Loop Checklist)

Mọi thay đổi trước khi merge phải pass các kiểm tra tự động và không làm hỏng bất kỳ mục cốt lõi nào dưới đây.

## 1. Cổng tự động bắt buộc (Automated Gates)
- [ ] `python scripts/check_invariants.py` — 0 vi phạm (`R1`..`R7`)
- [ ] `cargo test --lib -j 1 property_invariants` — toàn bộ property-based fuzz tests pass
- [ ] `cargo clippy --lib --bins` / `cargo check --bins -j 1` — 0 lỗi (`[lints.clippy]` deny `dbg_macro`, `todo`, `unimplemented`)
- [ ] Chạy test hồi quy liên quan (`cargo test --locked`)

## 2. Đăng nhập, Session & Đa tài khoản
- [ ] QR Login xử lý sạch race condition (`600` / `18012` cooldown), fallback PNG (`base64_png`)
- [ ] Khởi động lại app giữ nguyên session, không bắt quét QR lại khi session còn hạn
- [ ] Chuyển đổi / thêm tài khoản cô lập hoàn toàn DB, `threads.json`, avatar, `active_uin`
- [ ] Đồng bộ tin nhắn từ điện thoại (iOS V2 & Android V1) không đè mất dữ liệu acc khác

## 3. Nhắn tin, Album & Bộ gõ / Cuộn chuột
- [ ] Bộ gõ tiếng Việt trên Windows (`EVKey` / `UniKey`): nhấn `Enter` 1 lần gửi ngay (không nuốt Enter như CJK IME, không bôi đen khó chịu)
- [ ] Tốc độ cuộn chuột mượt, chỉnh được trong Cài đặt (`scroll_speed`: `0.5x`–`4.0x`) và lưu xuống đĩa
- [ ] Gửi nhiều ảnh đồng thời hoặc cùng `media_group` gộp đúng dạng Album grid (kể cả khi `sender` display name lệch ở tin outgoing)

## 4. Tương tác tin nhắn & Phân loại Thread
- [ ] Hover lên tin nhắn chỉ hiện 3 nút gọn (`Reply`, `Forward`, `...`) — không tự bật dải emoji gây rối; `...` vẽ vector rõ nét không lỗi glyph
- [ ] Double-click vào tin nhắn reply đúng tin nhắn (theo cài đặt `double_click_to_reply`)
- [ ] Thread hệ thống / OA (`Trợ lý Kiki`, `Zalo`, `Thời Tiết`, `CloudOTP`…) hiển thị nhãn `"Hệ thống"`, không hiện nút Gọi / Kết bạn / `"Người lạ"`

## 5. Responsive UI, RAM & Độ ổn định Idle
- [ ] Thu nhỏ cửa sổ ở mọi kích thước (`< 620px` đến `1920px+`) không chồng chữ ở Chat Header, thanh sidebar có nút thu/mở `«` / `»`
- [ ] Thu nhỏ / minimize xuống taskbar hoặc treo máy 15+ phút không crash / không panic (`clamp` an toàn)
- [ ] Mở/đóng xem ảnh HD giải phóng `hd_*` textures, bộ nhớ sticker giới hạn bởi `ResourceLru` (48 MB)
