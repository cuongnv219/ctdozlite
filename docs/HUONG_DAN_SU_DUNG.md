# Hướng dẫn sử dụng ctdozlite

Tài liệu tóm tắt các tùy chọn dòng lệnh, thao tác trên giao diện đồ họa (GUI) và bảng phím tắt của giao diện terminal (TUI).

---

## 1. Tham số dòng lệnh (CLI Flags)

| Tham số | Ý nghĩa | Ví dụ |
|---|---|---|
| `--data-dir <PATH>` | Chọn thư mục lưu phiên, cache, log và cơ sở dữ liệu; đường dẫn tương đối tính từ thư mục khởi chạy. | `ctdozlite --data-dir ./local-data --gui` |
| `--profile <TÊN>` | Chạy hồ sơ dữ liệu riêng trong thư mục con `profiles/<tên>`. | `ctdozlite --profile work --gui` |
| `--print-data-dir` | Chuẩn bị và in thư mục dữ liệu rồi thoát, không mở giao diện hay kết nối mạng. | `ctdozlite --print-data-dir` |
| `--gui` | Chạy chế độ giao diện đồ họa (mặc định nếu không truyền tham số). | `ctdozlite --gui` |
| `--tui` | Chạy chế độ giao diện dòng lệnh trong terminal. | `ctdozlite --tui` |
| `--qr` | Yêu cầu tạo mã QR đăng nhập ngay khi khởi động TUI. | `ctdozlite --tui --qr` |
| `--dev` | Bật thanh công cụ Dev Inspector để xem log mạng và payload giải mã. | `ctdozlite --gui --dev` |
| `--api-port <PORT>` | Mở Local HTTP REST API trên `127.0.0.1:<PORT>` (hoặc biến môi trường `CTDOZLITE_API_PORT`). | `ctdozlite --gui --api-port 8765` |
| `--mcp` | Chạy máy chủ Model Context Protocol (JSON-RPC 2.0 trên stdio). | `ctdozlite --mcp` |
| `--logout` | Xóa phiên đăng nhập hiện tại (`session.json`) rồi thoát. | `ctdozlite --logout` |
| `--phone <SỐ>` | Điền sẵn số điện thoại khi thử đăng nhập bằng mật khẩu. | `ctdozlite --tui --phone 0912345678` |
| `--pass <MK>` | Điền sẵn mật khẩu. | `ctdozlite --tui --phone 0912345678 --pass ...` |
| `--dump` | Lưu các gói phản hồi JSON ra thư mục `dump/` để phục vụ phân tích. | `ctdozlite --dump` |
| `--version` / `-V` | In phiên bản rồi thoát mà không khởi tạo thư mục dữ liệu. | `ctdozlite --version` |

---

Thư mục dữ liệu mặc định (phiên đăng nhập, lịch sử tin nhắn, cache, cài đặt) không phụ thuộc vào thư mục khởi chạy:

| Hệ điều hành | Thư mục mặc định |
| :--- | :--- |
| macOS | `~/Library/Application Support/ctdozlite` |
| Linux | `$XDG_DATA_HOME/ctdozlite`, nếu không đặt thì `~/.local/share/ctdozlite` |
| Windows | Thư mục làm việc hiện tại (như cũ) |

Vì vậy mở app từ menu ứng dụng hay từ terminal ở bất kỳ thư mục nào đều dùng chung một bộ dữ liệu. `--profile <tên>` dùng thư mục con `profiles/<tên>`. `--data-dir` ghi đè mọi lựa chọn trên. Thư mục mặc định trên macOS/Linux và thư mục chỉ định bằng `--data-dir` có quyền `0700` trên Unix. Nếu thư mục không ghi được, chương trình báo lỗi trước khi mở dữ liệu.

**Chuyển dữ liệu cũ trên Linux:** trước đây bản Linux lưu dữ liệu vào thư mục làm việc (thường là `~` khi mở từ menu). Lần đầu chạy bản mới, nếu thư mục mặc định còn trống, chương trình tự **sao chép** dữ liệu cũ từ thư mục làm việc hoặc `~` (chọn nơi dùng gần nhất) sang thư mục mới. Chỉ các tệp dữ liệu của app được sao chép (`session*.json`, `zalo_history.db`, `accounts/`, `app_settings.json`, `stickers_cache/`, …); log và tệp khác không bị đụng tới. Bản gốc được giữ nguyên, có thể tự xóa sau khi kiểm tra. Không sao chép khi đã dùng `--data-dir`.

`--version` không chuẩn bị thư mục dữ liệu. Có thể dùng `--data-dir` để chọn lại thư mục đã có dữ liệu.

## 2. Giao diện đồ họa (GUI)

### 2.1. Đăng nhập
1. Chạy `./target/release/ctdozlite --gui`.
2. Mở ứng dụng Zalo trên điện thoại, quét mã QR hiển thị trên màn hình và bấm **Đăng nhập**.
3. Sau khi xác nhận, phiên làm việc được lưu vào `session.json`. Ở những lần mở tiếp theo, ứng dụng tự nạp lại phiên này mà không cần quét lại mã QR.

