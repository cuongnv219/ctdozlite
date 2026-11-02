# ctdozlite — Phiên bản Native iOS / iPadOS / iPhone Duo (`min iOS 12.0+`)

Thư mục `ios/` chứa toàn bộ mã nguồn, nhân Rust FFI (`ctdozlite_ios_core`) và hệ thống đóng gói độc lập của `ctdozlite` dành cho **iPhone**, **iPad** và **iPhone Duo (`iPhone19,4` / `V68`)** với mục tiêu triển khai tối thiểu từ **iOS 12.0 trở lên** (`-mios-version-min=12.0`).

> **Độc lập hoàn toàn:** Mọi tệp nguồn, cấu hình Cargo và sản phẩm biên dịch của bản iOS đều nằm gọn trong `ios/`, không chỉnh sửa hay ảnh hưởng tới cấu trúc của bản Desktop (`Linux / Windows / macOS`).

---

## 1. Điểm nổi bật & Tối ưu hoá

- **Hỗ trợ từ iOS 12.0 trở lên (`iOS 12.0` → `iOS 27.0`):**
  - Sử dụng **Objective-C + UIKit thuần (`UIApplicationDelegate`)** kết hợp trực tiếp với thư viện tĩnh **Rust (`libctdozlite_ios_core.a`)**.
  - Không phụ thuộc vào `SwiftUI` hay `UIScene` hay `SF Symbols` (vốn chỉ có từ iOS 13+), giúp chạy mượt trên cả các thiết bị đời cũ chạy iOS 12 (iPhone 5s, iPhone 6/6s, iPad Air 1/2, iPad mini 2/3/4) lẫn các máy đời mới nhất.
  - Dung lượng gói `.ipa` siêu nhẹ (**~772 KB**), khởi động tức thì, mức tiêu thụ RAM chỉ **~18 MB**, tốc độ cuộn khung chat 60fps/120fps ProMotion.
- **Tối ưu đặc biệt cho iPhone Duo (`com.apple.CoreSimulator.SimDeviceType.iPhone-Duo` / `iPhone19,4`) & iPad:**
  - **4 chế độ bố cục thích ứng (Adaptive Layout):**
    1. **Tự động (`Auto`):** Tự nhận diện phần cứng `iPhone19,4` (`V68`), màn hình phụ `UIScreen` (`LCD-1`) và kích thước màn hình gập ngoài (`466×678 pt`) / mở gập (`951×669 pt`).
    2. **iPhone (`Compact`):** Giao diện 1 cột chuẩn cho màn hình dọc iPhone & màn hình ngoài của iPhone Duo.
    3. **iPhone Duo (`Màn hình đôi qua Bản lề trung tâm`):**
       - Chế độ **Danh sách + Khung Chat** qua bản lề trung tâm.
       - Chế độ **Song song 2 Khung Chat / 2 Tài khoản (`Duo Dual-Chat`)**: Mở đồng thời 2 cuộc trò chuyện khác nhau (hoặc **Tài khoản 1** ở màn hình Trái và **Tài khoản 2** ở màn hình Phải) để nhắn tin cùng lúc.
       - Tự động lắng nghe `UIScreenDidConnectNotification` để xuất cửa sổ độc lập thứ 2 lên màn hình `LCD-1` của iPhone Duo.
    4. **iPad Pro (`3-Cột Split View / Slide Over / Stage Manager`):** Hiển thị đồng thời Danh sách hội thoại + Khung chat chính + Khung chat phụ.
- **Đầy đủ tính năng cốt lõi từ `v0.0.5`:**
  - **Chống gộp nhầm/nhảy sai khi trùng tên Zalo (`#63`):** Định danh tuyệt đối theo `thread_id` (tích hợp sẵn 2 liên hệ cùng tên `"Nguyễn Văn An"` với `ID: 10001` và `ID: 20002` để kiểm chứng trực tiếp trên 2 màn hình).
  - **Tự động kiểm tra & thông báo cập nhật qua GitHub Releases API:** Truy vấn trực tiếp `https://api.github.com/repos/ctdoteam1110/ctdozlite/releases/latest`, hiển thị thanh thông báo nổi và hộp thoại chi tiết danh sách sửa lỗi/cập nhật.
  - **Hẹn giờ gửi tin nhắn & Nhắc hẹn (`#38`, `#24`):** Bộ điều phối nền kiểm tra mỗi 2 giây và tự động gửi tin nhắn khi đến hạn.
  - **Auto-Reply Bot (`#35`):** Quản lý quy tắc tự động trả lời (`autoreply_rules.json`) và nút giả lập tin nhắn đến để test trực tiếp.
  - **Thao tác vuốt (Swipe Actions) & Menu giữ lì (`#5`, `#24`):** Ghim/Bỏ ghim, Tắt/Bật thông báo, Ẩn/Hiện hội thoại, Mở ở màn hình Phải (Duo), Hẹn giờ gửi tin, Xoá lịch sử.
  - **Mã hoá `ParamCipher v2` (`AES-128-CBC` + `AES-256-GCM` `#19`, `#16`), `signkey` MD5 & Failover đa máy chủ (`#27`).**
  - **Nhập / Xuất (Migrate) lịch sử tin nhắn JSON idempotent (`#9`) & Đổi giao diện (`Zalo Dark`, `Zalo Light`, `OLED Midnight` `#7`).**

---

## 2. Lệnh Biên dịch & Kiểm thử (`Makefile`)

```bash
cd ios

# Biên dịch toàn bộ (Rust core arm64 + arm64-sim, ctdozlite.app và gói .ipa)
make all

# Chạy toàn bộ bộ kiểm thử (Rust unit tests + kiểm tra Mach-O minos 12.0 + Simulator self-test)
make test

# Chỉ đóng gói file .ipa (để cài qua TrollStore / Sideloadly / AltStore / vphone)
make ipa
```

### Sản phẩm đầu ra sau khi chạy `make all`
- `ios/build/device/ctdozlite.app` — Gói ứng dụng arm64 cho thiết bị thật (iPhone / iPad / iPhone Duo, `minos 12.0`).
- `ios/build/sim/ctdozlite.app` — Gói ứng dụng cho Xcode iOS Simulator.
- `ios/build/ctdozlite-0.0.5-ios12.0-universal.ipa` — File cài đặt `.ipa` hoàn chỉnh.