### 2.2. Nhắn tin & Gợi ý Sticker tự động
- **Tìm kiếm và lọc hội thoại:** Sử dụng ô tìm kiếm phía trên danh sách bên trái (hỗ trợ tìm tiếng Việt không dấu) hoặc lọc theo các tab *Tất cả*, *Cá nhân*, *Nhóm*.
- **Gợi ý nhãn dán khi đang gõ:**
  - Khi nhập từ khóa hoặc cụm từ ngắn trong ô chat (`xin chào`, `cảm ơn`, `ok`, `haha`, `buồn`, `khóc`, `nhớ`, `cố lên`, `mệt quá`, `đi chơi`, `trà sữa`...), ứng dụng tự động truy vấn API sticker của Zalo và hiển thị dải gợi ý phía trên khung soạn thảo.
  - Bấm trực tiếp vào sticker để gửi, hoặc dùng phím mũi tên `↑` để chọn lên thanh gợi ý, `←` / `→` để di chuyển, `Enter` để gửi và `Esc` để đóng.
  - Bấm biểu tượng kính lúp ở góc phải thanh gợi ý để mở toàn bộ kết quả tìm kiếm của từ khóa đó trong bảng Sticker Picker.
- **Kho nhãn dán (Sticker Picker):**
  - Bấm biểu tượng nhãn dán cạnh ô nhập tin nhắn để mở bảng chọn.
  - Tab **Tìm kiếm** tự động tìm nhãn dán ngay khi gõ từ khóa mà không cần bấm Enter.
  - Tab **Kho nhãn** cho phép duyệt và ghim các bộ nhãn dán từ máy chủ Zalo.
- **Tương tác trên tin nhắn:**
  - Rê chuột lên bong bóng tin nhắn để dùng nhanh nút **Thả tim (`❤️`)**, **Trả lời**, **Chuyển tiếp** hoặc mở menu mở rộng (`...`).
  - Nhấp chuột phải vào bong bóng tin nhắn để chọn đầy đủ cảm xúc (reaction), trả lời trích dẫn, chuyển tiếp, sao chép nội dung, ghim, đánh dấu sao, thu hồi tin nhắn với mọi người hoặc xóa ở phía cá nhân.
  - Hỗ trợ dán ảnh trực tiếp từ clipboard (`Ctrl+V` / `Cmd+V`), xem trước tài liệu (PDF/DOCX/XLSX/PPTX/CSV), bình chọn nhóm (Group Polls) và mở trình duyệt ảnh toàn hội thoại (`ImageViewerModal`).

### 2.3. Bảo mật PIN 4 số, Đa tài khoản, Khay hệ thống & Bộ nhớ
- **Khóa ứng dụng (App Lock PIN4):** Bật trong **Cài đặt → Quyền riêng tư** (hoặc modal thiết lập nhanh), hỗ trợ khóa tức thì bằng phím tắt `Ctrl+L` / nút ổ khóa trên thanh tiêu đề, tự động khóa khi không tương tác hoặc ngủ máy, ẩn nội dung thông báo và cách ly cuộc gọi khi đang khóa.
- **Quản lý bộ nhớ 3 tầng (Storage Manager):** Mở trong **Cài đặt → Lưu trữ** để xem dung lượng dữ liệu gốc, media, cache, dọn dẹp hoặc di chuyển thư mục lưu trữ.
- **Đa tài khoản (Multi-Account Switcher):** Quản lý và chuyển đổi tức thì giữa các tài khoản đã đăng nhập mà không lẫn DB/cache.
- **Thu gọn xuống khay hệ thống (Close-to-Tray):** Bật trong Cài đặt trên Windows (`#213`) hoặc Linux X11 (`#166`).

### 2.4. Gọi thoại và gọi video 1-1
- Trong cuộc trò chuyện cá nhân, bấm biểu tượng điện thoại hoặc máy quay ở góc phải thanh tiêu đề hội thoại để bắt đầu cuộc gọi.
- Thiết bị thu âm (Mic) và máy ảnh (Camera) có thể chọn trực tiếp trong cửa sổ cuộc gọi hoặc tại mục **Cài đặt -> Cài đặt chung**.

---

## 3. Giao diện dòng lệnh (TUI)

Khởi chạy:
```bash
./target/release/ctdozlite --tui --qr
```

| Phím tắt | Chức năng |
|---|---|
| `↑` / `↓` hoặc `k` / `j` | Di chuyển giữa các cuộc trò chuyện trong danh sách bên trái |
| `Enter` | Gửi tin nhắn đang soạn hoặc chuyển con trỏ vào ô nhập |
| Gõ ký tự bất kỳ | Tự động đưa con trỏ vào ô soạn thảo |
| `Esc` | Rời ô soạn thảo, quay lại danh sách hội thoại |
| `Tab` | Chuyển đổi qua lại giữa danh sách hội thoại và ô nhập |
| `r` | Đồng bộ lại danh bạ và tin nhắn mới |
| `h` | Tải thêm lịch sử tin nhắn của nhóm đang chọn |
| `x` | Đăng xuất và xóa `session.json` |
| `l` | Mở bảng xem log mạng (khi chạy kèm cờ `--dev`) |
| `q` | Thoát ứng dụng |

---

## 4. Tiện ích kiểm thử dòng lệnh (`test_fetch`)

Binary phụ `test_fetch` dùng để kiểm tra nhanh kết nối API trên phiên đã lưu mà không cần mở giao diện:

```bash
# Kiểm tra phiên và in danh sách hội thoại
cargo run --release --bin test_fetch

# Gửi thử tin nhắn cá nhân
cargo run --release --bin test_fetch -- --send <USER_ID> "Nội dung tin nhắn"

# Gửi thử tin nhắn nhóm
cargo run --release --bin test_fetch -- --send-group <GROUP_ID> "Nội dung tin nhắn"
```
