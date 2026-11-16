window.CTD_GITHUB_ISSUES = {
  "fetchedAt": "2026-11-15T17:37:56.000Z",
  "repo": "ctdoteam1110/ctdozlite",
  "totalIssues": 89,
  "totalPrs": 62,
  "issues": [
    {
      "author": {
        "id": "MDQ6VXNlcjc3MTQyMDUw",
        "is_bot": false,
        "login": "nguyenhoclaptrinh",
        "name": "Nguyen Truong"
      },
      "body": "Yêu cầu đồng bộ tin nhắn Sync V2 không hiển thị thông báo trên điện thoại\n\n## Tái hiện\n1. Vào Cài đặt, chọn tab Đồng bộ tin nhắn.\n2. Bấm 'Bắt đầu đồng bộ'.\n3. Zalo trên điện thoại không hiện popup xác nhận đồng bộ; giao diện dừng ở 10% và báo hết thời gian chờ sau 90 giây.\n\n## Nguyên nhân\n- `request_phone_sync_v2_http` tại `src/chat.rs:8867` gửi danh sách phân vùng lớn qua query string của HTTP GET, vượt giới hạn độ dài URL (lỗi HTTP 414).\n- `src/logger.rs:154` ghi log đồng bộ vào `/tmp/sync_debug.log`, bị nuốt log trên Windows do không tồn tại thư mục /tmp.\n- Gói CMD 590 ban đầu gửi truy vấn phân vùng tin nhắn chi tiết thay vì gói bắt tay phân vùng conversations chuẩn của Zalo PC.\n\n## Tiêu chí nghiệm thu\n- Yêu cầu đồng bộ làm xuất hiện popup xác nhận trên điện thoại Zalo.\n- Log đồng bộ ghi đúng vào `%APPDATA%/ctdozlite/sync_debug.log` trên Windows.\n- Pass unit test kiểm tra cấu trúc payload khởi tạo phiên.\n",
      "closedAt": "2026-11-15T09:22:10Z",
      "createdAt": "2026-11-15T05:54:07Z",
      "labels": [],
      "number": 151,
      "state": "CLOSED",
      "title": "fix(net): Sửa lỗi gửi yêu cầu đồng bộ Sync V2 về điện thoại và đường dẫn log trên Windows"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjc3MTQyMDUw",
        "is_bot": false,
        "login": "nguyenhoclaptrinh",
        "name": "Nguyen Truong"
      },
      "body": "Hiện tượng nháy cửa sổ console khi duyệt tin nhắn nhóm trên Windows\n\n## Tái hiện\n1. Mở 'Cloud của tôi' hoặc bất kỳ cuộc trò chuyện nào.\n2. Bấm vào một nhóm có tin nhắn video hoặc sticker động.\n3. Xuất hiện cửa sổ conhost.exe / cmd.exe màu đen nháy lên trong khoảng 100ms rồi biến mất khi trích xuất thumbnail.\n\n## Nguyên nhân\nHàm `extract_first_video_frame` tại `src/gui/worker.rs:4938` gọi `ffmpeg` thiếu cờ `CREATE_NO_WINDOW` (`0x0800_0000`).\n\n## Tiêu chí nghiệm thu\n- Không còn nháy cửa sổ cmd khi mở nhóm có video hoặc sticker.\n- Cờ tiến trình được bọc an toàn dưới `#[cfg(target_os = \"windows\")]`.\n",
      "closedAt": "2026-11-15T09:24:24Z",
      "createdAt": "2026-11-15T05:53:53Z",
      "labels": [],
      "number": 150,
      "state": "CLOSED",
      "title": "fix(ui): Triệt tiêu hiện tượng nháy cửa sổ console khi tải thumbnail media trên Windows"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjc3MTQyMDUw",
        "is_bot": false,
        "login": "nguyenhoclaptrinh",
        "name": "Nguyen Truong"
      },
      "body": "> **Trước khi mở issue:** Vui lòng kiểm tra bảng **[Tính năng & giới hạn (`docs/TINH_NANG.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/TINH_NANG.md)** và **[Lỗi đã biết (`docs/DA_BIET.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/DA_BIET.md)** để xem vấn đề đã được ghi nhận chưa.\n\n## Môi trường\n- **Hệ điều hành:** Windows 11 26H2 \n- **Phiên bản `ctdozlite`:** v0.1.1\n- **Chế độ chạy:** GUI\n\n## Các bước tái hiện\n1. Mở Zalo và đăng nhập mới\n2. Mở cài đặt\n3. Nhấn nút đồng bộ tin nhắn từ điện thoại ngay\n\n## Kết quả thực tế vs. kỳ vọng\n- **Thực tế:** Lỗi không thể đồng bộ\n\n<img width=\"545\" height=\"247\" alt=\"image\" src=\"https://github.com/user-attachments/assets/318cdef3-9966-4a46-92a8-31ea0d0adc8e\" />\n\n- **Kỳ vọng:**  Đồng bộ được cả tin nhắn 1-1 và tin nhắn nhóm\n\n## Log (`--dev`)\nChạy lại với `ctdozlite --gui --dev` và dán đoạn log liên quan tại đây (nhớ xoá thông tin nhạy cảm / cookie nếu có):\n- Log toàn ra 200 OK thôi nhưng có 1 đoạn log bất thường khi khởi động app:\n```text\n→ GET https://p4-msg-chat.zaloapp.com/?zpw_ver=694&zpw_type=30&params=<redacted>&ts=1794576577\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"censored\"}\n\n  data decrypted (162 bytes)\n← 404 Not Found <html>\n<head><title>404 Not Found</title></head>\n<body>\n<center><h1>404 Not Found</h1></center>\n<hr><center>nginx</center>\n</body>\n</html>\n<!-- a padding to disable MSIE and Chrome friendly error page -->\n<!-- a padding to disable MSIE and Chrome friendly error page -->\n<!-- a padding to disable MSIE and Chrome friendly error page -->\n<!-- a padding to disable MSIE and Chrome friendly er…\n← 404 Not Found <html>\n<head><title>404 Not Found</title></head>\n<body>\n<center><h1>404 Not Found</h1></center>\n<hr><center>nginx</center>\n</body>\n</html>\n<!-- a padding to disable MSIE and Chrome friendly error page -->\n<!-- a padding to disable MSIE and Chrome friendly error page -->\n<!-- a padding to disable MSIE and Chrome friendly error page -->\n<!-- a padding to disable MSIE and Chrome friendly er…\n```\n",
      "closedAt": "2026-11-15T02:33:54Z",
      "createdAt": "2026-11-13T13:34:04Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 147,
      "state": "CLOSED",
      "title": "bug: Tính năng đồng bộ tin nhắn từ điện thoại bị lỗi không hoạt động"
    },
    {
      "author": {
        "id": "U_kgDODZ7oPw",
        "is_bot": false,
        "login": "vudubai-2026",
        "name": "Tom"
      },
      "body": "> **Trước khi mở issue:** Vui lòng kiểm tra bảng **[Tính năng & giới hạn (`docs/TINH_NANG.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/TINH_NANG.md)** và **[Lỗi đã biết (`docs/DA_BIET.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/DA_BIET.md)** để xem vấn đề đã được ghi nhận chưa.\n\n## Môi trường\n- **Hệ điều hành:** Arch Linux\n- **Phiên bản `ctdozlite`:** v0.1.1\n- **Chế độ chạy:** GUI\n\n## Các bước tái hiện\n1. Mở app\n2. Thông báo đọc r hiện lại\n\n## Kết quả thực tế vs. kỳ vọng\n- **Thực tế:** thông báo đã đọc hiện đi hiện lạilại\n- **Kỳ vọng:** chỉ hiện thông báo mới ch đọc\n\n## Log (`--dev`)\n$ ./ctdozlite-0.1.1-x86_64.AppImage --gui --dev\n🔧 [DEV MODE ACTIVATED] – Quét log, chẩn đoán giao thức và trace chi tiết được bật.\n✓ Đã khôi phục phiên từ session.json: <REDACTED_USER> (UIN: <REDACTED_UIN>)\n🚀 Đang khởi chạy giao diện đồ họa ctdozlite (egui)…\n\n--- 1. Khởi tạo & Lấy cấu hình Server ---\n→ GET https://wpa.zaloapp.com/api/login/getServerInfo?bkt=-1&client_version=694&computer_name=DESKTOP&imei=<redacted>&sp_mtn=-1&type=30&signkey=<redacted>\n← 200 OK (error_code: 0 - Successful)\n\n--- 2. Kết nối kênh Polling / Realtime ---\n→ GET https://p5-ctl-chat.zaloapp.com/?zpw_ver=694&zpw_type=30&params=<redacted>&ts=1794564571\n→ GET https://p5-msg-chat.zaloapp.com/?zpw_ver=694&zpw_type=30&params=<redacted>&ts=1794564571..1794564578\n← 200 OK (data decrypted: 149 bytes -> 84,870 bytes)\n\n--- 3. Đồng bộ thông tin cá nhân & Danh sách bạn bè ---\n→ GET https://tt-friend-wpa.chat.zalo.me/api/friend/reqstatus?zpw_ver=694&zpw_type=30&params=<redacted>\n→ GET https://tt-profile-wpa.chat.zalo.me/api/social/profile/me-v2?zpw_ver=694&zpw_type=30&params=<redacted>&os=8\n← 200 OK (data decrypted: 121 bytes / 1,251 bytes)\n→ GET https://tt-profile-wpa.chat.zalo.me/api/social/profile/lastOnline?zpw_ver=694&zpw_type=30&params=<redacted>\n← 200 OK (data decrypted: 1,251 bytes)\n→ GET https://tt-friend-wpa.chat.zalo.me/api/social/friend/getfriends?zpw_ver=694&zpw_type=30&params=<redacted>\n← 404 Not Found (nginx) [Fallback sang endpoint bên dưới]\n→ GET https://tt-profile-wpa.chat.zalo.me/api/social/friend/getfriends?zpw_ver=694&zpw_type=30&params=<redacted>\n← 200 OK (data decrypted: 99,682 bytes)\n\n--- 4. Đồng bộ Nhóm & Thành viên nhóm ---\n→ GET https://tt-group-wpa.chat.zalo.me/api/group/getlg/v4?zpw_ver=694&zpw_type=30\n← 200 OK (data decrypted: 821 bytes / 1,917 bytes)\n→ POST https://tt-group-wpa.chat.zalo.me/api/group/getmg-v2?zpw_ver=694&zpw_type=30 (body: <REDACTED>)\n← 200 OK (data decrypted: 821 bytes -> 28,003 bytes)\n→ POST https://tt-profile-wpa.chat.zalo.me/api/social/group/members?zpw_ver=694&zpw_type=30 (body: <REDACTED>)\n← 200 OK (data decrypted: 245 bytes -> 28,003 bytes)\n→ POST https://tt-group-wpa.chat.zalo.me/api/group/seenv2?zpw_ver=694&zpw_type=30 (body: <REDACTED>)\n← 200 OK (data decrypted: 66 bytes)\n\n--- 5. Đồng bộ Hội thoại, Ghim & Tắt thông báo ---\n→ GET https://tt-convers-wpa.chat.zalo.me/api/preloadconvers/get-last-msgs?zpw_ver=694&zpw_type=30&params=<redacted>\n← 200 OK (data decrypted: 22,633 bytes -> 122,020 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK (data decrypted: 276 bytes -> 92,892 bytes)\n→ GET https://tt-profile-wpa.chat.zalo.me/api/social/profile/getmute?zpw_ver=694&zpw_type=30&params=<redacted>\n← 200 OK (data decrypted: 1,740 bytes)\n→ GET https://tt-convers-wpa.chat.zalo.me/api/pinconvers/list?zpw_ver=694&zpw_type=30\n← 200 OK (data decrypted: 139 bytes)\nKhông chắc log hữu ích\n```text\n\n```\n",
      "closedAt": "2026-11-15T02:34:18Z",
      "createdAt": "2026-11-13T10:11:32Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 146,
      "state": "CLOSED",
      "title": "bug: Fix tình trạng thông báo đã đọc hiện đi hiện lại"
    },
    {
      "author": {
        "id": "U_kgDOCwSJjw",
        "is_bot": false,
        "login": "Lean-Duong",
        "name": "Lucian Duong"
      },
      "body": "> **Trước khi mở issue:** Vui lòng kiểm tra bảng **[Tính năng & giới hạn (`docs/TINH_NANG.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/TINH_NANG.md)** và **[Lỗi đã biết (`docs/DA_BIET.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/DA_BIET.md)** để xem vấn đề đã được ghi nhận chưa.\n\n## Môi trường\n- **Hệ điều hành:** Windows 10 (bản 64-bit)\n- **Phiên bản `ctdozlite`:** bản cập nhật mới nhất (sau khi PR #78 được merge ngày 06/11/2026) — người dùng sẽ bổ sung số phiên bản chính xác nếu cần\n- **Chế độ chạy:** GUI (`--gui`)\n- **Người báo cáo:** @Lean-Duong\n\n## Các bước tái hiện\n1. Người kia gửi một tin nhắn (text / ảnh), sau đó **thu hồi** tin nhắn đó.\n2. Trong bản ctdozlite mới, bubble tin nhắn hiển thị **\"Tin nhắn đã được thu hồi\"** (chữ xiên gạch ngang) — nội dung gốc hoàn toàn biến mất.\n3. Đối chiếu với bản trước đó (theo issue #79 và PR #78 đã merge): tin nhắn thu hồi được giữ nguyên **text + media/ảnh/sticker**, hiển thị badge đỏ `🚫 Đã thu hồi` phía trên bubble, vẫn nhấp phóng to ảnh và sao chép văn bản được.\n\n## Kết quả thực tế vs. kỳ vọng\n- **Thực tế:** chỉ còn dòng \"Tin nhắn đã được thu hồi\", tính năng chống thu hồi (Anti-Recall) không còn hoạt động như trước.\n- **Kỳ vọng:** khôi phục lại hành vi như đã triển khai qua #78 — giữ nguyên nội dung gốc của tin nhắn đã thu hồi (text, media, link ảnh, sticker), kèm badge đánh dấu đã thu hồi.\n\n## Bối cảnh & tham chiếu\n- Feature gốc: #79 (Anti-Recall), đã được triển khai và merge trong PR #78.\n- PR #62 mục 4 có ghi chú *\"Chặn cache khôi phục nội dung và attachment của tin đã Recall\"* — có thể đây là nguyên nhân làm hành vi thay đổi ở bản mới.\n\n## Mong muốn\nTính năng **chống thu hồi / xem lại tin nhắn đã thu hồi** là một trong những tính năng **đặc biệt và khác biệt nhất** của ctdozlite so với Zalo PC chính thức — nó rất thiết thực cho việc nhắn tin hằng ngày (không bỏ lỡ nội dung khi đối phương thu hồi). Rất mong team cân nhắc mở lại tính năng này, hoặc ít nhất thêm một tuỳ chọn bật/tắt rõ ràng trong Cài đặt cho người dùng chủ động chọn.\n\n## Log (`--dev`)\nChưa có log liên quan; bubble thu hồi hiển thị bình thường, không thấy lỗi nào trên giao diện. Có thể bổ sung log `--gui --dev` khi tái hiện nếu team cần.\n",
      "closedAt": "2026-11-15T02:34:21Z",
      "createdAt": "2026-11-13T06:27:58Z",
      "labels": [],
      "number": 141,
      "state": "CLOSED",
      "title": "regression: Mất tính năng chống thu hồi tin nhắn (Anti-Recall) ở bản cập nhật mới"
    },
    {
      "author": {
        "id": "U_kgDOBX3z_w",
        "is_bot": false,
        "login": "Shigure-Ruiseki",
        "name": "Ruiseki"
      },
      "body": "### Summary\n\nCó thể tải và cập nhật ở Arch đơn giản và nhanh hơn.\n\n### Proposal\n\nNhư title.\n\n### Evidence\n\n- Evidence / capture:\n- Observed error_code:\n\n\n### Context\n\n- ADR / RFC: None\n- Alternatives considered:\n",
      "closedAt": null,
      "createdAt": "2026-11-13T03:31:31Z",
      "labels": [],
      "number": 139,
      "state": "OPEN",
      "title": "feat(install): Hỗ trợ AUR và FlatHub"
    },
    {
      "author": {
        "id": "U_kgDOFTk0dg",
        "is_bot": false,
        "login": "dinhhanh37k14-code",
        "name": ""
      },
      "body": "> **Trước khi mở issue:** Vui lòng kiểm tra bảng **[Tính năng & giới hạn (`docs/TINH_NANG.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/TINH_NANG.md)** và **[Lỗi đã biết (`docs/DA_BIET.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/DA_BIET.md)** để xem vấn đề đã được ghi nhận chưa.\n\n## Môi trường\n- **Hệ điều hành:** (ví dụ: Windows 11 23H2 / macOS 15.2 Apple Silicon / Ubuntu 24.04)\n- **Phiên bản `ctdozlite`:** (xem góc trên bên trái cửa sổ ứng dụng hoặc chạy `ctdozlite --version`)\n- **Chế độ chạy:** GUI (`--gui`) / TUI (`--tui`) / Headless (`--mcp` hoặc `--api-port`)\n\n## Các bước tái hiện\n1. ...\n2. ...\n3. ...\n\n## Kết quả thực tế vs. kỳ vọng\n- **Thực tế:** ...\n- **Kỳ vọng:** ...\n\n## Log (`--dev`)\nChạy lại với `ctdozlite --gui --dev` và dán đoạn log liên quan tại đây (nhớ xoá thông tin nhạy cảm / cookie nếu có):\n```text\n\n```\n",
      "closedAt": null,
      "createdAt": "2026-11-13T03:21:46Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 138,
      "state": "OPEN",
      "title": "bug: Chat chưa tag tên được, chưa ghim tin nhắn lên được, không xoá nhóm được, không thu hồi tin nhắn được, file chia sẻ đi người được chia sẻ sẽ không đọc được file"
    },
    {
      "author": {
        "id": "U_kgDOC0irsg",
        "is_bot": false,
        "login": "waynenguyen2212",
        "name": ""
      },
      "body": "<img width=\"670\" height=\"319\" alt=\"1K770360V_448BDM\" src=\"https://github.com/user-attachments/assets/01713b20-0f63-4167-8371-f4d9ddc02e20\" />\nBản 0.1.1 ăn CPU quá nhiều",
      "closedAt": "2026-11-13T02:04:04Z",
      "createdAt": "2026-11-12T04:27:55Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 135,
      "state": "CLOSED",
      "title": "bug: CPU 100%"
    },
    {
      "author": {
        "id": "U_kgDOCbrkvQ",
        "is_bot": false,
        "login": "tinzisreal",
        "name": "Nguyen Huu Tung"
      },
      "body": "### Summary\n\n<img width=\"511\" height=\"82\" alt=\"image\" src=\"https://github.com/user-attachments/assets/3fd1052b-1595-41bd-be94-10c3d3b23729\" />\n\n### Proposal\n\nbị lỗi UI khi active search\n\n<img width=\"511\" height=\"82\" alt=\"image\" src=\"https://github.com/user-attachments/assets/1bad0f32-cde4-4990-a55e-2716c6cdb3c9\" />\n\n### Evidence\n\n- Evidence / capture:\n- Observed error_code:\n\n\n### Context\n\n- ADR / RFC: None\n- Alternatives considered:\n",
      "closedAt": "2026-11-11T18:29:38Z",
      "createdAt": "2026-11-11T12:29:44Z",
      "labels": [],
      "number": 133,
      "state": "CLOSED",
      "title": "feat(<scope>): <summary> bị lỗi UI khi active search"
    },
    {
      "author": {
        "id": "U_kgDOCwSJjw",
        "is_bot": false,
        "login": "Lean-Duong",
        "name": "Lucian Duong"
      },
      "body": "### Summary\r\n\r\nAvatar của liên hệ/nhóm trong danh sách hội thoại (sidebar trái) hiển thị bị **phóng to quá mức và cắt xén nội dung**: chỉ thấy phần giữa ảnh, phần xung quanh (mặt, logo, chữ, hoạ tiết) bị cắt mất.\r\n\r\n- Không xảy ra với tất cả avatar. Chủ yếu là **avatar mặc định của Zalo gốc**; đôi khi ảnh do người dùng tự tải lên cũng bị.\r\n- Hậu quả: không nhìn/nhận diện nhanh được nhóm của mình, danh sách hội thoại dài càng khó phân biệt.\r\n\r\nKỳ vọng: avatar hiển thị đủ nội dung và đúng tỉ lệ như Zalo PC / Zalo điện thoại, không bị zoom/cắt quá mức.\r\n\r\n### Reproduction\r\n\r\n1. Mở ctdozlite (GUI) trên Windows 10 và đăng nhập.\r\n2. Xem avatar các liên hệ/nhóm ở sidebar trái, chú ý các tài khoản dùng avatar mặc định của Zalo.\r\n3. So sánh với cùng tài khoản trên Zalo PC/điện thoại: bên ctdozlite ảnh bị phóng to, phần nội dung bị cắt.\r\n4. Kiểm tra thêm một số avatar là ảnh tự tải lên — một vài ảnh cũng bị tương tự, số khác vẫn bình thường.\r\n\r\nTái hiện: ổn định với các avatar bị ảnh hưởng (không phải toàn bộ danh sách).\r\n\r\n### Area\r\n\r\nui\r\n\r\n### Server error_code\r\n\r\nKhông có — không phải lỗi phía server, thuần hiển thị.\r\n\r\n### Environment\r\n\r\n- OS / Version: Windows 10\r\n- ctdozlite version: v0.1.0\r\n- Interface: GUI\r\n- Logs (redacted): không có log lỗi (không phải lỗi runtime)\r\n\r\n### Ghi chú\r\n\r\n- Ảnh chụp minh hoạ (vùng avatar trong sidebar) sẽ được dán vào bình luận ngay dưới issue này.\r\n- Liên quan #77: cùng vùng render avatar (`src/gui/theme.rs`, `paint_circular_image`) đã sửa tỉ lệ/crop trước đây — có thể là regression hoặc trường hợp chưa xử lý (ảnh không vuông / avatar mặc định của Zalo).\r\n- Không đính kèm token, cookie, IMEI hay số điện thoại.\r\n\r\n",
      "closedAt": "2026-11-11T18:29:41Z",
      "createdAt": "2026-11-10T14:57:46Z",
      "labels": [],
      "number": 131,
      "state": "CLOSED",
      "title": "bug(gui): avatar liên hệ/nhóm bị phóng to và cắt xén nội dung (chủ yếu avatar mặc định của Zalo)"
    },
    {
      "author": {
        "id": "U_kgDOCwSJjw",
        "is_bot": false,
        "login": "Lean-Duong",
        "name": "Lucian Duong"
      },
      "body": "### Summary\n\nHai vấn đề nằm cùng luồng đăng xuất / đăng nhập, nên gộp chung một issue:\n\n1. **Đăng xuất không có bước xác nhận.** Trong Cài đặt và menu hồ sơ, nút \"Đăng xuất tài khoản\" thực hiện ngay khi bấm một lần — cả 3 điểm vào đều đặt `do_logout = true` rồi gửi `WorkerCmd::Logout` luôn. Với trackpad hoặc màn hình cảm ứng, một cú chạm nhầm là phiên bị xoá ngay và phải quét lại mã QR.\n2. **Chưa có danh sách tài khoản đã lưu + đăng nhập nhanh.** Multi-account đã có sẵn (`accounts/<uin>.json`, `WorkerCmd::SwitchAccount` chuyển tài khoản **không cần QR**), nhưng danh sách này chỉ xuất hiện trong Cài đặt khi đang đăng nhập. Sau khi đăng xuất, màn hình login chỉ còn QR. Mong muốn: trải nghiệm như Facebook — tài khoản vẫn nằm trong danh sách sau khi đăng xuất, mở app bấm một lần là vào lại, không phải quét mã lại.\n\nKết quả kỳ vọng:\n\n- Mọi hành động đăng xuất đều phải xác nhận trước khi thực hiện.\n- Có danh sách tài khoản đã dùng (đang đăng nhập hoặc đã đăng xuất), phân biệt rõ \"Đăng xuất\" (giữ tài khoản để đăng nhập nhanh) và \"Xoá tài khoản khỏi thiết bị\".\n\n### Proposal\n\n#### A. Xác nhận trước khi đăng xuất\n\n- Thêm hộp thoại xác nhận cho **mọi điểm vào đăng xuất** trong GUI (`src/gui/modals.rs`): menu hồ sơ (~dòng 842), thanh điều hướng Cài đặt (~dòng 3156), panel tài khoản trong Cài đặt (~dòng 4239).\n- Tái dùng đúng pattern đã có cho \"Xoá dữ liệu tin nhắn cục bộ\": cờ `*_confirm_open` + `render_wipe_confirm` (`src/gui/modals.rs:15108`). Thêm `logout_confirm_open` và `render_logout_confirm`, không cần cơ chế mới.\n- Nội dung hộp thoại:\n  - Tiêu đề: \"Đăng xuất tài khoản?\"\n  - Mô tả: phiên đăng nhập hiện tại sẽ kết thúc; lịch sử tin nhắn vẫn được giữ; có thể đăng nhập lại bằng tài khoản đã lưu hoặc quét mã QR.\n  - \"Huỷ\" là lựa chọn mặc định (Esc đóng, click ra ngoài đóng); \"Đăng xuất\" dùng `pal.danger` và **không** nhận focus mặc định để Enter không vô tình kích hoạt.\n- Tuỳ chọn (chọn một, không cần cả hai): bấm 2 lần kiểu \"Bấm lần nữa để đăng xuất\" (tự reset sau ~3s) thay cho hộp thoại ở menu hồ sơ; hoặc checkbox \"Không hỏi lại\" (mặc định tắt).\n\n#### B. Lưu tài khoản đã đăng nhập / đăng xuất + đăng nhập nhanh (kiểu Facebook)\n\nNền tảng đã có, nên đây là mở rộng vòng đời + UI, không phải viết lại:\n\n- `src/account_manager.rs`: `save_current_account_snapshot`, `list_saved_accounts`, `switch_to_account`, `delete_saved_account` — snapshot gồm `session.json`, `threads_cache.json`, `zalo_history.db` cho từng `<uin>`.\n- `WorkerCmd::Logout` đã lưu snapshot trước khi xoá file gốc (`src/gui/worker.rs:1963-1968`), và `WorkerCmd::SwitchAccount` khôi phục session **không cần QR**.\n- Thiếu: màn hình login không hiển thị danh sách tài khoản (`list_saved_accounts` hiện chỉ dùng trong Cài đặt, `modals.rs:4353`); chưa phân biệt đăng xuất vs xoá tài khoản khỏi máy (`delete_saved_account` chỉ nằm trong Cài đặt, `modals.rs:4529`); chưa có trạng thái \"phiên còn hiệu lực / cần đăng nhập lại\".\n\nPhần cần bổ sung:\n\n- **Màn hình đăng nhập**: hiển thị danh sách account card (avatar, tên, \"hoạt động gần nhất\", badge trạng thái phiên). Một chạm:\n  - phiên còn hiệu lực → `WorkerCmd::SwitchAccount`, vào thẳng danh sách hội thoại;\n  - phiên hết hạn → mở QR nhưng **giữ nguyên card**, đăng nhập xong cập nhật lại snapshot.\n- **Đăng xuất** = kết thúc phiên nhưng giữ tài khoản trong danh sách. Thêm hành động riêng **\"Xoá tài khoản khỏi thiết bị\"** (gọi `delete_saved_account`) với xác nhận mạnh hơn (2 bước / gõ lại tên), không gộp chung nút đăng xuất.\n- **Trạng thái phiên**: thêm field trạng thái vào account card; khi Zalo trả `error_code` phiên hết hạn (vd. 114 / 600) thì đánh dấu \"cần đăng nhập lại\" và fallback QR — không xoá account.\n- **Cài đặt**: hiển thị danh sách tài khoản đã lưu kèm dung lượng `accounts/` và nút \"Xoá toàn bộ tài khoản đã lưu\".\n- **Bảo mật (cần chốt trước khi ship)**: `accounts/<uin>*.json` chứa `zpw_sek` / `zpw_enk` còn sống, liên quan trực tiếp `SESSION_SECURITY.md`. Đề xuất: mã hoá at rest (OS keyring / DPAPI), tuỳ chọn \"Lưu tài khoản trên thiết bị\" kèm cảnh báo, và nút xoá nhanh toàn bộ. Không đưa credential thật vào issue/PR.\n- **TUI**: thêm lệnh chọn tài khoản đã lưu tương ứng, tối thiểu hiển thị danh sách để hành vi hai giao diện đồng nhất.\n\nNon-goals:\n\n- Không đổi format `session.json` hay cấu trúc snapshot `accounts/<uin>.*`.\n- Không thêm đăng nhập bằng mật khẩu; QR vẫn là cách đăng nhập chính.\n- Không tự động đăng nhập khi mở app — người dùng chủ động chọn tài khoản.\n- Không đồng bộ danh sách tài khoản qua mạng / giữa các máy.\n\n### Evidence\n\n- Phản hồi người dùng: đang mở Cài đặt, chạm tay vào nút đăng xuất → bị đăng xuất ngay; sau đó đăng nhập lại bằng quét QR được, dữ liệu tin nhắn vẫn còn (snapshot `accounts/<uin>_history.db` được lưu trước khi xoá file gốc).\n- Đề xuất lưu tài khoản để không phải quét QR lại mỗi lần cũng đã được nêu ở #80 (đang mở) — issue này bổ sung phần UI + vòng đời đăng xuất.\n- Code: `src/gui/modals.rs` — \"Đăng xuất tài khoản\" (~842), \"Đăng xuất\" ở sidebar (~3156), \"Đăng xuất tài khoản này\" (~4239) đặt `do_logout = true` rồi gửi `WorkerCmd::Logout`; ngược lại `do_wipe` đã hỏi lại trước (`modals.rs:973-979` — \"Destructive and irreversible, so it asks first\").\n- Observed error_code: không có — không phải lỗi phía server, đây là vấn đề UX/luồng phía client.\n\n### Context\n\n- ADR / RFC: cần ADR cho việc lưu credential at rest (mã hoá, mặc định bật/tắt, cách xoá) trước khi phát triển phần B; phần A không cần ADR.\n- Alternatives considered:\n  - Toast \"Đã đăng xuất — Hoàn tác\": không khả thi về mặt bảo mật, tương đương giữ credential sống trên máy mà người dùng tưởng đã đăng xuất; nếu muốn \"hoàn tác\" thì nên đi theo hướng B (giữ tài khoản, đăng nhập nhanh) với thông báo rõ.\n  - Chỉ dời/ẩn nút đăng xuất vào submenu: giảm khả năng chạm nhầm nhưng không chặn được.\n  - Giữ `session.json` sau khi đăng xuất thay vì snapshot trong `accounts/`: làm mất ý nghĩa của \"đăng xuất\", khó phân biệt đã đăng xuất hay chưa, và dễ khiến người dùng hiểu nhầm là đã thoát phiên.\n  - Hộp thoại xác nhận + danh sách tài khoản đã lưu (phương án chọn): rẻ, đồng nhất với luồng xoá dữ liệu đã có, đúng trải nghiệm quen thuộc (Facebook, Zalo PC).\n",
      "closedAt": "2026-11-11T18:29:44Z",
      "createdAt": "2026-11-10T14:46:12Z",
      "labels": [],
      "number": 130,
      "state": "CLOSED",
      "title": "feat(auth): xác nhận trước khi đăng xuất và đăng nhập nhanh bằng tài khoản đã lưu (kiểu Facebook)"
    },
    {
      "author": {
        "id": "U_kgDOEOeK_A",
        "is_bot": false,
        "login": "Dragon-pixel-archive",
        "name": ""
      },
      "body": "\n## Môi trường\n- **Hệ điều hành:** Windows 10\n- **Phiên bản `ctdozlite`:** v0.1.0\n- **Chế độ chạy:** app\n\n## Các bước tái hiện\n1. đăng nhập tài khoản\n2. mở cloud cá nhân\n3. mở tường nhà và nhật kí bạn bè\n4. chọn đồng bộ tin nhắn\n5. gửi tin nhắn từ tài khoản khác và thu hồi\n\n## Kết quả thực tế vs. kỳ vọng\n- **Thực tế:** \n- could cá nhân ko hiện tin nhắn văn bản, chỉ hiện thời gian\n- tường nhà và nhật kí bạn bè không hiện ảnh, chỉ hiện các skeleton, nếu mở link ảnh bằng trình duyệt ngoài thì vẫn thấy đc ảnh bth\n- đồng bộ tin nhắn không thành công\n- hiện rất nhiều event thu hồi tin nhắn từ người gửi\n- **Kỳ vọng:** \n- fix lỗi hiện cloud cá nhân\n- tường nhà và nhật kí bạn bè hiện ảnh\n- sửa lỗi đồng bộ tin nhắn\n- chỉ hiện một lần event thu hồi tin nhắn\n\n## Rất mong admin phát triển và hoàn thiện app vì đây là một ứng dụng rất tiềm năng và tuyệt vời\n\nảnh lỗi tin nhắn cloud\n<img width=\"979\" height=\"479\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/4e3cb781-2086-4820-a90d-f275f78098d5\" />\nlỗi tường nhà\n<img width=\"606\" height=\"885\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/aea262f3-48fb-4216-99de-91c3cb3daf3b\" />\nlỗi đồng bộ\n<img width=\"1909\" height=\"39\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/223dba33-7cd0-4414-baae-267a4dd53e68\" />\nlỗi event thu hồi\n<img width=\"525\" height=\"739\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/5a736f40-2ba2-4cf2-8c8f-99663494bfac\" />",
      "closedAt": "2026-11-11T18:29:47Z",
      "createdAt": "2026-11-09T06:35:24Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 128,
      "state": "CLOSED",
      "title": "bug: lỗi hiển thị và đồng bộ"
    },
    {
      "author": {
        "is_bot": true,
        "login": "app/coderabbitai"
      },
      "body": "## Summary\n\nContact aliases that resemble generated placeholders, such as `123` or `Nhóm học`, may not replace an older thread name in SQLite. If the thread cache is unavailable during an offline restart, the older name can be displayed.\n\n## Required change\n\nUpdate the thread persistence path in `src/store.rs` and related contact-name handling in `src/chat.rs` so that persistence can distinguish contact aliases from generated placeholders. Preserve the safeguard that prevents generated placeholders from overwriting stored names.\n\n## Acceptance criteria\n\n- Persist and restore numeric aliases and aliases beginning with `Nhóm ` when the thread cache is unavailable.\n- Do not let generated placeholders replace existing names.\n- Add regression tests for persistence and offline restoration, including both alias examples and placeholder protection.\n\nRequested by @gr219 as a follow-up to https://github.com/ctdoteam1110/ctdozlite/pull/125.\n\nReview finding: https://github.com/ctdoteam1110/ctdozlite/pull/125#discussion_r4509078210",
      "closedAt": "2026-11-11T18:29:50Z",
      "createdAt": "2026-11-09T03:15:36Z",
      "labels": [],
      "number": 126,
      "state": "CLOSED",
      "title": "Preserve placeholder-like contact aliases across offline restarts"
    },
    {
      "author": {
        "id": "U_kgDOCWTD4Q",
        "is_bot": false,
        "login": "ngthzuan",
        "name": ""
      },
      "body": "<img width=\"1069\" height=\"750\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/0622076e-0989-472c-ae3e-36d4f6ff17d2\" />\nBị như hình-=))\n",
      "closedAt": "2026-11-08T17:12:20Z",
      "createdAt": "2026-11-08T10:23:35Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 123,
      "state": "CLOSED",
      "title": "bug:"
    },
    {
      "author": {
        "id": "U_kgDOCIdiXA",
        "is_bot": false,
        "login": "Trinity030-dev",
        "name": ""
      },
      "body": "win 11 26h2\nLỗi gửi lại thông báo cũ khi mở lại app trên win.\nthông báo của tin nhắn đã đọc và cuộc gọi cũ cũng bị gửi lại",
      "closedAt": "2026-11-08T17:00:36Z",
      "createdAt": "2026-11-08T06:51:39Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 122,
      "state": "CLOSED",
      "title": "gửi lại thông báo cũ khi mở app"
    },
    {
      "author": {
        "id": "U_kgDOClNNZA",
        "is_bot": false,
        "login": "bao2222-hub",
        "name": ""
      },
      "body": "## Môi trường\n- **Hệ điều hành: Windows 11 26H2 Insider Preview (26340.9843)\n- **Phiên bản `ctdozlite`:** v 0.0.9 Latest\n- **Chế độ chạy:** GUI (`--gui`) \n\n## Các bước tái hiện\n1. nhấn vào đồng bộ tin nhắn trên điện thoại trên ctdozlite\n2. điện thoại nhận được yêu cầu đồng bộ tin nhắn\n3. app Zalo trên điện thoại crash ngay khi nhấn \"đồng bộ ngay\"\n\n## Kết quả thực tế vs. kỳ vọng\n- **Thực tế:** Ctdozlite treo hoặc không đồng bộ được tin nhắn\n- **Kỳ vọng:** đồng bộ được tin nhắn\n\n## Log (`--dev`)\ntreo ở khi nhấn \"đồng bộ tin nhắn) ```text\n  data decrypted (2706 bytes)\n→ GET https://p5-ctl-chat.zaloapp.com/?zpw_ver=694&zpw_type=30&params=<redacted>&ts=1794103438\n→ GET https://p5-msg-chat.zaloapp.com/?zpw_ver=694&zpw_type=30&params=<redacted>&ts=1794103438\n← 200 OK {\"error_code\":0,\"error_message\":\"\",\"data\":\"ahDNjEyWygKDAV8T2JMFL7txo9Qpew/OZe/Rj/hVRDHAVfQEM0PDvIc1n4ZDqL+rVVRWz6v4fwucH9GzUTicRnRWLk66xFzFy5+ZhvNbbZvCBkQSOAfCkJsiyZQj/DxUjohNSfWDd/KI5BOoOwxPFQpq81zdp+C18TCu+XANag+nBxf/tHjszA6rTemMlCvrNOff49v2jH/N4XYTTlKXXpJMGrL5UDtw4Cv9iCoWSFAWCVSYnFunPhkP8/bcgb+LnH9nYa1gyg5AcNXl9GQ5ZQ/z0nQ4+Oea3hUYPT0GyrPqSzy4YEz0sIqsl0HjmnVbParHDAHBadvIbnvRfw4uqamXkTWk4YFDBfmBr…\n  data decrypted (545 bytes)\n→ GET https://p5-msg-chat.zaloapp.com/?zpw_ver=694&zpw_type=30&params=<redacted>&ts=1794103439\n← 200 OK {\"error_code\":0,\"error_message\":\"\",\"data\":\"ahDNjEyWygKDAV8T2JMFL7txo9Qpew/OZe/Rj/hVRDHAVfQEM0PDvIc1n4ZDqL+rVVRWz6v4fwucH9GzUTicRnRWLk66xFzFy5+ZhvNbbZvCBkQSOAfCkJsiyZQj/DxUjohNSfWDd/KI5BOoOwxPFQpq81zdp+C18TCu+XANag+nBxf/tHjszA6rTemMlCvrNOff49v2jH/N4XYTTlKXXpJMGrL5UDtw4Cv9iCoWSFAWCVSYnFunPhkP8/bcgb+LnH9nYa1gyg5AcNXl9GQ5ZQ/z0nQ4+Oea3hUYPT0GyrPqSzy4YEz0sIqsl0HjmnVbParHDAHBadvIbnvRfw4uqamXkTWk4YFDBfmBr…\n  data decrypted (545 bytes)\n← 200 OK {\"error_code\":0,\"error_message\":\"\",\"data\":\"ahDNjEyWygKDAV8T2JMFL7txo9Qpew/OZe/Rj/hVRDHAVfQEM0PDvIc1n4ZDqL+rN4cwjJVsNYWzqQSo2sP4kuuZrJf9C4OZlLNh33UQxuZ7+DfOS05EBTaQDdBS3RmDKGJAIU1HoemKaj8wQiTlvwD6ubNEgNIM0H59Yfi4jk37OT2msrkKeA4LVETOI19YWBKSFk7QnGUElCD/hvNIzN2hnbTsghhlKf0kYexqWmRTbp3rocRTZOdidNYqu09X1N3JuIahs4OsmZdP5w4sscdMHhy/SNhiha4tetPT2Ed5fWIneFSef/GVQHTPwar26nt+q8tIPHIcCSbJ3mjNkhYa16UTTYD8+3QYI…\n  data decrypted (688 bytes)\n→ GET https://p5-ctl-chat.zaloapp.com/?zpw_ver=694&zpw_type=30&params=<redacted>&ts=1794103440\n→ GET https://p5-msg-chat.zaloapp.com/?zpw_ver=694&zpw_type=30&params=<redacted>&ts=1794103441\n← 200 OK {\"error_code\":0,\"error_message\":\"\",\"data\":\"ahDNjEyWygKDAV8T2JMFL7txo9Qpew/OZe/Rj/hVRDHAVfQEM0PDvIc1n4ZDqL+rVVRWz6v4fwucH9GzUTicRnRWLk66xFzFy5+ZhvNbbZvCBkQSOAfCkJsiyZQj/DxUjohNSfWDd/KI5BOoOwxPFQpq81zdp+C18TCu+XANag+nBxf/tHjszA6rTemMlCvrNOff49v2jH/N4XYTTlKXXpJMGrL5UDtw4Cv9iCoWSFAWCVSYnFunPhkP8/bcgb+LnH9nYa1gyg5AcNXl9GQ5ZQ/z0nQ4+Oea3hUYPT0GyrPqSzy4YEz0sIqsl0HjmnVbParHDAHBadvIbnvRfw4uqamXkTWk4YFDBfmBr…\n  data decrypted (545 bytes)\n→ GET https://p5-msg-chat.zaloapp.com/?zpw_ver=694&zpw_type=30&params=<redacted>&ts=1794103442\n← 200 OK {\"error_code\":0,\"error_message\":\"\",\"data\":\"ahDNjEyWygKDAV8T2JMFL7txo9Qpew/OZe/Rj/hVRDHAVfQEM0PDvIc1n4ZDqL+rVVRWz6v4fwucH9GzUTicRnRWLk66xFzFy5+ZhvNbbZvCBkQSOAfCkJsiyZQj/DxUjohNSfWDd/KI5BOoOwxPFQpq81zdp+C18TCu+XANag+nBxf/tHjszA6rTemMlCvrNOff49v2jH/N4XYTTlKXXpJMGrL5UDtw4Cv9iCoWSFAWCVSYnFunPhkP8/bcgb+LnH9nYa1gyg5AcNXl9GQ5ZQ/z0nQ4+Oea3hUYPT0GyrPqSzy4YEz0sIqsl0HjmnVbParHDAHBadvIbnvRfw4uqamXkTWk4YFDBfmBr…\n  data decrypted (545 bytes)\n→ GET https://p5-msg-chat.zaloapp.com/?zpw_ver=694&zpw_type=30&params=<redacted>&ts=1794103443\n← 200 OK {\"error_code\":0,\"error_message\":\"\",\"data\":\"ahDNjEyWygKDAV8T2JMFL7txo9Qpew/OZe/Rj/hVRDHAVfQEM0PDvIc1n4ZDqL+rVVRWz6v4fwucH9GzUTicRnRWLk66xFzFy5+ZhvNbbZvCBkQSOAfCkJsiyZQj/DxUjohNSfWDd/KI5BOoOwxPFQpq81zdp+C18TCu+XANag+nBxf/tHjszA6rTemMlCvrNOff49v2jH/N4XYTTlKXXpJMGrL5UDtw4Cv9iCoWSFAWCVSYnFunPhkP8/bcgb+LnH9nYa1gyg5AcNXl9GQ5ZQ/z0nQ4+Oea3hUYPT0GyrPqSzy4YEz0sIqsl0HjmnVbParHDAHBadvIbnvRfw4uqamXkTWk4YFDBfmBr…\n  data decrypted (545 bytes)\n→ GET https://p5-msg-chat.zaloapp.com/?zpw_ver=694&zpw_type=30&params=<redacted>&ts=1794103444\n← 200 OK {\"error_code\":0,\"error_message\":\"\",\"data\":\"ahDNjEyWygKDAV8T2JMFL7txo9Qpew/OZe/Rj/hVRDHAVfQEM0PDvIc1n4ZDqL+rVVRWz6v4fwucH9GzUTicRnRWLk66xFzFy5+ZhvNbbZvCBkQSOAfCkJsiyZQj/DxUjohNSfWDd/KI5BOoOwxPFQpq81zdp+C18TCu+XANag+nBxf/tHjszA6rTemMlCvrNOff49v2jH/N4XYTTlKXXpJMGrL5UDtw4Cv9iCoWSFAWCVSYnFunPhkP8/bcgb+LnH9nYa1gyg5AcNXl9GQ5ZQ/z0nQ4+Oea3hUYPT0GyrPqSzy4YEz0sIqsl0HjmnVbParHDAHBadvIbnvRfw4uqamXkTWk4YFDBfmBr…\n  data decrypted (545 bytes)\n→ GET https://p5-msg-chat.zaloapp.com/?zpw_ver=694&zpw_type=30&params=<redacted>&ts=1794103445\n← 200 OK {\"error_code\":0,\"error_message\":\"\",\"data\":\"ahDNjEyWygKDAV8T2JMFL7txo9Qpew/OZe/Rj/hVRDHAVfQEM0PDvIc1n4ZDqL+rVVRWz6v4fwucH9GzUTicRnRWLk66xFzFy5+ZhvNbbZvCBkQSOAfCkJsiyZQj/DxUjohNSfWDd/KI5BOoOwxPFQpq81zdp+C18TCu+XANag+nBxf/tHjszA6rTemMlCvrNOff49v2jH/N4XYTTlKXXpJMGrL5UDtw4Cv9iCoWSFAWCVSYnFunPhkP8/bcgb+LnH9nYa1gyg5AcNXl9GQ5ZQ/z0nQ4+Oea3hUYPT0GyrPqSzy4YEz0sIqsl0HjmnVbParHDAHBadvIbnvRfw4uqamXkTWk4YFDBfmBr…\n  data decrypted (545 bytes)\n→ GET https://p5-msg-chat.zaloapp.com/?zpw_ver=694&zpw_type=30&params=<redacted>&ts=1794103447\n← 200 OK {\"error_code\":0,\"error_message\":\"\",\"data\":\"ahDNjEyWygKDAV8T2JMFL7txo9Qpew/OZe/Rj/hVRDHAVfQEM0PDvIc1n4ZDqL+rVVRWz6v4fwucH9GzUTicRnRWLk66xFzFy5+ZhvNbbZvCBkQSOAfCkJsiyZQj/DxUjohNSfWDd/KI5BOoOwxPFQpq81zdp+C18TCu+XANag+nBxf/tHjszA6rTemMlCvrNOff49v2jH/N4XYTTlKXXpJMGrL5UDtw4Cv9iCoWSFAWCVSYnFunPhkP8/bcgb+LnH9nYa1gyg5AcNXl9GQ5ZQ/z0nQ4+Oea3hUYPT0GyrPqSzy4YEz0sIqsl0HjmnVbParHDAHBadvIbnvRfw4uqamXkTWk4YFDBfmBr…\n  data decrypted (545 bytes)\n```\n",
      "closedAt": "2026-11-08T17:07:41Z",
      "createdAt": "2026-11-08T02:04:45Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 120,
      "state": "CLOSED",
      "title": "bug:"
    },
    {
      "author": {
        "id": "U_kgDOCIdiXA",
        "is_bot": false,
        "login": "Trinity030-dev",
        "name": ""
      },
      "body": "<img width=\"122\" height=\"789\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/080cae27-8a60-48ac-9efe-445da72b3c58\" /> đã đọc tin nhưng khi mở app vô lại các dấu chọn báo tin nhắn không biến mất mà vẫn hiện như cũ\n",
      "closedAt": "2026-11-08T12:41:39Z",
      "createdAt": "2026-11-08T00:08:19Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 119,
      "state": "CLOSED",
      "title": "Lỗi đã đọc tin nhắn rồi nhưng vẫn còn báo chưa đọc"
    },
    {
      "author": {
        "id": "U_kgDOCIdiXA",
        "is_bot": false,
        "login": "Trinity030-dev",
        "name": ""
      },
      "body": "<img width=\"691\" height=\"367\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/13be682e-d722-4d09-811c-1f02e487e69d\" />\nlỗi không hiển thị ảnh mà hiện link trong tin nhắn gửi",
      "closedAt": "2026-11-08T13:04:56Z",
      "createdAt": "2026-11-08T00:06:46Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 118,
      "state": "CLOSED",
      "title": "lỗi không hiển thị ảnh mà hiện link trong tin nhắn gửi"
    },
    {
      "author": {
        "id": "U_kgDOCIdiXA",
        "is_bot": false,
        "login": "Trinity030-dev",
        "name": ""
      },
      "body": "win 26h2 win 11\nbug tin nhắn đọc rồi nhưng khi vào lại vẫn hiện số đỏ bên phải hộp tin nhắn \nCác tin nhắn group đọc hết vẫn hiện số đỏ báo tin nhắn mới kế bên như cũ gây nhầm lẫn cho người sử dụng",
      "closedAt": "2026-11-08T00:07:25Z",
      "createdAt": "2026-11-08T00:04:30Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 117,
      "state": "CLOSED",
      "title": "bug:"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjU3MzY3Njc0",
        "is_bot": false,
        "login": "nguyenvanhuy0612",
        "name": "Nguyen Van Huy"
      },
      "body": "## Môi trường\n- **Hệ điều hành:** Windows 11 (x86_64)\n- **Phiên bản `ctdozlite`:** v0.0.8 (bản portable phát hành sẵn)\n- **Chế độ chạy:** GUI (`--gui --dev`)\n- **Môi trường phụ trợ:** Node.js v26.13.0\n\n## Tóm tắt lỗi\nKhi thực hiện tính năng \"Đồng bộ tin nhắn từ điện thoại ngay\" (`WorkerCmd::SyncPhoneMessages`), điện thoại có nhận được thông báo và người dùng đã bấm xác nhận đồng bộ, tuy nhiên phía client PC không thể giải mã và không kéo được tin nhắn về `zalo_history.db`.\n\nKhi kiểm tra luồng xử lý, `src/history_sync.rs` gọi lệnh `node sync_v2_helper.js gen-keys` để sinh cặp khóa Curve25519 cho Sync v2. Tuy nhiên, `sync_v2_helper.js` đang trỏ cứng tới đường dẫn WebAssembly cục bộ không tồn tại trên bản phát hành, dẫn tới lỗi `ENOENT`. Khi lệnh Node.js thất bại, code Rust fallback sang sinh khóa public key ngẫu nhiên không có private key tương ứng, làm cho client không thể giải mã gói tin mã hóa từ điện thoại.\n\n## Các bước tái hiện\n1. Mở `ctdozlite` trên Windows 11 (`ctdozlite.exe --gui --dev`).\n2. Đăng nhập bằng mã QR qua ứng dụng Zalo trên điện thoại.\n3. Vào **Cài đặt** -> **Đồng bộ tin nhắn**.\n4. Bấm **\"Đồng bộ tin nhắn từ điện thoại ngay\"** (`WorkerCmd::SyncPhoneMessages`).\n5. Bấm xác nhận đồng bộ trên điện thoại.\n6. Quan sát phía client PC: tiến trình đồng bộ bị treo, báo timeout hoặc kết thúc với 0 tin nhắn được thêm vào.\n\nKiểm tra trực tiếp file helper ngoài terminal:\n```bash\nnode sync_v2_helper.js gen-keys\n```\nKết quả trả về:\n```text\nError: ENOENT: no such file or directory, open 'C:\\Users\\...\\temp\\asar_extracted\\pc-dist\\libs\\libzproto_wasm_bg.ecc0555ba9d0f8867a4699ce943ec745.wasm'\n    at Object.openSync (node:fs:782:18)\n    at Object.readFileSync (node:fs:628:35)\n    at initWasm (.../sync_v2_helper.js:11:28)\n    at generateV2Keys (.../sync_v2_helper.js:18:11)\n```\n\n## Kết quả thực tế vs. kỳ vọng\n- **Thực tế:** Lệnh sinh khóa trong `sync_v2_helper.js` bị lỗi `ENOENT` do đường dẫn `WASM_PATH` trỏ tới `../asar_extracted/...` (thư mục giải nén trên máy phát triển của tác giả). Code trong `src/chat.rs` fallback sang `generate_sync_v2_ed25519_public_key` tạo public key ngẫu nhiên không có private key tương ứng, khiến dữ liệu nhận từ điện thoại không thể giải mã được.\n- **Kỳ vọng:** `sync_v2_helper.js` tìm được file WASM hoặc logic sinh khóa Curve25519 / Ed25519 được cài đặt bằng thư viện Rust thuần (như `curve25519-dalek` / `ed25519-dalek`), giúp client hoàn tất bắt tay Sync v2 và giải mã tin nhắn độc lập không phụ thuộc Node.js hay file WASM bên ngoài.\n\n## Đoạn code liên quan\n- `sync_v2_helper.js:5`: Đường dẫn `WASM_PATH` bị hardcode:\n  ```javascript\n  const WASM_PATH = path.resolve(__dirname, '../asar_extracted/pc-dist/libs/libzproto_wasm_bg.ecc0555ba9d0f8867a4699ce943ec745.wasm');\n  ```\n- `src/history_sync.rs:744-754`: Hàm `generate_sync_v2_keys()` gọi Node.js và fail khi script ném lỗi.\n- `src/chat.rs:7097-7103`: Fallback sinh khóa giả khi lệnh thất bại:\n  ```rust\n  let (ik_pub, ek_pub) = match crate::history_sync::generate_sync_v2_keys() {\n      Ok(k) => (k.ik_pub, k.ek_pub),\n      Err(_) => (\n          crate::history_sync::generate_sync_v2_ed25519_public_key(0xA1),\n          crate::history_sync::generate_sync_v2_ed25519_public_key(0xB2),\n      ),\n  };\n  ```\n- `src/history_sync.rs:777-787`: Hàm `import_sync_v2_meta()` cũng gọi `sync_v2_helper.js decrypt-meta` và gặp lỗi tương tự khi giải mã metadata.\n\n## Đề xuất giải pháp\n1. Viết lại phần sinh khóa và bắt tay Sync v2 bằng crate Rust thuần (`curve25519-dalek` / `ed25519-dalek`) để client chạy độc lập hoàn toàn, không phụ thuộc Node.js.\n2. Nếu vẫn cần dùng WASM của Zalo, nên đóng gói file `libzproto_wasm_bg.wasm` đi kèm trong thư mục phát hành hoặc hỗ trợ cơ chế quét tự động thư mục cài đặt Zalo PC gốc trên máy (`%LOCALAPPDATA%\\Programs\\Zalo`).\n",
      "closedAt": "2026-11-08T13:05:00Z",
      "createdAt": "2026-11-07T08:14:13Z",
      "labels": [],
      "number": 116,
      "state": "CLOSED",
      "title": "bug: Đồng bộ tin nhắn điện thoại thất bại do đường dẫn WASM bị hardcode trong sync_v2_helper.js"
    },
    {
      "author": {
        "id": "U_kgDODU6QWg",
        "is_bot": false,
        "login": "lttkiet",
        "name": "lk"
      },
      "body": "## Môi trường\n- **Hệ điều hành:** Ubuntu server 24.04.5 LTS headless\n- **Phiên bản `ctdozlite`:** v0.0.8\n- **Chế độ chạy:** TUI\n\n## Các bước tái hiện\n1. Chạy ctdozlite TUI `ctdozlite --tui --qr`\n2. Đăng nhập qua qr, xác nhận đăng nhập\n\n## Kết quả thực tế vs. kỳ vọng\n- **Thực tế:** App báo đăng nhập thành công, giao diện chính hiện lên nhưng không có tin nhắn, không có danh sách tin nhắn.\n- **Kỳ vọng:** App tải được danh sách tin nhắn và nội dung tin nhắn\n\n## Log (`--dev`)\nChạy lại với `ctdozlite --gui --dev` và dán đoạn log liên quan tại đây (nhớ xoá thông tin nhạy cảm / cookie nếu có):\n```text\nQuét mã bằng Zalo trên điện thoại…\n→ GET https://qr-id.zaloapp.com/polling/qr/waiting?src=pc&code=REDACTED&lang=vi\n← 200 {\"error_message\":\"Quét mã thành công. Vui lòng chọn Có trên thiết bị di động để đăng nhập\",\"data\":{\"avatar\":\"http:\\/\\/s120.avatar.talk.zdn.vn\\/default\",\"display_name\":\"Lê Trọng Tuấn Kiệt\"},\"error_code…\n✓ Đã quét: Lê Trọng Tuấn Kiệt\nĐã quét mã QR! Vui lòng bấm 'Đăng nhập' trên điện thoại…\n→ GET https://qr-id.zaloapp.com/polling/qr/pc-status?src=pc&code=I5111.0.1794019865783.d7335a45095c8fcd6573501fe3e440c8\n← 200 {\"error_message\":\"\",\"data\":{\"is_pcweb\":1,\"uid\":\"REDACTED\",\"dkey\":\"REDACTED\",\"zpw_kt\":\"1.ByTK73jqiuSTKc4eidBMBQyC47N5ZtR7\\/cX6sAHDul5ScSZrl4qz5tROngBTBVzkTOP1XRwWCilm5…\n  Captured zpw_sek cookie: xQjw.462…<redacted>\n✓ Đăng nhập thành công trên điện thoại! Đang đồng bộ khoá phiên…\n→ GET https://wpa.zaloapp.com/api/login/getLoginInfo?client_version=694&enc_ver=v2&params=<redacted>&signkey=<redacted>&type=30&zcid=REDACTED\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"REDACTED\n  data decrypted (27281 bytes)\n✓ Đã nạp khoá phiên zpw_enk và cập nhật endpoints!\n✓ Đã lưu phiên vào session.json (tự động khôi phục cho các lần chạy sau)\n→ GET https://tt-friend-wpa.chat.zalo.me/api/social/friend/getfriends?zpw_ver=694&zpw_type=30&params=<redacted>\n← 404 Not Found <html>\n<head><title>404 Not Found</title></head>\n<body>\n<center><h1>404 Not Found</h1></center>\n<hr><center>nginx</center>\n</body>\n</html>\n<!-- a padding to disable MSIE and Chrome friendly error page -->\n<!-- a padding to disable MSIE and Chrome friendly error page -->\n<!-- a padding to disable MSIE and Chrome friendly error page -->\n<!-- a padding to disable MSIE and Chrome friendly er…\n→ GET https://tt-friend-wpa.zaloapp.com/api/social/friend/getfriends?zpw_ver=694&zpw_type=30&params=<redacted>\n← 404 Not Found <html>\n<head><title>404 Not Found</title></head>\n<body>\n<center><h1>404 Not Found</h1></center>\n<hr><center>nginx</center>\n</body>\n</html>\n<!-- a padding to disable MSIE and Chrome friendly error page -->\n<!-- a padding to disable MSIE and Chrome friendly error page -->\n<!-- a padding to disable MSIE and Chrome friendly error page -->\n<!-- a padding to disable MSIE and Chrome friendly er…\n→ GET https://tt-profile-wpa.chat.zalo.me/api/social/friend/getfriends?zpw_ver=694&zpw_type=30&params=<redacted>\n← 200 OK {\"error_code\":600,\"error_message\":\"zpw_sek bị thiếu hoặc không đúng\",\"data\":null}\n```\n",
      "closedAt": "2026-11-11T18:29:53Z",
      "createdAt": "2026-11-07T03:00:19Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 115,
      "state": "CLOSED",
      "title": "bug: TUI trên ubuntu server 22.04.5 LTS không đồng bộ được danh sách tin nhắn"
    },
    {
      "author": {
        "id": "U_kgDODCW7uA",
        "is_bot": false,
        "login": "timaivn",
        "name": "TìmAI"
      },
      "body": "> **Trước khi mở issue:** Vui lòng kiểm tra bảng **[Tính năng & giới hạn (`docs/TINH_NANG.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/TINH_NANG.md)** và **[Lỗi đã biết (`docs/DA_BIET.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/DA_BIET.md)** để xem vấn đề đã được ghi nhận chưa.\n\n## Môi trường\n- **Hệ điều hành:** : Debian 13 Trixie Wayland\n- **Phiên bản `ctdozlite`:** (xem góc trên bên trái cửa sổ ứng dụng hoặc chạy `ctdozlite --version`)\n- **Chế độ chạy:** GUI (`--gui`) / TUI (`--tui`) / Headless (`--mcp` hoặc `--api-port`)\n\n## Các bước tái hiện\n1. Không gõ được Fcitcx5 - bamboo (lotus) chế độ unicode trên ctdozlite \n2. ...\n3. ...\n\n## Kết quả thực tế vs. kỳ vọng\n-Hãy fix giúp cộng đồn \n\n## Log (`--dev`)\nChạy lại với `ctdozlite --gui --dev` và dán đoạn log liên quan tại đây (nhớ xoá thông tin nhạy cảm / cookie nếu có):\n```text\n\n```\n",
      "closedAt": "2026-11-11T18:29:56Z",
      "createdAt": "2026-11-06T14:50:49Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 112,
      "state": "CLOSED",
      "title": "bug: Gõ tiếng việt trên debian 13 kde wayland"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjcxOTM1MzQx",
        "is_bot": false,
        "login": "fouze555",
        "name": ""
      },
      "body": "## Môi trường\n- **Hệ điều hành:** (ví dụ: Windows 11 24H2 LTSC\n- **Phiên bản `ctdozlite`:** v0.07\n- **Chế độ chạy:** GUI (`--gui`) \n\n## Các bước tái hiện\n1. Gửi đường link một trang web bất kỳ\n2. Zalo tự tạo thumbnail cho đường link\n3. Chỉ hiện lên và mở được thumbnail như ảnh chứ không thấy được đường link\n\n## Kết quả thực tế vs. kỳ vọng\n- **Thực tế:** Chỉ thấy được thumbnail và mở lên như ảnh\n- **Kỳ vọng:** Thấy được đường link và có thể click vào\n\n\n",
      "closedAt": "2026-11-08T13:04:53Z",
      "createdAt": "2026-11-06T06:35:04Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 111,
      "state": "CLOSED",
      "title": "bug: Link không xuất hiện trong tin nhắn"
    },
    {
      "author": {
        "id": "U_kgDODWIg2Q",
        "is_bot": false,
        "login": "LKW1989-kw",
        "name": "Penguinitty"
      },
      "body": "### Summary\n\nTrong zalo PC có 1 chức năng về privacy khoá zalo bằng mặt mã 4 số và shortcut Ctrl + L để khoá nhanh zalo và nhập mk 4 số dc cài sẵn để truy cập vào zalo \n\n### Proposal\n\n.\n\n### Evidence\n\n- Evidence / capture:\n- Observed error_code:\n\n\n### Context\n\n- ADR / RFC: None\n- Alternatives considered:\n",
      "closedAt": null,
      "createdAt": "2026-11-06T03:51:30Z",
      "labels": [],
      "number": 110,
      "state": "OPEN",
      "title": "feat(<scope>): <summary>Thêm mã khoá Zalo để mục đích riêng tư hơn (có thể 4 chữ số hoặc custom lên)"
    },
    {
      "author": {
        "id": "U_kgDOCIdiXA",
        "is_bot": false,
        "login": "Trinity030-dev",
        "name": ""
      },
      "body": "win 11 26h2 \nv0.0.8\nMở app lên thì một số chữ trong chat cá nhân bị giật khung hình nhòe chữ\n",
      "closedAt": null,
      "createdAt": "2026-11-06T00:24:26Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 109,
      "state": "OPEN",
      "title": "bug: tin nhắn người khác gửi chữ bị giật"
    },
    {
      "author": {
        "id": "U_kgDOEShh-Q",
        "is_bot": false,
        "login": "clockertron",
        "name": ""
      },
      "body": "## Môi trường\n-Hệ điều hành: Windows 10 IoT Enterprise LTSC 2021\n-Phiên bản `ctdozlite`: 0.0.8\n-Chế độ chạy GUI\n\n## Các bước tái hiện\n1. Bấm đồng bộ trong zlite\n2. Điện thoại báo về \n3. Bấm đồng bộ mà bên PC thì ko có đồng bộ dcd\n",
      "closedAt": "2026-11-08T17:07:45Z",
      "createdAt": "2026-11-05T15:06:44Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 108,
      "state": "CLOSED",
      "title": "bug: Không đồng bộ đc tin nhăn ở phiên bản mới nhất"
    },
    {
      "author": {
        "id": "U_kgDOBbygng",
        "is_bot": false,
        "login": "SanRocKk2005",
        "name": "SRKK"
      },
      "body": "> **Trước khi mở issue:** Tôi đã kiểm tra danh sách tính năng và lỗi đã biết.\n\n## Môi trường\n- **Hệ điều hành:** Windows 10\n- **Phiên bản `ctdozlite`:** 0.0.8\n- **Chế độ chạy:** GUI (`--gui`)\n\n## Các bước tái hiện\n1. Mở ứng dụng `ctdozlite` trên Windows 10.\n2. Gửi một đường dẫn (link) trong cửa sổ hội thoại.\n3. Nhấp vào đường dẫn đã gửi để mở hoặc xem.\n\n## Kết quả thực tế vs. kỳ vọng\n- **Thực tế:** Nhấp vào link không có phản hồi, link không thể xem/mở được trong trình duyệt.\n- **Kỳ vọng:** Khi bấm vào link, ứng dụng sẽ mở liên kết đó bằng trình duyệt mặc định trên Windows.\n\n## Log (`--dev`)\nChạy lại với `ctdozlite --gui --dev` và dán đoạn log liên quan tại đây:\n```text\n\n<img width=\"671\" height=\"574\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/9899d493-ebc3-4fb1-abc1-3cdf9dd57ecb\" />",
      "closedAt": "2026-11-11T18:29:58Z",
      "createdAt": "2026-11-05T13:07:52Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 107,
      "state": "CLOSED",
      "title": "bug: Không mở/xem được link trên Windows 10"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjQ2NzAwNjYz",
        "is_bot": false,
        "login": "LeCmnGend",
        "name": "LeCmnGend"
      },
      "body": "> **Trước khi mở issue:** Vui lòng kiểm tra bảng **[Tính năng & giới hạn (`docs/TINH_NANG.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/TINH_NANG.md)** và **[Lỗi đã biết (`docs/DA_BIET.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/DA_BIET.md)** để xem vấn đề đã được ghi nhận chưa.\n\n## Môi trường\n- **Hệ điều hành:** (ví dụ: Windows 11 23H2 / macOS 15.2 Apple Silicon / Ubuntu 24.04)\n- **Phiên bản `ctdozlite`:** (xem góc trên bên trái cửa sổ ứng dụng hoặc chạy `ctdozlite --version`)\n- **Chế độ chạy:** GUI (`--gui`) / TUI (`--tui`) / Headless (`--mcp` hoặc `--api-port`)\n\n## Các bước tái hiện\n1. Cài đặt -> cài đặt chung -> thư mục lưu dữ liệu\n2. ...\n3. ...\n\n## Kết quả thực tế vs. kỳ vọng\n- **Thực tế:** Không cho chọn OK, có vẻ hộp thoại này đang thực hiện chọn file thay vì chọn thư mục\n- **Kỳ vọng:** Ấn xong bước 1 sẽ hiển thị ok thành công\n\n## Log (`--dev`)\nChạy lại với `ctdozlite --gui --dev` và dán đoạn log liên quan tại đây (nhớ xoá thông tin nhạy cảm / cookie nếu có):\n```text\n\n```\n\n<img width=\"539\" height=\"86\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/6c4acfed-3254-488d-9e55-769ccd5f5bd0\" />",
      "closedAt": "2026-11-11T18:30:01Z",
      "createdAt": "2026-11-05T08:41:47Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 106,
      "state": "CLOSED",
      "title": "bug: Không đổi được thư mục lưu dữ liệu"
    },
    {
      "author": {
        "id": "U_kgDOBqPbxA",
        "is_bot": false,
        "login": "vietsonbui",
        "name": "Bùi Việt Sơn"
      },
      "body": "> **Trước khi mở issue:** Vui lòng kiểm tra bảng **[Tính năng & giới hạn (`docs/TINH_NANG.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/TINH_NANG.md)** và **[Lỗi đã biết (`docs/DA_BIET.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/DA_BIET.md)** để xem vấn đề đã được ghi nhận chưa.\n\n## Môi trường\n- **Hệ điều hành:**Windows 11 26H2\n- **Phiên bản `ctdozlite`:** 0.0.8\n- **Chế độ chạy:** GUI\n\n## Các bước tái hiện\n1. Cài đặt\n2. Tài khoản\n3. Đa tài khoản\n4. Không thêm tài khoản\n\n## Kết quả thực tế vs. kỳ vọng\n- **Thực tế:**  Dừng ở màn hình thêm tài khoản, không thể quay lại\n- **Kỳ vọng:** Huỷ thêm, quay lại tài khoản đang sử dụng\n",
      "closedAt": "2026-11-11T18:30:06Z",
      "createdAt": "2026-11-05T03:54:20Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 103,
      "state": "CLOSED",
      "title": "bug: Lỗi đa tài khoản"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjk2NTQ3MzI=",
        "is_bot": false,
        "login": "UhhhhOkayyyy",
        "name": "UhhhhOkayyyy"
      },
      "body": "\n## Môi trường\n- **Hệ điều hành:** Windows 10 \n- **Phiên bản `ctdozlite`:** ctdozlite-0.0.8-setup-windows-x86_64.exe\n\n\nHi ad, mình thấy người khác gởi video nhưng mình ko bấm xem được, bấm nó tải file về máy tính nhưng cũng ko mở thư mục chứa file nên mình mất công tìm kiếm nữa. Nhờ bạn fix với ạ. Cảm ơn bạn. \n<img width=\"634\" height=\"1025\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/916fdb2f-49e9-4eb9-93f3-cea479d25735\" />\n\nKhi tải xong file bị mã hóa cũng ko mở được ạ\n\n<img width=\"905\" height=\"312\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/787c52d0-6a79-4356-b359-73eaec4a919d\" />",
      "closedAt": "2026-11-11T18:30:09Z",
      "createdAt": "2026-11-05T03:01:04Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 102,
      "state": "CLOSED",
      "title": "bug: Không xem video được trực tiếp trong tin nhắn"
    },
    {
      "author": {
        "id": "U_kgDOCfKpOQ",
        "is_bot": false,
        "login": "baodinhgia2011github",
        "name": ""
      },
      "body": "### Summary\n\nKhi sử dụng Zalo Rust GUI, không thể trò chuyện qua thanh chat hoặc tìm kiếm tên người dùng bằng tiếng Việt\n\n### Reproduction\n\n1. Mở ctdozlite\n2. Chạy Unikey bằng fcitx5\n3. Gõ trò chuyện vào thanh chat và tìm kiếm người dùng\n\n\n### Area\n\nauth\n\n### Server error_code (if any)\n\n_No response_\n\n### Environment\n\n- OS / Version: Arch Linux x86_64 6.18.55-1-lts\n- ctdozlite version: v0.0.8\n- Interface: GUI\n- Logs (redacted): None\n\n\n### Secret check\n\n- [x] I confirm no session tokens, cookies, IMEI, phone numbers or raw captures are included.",
      "closedAt": "2026-11-11T18:30:12Z",
      "createdAt": "2026-11-05T02:41:10Z",
      "labels": [],
      "number": 101,
      "state": "CLOSED",
      "title": "Bug: Không gõ được tiếng Việt"
    },
    {
      "author": {
        "id": "U_kgDOCvFoNQ",
        "is_bot": false,
        "login": "lebad280324",
        "name": "Lê Bá Dũng"
      },
      "body": "> **Trước khi mở issue:** Vui lòng kiểm tra bảng **[Tính năng & giới hạn (`docs/TINH_NANG.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/TINH_NANG.md)** và **[Lỗi đã biết (`docs/DA_BIET.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/DA_BIET.md)** để xem vấn đề đã được ghi nhận chưa.\n\n## Môi trường\n- **Hệ điều hành:** (ví dụ: Windows 11 23H2 / macOS 15.2 Apple Silicon / Ubuntu 24.04)\n- **Phiên bản `ctdozlite`:** (xem góc trên bên trái cửa sổ ứng dụng hoặc chạy `ctdozlite --version`)\n- **Chế độ chạy:** GUI (`--gui`) / TUI (`--tui`) / Headless (`--mcp` hoặc `--api-port`)\n\n## Các bước tái hiện\n1. ...\n2. ...\n3. ...\n\n## Kết quả thực tế vs. kỳ vọng\n- **Thực tế:** ...\n- **Kỳ vọng:** ...\n\n## Log (`--dev`)\nChạy lại với `ctdozlite --gui --dev` và dán đoạn log liên quan tại đây (nhớ xoá thông tin nhạy cảm / cookie nếu có):\n```text\n\n```\n",
      "closedAt": "2026-11-08T17:07:47Z",
      "createdAt": "2026-11-05T01:23:19Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 100,
      "state": "CLOSED",
      "title": "bug: trên mac chip apple , lỗi không đồng bộ tin nhắn được"
    },
    {
      "author": {
        "id": "U_kgDOEuocxQ",
        "is_bot": false,
        "login": "phamducit2005-bot",
        "name": ""
      },
      "body": "> **Trước khi mở issue:** Vui lòng kiểm tra bảng **[Tính năng & giới hạn (`docs/TINH_NANG.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/TINH_NANG.md)** và **[Lỗi đã biết (`docs/DA_BIET.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/DA_BIET.md)** để xem vấn đề đã được ghi nhận chưa.\n\n## Môi trường\n- **Hệ điều hành:** (ví dụ: Windows 11 23H2 / macOS 15.2 Apple Silicon / Ubuntu 24.04)\n- **Phiên bản `ctdozlite`:** (xem góc trên bên trái cửa sổ ứng dụng hoặc chạy `ctdozlite --version`)\n- **Chế độ chạy:** GUI (`--gui`) / TUI (`--tui`) / Headless (`--mcp` hoặc `--api-port`)\n\n## Các bước tái hiện\n1. ...\n2. ...\n3. ...\n\n## Kết quả thực tế vs. kỳ vọng\n- **Thực tế:** ...\n- **Kỳ vọng:** ...\n\n## Log (`--dev`)\nChạy lại với `ctdozlite --gui --dev` và dán đoạn log liên quan tại đây (nhớ xoá thông tin nhạy cảm / cookie nếu có):\n```text\n\n```\n",
      "closedAt": "2026-11-07T19:06:15Z",
      "createdAt": "2026-11-04T18:26:59Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 98,
      "state": "CLOSED",
      "title": "Lỗi hiện tại là không trả lời tin nhắn chỉ định , không thả hiệu ứng được cho tin nhắn , âm thanh thông báo của app vẫn chưa có và chưa thấy nổi thông báo mới khi người khác nhắn tin cần ưu tiên hoàn thiện các tính năng thông báo trước"
    },
    {
      "author": {
        "id": "U_kgDODU6QWg",
        "is_bot": false,
        "login": "lttkiet",
        "name": "lk"
      },
      "body": "## Môi trường\n- **Hệ điều hành:** openSUSE Tumbleweed 20261102\n- **Phiên bản `ctdozlite`:** 0.0.7\n- **Chế độ chạy:** GUI (`--gui`)\n\n## Các bước tái hiện\n1. Chạy ctdozlite\n2. Đăng nhập bằng mã QR\n3. Bấm đồng bộ tin nhắn tại màn hình chính hoặc qua tab đồng bộ trong menu cài đặt\n\n## Kết quả thực tế vs. kỳ vọng\n- **Thực tế:** App hiện thông báo đồng bộ thành công, lấy được danh sách nhóm và người đã có tin nhắn, nhưng không hiển thị nội dung tin nhắn. \n> [!NOTE]\n> App vẫn lấy được nội dung trong phần My Documents.\n- **Kỳ vọng:** App tải và hiển thị đầy đủ nội dung tin nhắn của các nhóm/cá nhân sau khi đồng bộ.\n\n## Log (`--dev`)\n\n[dev-gui.redacted.log](https://github.com/user-attachments/files/34372466/dev-gui.redacted.log)\n",
      "closedAt": "2026-11-04T15:39:02Z",
      "createdAt": "2026-11-04T12:54:43Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 95,
      "state": "CLOSED",
      "title": "bug: Đồng bộ thành công nhưng không tải được nội dung tin nhắn (openSUSE)(GUI)"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjk2NTQ3MzI=",
        "is_bot": false,
        "login": "UhhhhOkayyyy",
        "name": "UhhhhOkayyyy"
      },
      "body": "> **Trước khi mở issue:** Vui lòng kiểm tra bảng **[Tính năng & giới hạn (`docs/TINH_NANG.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/TINH_NANG.md)** và **[Lỗi đã biết (`docs/DA_BIET.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/DA_BIET.md)** để xem vấn đề đã được ghi nhận chưa.\n\n## Môi trường\n- **Hệ điều hành:**  macos 26.7 + Windows 10 \n- **Phiên bản `ctdozlite`:** ctdozlite-0.0.7 \n\nHi bạn, Cảm ơn bạn đã tạo app này, rất hữu ích cho mình cần đăng nhập zalo trên nhiều máy tính. \n\nMình muốn báo 1 vài lỗi là:\n- mình không bấm chuột phải vào tin nhắn để tạo reminder được, ko hiện các reminder đã thông báo trước đó\n(dù trong tài liệu bên bạn ghi là Nhắn tin & Hẹn giờ có thực hiện được)\n- ko sync tin nhắn mới nhất được, các tin nhắn không hiện\n- Ko có âm thanh khi có tin nhắn mới hoặc chờ 5s mới nghe thấy âm thanh, trên Windows \n- ko thả icon cho tin nhắn được\n\nCảm ơn bạn \n\n",
      "closedAt": "2026-11-07T19:06:17Z",
      "createdAt": "2026-11-04T09:40:43Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 94,
      "state": "CLOSED",
      "title": "bug: Không tạo được reminder hẹn giờ, không có âm thanh tin nhắn mới, không thả icon được, không sync tin nhắn được"
    },
    {
      "author": {
        "id": "U_kgDODv7yIA",
        "is_bot": false,
        "login": "Lexus101987",
        "name": ""
      },
      "body": "change the languages the saved download address is try to choose a file instead of a folder and although I selected english it still in the vietnamese interface without any button to make the change however the setting still remain in english even after restart and well the message sync future doesn't work at all although some new message of today still recived but the old messages on my personal phone don't And beside that hope you also reduce the annoying amount of notifications that show up in use. Thank you for hearing us.",
      "closedAt": "2026-11-08T17:07:50Z",
      "createdAt": "2026-11-04T06:31:12Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 91,
      "state": "CLOSED",
      "title": "bug: Changing the languages the saved download file address problem and the sync message future doesn't seems to work"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjgwOTM2MjAw",
        "is_bot": false,
        "login": "cuongdm75",
        "name": ""
      },
      "body": "## Môi trường\n- **Hệ điều hành:** Windows 11 23H2\n- **Phiên bản `ctdozlite`:** v0.0.7\n- **Chế độ chạy:** GUI\n\n## Các bước tái hiện\n1. mở app\n2. vào một group bất kỳ\n3. nhấn @ để kiểm tra, list tên người không hiện ra, cũng không có đề xuất người trong group\n\n## Kết quả thực tế vs. kỳ vọng\n- **Thực tế:** Không sử dụng được tag @name\n- **Kỳ vọng:** Sử dụng được\n\n## Log (`--dev`)\nChạy lại với `ctdozlite --gui --dev` và dán đoạn log liên quan tại đây (nhớ xoá thông tin nhạy cảm / cookie nếu có):\n```text\n\n```\n",
      "closedAt": "2026-11-06T16:04:47Z",
      "createdAt": "2026-11-04T04:23:46Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 89,
      "state": "CLOSED",
      "title": "bug: tag tên người sử dụng @ không hoạt động"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjgwOTM2MjAw",
        "is_bot": false,
        "login": "cuongdm75",
        "name": ""
      },
      "body": "### Summary\n\ngiao diện hiện tại không thể đổi được font chữ trên windows 64\n\n### Proposal\n\nĐiều chỉnh trong setting của app thay vì sửa đổi json\n\n### Evidence\n\n- Evidence / capture:\n- Observed error_code:\n\n\n### Context\n\n- ADR / RFC: None\n- Alternatives considered:\n",
      "closedAt": "2026-11-11T18:30:15Z",
      "createdAt": "2026-11-04T04:21:04Z",
      "labels": [],
      "number": 88,
      "state": "CLOSED",
      "title": "feat(<scope>): <summary>Cho phép đổi font chữ của giao diện"
    },
    {
      "author": {
        "id": "U_kgDOCRnj5Q",
        "is_bot": false,
        "login": "ubuntu2310fake",
        "name": "Truong Thanh Hieu"
      },
      "body": "> **Trước khi mở issue:** Vui lòng kiểm tra bảng **[Tính năng & giới hạn (`docs/TINH_NANG.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/TINH_NANG.md)** và **[Lỗi đã biết (`docs/DA_BIET.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/DA_BIET.md)** để xem vấn đề đã được ghi nhận chưa.\n\n## Môi trường\n- **Hệ điều hành:** Fedora Linux 44\n- **Phiên bản `ctdozlite`:** Native 0.0.7\n- **Chế độ chạy:** GUI\n\n## Các bước tái hiện\n1. Đăng nhập Zalo\n2. Chọn đồng bộ tin nhắn\n\n## Kết quả thực tế vs. kỳ vọng\n- **Thực tế:** Không hiện yêu cầu đồng bộ lên điện thoại, và tin nhắn không hiện đúng ds đã ghim\n- **Kỳ vọng:** Hiện thông báo lên đt\n\n## Log (`--dev`)\nChạy lại với `ctdozlite --gui --dev` và dán đoạn log liên quan tại đây (nhớ xoá thông tin nhạy cảm / cookie nếu có):\n```text\n🔧 [DEV MODE ACTIVATED] – Quét log, chẩn đoán giao thức và trace chi tiết được bật.\n🚀 Đang khởi chạy giao diện đồ họa ctdozlite (egui)…\n→ GET https://wpa.zaloapp.com/api/login/getServerInfo?bkt=-1&client_version=694&computer_name=DESKTOP&imei=<redacted>&sp_mtn=-1&type=30&signkey=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":{\"server_time\":1793765873,\"setttings\":{\"authen\":{\"passw_rules\":[{\"regex\":\"^((?=.*[0-9])(?=.*[a-zA-Z]).{6,32})$\",\"msg\":\"Mật khẩu phải có ít nhất 6 ký tự, nhiều nhất 32 ký tự và bao gồm cả chữ lẫn số\"},{\"regex\":\"^\\\\S+.*?\",\"msg\":\"Mật khẩu không thể bắt đầu bằng khoảng trắng\"}],\"encryption\":{\"enable\":1},\"enable_change_password\":0,\"login_scr\":{\"flag_…\n→ GET https://wpa.zaloapp.com/api/login/reqqr?client_time=1793765873&client_version=694&computer_name=DESKTOP&imei=<redacted>&language=vi&logged_uids=%5B%5D&qr_form=0&self_g=1&signkey=<redacted>&type=30\n← 200 {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":{\"qr_timeout\":180,\"token_id\":\"<redacted>\",\"qr_retry\":3,\"chk_wait_cfirm\":\"ht…\n→ GET https://qr-id.zaloapp.com/polling/qr/waiting?src=pc&code=<redacted>&lang=vi\n← 200 {\"error_message\":\"Quét mã thành công. Vui lòng chọn Có trên thiết bị di động để đăng nhập\",\"data\":{\"avatar\":\"<redacted>\"…\nĐã quét mã QR! Vui lòng bấm 'Đăng nhập' trên điện thoại…\n→ GET https://qr-id.zaloapp.com/polling/qr/pc-status?src=pc&code=<redacted>\n← 200 {\"error_message\":\"\",\"data\":{\"is_pcweb\":1,\"uid\":\"<redacted>\",\"dkey\":\"<redacted>\",\"zpw_kt\":\"<redacted>\"…\n  Captured zpw_sek cookie: <redacted>\n→ GET https://wpa.zaloapp.com/api/login/getLoginInfo?client_version=694&enc_ver=v2&params=<redacted>&signkey=<redacted>&type=30&zcid=<redacted>&zcid_ext=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"<redacted>\"}\n  data decrypted (27294 bytes)\n→ GET https://tt-profile-wpa.chat.zalo.me/api/social/profile/me-v2?zpw_ver=694&zpw_type=30&params=<redacted>&os=8\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"<redacted>\"}\n  data decrypted (852 bytes)\n→ GET https://tt-friend-wpa.chat.zalo.me/api/social/friend/getfriends?zpw_ver=694&zpw_type=30&params=<redacted>\n← 404 Not Found <html>\n<head><title>404 Not Found</title></head>\n<body>\n<center><h1>404 Not Found</h1></center>\n<hr><center>nginx</center>\n</body>\n</html>\n→ GET https://tt-friend-wpa.zaloapp.com/api/social/friend/getfriends?zpw_ver=694&zpw_type=30&params=<redacted>\n← 404 Not Found <html>\n<head><title>404 Not Found</title></head>\n<body>\n<center><h1>404 Not Found</h1></center>\n<hr><center>nginx</center>\n</body>\n</html>\n→ GET https://tt-profile-wpa.chat.zalo.me/api/social/friend/getfriends?zpw_ver=694&zpw_type=30&params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"<redacted>\"}\n  data decrypted (164712 bytes)\n→ GET https://tt-group-wpa.zaloapp.com/api/group/getlg/v4?zpw_ver=694&zpw_type=30\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"<redacted>\"}\n  data decrypted (2220 bytes)\n→ POST https://tt-group-wpa.zaloapp.com/api/group/getmg-v2?zpw_ver=694&zpw_type=30\n  body: params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"<redacted>\"}\n  data decrypted (65918 bytes)\n→ POST https://tt-profile-wpa.chat.zalo.me/api/social/group/members?zpw_ver=694&zpw_type=30\n  body: params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"<redacted>\"}\n  data decrypted (16478 bytes)\n→ POST https://tt-profile-wpa.chat.zalo.me/api/social/group/members?zpw_ver=694&zpw_type=30\n  body: params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"<redacted>\"}\n  data decrypted (1689 bytes)\n→ POST https://tt-profile-wpa.chat.zalo.me/api/social/group/members?zpw_ver=694&zpw_type=30\n  body: params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"<redacted>\"}\n  data decrypted (1433 bytes)\n→ POST https://tt-group-wpa.zaloapp.com/api/group/getmg-v2?zpw_ver=694&zpw_type=30\n  body: params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"<redacted>\"}\n  data decrypted (10079 bytes)\n→ POST https://tt-profile-wpa.chat.zalo.me/api/social/group/members?zpw_ver=694&zpw_type=30\n  body: params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"<redacted>\"}\n  data decrypted (5783 bytes)\n→ GET https://tt-group-wpa.zaloapp.com/api/group/getlg/v4?zpw_ver=694&zpw_type=30\nALSA lib pcm_dmix.c:1000:(snd_pcm_dmix_open) [error.pcm] unable to open slave\nALSA lib pcm_route.c:886:(find_matching_chmap) [error.pcm] Found no matching channel map\nALSA lib pcm_route.c:886:(find_matching_chmap) [error.pcm] Found no matching channel map\nALSA lib pcm_route.c:886:(find_matching_chmap) [error.pcm] Found no matching channel map\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"<redacted>\"}\n  data decrypted (2220 bytes)\n→ POST https://tt-group-wpa.zaloapp.com/api/group/getmg-v2?zpw_ver=694&zpw_type=30\n  body: params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"<redacted>\"}\n  data decrypted (65918 bytes)\n→ POST https://tt-profile-wpa.chat.zalo.me/api/social/group/members?zpw_ver=694&zpw_type=30\n  body: params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"<redacted>\"}\n  data decrypted (16478 bytes)\n→ POST https://tt-profile-wpa.chat.zalo.me/api/social/group/members?zpw_ver=694&zpw_type=30\n  body: params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"<redacted>\"}\n  data decrypted (1689 bytes)\n→ GET https://p4-msg-chat.zaloapp.com/?zpw_ver=694&zpw_type=30&params=<redacted>&ts=1793765890\n← 200 OK {\"error_code\":0,\"error_message\":\"\",\"data\":\"<redacted>\"}\n  data decrypted (525 bytes)\n→ GET https://tt-convers-wpa.chat.zalo.me/api/preloadconvers/get-last-msgs?zpw_ver=694&zpw_type=30&params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"<redacted>\"}\n  data decrypted (18080 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"<redacted>\"}\n  data decrypted (66814 bytes)\n→ POST https://tt-group-wpa.zaloapp.com/api/group/getmg-v2?zpw_ver=694&zpw_type=30\n  body: params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"<redacted>\"}\n  data decrypted (10325 bytes)\n→ POST https://tt-profile-wpa.chat.zalo.me/api/social/group/members?zpw_ver=694&zpw_type=30\n  body: params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"<redacted>\"}\n  data decrypted (4837 bytes)\n→ GET https://tt-profile-wpa.chat.zalo.me/api/social/profile/getmute?zpw_ver=694&zpw_type=30&params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"<redacted>\"}\n  data decrypted (678 bytes)\n→ GET https://tt-files-wpa.zaloapp.com/api/message/parselink?zpw_ver=694&zpw_type=30&params=<redacted>\n→ GET https://p4-msg-chat.zaloapp.com/?zpw_ver=694&zpw_type=30&params=<redacted>&ts=1793765898\n→ GET https://p4-ctl-chat.zaloapp.com/?zpw_ver=694&zpw_type=30&params=<redacted>&ts=1793765898\n→ GET https://tt-files-wpa.zaloapp.com/api/message/pull_mobile_msg?zpw_ver=694&zpw_type=30&params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"\",\"data\":\"<redacted>\"}\n  data decrypted (545 bytes)\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"<redacted>\"}\n  data decrypted (695 bytes)\n→ POST https://tt-files-wpa.zaloapp.com/api/message/pull_mobile_msg?zpw_ver=694&zpw_type=30\n  body: params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"<redacted>\"}\n  data decrypted (572 bytes)\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"<redacted>\"}\n  data decrypted (76 bytes)\n→ GET https://tt-files-wpa.zaloapp.com/api/transfer-sync-v2/request-sync?zpw_ver=694&zpw_type=30&params=<redacted>\n← 404 Not Found <html><head><title>404 Not Found</title></head><body><center><h1>404 Not Found</h1></center><hr><center>nginx</center></body></html>\n→ POST https://tt-files-wpa.zaloapp.com/api/transfer-sync-v2/request-sync?zpw_ver=694&zpw_type=30\n  body: params=<redacted>\n← 404 Not Found <html><head><title>404 Not Found</title></head><body><center><h1>404 Not Found</h1></center><hr><center>nginx</center></body></html>\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getoldv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":-69,\"error_message\":\"Stop retry\",\"data\":null}\n→ POST https://tt-group-wpa.zaloapp.com/api/group/getmg-v2?zpw_ver=694&zpw_type=30\n  body: params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"<redacted>\"}\n  data decrypted (9639 bytes)\n→ POST https://tt-profile-wpa.chat.zalo.me/api/social/group/members?zpw_ver=694&zpw_type=30\n  body: params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"<redacted>\"}\n  data decrypted (5071 bytes)\n```\n\n<img width=\"875\" height=\"191\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/395dd644-d887-4fcd-a034-c52b97ae449c\" />\n\n<img width=\"1920\" height=\"1080\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/8d0e32cc-27e5-407f-bc44-8e5a57a5e020\" />\n\n<img width=\"1115\" height=\"754\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/57a1281a-29a9-4ee6-91c3-bdea68f245d3\" />\nĐiện thoại không hiện bảng đồng bộ nào và báo đồng bộ thất bại\nkhông hiện đúng hội thoại ghim (như zl web ở ảnh dưới)\n\n<img width=\"1920\" height=\"1080\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/7b4f5fb3-4871-44eb-b9f9-741218fe6b2c\" />",
      "closedAt": "2026-11-08T17:07:53Z",
      "createdAt": "2026-11-04T04:20:09Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 87,
      "state": "CLOSED",
      "title": "bug: Lỗi đồng bộ tin nhắn, đồng bộ trạng thái GHIM TIN NHẮN trên Zalo"
    },
    {
      "author": {
        "id": "U_kgDODM966Q",
        "is_bot": false,
        "login": "toilinh237",
        "name": ""
      },
      "body": "> **Trước khi mở issue:** Vui lòng kiểm tra bảng **[Tính năng & giới hạn (`docs/TINH_NANG.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/TINH_NANG.md)** và **[Lỗi đã biết (`docs/DA_BIET.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/DA_BIET.md)** để xem vấn đề đã được ghi nhận chưa.\n\n## Môi trường\n- **Hệ điều hành:** Ubuntu 26.04 Gnome 50\n- **Phiên bản `ctdozlite`: Native v0.0.7\n- **Chế độ chạy:** GUI \n",
      "closedAt": "2026-11-08T17:07:56Z",
      "createdAt": "2026-11-04T03:18:28Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 86,
      "state": "CLOSED",
      "title": "bug: Lỗi ko có icon dưới thanh dock trong ubuntu 26.04, ko đồng bộ đc tin nhắn vs điện thoại. Ko tương thích với bộ gõ ngó sen (gõ bị mất chữ tiếng Việt"
    },
    {
      "author": {
        "id": "U_kgDOCbrkvQ",
        "is_bot": false,
        "login": "tinzisreal",
        "name": "Nguyen Huu Tung"
      },
      "body": "### Summary\n\n<img width=\"1497\" height=\"903\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/63302807-377a-4058-b074-148e647579b6\" />\n\n### Proposal\n\nNên thêm nút Auto-Scroll button để xuống tin nhắn mới nhất, và hay bị flick khi ấn vào cuộc hội thoại có tin nhắn mới\n\n### Evidence\n\n- Evidence / capture:\n- Observed error_code:\n\n\n### Context\n\n- ADR / RFC: None\n- Alternatives considered:\n",
      "closedAt": "2026-11-04T15:53:31Z",
      "createdAt": "2026-11-04T02:59:29Z",
      "labels": [],
      "number": 84,
      "state": "CLOSED",
      "title": "feat(<scope>): <summary> Nên thêm nút Auto-Scroll button để xuống tin nhắn mới nhất, và hay bị flick khi ấn vào cuộc hội thoại có tin nhắn mới"
    },
    {
      "author": {
        "id": "U_kgDOFD9k6w",
        "is_bot": false,
        "login": "Subgeneric",
        "name": "Subgeneric"
      },
      "body": "### Summary\n\nI'm trying to switch from the official Zalo PC client to ctdozlite, but I don't want to lose my existing chat history. Although ctdozlite supports importing certain database and JSON formats, support for the official Zalo PC client's native data or backup format isn't clear such as zl.zip files.\n\n\n### Proposal\n\nPlease add support for importing chat history directly from the official Zalo PC client, including:\n\n- Detecting or selecting the official Zalo PC data folder.\n- Reading and decoding its native database or backup format.\n- Preserving messages, timestamps, sender information, group conversations, and attachments where possible.\n- Avoiding duplicate messages when merging imported history.\n- Showing an import report with successful and failed items.\n\n### Evidence\n\nAlthough in your docs you mentioned Zalo PC backup decoding, upon actual review, I had notice the absence of the native zl.zip file format of the classic zalo pc export data.  \n\n### Context\n\nThe goal is to make switching from official Zalo PC to ctdozlite easier without losing years of existing conversations.\n\nThanks for considering this feature!\n",
      "closedAt": null,
      "createdAt": "2026-11-03T22:56:10Z",
      "labels": [],
      "number": 83,
      "state": "OPEN",
      "title": "[Feature Request] Support importing chat history from the official Zalo PC client"
    },
    {
      "author": {
        "id": "U_kgDOCF93ig",
        "is_bot": false,
        "login": "NahXD",
        "name": ""
      },
      "body": "\n## Môi trường\n- **Hệ điều hành:** Windows 11 Pro 26H2\n- **Phiên bản `ctdozlite`:** 0.07\n- **Chế độ chạy:** GUI \n\n## Các bước tái hiện\n1. Chat bình thường trên điện thoại\n2. mở hộp thoại của người mình vưa nhắn\n\n## Kết quả thực tế vs. kỳ vọng\n- **Thực tế:** Bị lỗi phải tắt đi mở lại mới thấy tin mà tin nhắn bị lỗi ko đọc được\n- **Kỳ vọng:** tin nhắn sync tự điện thoại sang pc nhanh chóng, nhắn tiếng việt có dấu bình thường\n\n<img width=\"1039\" height=\"717\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/fd12d492-ea84-4b7c-81f9-da0c0359b293\" />\n\n\n## Log (`--dev`)\n→ GET https://wpa.zaloapp.com/api/login/getServerInfo?bkt=-1&client_version=694&computer_name=PC&imei=<redacted>&sp_mtn=-1&type=30&signkey=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":{\"server_time\":1793714265,\"setttings\":{\"authen\":{\"passw_rules\":[{\"regex\":\"^((?=.*[0-9])(?=.*[a-zA-Z]).{6,32})$\",\"msg\":\"Mật khẩu phải có ít nhất 6 ký tự, nhiều nhất 32 ký tự và bao gồm cả chữ lẫn số\"},{\"regex\":\"^\\\\S+.*?\",\"msg\":\"Mật khẩu không thể bắt đầu bằng khoảng trắng\"}],\"encryption\":{\"enable\":1},\"enable_change_password\":0},\"features\":{\"stick…\n→ GET https://tt-profile-wpa.chat.zalo.me/api/social/profile/me-v2?zpw_ver=694&zpw_type=30&params=<redacted>&os=8\n→ GET https://tt-profile-wpa.chat.zalo.me/api/social/profile/me-v2?zpw_ver=694&zpw_type=30&params=<redacted>&os=8\n→ GET https://p4-ctl-chat.zaloapp.com/?zpw_ver=694&zpw_type=30&params=<redacted>&ts=1793714264\n→ GET https://p4-msg-chat.zaloapp.com/?zpw_ver=694&zpw_type=30&params=<redacted>&ts=1793714264\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/a2jiUQgXaGF5JXinVV9vy4N5O9UnaICFrtbnBZFxOL0+bGoSc9rWQCD/kLZAc3qCbzID2xMZAxIv0wn7JakecX+cbhFf2FEcQ1t+PjiPQjnGXXgQ5FvGFOiW6Wm/7YhBqKwuS5sV4XUCcaJDp8I53dRVd8jeMe/83VN+HFEXPu+Dm/Yw21wkCz+1JfgMKbXQ+JZELWmbB/V4/qHPR6b6qVcAgTFFIikMP/EhsqsNKULEoMQNgbZMMGOg+GWqJfh2d+gFcmPxU1E6kXOScXikZul1ZBH…\n  data decrypted (843 bytes)\n→ GET https://tt-profile-wpa.chat.zalo.me/api/social/profile/me-v2?zpw_ver=694&zpw_type=30&params=<redacted>&os=8\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/a2jiUQgXaGF5JXinVV9vy4N5O9UnaICFrtbnBZFxOL0+bGoSc9rWQCD/kLZAc3qCbzID2xMZAxIv0wn7JakecX+cbhFf2FEcQ1t+PjiPQjnGXXgQ5FvGFOiW6Wm/7YhBqKwuS5sV4XUCcaJDp8I53dRVd8jeMe/83VN+HFEXPu+Dm/Yw21wkCz+1JfgMKbXQ+JZELWmbB/V4/qHPR6b6qVcAgTFFIikMP/EhsqsNKULEoMQNgbZMMGOg+GWqJfh2d+gFcmPxU1E6kXOScXikZul1ZBH…\n  data decrypted (843 bytes)\n→ GET https://tt-profile-wpa.chat.zalo.me/api/social/profile/me-v2?zpw_ver=694&zpw_type=30&params=<redacted>&os=8\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/a2jiUQgXaGF5JXinVV9vy4N5O9UnaICFrtbnBZFxOL0+bGoSc9rWQCD/kLZAc3qCbzID2xMZAxIv0wn7JakecX+cbhFf2FEcQ1t+PjiPQjnGXXgQ5FvGFOiW6Wm/7YhBqKwuS5sV4XUCcaJDp8I53dRVd8jeMe/83VN+HFEXPu+Dm/Yw21wkCz+1JfgMKbXQ+JZELWmbB/V4/qHPR6b6qVcAgTFFIikMP/EhsqsNKULEoMQNgbZMMGOg+GWqJfh2d+gFcmPxU1E6kXOScXikZul1ZBH…\n  data decrypted (843 bytes)\n→ GET https://tt-friend-wpa.chat.zalo.me/api/social/friend/getfriends?zpw_ver=694&zpw_type=30&params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/a2jiUQgXaGF5JXinVV9vy4N5O9UnaICFrtbnBZFxOL0+bGoSc9rWQCD/kLZAc3qCbzID2xMZAxIv0wn7JakecX+cbhFf2FEcQ1t+PjiPQjnGXXgQ5FvGFOiW6Wm/7YhBqKwuS5sV4XUCcaJDp8I53dRVd8jeMe/83VN+HFEXPu+Dm/Yw21wkCz+1JfgMKbXQ+JZELWmbB/V4/qHPR6b6qVcAgTFFIikMP/EhsqsNKULEoMQNgbZMMGOg+GWqJfh2d+gFcmPxU1E6kXOScXikZul1ZBH…\n← 200 OK {\"error_code\":0,\"error_message\":\"\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVaux4zQ/8TsNHmp9153vYMAxKeSgKM/ybhbQJO8SygpRg+T3QC5Ht4ypZ11ekgT1JEF5CmUdYjqKd1ujq72YaIN/DjTxZyF0+kvkEQKmmPImmvfe5BCTxweZsfFD2VQ/XF4/U4iVjPm92umevDQRCn1F+EVaAkALlPL/2FX1PIJ7kwmuHGvNK0r/XvHJyJ3+rcwfzs6ct6/7ArYWEAgx9ZjXC2Kq87AI8RkQDSS60OOioircDUhRCWpPvXNez84zFMnHN1iQ0BheRXxBprA8yN6plGDvwXmc5TCjxax2ckbBU9vTd0ubNtwc+ZFsn4…\n  data decrypted (843 bytes)\n  data decrypted (15292 bytes)\n← 200 OK {\"error_code\":0,\"error_message\":\"\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVaux4zQ/8TsNHmp9153vYMAxKsFiBoOxdp5eminGaWpYM0/f2IUdfNJxuPv7TY9GrfFYYEE9AGuk6IEuzpMXLnUdA9WMslZZRFnlH6yj+ugLB0vScBYNQMdawz8CRhcCkYuQsGm7gc7+mR2GQn8y2wq5HSb4TpGTVAiK+pQYcFbpHoGQeMDqsprzV6qrMlli1c+yDjb2VccNvxLaS6y+njkoxJpUhquoZZTC69dPTgCK5gYkYQHfBVGf92qBITbxJ5+hppOWFIso8FWGCmr780sEm1Iw7NslCITgueSmMPRkVSNuD/sdCA1inTLZib…\n  data decrypted (6351 bytes)\n→ POST https://tt-profile-wpa.chat.zalo.me/api/social/group/members?zpw_ver=694&zpw_type=30\n  body: params=DKN%2BFfOaJj8Qa9uV43uWPlIcLWqnzkAyVNQspnw0m%2FRKA1KlNPTsO4iUdXUdgaKbq3TQqWV4LfRfVG%2BRFbHOsYabK6YMMXgcaquQ31FnKdc%3D\n→ GET https://tt-friend-wpa.chat.zalo.me/api/social/friend/getfriends?zpw_ver=694&zpw_type=30&params=<redacted>\n→ GET https://p4-ctl-chat.zaloapp.com/?zpw_ver=694&zpw_type=30&params=<redacted>&ts=1793714264\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/abJVlG4WIkDckEJhSf8xuRKyV7y3cg5vuhMi4WvHkzRWOUzUnBLplujPQv5y4a7onYszSC2ik1WcETFDx4toPQJOdiGW03Fwo5S/xB7YidHhhTH+DOI3JWdI7ki5Xt655MD3rM3cRhyXbvAjWsaJ6awqama3cbequJ0iKTLPtNrGe79Qhs5ZYq7hGJyxf/VASLbg5K4JxO0TChSop8+LTyQe2Su1urrRU/T8oW9FwKZUQPmOxljVcfrK+/yaMpr9PyZtpBty02O2wA0C4m2hXot+kd8…\n  data decrypted (759 bytes)\n← 404 Not Found <html>\n<head><title>404 Not Found</title></head>\n<body>\n<center><h1>404 Not Found</h1></center>\n<hr><center>nginx</center>\n</body>\n</html>\n<!-- a padding to disable MSIE and Chrome friendly error page -->\n<!-- a padding to disable MSIE and Chrome friendly error page -->\n<!-- a padding to disable MSIE and Chrome friendly error page -->\n<!-- a padding to disable MSIE and Chrome friendly er…\n→ GET https://tt-profile-wpa.chat.zalo.me/api/social/friend/getfriends?zpw_ver=694&zpw_type=30&params=<redacted>\n← 404 Not Found <html>\n<head><title>404 Not Found</title></head>\n<body>\n<center><h1>404 Not Found</h1></center>\n<hr><center>nginx</center>\n</body>\n</html>\n<!-- a padding to disable MSIE and Chrome friendly error page -->\n<!-- a padding to disable MSIE and Chrome friendly error page -->\n<!-- a padding to disable MSIE and Chrome friendly error page -->\n<!-- a padding to disable MSIE and Chrome friendly er…\n→ GET https://tt-profile-wpa.chat.zalo.me/api/social/friend/getfriends?zpw_ver=694&zpw_type=30&params=<redacted>\n→ POST https://tt-profile-wpa.chat.zalo.me/api/social/group/members?zpw_ver=694&zpw_type=30\n  body: params=DKN%2BFfOaJj8Qa9uV43uWPlIcLWqnzkAyVNQspnw0m%2FRKA1KlNPTsO4iUdXUdgaKbq3TQqWV4LfRfVG%2BRFbHOsYabK6YMMXgcaquQ31FnKdc%3D\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/abJVlG4WIkDckEJhSf8xuRKyV7y3cg5vuhMi4WvHkzRWOUzUnBLplujPQv5y4a7onYszSC2ik1WcETFDx4toPQJOdiGW03Fwo5S/xB7YidHhhTH+DOI3JWdI7ki5Xt655MD3rM3cRhyXbvAjWsaJ6awqama3cbequJ0iKTLPtNrGe79Qhs5ZYq7hGJyxf/VASLbg5K4JxO0TChSop8+LTyQe2Su1urrRU/T8oW9FwKZUQPmOxljVcfrK+/yaMpr9PyZtpBty02O2wA0C4m2hXot+kd8…\n  data decrypted (759 bytes)\n→ POST https://tt-profile-wpa.chat.zalo.me/api/social/group/members?zpw_ver=694&zpw_type=30\n  body: params=DKN%2BFfOaJj8Qa9uV43uWPlIcLWqnzkAyVNQspnw0m%2FQ6Pz2Kf0SO1NTGQumD%2BMbNXuVBG5l5hUHEp5reoIFpzQ%3D%3D\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aXKHqWqBXo8/gAhsioWcLGPD5Zg3CJp7f9tbS2vWNWhkWC5LbfHxkl54Ffjt6H32GGmOGGOQtKj2tn6yKRlXVQsYARTGNxyK9K6WdOs57HDivgX4h65QtMMgLi3Y2Ni177kDGFvwsPgTKzDcQQ5JGa7HJ62bEPon3UJf/9JD2cEWcbD5yhl70wGLWVn5673HuS27KYDdz3n1HBM4AlD7dJrXqYDzhn6q5V/CwGwgP3+j+gvItkv9PWy8jmIaWmdkt/8UNjiwMjYh6Im+nZfBH14ZL5S…\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/abJVlG4WIkDckEJhSf8xuRNBa44PTDADkvyE0otrTtP2dab9XiF0m0TRsvcAe5aZ+df81AobIZGFW9dWLPDBbUwqXBS84BbLQsx/Exgma25LMiNx5H2LP5mvIesPcXkt1Bec/RUbXkRvvs9EEFKkDP0b1Fz6iJ+67AzRgdLMGvAmZu6hGlYULUL1Wc0ppUy3qKX6RpfSmBHG1aDmzEEFn+J+E7PeYL/gPphOk/6sN8QE8rvDhXRc3Pjaobn0xlxGaUszv7sKXDNYb2Q9htAT4xqeS7P…\n  data decrypted (439 bytes)\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aXKHqWqBXo8/gAhsioWcLGPD5Zg3CJp7f9tbS2vWNWhkWC5LbfHxkl54Ffjt6H32GGmOGGOQtKj2tn6yKRlXVQsYARTGNxyK9K6WdOs57HDivgX4h65QtMMgLi3Y2Ni177kDGFvwsPgTKzDcQQ5JGa7HJ62bEPon3UJf/9JD2cEWcbD5yhl70wGLWVn5673HuS27KYDdz3n1HBM4AlD7dJrXqYDzhn6q5V/CwGwgP3+j+gvItkv9PWy8jmIaWmdkt/8UNjiwMjYh6Im+nZfBH14ZL5S…\n  data decrypted (73055 bytes)\n  data decrypted (73055 bytes)\n→ GET https://tt-group-wpa.zaloapp.com/api/group/getlg/v4?zpw_ver=694&zpw_type=30\n→ GET https://tt-group-wpa.zaloapp.com/api/group/getlg/v4?zpw_ver=694&zpw_type=30\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/a0yW6mn2hderwiKu85j8d4KiNUj+T1rQwf6u0RufHDIn+rIu71+WBUHv+fCtIt/9T0u5mMFGnI4uEMLJM6rhtv5N3O5QpV13B34didus5fa+T9smif4W86wYK/cdQfbdj1sbjds5iQ1FTRrLzxHye0bp2+45KDW51bPGAkuO3cD60sc2bCNbkdAVdXdIM62SQRWh5fQaiSADOvzZGFnUdU6I7qz5dyzYHG3LkpVbA/4FW6wXYzK45ekg5+/gLOTHVyRB8Aif77tvz46F0LcfJG6vi5o…\n  data decrypted (782 bytes)\n→ POST https://tt-group-wpa.zaloapp.com/api/group/getmg-v2?zpw_ver=694&zpw_type=30\n  body: params=ufFy8KAtP83H8KYzSWV6b6pZfNSpE9%2FP4S2s8jn4kcCM63qNoyhjbFfuumCOXos1QOl%2FeQQGKcdA1xut1ymJbQvzl86%2BMz9561guQOBYSRfhu1gE90JYKhopPIuwOvN2R6P4lUyk2QhC8%2Bw8jDYu96FeZA%2BkVaS%2BZEBTXXsyzkfIF1SVKHCVBfL7LNkl0oA5NvUqws5df4Mk7RHFVLDX8RFHT1G%2F%2FWpY7BPkIYF2f%2BpMNkFe%2B2OK9YqOGmeO07oA10fluH6YiNwAlL%2BUOSXKjxPrno3AudSjF1ZR8d2ShVus1Baik9cXZvSDdL5oZgh0L3Tk5T5RdzNXuoXjOC7TGzhfryldMNPal3PG7ljcbD%2FMkT%2F…\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/a0yW6mn2hderwiKu85j8d4KiNUj+T1rQwf6u0RufHDIn+rIu71+WBUHv+fCtIt/9T0u5mMFGnI4uEMLJM6rhtv5N3O5QpV13B34didus5fa+T9smif4W86wYK/cdQfbdj1sbjds5iQ1FTRrLzxHye0bp2+45KDW51bPGAkuO3cD60sc2bCNbkdAVdXdIM62SQRWh5fQaiSADOvzZGFnUdU6I7qz5dyzYHG3LkpVbA/4FW6wXYzK45ekg5+/gLOTHVyRB8Aif77tvz46F0LcfJG6vi5o…\n  data decrypted (782 bytes)\n→ POST https://tt-group-wpa.zaloapp.com/api/group/getmg-v2?zpw_ver=694&zpw_type=30\n  body: params=ufFy8KAtP83H8KYzSWV6b6pZfNSpE9%2FP4S2s8jn4kcCM63qNoyhjbFfuumCOXos1QOl%2FeQQGKcdA1xut1ymJbQvzl86%2BMz9561guQOBYSRfhu1gE90JYKhopPIuwOvN2R6P4lUyk2QhC8%2Bw8jDYu96FeZA%2BkVaS%2BZEBTXXsyzkfIF1SVKHCVBfL7LNkl0oA5NvUqws5df4Mk7RHFVLDX8RFHT1G%2F%2FWpY7BPkIYF2f%2BpMNkFe%2B2OK9YqOGmeO07oA10fluH6YiNwAlL%2BUOSXKjxPrno3AudSjF1ZR8d2ShVus1Baik9cXZvSDdL5oZgh0L3Tk5T5RdzNXuoXjOC7TGzhfryldMNPal3PG7ljcbD%2FMkT%2F…\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/auRfO7mg2xvP99Q9keko/sSEod8dMdjnUlq3PYwOdM0KpeVNHNlgHdU6l8aNyFEi6TqPpMNDoHMAZ5PJ+AzpmWl8rdAZ+nONZss7qb+zKUWDnOB2emU91hCJnAzk1/KVvT1cDD47krsddZ/nuV0kBOCRmDgLqeTDHo/b8KIiiDzTjaKJ/zzfl/XyrztCT8cgBJMF5eAeP54HlpKVDCZSVIo86Gz6T/8De8n5n4RV7543glhxQr0JNSvX9328M2X8BjYRwgkuaH28J+NddomM2pSkb43…\n  data decrypted (22724 bytes)\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/auRfO7mg2xvP99Q9keko/sSEod8dMdjnUlq3PYwOdM0KpeVNHNlgHdU6l8aNyFEi6TqPpMNDoHMAZ5PJ+AzpmWl8rdAZ+nONZss7qb+zKUWDnOB2emU91hCJnAzk1/KVvT1cDD47krsddZ/nuV0kBOCRmDgLqeTDHo/b8KIiiDzTjaKJ/zzfl/XyrztCT8cgBJMF5eAeP54HlpKVDCZSVIo86Gz6T/8De8n5n4RV7543glhxQr0JNSvX9328M2X8BjYRwgkuaH28J+NddomM2pSkb43…\n→ POST https://tt-profile-wpa.chat.zalo.me/api/social/group/members?zpw_ver=694&zpw_type=30\n  data decrypted (22724 bytes)\n  body: params=DKN%2BFfOaJj8Qa9uV43uWPr1T0fSubFcXSomVAzuOjqPGRie5cjACYJR%2B7%2BOG5dyM5obhvgv97Srim326LpLYN1fRDq5PZRuA0QmMRH%2BpZoM%2F0ivluYCTytyqs35zIrv6dix%2BQ9eQ5ONd3lXRQxE9ITHXVl%2B%2FyUVrHPNmFh1Lj8me1Z9q1DZHqKvEqVhSeDktn6J5lufX9VtCcU2Ux6ljmNw5uBDHfChrFezgzJ3FJ8XF7Q3v2GS86Dlag9tbsPSPQnvp6N44U0zaNqclblORu4iTpAUAdetgn75Mw1Kjy4NatPNNg%2B%2FQcWcDh4rWVrtGBwZpioqb4ckn09gppHZAwAGuDlheW3qMpZ7CVHjrc3dIfWPesUjrN…\n→ POST https://tt-profile-wpa.chat.zalo.me/api/social/group/members?zpw_ver=694&zpw_type=30\n  body: params=DKN%2BFfOaJj8Qa9uV43uWPr1T0fSubFcXSomVAzuOjqPGRie5cjACYJR%2B7%2BOG5dyM5obhvgv97Srim326LpLYN1fRDq5PZRuA0QmMRH%2BpZoM%2F0ivluYCTytyqs35zIrv6dix%2BQ9eQ5ONd3lXRQxE9ITHXVl%2B%2FyUVrHPNmFh1Lj8me1Z9q1DZHqKvEqVhSeDktn6J5lufX9VtCcU2Ux6ljmNw5uBDHfChrFezgzJ3FJ8XF7Q3v2GS86Dlag9tbsPSPQnvp6N44U0zaNqclblORu4iTpAUAdetgn75Mw1Kjy4NatPNNg%2B%2FQcWcDh4rWVrtGBwZpioqb4ckn09gppHZAwAGuDlheW3qMpZ7CVHjrc3dIfWPesUjrN…\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/abJVlG4WIkDckEJhSf8xuRMP3kl50Mjlt4N8JUePXX4cgy5iqocUHL5z8N5MNjZ8NaM98vl87Jbjwwrad+4dAkyeCMoK5+oRueX+0YLw9bU7ArzzMMqMHIpEqXlvZX7j5+VppgIRvyoz/oOfeUjd6twtQnjI+/m6MJ/xxrTSoXtjlFP7n5GP5vtl6M1DIAxC3Bj4mFfP7Zyn/ZzujBEI9yxbADJTdRc0X/SY6biy61QrLGnCtrEBcmwtE86KGkj9syDZYJa7wq3HGD06l7KYmezfhzJ…\n  data decrypted (6432 bytes)\n→ POST https://tt-profile-wpa.chat.zalo.me/api/social/group/members?zpw_ver=694&zpw_type=30\n  body: params=DKN%2BFfOaJj8Qa9uV43uWPgYuqEAJSV5knkPDuqYstNoLCtqRs8HusSQkWKQn0b6QDy1wUaLz60QwJCj91G5%2Fg0B7V49l%2BuzToUslir7CZevZ8KdVLPCkEsYpxfk5lGq0%2BD%2BaBTpIMlV6ERH%2BgU9qYjB99AL3aQGC3GBt15gbFxYZmHL2PtZbIBtTjimZvo%2FxlH34M9G%2FHn5O%2BFvxhbPMetqEhIy4kQQKhIoKtuuqbENPPs5Y%2FB7KfFAkwzVQ4fYjhS7rre48kA1uhgnojGZmC2OakS7qwK%2FTmtoN2An3OWOc6eDFg%2FnzbXF9xOo4ho%2FaV6NKysGfVrW%2B%2FpqS%2FB5qN%2Bo%2BkfYlq75%2BLy3…\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/abJVlG4WIkDckEJhSf8xuRMP3kl50Mjlt4N8JUePXX4cgy5iqocUHL5z8N5MNjZ8NaM98vl87Jbjwwrad+4dAkyeCMoK5+oRueX+0YLw9bU7ArzzMMqMHIpEqXlvZX7j5+VppgIRvyoz/oOfeUjd6twtQnjI+/m6MJ/xxrTSoXtjlFP7n5GP5vtl6M1DIAxC3Bj4mFfP7Zyn/ZzujBEI9yxbADJTdRc0X/SY6biy61QrLGnCtrEBcmwtE86KGkj9syDZYJa7wq3HGD06l7KYmezfhzJ…\n  data decrypted (6432 bytes)\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/abJVlG4WIkDckEJhSf8xuRHCNYdW0Su4wib/YCf0Y5P1hPSZQWnWDxSbncESkWUMG5zTJrovA/cpfMHFBPUzvq3n5YjcFiaphYX7OeydNrY2+nrvVvmEq6SavIl9TJhdUHfsXUzrQSqi6gSvPs6bisRZJWkXmKkj6ozt0B8bewjHRlPRjeO12/2bXXcfUybDkg/RPKwhOl34V4bDHueov8JMUM8+0r1MH1V9lzzfEdIN3U8tE8zIRejuUn0LdN5C8fqrcPl/6KGuSqI3DmSzs5UFecY…\n→ POST https://tt-profile-wpa.chat.zalo.me/api/social/group/members?zpw_ver=694&zpw_type=30\n  data decrypted (5940 bytes)\n  body: params=DKN%2BFfOaJj8Qa9uV43uWPgYuqEAJSV5knkPDuqYstNoLCtqRs8HusSQkWKQn0b6QDy1wUaLz60QwJCj91G5%2Fg0B7V49l%2BuzToUslir7CZevZ8KdVLPCkEsYpxfk5lGq0%2BD%2BaBTpIMlV6ERH%2BgU9qYjB99AL3aQGC3GBt15gbFxYZmHL2PtZbIBtTjimZvo%2FxlH34M9G%2FHn5O%2BFvxhbPMetqEhIy4kQQKhIoKtuuqbENPPs5Y%2FB7KfFAkwzVQ4fYjhS7rre48kA1uhgnojGZmC2OakS7qwK%2FTmtoN2An3OWOc6eDFg%2FnzbXF9xOo4ho%2FaV6NKysGfVrW%2B%2FpqS%2FB5qN%2Bo%2BkfYlq75%2BLy3…\n→ POST https://tt-group-wpa.zaloapp.com/api/group/getmg-v2?zpw_ver=694&zpw_type=30\n  body: params=ufFy8KAtP83H8KYzSWV6b%2BtY%2Bx2kb%2FzFVuL6%2F93TTbTLj6UWVbs%2BnwM1pk4hWg0910HmjXKEvozj%2FjK9Fc6OQQHrW6rj%2BTWF4S44ziKdpScHX%2Bcs7HdyOOuKmO26Z3K0%2FFTMhW1%2F916VoefzhL8oMd4A4P0xdzH3Vb6IZMZq%2B8G%2BTLHons69k5FlOP3gkOD6OrW1z4HQiD03%2FkdovSBbWQ%3D%3D\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/auRfO7mg2xvP99Q9keko/sSEod8dMdjnUlq3PYwOdM0KpeVNHNlgHdU6l8aNyFEi6ZLGYqVQ02w/R7hlsuH1AlVG1iuUjVgrgXC4X+OZs7rsR2UY2ZLv6YnAr2cY/FMIRql/7AlCkUPCPZn8k2ahAaHjV3GqGUult2x4s828dwQtrJK9a7M+MzqFvrsD/pdHV7D0fXKeCW65AhYCyApUXHeRPnurvVW4WZQUMaYe0HSd7Q5k//nB0xwBDUDiaoMfoo0W4mqk1GCvThjgVjSXZsQs7LN…\n  data decrypted (2909 bytes)\n→ POST https://tt-profile-wpa.chat.zalo.me/api/social/group/members?zpw_ver=694&zpw_type=30\n  body: params=DKN%2BFfOaJj8Qa9uV43uWPtxKgHvfEQyp%2BIOvJaxS8O3pKh0u0ET3GkqQcOzLQC0fxm5TMjB474AlemcZCrcFpG%2FZKRQqwhLHAWY8QvHy3vbqlsE6vTOL3aJOfkv6Tqn7J%2BAFboqPLX5OHZ%2F7O77SQKjfOgHasdj8Z09Mxt0U%2FMA%3D\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/abJVlG4WIkDckEJhSf8xuRHCNYdW0Su4wib/YCf0Y5P1hPSZQWnWDxSbncESkWUMG5zTJrovA/cpfMHFBPUzvq3n5YjcFiaphYX7OeydNrY2+nrvVvmEq6SavIl9TJhdUHfsXUzrQSqi6gSvPs6bisRZJWkXmKkj6ozt0B8bewjHRlPRjeO12/2bXXcfUybDkg/RPKwhOl34V4bDHueov8JMUM8+0r1MH1V9lzzfEdIN3U8tE8zIRejuUn0LdN5C8fqrcPl/6KGuSqI3DmSzs5UFecY…\n  data decrypted (5940 bytes)\n→ POST https://tt-group-wpa.zaloapp.com/api/group/getmg-v2?zpw_ver=694&zpw_type=30\n  body: params=ufFy8KAtP83H8KYzSWV6b%2BtY%2Bx2kb%2FzFVuL6%2F93TTbTLj6UWVbs%2BnwM1pk4hWg0910HmjXKEvozj%2FjK9Fc6OQQHrW6rj%2BTWF4S44ziKdpScHX%2Bcs7HdyOOuKmO26Z3K0%2FFTMhW1%2F916VoefzhL8oMd4A4P0xdzH3Vb6IZMZq%2B8G%2BTLHons69k5FlOP3gkOD6OrW1z4HQiD03%2FkdovSBbWQ%3D%3D\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/abJVlG4WIkDckEJhSf8xuRKH4347snN/w7veSp7l+ww69tEgYc2Oz+zVYGuwqw3rhRH1rP90vb9YaNWC/PS+sP82bR+YssjkO/U1RZDlv++CED36d7Om1bWQAesLMQKf0kTJ3pP8znd3QaLBcBpyZXbvgM27tO4Tmz4Ov6+rPtIsg4rrshJSQo8k52DMhsGH00FwVVBFCGvsizEStVHiiKtAf47grOGvpNgOE6NfFa47st9Wpa8s1aOU02Wmo06rOu3wuIAtu341NK594s4yr8iCMo0…\n  data decrypted (1452 bytes)\n→ GET https://tt-group-wpa.zaloapp.com/api/group/getlg/v4?zpw_ver=694&zpw_type=30\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/auRfO7mg2xvP99Q9keko/sSEod8dMdjnUlq3PYwOdM0KpeVNHNlgHdU6l8aNyFEi6ZLGYqVQ02w/R7hlsuH1AlVG1iuUjVgrgXC4X+OZs7rsR2UY2ZLv6YnAr2cY/FMIRql/7AlCkUPCPZn8k2ahAaHjV3GqGUult2x4s828dwQtrJK9a7M+MzqFvrsD/pdHV7D0fXKeCW65AhYCyApUXHeRPnurvVW4WZQUMaYe0HSd7Q5k//nB0xwBDUDiaoMfoo0W4mqk1GCvThjgVjSXZsQs7LN…\n  data decrypted (2909 bytes)\n→ POST https://tt-profile-wpa.chat.zalo.me/api/social/group/members?zpw_ver=694&zpw_type=30\n  body: params=DKN%2BFfOaJj8Qa9uV43uWPtxKgHvfEQyp%2BIOvJaxS8O3pKh0u0ET3GkqQcOzLQC0fxm5TMjB474AlemcZCrcFpG%2FZKRQqwhLHAWY8QvHy3vbqlsE6vTOL3aJOfkv6Tqn7J%2BAFboqPLX5OHZ%2F7O77SQKjfOgHasdj8Z09Mxt0U%2FMA%3D\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/a0yW6mn2hderwiKu85j8d4KiNUj+T1rQwf6u0RufHDIn+rIu71+WBUHv+fCtIt/9T0u5mMFGnI4uEMLJM6rhtv5N3O5QpV13B34didus5fa+T9smif4W86wYK/cdQfbdj1sbjds5iQ1FTRrLzxHye0bp2+45KDW51bPGAkuO3cD60sc2bCNbkdAVdXdIM62SQRWh5fQaiSADOvzZGFnUdU6I7qz5dyzYHG3LkpVbA/4FW6wXYzK45ekg5+/gLOTHVyRB8Aif77tvz46F0LcfJG6vi5o…\n  data decrypted (782 bytes)\n→ POST https://tt-group-wpa.zaloapp.com/api/group/getmg-v2?zpw_ver=694&zpw_type=30\n  body: params=ufFy8KAtP83H8KYzSWV6b6pZfNSpE9%2FP4S2s8jn4kcCM63qNoyhjbFfuumCOXos1QOl%2FeQQGKcdA1xut1ymJbQvzl86%2BMz9561guQOBYSRfhu1gE90JYKhopPIuwOvN2R6P4lUyk2QhC8%2Bw8jDYu96FeZA%2BkVaS%2BZEBTXXsyzkfIF1SVKHCVBfL7LNkl0oA5NvUqws5df4Mk7RHFVLDX8RFHT1G%2F%2FWpY7BPkIYF2f%2BpMNkFe%2B2OK9YqOGmeO07oA10fluH6YiNwAlL%2BUOSXKjxPrno3AudSjF1ZR8d2ShVus1Baik9cXZvSDdL5oZgh0L3Tk5T5RdzNXuoXjOC7TGzhfryldMNPal3PG7ljcbD%2FMkT%2F…\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/abJVlG4WIkDckEJhSf8xuRKH4347snN/w7veSp7l+ww69tEgYc2Oz+zVYGuwqw3rhRH1rP90vb9YaNWC/PS+sP82bR+YssjkO/U1RZDlv++CED36d7Om1bWQAesLMQKf0kTJ3pP8znd3QaLBcBpyZXbvgM27tO4Tmz4Ov6+rPtIsg4rrshJSQo8k52DMhsGH00FwVVBFCGvsizEStVHiiKtAf47grOGvpNgOE6NfFa47st9Wpa8s1aOU02Wmo06rOu3wuIAtu341NK594s4yr8iCMo0…\n  data decrypted (1452 bytes)\n→ GET https://tt-group-wpa.zaloapp.com/api/group/getlg/v4?zpw_ver=694&zpw_type=30\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/auRfO7mg2xvP99Q9keko/sSEod8dMdjnUlq3PYwOdM0KpeVNHNlgHdU6l8aNyFEi6TqPpMNDoHMAZ5PJ+AzpmWl8rdAZ+nONZss7qb+zKUWDnOB2emU91hCJnAzk1/KVvT1cDD47krsddZ/nuV0kBOCRmDgLqeTDHo/b8KIiiDzTjaKJ/zzfl/XyrztCT8cgBJMF5eAeP54HlpKVDCZSVIo86Gz6T/8De8n5n4RV7543glhxQr0JNSvX9328M2X8BjYRwgkuaH28J+NddomM2pSkb43…\n  data decrypted (22724 bytes)\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/a0yW6mn2hderwiKu85j8d4KiNUj+T1rQwf6u0RufHDIn+rIu71+WBUHv+fCtIt/9T0u5mMFGnI4uEMLJM6rhtv5N3O5QpV13B34didus5fa+T9smif4W86wYK/cdQfbdj1sbjds5iQ1FTRrLzxHye0bp2+45KDW51bPGAkuO3cD60sc2bCNbkdAVdXdIM62SQRWh5fQaiSADOvzZGFnUdU6I7qz5dyzYHG3LkpVbA/4FW6wXYzK45ekg5+/gLOTHVyRB8Aif77tvz46F0LcfJG6vi5o…\n→ POST https://tt-profile-wpa.chat.zalo.me/api/social/group/members?zpw_ver=694&zpw_type=30\n  data decrypted (782 bytes)\n  body: params=DKN%2BFfOaJj8Qa9uV43uWPr1T0fSubFcXSomVAzuOjqPGRie5cjACYJR%2B7%2BOG5dyM5obhvgv97Srim326LpLYN1fRDq5PZRuA0QmMRH%2BpZoM%2F0ivluYCTytyqs35zIrv6dix%2BQ9eQ5ONd3lXRQxE9ITHXVl%2B%2FyUVrHPNmFh1Lj8me1Z9q1DZHqKvEqVhSeDktn6J5lufX9VtCcU2Ux6ljmNw5uBDHfChrFezgzJ3FJ8XF7Q3v2GS86Dlag9tbsPSPQnvp6N44U0zaNqclblORu4iTpAUAdetgn75Mw1Kjy4NatPNNg%2B%2FQcWcDh4rWVrtGBwZpioqb4ckn09gppHZAwAGuDlheW3qMpZ7CVHjrc3dIfWPesUjrN…\n→ POST https://tt-group-wpa.zaloapp.com/api/group/getmg-v2?zpw_ver=694&zpw_type=30\n  body: params=ufFy8KAtP83H8KYzSWV6b6pZfNSpE9%2FP4S2s8jn4kcCM63qNoyhjbFfuumCOXos1QOl%2FeQQGKcdA1xut1ymJbQvzl86%2BMz9561guQOBYSRfhu1gE90JYKhopPIuwOvN2R6P4lUyk2QhC8%2Bw8jDYu96FeZA%2BkVaS%2BZEBTXXsyzkfIF1SVKHCVBfL7LNkl0oA5NvUqws5df4Mk7RHFVLDX8RFHT1G%2F%2FWpY7BPkIYF2f%2BpMNkFe%2B2OK9YqOGmeO07oA10fluH6YiNwAlL%2BUOSXKjxPrno3AudSjF1ZR8d2ShVus1Baik9cXZvSDdL5oZgh0L3Tk5T5RdzNXuoXjOC7TGzhfryldMNPal3PG7ljcbD%2FMkT%2F…\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/abJVlG4WIkDckEJhSf8xuRMP3kl50Mjlt4N8JUePXX4cgy5iqocUHL5z8N5MNjZ8NaM98vl87Jbjwwrad+4dAkyeCMoK5+oRueX+0YLw9bU7ArzzMMqMHIpEqXlvZX7j5+VppgIRvyoz/oOfeUjd6twtQnjI+/m6MJ/xxrTSoXtjlFP7n5GP5vtl6M1DIAxC3Bj4mFfP7Zyn/ZzujBEI9yxbADJTdRc0X/SY6biy61QrLGnCtrEBcmwtE86KGkj9syDZYJa7wq3HGD06l7KYmezfhzJ…\n  data decrypted (6432 bytes)\n→ GET https://p4-msg-chat.zaloapp.com/?zpw_ver=694&zpw_type=30&params=<redacted>&ts=1793714265\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/auRfO7mg2xvP99Q9keko/sSEod8dMdjnUlq3PYwOdM0KpeVNHNlgHdU6l8aNyFEi6TqPpMNDoHMAZ5PJ+AzpmWl8rdAZ+nONZss7qb+zKUWDnOB2emU91hCJnAzk1/KVvT1cDD47krsddZ/nuV0kBOCRmDgLqeTDHo/b8KIiiDzTjaKJ/zzfl/XyrztCT8cgBJMF5eAeP54HlpKVDCZSVIo86Gz6T/8De8n5n4RV7543glhxQr0JNSvX9328M2X8BjYRwgkuaH28J+NddomM2pSkb43…\n  data decrypted (22724 bytes)\n→ POST https://tt-profile-wpa.chat.zalo.me/api/social/group/members?zpw_ver=694&zpw_type=30\n  body: params=DKN%2BFfOaJj8Qa9uV43uWPr1T0fSubFcXSomVAzuOjqPGRie5cjACYJR%2B7%2BOG5dyM5obhvgv97Srim326LpLYN1fRDq5PZRuA0QmMRH%2BpZoM%2F0ivluYCTytyqs35zIrv6dix%2BQ9eQ5ONd3lXRQxE9ITHXVl%2B%2FyUVrHPNmFh1Lj8me1Z9q1DZHqKvEqVhSeDktn6J5lufX9VtCcU2Ux6ljmNw5uBDHfChrFezgzJ3FJ8XF7Q3v2GS86Dlag9tbsPSPQnvp6N44U0zaNqclblORu4iTpAUAdetgn75Mw1Kjy4NatPNNg%2B%2FQcWcDh4rWVrtGBwZpioqb4ckn09gppHZAwAGuDlheW3qMpZ7CVHjrc3dIfWPesUjrN…\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/abJVlG4WIkDckEJhSf8xuRMP3kl50Mjlt4N8JUePXX4cgy5iqocUHL5z8N5MNjZ8NaM98vl87Jbjwwrad+4dAkyeCMoK5+oRueX+0YLw9bU7ArzzMMqMHIpEqXlvZX7j5+VppgIRvyoz/oOfeUjd6twtQnjI+/m6MJ/xxrTSoXtjlFP7n5GP5vtl6M1DIAxC3Bj4mFfP7Zyn/ZzujBEI9yxbADJTdRc0X/SY6biy61QrLGnCtrEBcmwtE86KGkj9syDZYJa7wq3HGD06l7KYmezfhzJ…\n  data decrypted (6432 bytes)\n→ GET https://p4-msg-chat.zaloapp.com/?zpw_ver=694&zpw_type=30&params=<redacted>&ts=1793714265\n← 200 OK {\"error_code\":0,\"error_message\":\"\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVaux4zQ/8TsNHmp9153vYMAxKsFiBoOxdp5eminGaWpYM0/f2IUdfNJxuPv7TY9GrfFYYEE9AGuk6IEuzpMXLnUdA9WMslZZRFnlH6yj+ugLB0vScBYNQMdawz8CRhcCkYuQsGm7gc7+mR2GQn8y2wq5HSb4TpGTVAiK+pQYcFbpHoGQeMDqsprzV6qrMlli1c+yDjb2VccNvxLaS6y+njkoxJpUhquoZZTC69dPTgCK5gYkYQHfBVGf92qBITbxJ5+hppOWFIso8FWGCmr780sEm1Iw7NslCITgueSmMPRkVSNuD/sdCA1inTLZib…\n  data decrypted (6351 bytes)\n→ GET https://tt-convers-wpa.chat.zalo.me/api/preloadconvers/get-last-msgs?zpw_ver=694&zpw_type=30&params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVaux4zQ/8TsNHmp9153vYMAxKsFiBoOxdp5eminGaWpYM0/f2IUdfNJxuPv7TY9GrfFYYEE9AGuk6IEuzpMXLnUdA9WMslZZRFnlH6yj+ugLB0vScBYNQMdawz8CRhcCkYuQsGm7gc7+mR2GQn8y2wq5HSb4TpGTVAiK+pQYcFbpHoGQeMDqsprzV6qrMlli1c+yDjb2VccNvxLaS6y+njkoxJpUhquoZZTC69dPTgCK5gYkYQHfBVGf92qBITbxJ5+hppOWFIso8FWGCmr780sEm1Iw7NslCITgueSmMPRkVSNuD/sdCA1inTLZib…\n  data decrypted (6351 bytes)\n→ GET https://tt-convers-wpa.chat.zalo.me/api/preloadconvers/get-last-msgs?zpw_ver=694&zpw_type=30&params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aPMY1Ab09jDIUre+Op198w5PO5nbf4Z3pNQPEz+F5zrtoiwey8Q0sJc2MZK6em6dDEeQA4YUw94lShS2PqSyuqbtbXosulBQBuNASJWtAGxPNrJrrjVzk/PAoI0wKqrIWnJxoUB40KTPCddrCs7Cuj7wToUUF3Y4+2SK7d3b10CGoaeeiP/ZfN6UuATXu+laJeUa7qDGcHRZmX6zX6ge6xkibtx6mbm3RHQezyrnWDqcDfWMYBEXxP+HiblyoT/+rpH4EYdWigxXGV+h35D9etXsaLy…\n  data decrypted (15737 bytes)\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aPMY1Ab09jDIUre+Op198w5PO5nbf4Z3pNQPEz+F5zrtoiwey8Q0sJc2MZK6em6dDEeQA4YUw94lShS2PqSyuqbtbXosulBQBuNASJWtAGxPNrJrrjVzk/PAoI0wKqrIWnJxoUB40KTPCddrCs7Cuj7wToUUF3Y4+2SK7d3b10CGoaeeiP/ZfN6UuATXu+laJeUa7qDGcHRZmX6zX6ge6xkibtx6mbm3RHQezyrnWDqcDfWMYBEXxP+HiblyoT/+rpH4EYdWigxXGV+h35D9etXsaLy…\n→ GET https://tt-convers-wpa.chat.zalo.me/api/preloadconvers/get-last-msgs?zpw_ver=694&zpw_type=30&params=<redacted>\n  data decrypted (15737 bytes)\n→ GET https://tt-convers-wpa.chat.zalo.me/api/preloadconvers/get-last-msgs?zpw_ver=694&zpw_type=30&params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aPMY1Ab09jDIUre+Op198w5PO5nbf4Z3pNQPEz+F5zrtoiwey8Q0sJc2MZK6em6dDEeQA4YUw94lShS2PqSyuqbtbXosulBQBuNASJWtAGxPNrJrrjVzk/PAoI0wKqrIWnJxoUB40KTPCddrCs7Cuj7wToUUF3Y4+2SK7d3b10CGoaeeiP/ZfN6UuATXu+laJeUa7qDGcHRZmX6zX6ge6xkibtx6mbm3RHQezyrnWDqcDfWMYBEXxP+HiblyoT/+rpH4EYdWigxXGV+h35D9etXsaLy…\n  data decrypted (15737 bytes)\n→ GET https://tt-convers-wpa.chat.zalo.me/api/preloadconvers/get-last-msgs?zpw_ver=694&zpw_type=30&params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aPMY1Ab09jDIUre+Op198w5PO5nbf4Z3pNQPEz+F5zrtoiwey8Q0sJc2MZK6em6dDEeQA4YUw94lShS2PqSyuqbtbXosulBQBuNASJWtAGxPNrJrrjVzk/PAoI0wKqrIWnJxoUB40KTPCddrCs7Cuj7wToUUF3Y4+2SK7d3b10CGoaeeiP/ZfN6UuATXu+laJeUa7qDGcHRZmX6zX6ge6xkibtx6mbm3RHQezyrnWDqcDfWMYBEXxP+HiblyoT/+rpH4EYdWigxXGV+h35D9etXsaLy…\n  data decrypted (15737 bytes)\n→ GET https://tt-convers-wpa.chat.zalo.me/api/preloadconvers/get-last-msgs?zpw_ver=694&zpw_type=30&params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aPMY1Ab09jDIUre+Op198w5PO5nbf4Z3pNQPEz+F5zrtoiwey8Q0sJc2MZK6em6dDEeQA4YUw94lShS2PqSyuqbtbXosulBQBuNASJWtAGxPNrJrrjVzk/PAoI0wKqrIWnJxoUB40KTPCddrCs7Cuj7wToUUF3Y4+2SK7d3b10CGoaeeiP/ZfN6UuATXu+laJeUa7qDGcHRZmX6zX6ge6xkibtx6mbm3RHQezyrnWDqcDfWMYBEXxP+HiblyoT/+rpH4EYdWigxXGV+h35D9etXsaLy…\n  data decrypted (15737 bytes)\n→ GET https://tt-convers-wpa.chat.zalo.me/api/preloadconvers/get-last-msgs?zpw_ver=694&zpw_type=30&params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aPMY1Ab09jDIUre+Op198w5PO5nbf4Z3pNQPEz+F5zrtoiwey8Q0sJc2MZK6em6dDEeQA4YUw94lShS2PqSyuqbtbXosulBQBuNASJWtAGxPNrJrrjVzk/PAoI0wKqrIWnJxoUB40KTPCddrCs7Cuj7wToUUF3Y4+2SK7d3b10CGoaeeiP/ZfN6UuATXu+laJeUa7qDGcHRZmX6zX6ge6xkibtx6mbm3RHQezyrnWDqcDfWMYBEXxP+HiblyoT/+rpH4EYdWigxXGV+h35D9etXsaLy…\n  data decrypted (15737 bytes)\n→ GET https://tt-convers-wpa.chat.zalo.me/api/preloadconvers/get-last-msgs?zpw_ver=694&zpw_type=30&params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aPMY1Ab09jDIUre+Op198w5PO5nbf4Z3pNQPEz+F5zrtoiwey8Q0sJc2MZK6em6dDEeQA4YUw94lShS2PqSyuqbtbXosulBQBuNASJWtAGxPNrJrrjVzk/PAoI0wKqrIWnJxoUB40KTPCddrCs7Cuj7wToUUF3Y4+2SK7d3b10CGoaeeiP/ZfN6UuATXu+laJeUa7qDGcHRZmX6zX6ge6xkibtx6mbm3RHQezyrnWDqcDfWMYBEXxP+HiblyoT/+rpH4EYdWigxXGV+h35D9etXsaLy…\n  data decrypted (15737 bytes)\n→ GET https://tt-convers-wpa.chat.zalo.me/api/preloadconvers/get-last-msgs?zpw_ver=694&zpw_type=30&params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aPMY1Ab09jDIUre+Op198w5PO5nbf4Z3pNQPEz+F5zrtoiwey8Q0sJc2MZK6em6dDEeQA4YUw94lShS2PqSyuqbtbXosulBQBuNASJWtAGxPNrJrrjVzk/PAoI0wKqrIWnJxoUB40KTPCddrCs7Cuj7wToUUF3Y4+2SK7d3b10CGoaeeiP/ZfN6UuATXu+laJeUa7qDGcHRZmX6zX6ge6xkibtx6mbm3RHQezyrnWDqcDfWMYBEXxP+HiblyoT/+rpH4EYdWigxXGV+h35D9etXsaLy…\n  data decrypted (15737 bytes)\n→ GET https://tt-convers-wpa.chat.zalo.me/api/preloadconvers/get-last-msgs?zpw_ver=694&zpw_type=30&params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aPMY1Ab09jDIUre+Op198w5PO5nbf4Z3pNQPEz+F5zrtoiwey8Q0sJc2MZK6em6dDEeQA4YUw94lShS2PqSyuqbtbXosulBQBuNASJWtAGxPNrJrrjVzk/PAoI0wKqrIWnJxoUB40KTPCddrCs7Cuj7wToUUF3Y4+2SK7d3b10CGoaeeiP/ZfN6UuATXu+laJeUa7qDGcHRZmX6zX6ge6xkibtx6mbm3RHQezyrnWDqcDfWMYBEXxP+HiblyoT/+rpH4EYdWigxXGV+h35D9etXsaLy…\n  data decrypted (15737 bytes)\n→ GET https://tt-convers-wpa.chat.zalo.me/api/preloadconvers/get-last-msgs?zpw_ver=694&zpw_type=30&params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aPMY1Ab09jDIUre+Op198w5PO5nbf4Z3pNQPEz+F5zrtoiwey8Q0sJc2MZK6em6dDEeQA4YUw94lShS2PqSyuqbtbXosulBQBuNASJWtAGxPNrJrrjVzk/PAoI0wKqrIWnJxoUB40KTPCddrCs7Cuj7wToUUF3Y4+2SK7d3b10CGoaeeiP/ZfN6UuATXu+laJeUa7qDGcHRZmX6zX6ge6xkibtx6mbm3RHQezyrnWDqcDfWMYBEXxP+HiblyoT/+rpH4EYdWigxXGV+h35D9etXsaLy…\n  data decrypted (15737 bytes)\n→ GET https://tt-convers-wpa.chat.zalo.me/api/preloadconvers/get-last-msgs?zpw_ver=694&zpw_type=30&params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aPMY1Ab09jDIUre+Op198w5PO5nbf4Z3pNQPEz+F5zrtoiwey8Q0sJc2MZK6em6dDEeQA4YUw94lShS2PqSyuqbtbXosulBQBuNASJWtAGxPNrJrrjVzk/PAoI0wKqrIWnJxoUB40KTPCddrCs7Cuj7wToUUF3Y4+2SK7d3b10CGoaeeiP/ZfN6UuATXu+laJeUa7qDGcHRZmX6zX6ge6xkibtx6mbm3RHQezyrnWDqcDfWMYBEXxP+HiblyoT/+rpH4EYdWigxXGV+h35D9etXsaLy…\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aPMY1Ab09jDIUre+Op198w5PO5nbf4Z3pNQPEz+F5zrtoiwey8Q0sJc2MZK6em6dDEeQA4YUw94lShS2PqSyuqbtbXosulBQBuNASJWtAGxPNrJrrjVzk/PAoI0wKqrIWnJxoUB40KTPCddrCs7Cuj7wToUUF3Y4+2SK7d3b10CGoaeeiP/ZfN6UuATXu+laJeUa7qDGcHRZmX6zX6ge6xkibtx6mbm3RHQezyrnWDqcDfWMYBEXxP+HiblyoT/+rpH4EYdWigxXGV+h35D9etXsaLy…\n  data decrypted (15737 bytes)\n  data decrypted (15737 bytes)\n→ GET https://tt-convers-wpa.chat.zalo.me/api/preloadconvers/get-last-msgs?zpw_ver=694&zpw_type=30&params=<redacted>\n→ GET https://tt-convers-wpa.chat.zalo.me/api/preloadconvers/get-last-msgs?zpw_ver=694&zpw_type=30&params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aPMY1Ab09jDIUre+Op198w5PO5nbf4Z3pNQPEz+F5zrtoiwey8Q0sJc2MZK6em6dDEeQA4YUw94lShS2PqSyuqbtbXosulBQBuNASJWtAGxPNrJrrjVzk/PAoI0wKqrIWnJxoUB40KTPCddrCs7Cuj7wToUUF3Y4+2SK7d3b10CGoaeeiP/ZfN6UuATXu+laJeUa7qDGcHRZmX6zX6ge6xkibtx6mbm3RHQezyrnWDqcDfWMYBEXxP+HiblyoT/+rpH4EYdWigxXGV+h35D9etXsaLy…\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aPMY1Ab09jDIUre+Op198w5PO5nbf4Z3pNQPEz+F5zrtoiwey8Q0sJc2MZK6em6dDEeQA4YUw94lShS2PqSyuqbtbXosulBQBuNASJWtAGxPNrJrrjVzk/PAoI0wKqrIWnJxoUB40KTPCddrCs7Cuj7wToUUF3Y4+2SK7d3b10CGoaeeiP/ZfN6UuATXu+laJeUa7qDGcHRZmX6zX6ge6xkibtx6mbm3RHQezyrnWDqcDfWMYBEXxP+HiblyoT/+rpH4EYdWigxXGV+h35D9etXsaLy…\n  data decrypted (15737 bytes)\n  data decrypted (15737 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zoGoeJwZv2Il2rW6dab0HrPIjV2TvIHm0p/qI0GPblmhXcJSdjsVCVOWSMZKHIRP30IGElVSeR04ueDtansFz/1vLgnQ00WkSXBcjR9UofZw8g6O46VoZ/nljzjrwaRZosmdnKl0a/Gpk+H4PUdiP1fnVwMswovzBdZY52E7fEg9OH/wdSLFWBlJH1FQe4wC3q1oZKyiny1t8KCtDXoyGeIPXAkPH0WUmBiQn7UM+BGXSrK8I…\n  data decrypted (38213 bytes)\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zoGoeJwZv2Il2rW6dab0HrPIjV2TvIHm0p/qI0GPblmhXcJSdjsVCVOWSMZKHIRP30IGElVSeR04ueDtansFz/1vLgnQ00WkSXBcjR9UofZw8g6O46VoZ/nljzjrwaRZosmdnKl0a/Gpk+H4PUdiP1fnVwMswovzBdZY52E7fEg9OH/wdSLFWBlJH1FQe4wC3q1oZKyiny1t8KCtDXoyGeIPXAkPH0WUmBiQn7UM+BGXSrK8I…\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n  data decrypted (38213 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zpaj7+bIi/umCdbIZ7goqaEABhU9sqzP6boeWOLrs0Dq3GtF+vdQJtVvUwsDiDWvMBrswsNaAsNBAZAMNDld0ZUbyU0ZKXijCBjR2F/3E9Xt7zAwcSMY4Tjetj+L1DrTkC9hOxXUsDENYwrgLLeJ24HdffcQwt8hIPCY4EbvITx6Jdp+R3BsFXaqiPs8UJPpCg2WV+BsMohhwCsY7YcDf0FmSS789h7mc6KGgxeFW2ZS20P2p…\n  data decrypted (276 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zpaj7+bIi/umCdbIZ7goqaEABhU9sqzP6boeWOLrs0Dq3GtF+vdQJtVvUwsDiDWvMBrswsNaAsNBAZAMNDld0ZUbyU0ZKXijCBjR2F/3E9Xt7zAwcSMY4Tjetj+L1DrTkC9hOxXUsDENYwrgLLeJ24HdffcQwt8hIPCY4EbvITx6Jdp+R3BsFXaqiPs8UJPpCg2WV+BsMohhwCsY7YcDf0FmSS789h7mc6KGgxeFW2ZS20P2p…\n  data decrypted (276 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zpaj7+bIi/umCdbIZ7goqaEABhU9sqzP6boeWOLrs0Dq3GtF+vdQJtVvUwsDiDWvMBrswsNaAsNBAZAMNDld0ZUbyU0ZKXijCBjR2F/3E9Xt7zAwcSMY4Tjetj+L1DrTkC9hOxXUsDENYwrgLLeJ24HdffcQwt8hIPCY4EbvITx6Jdp+R3BsFXaqiPs8UJPpCg2WV+BsMohhwCsY7YcDf0FmSS789h7mc6KGgxeFW2ZS20P2p…\n  data decrypted (276 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zpaj7+bIi/umCdbIZ7goqaEABhU9sqzP6boeWOLrs0Dq3GtF+vdQJtVvUwsDiDWvMBrswsNaAsNBAZAMNDld0ZUbyU0ZKXijCBjR2F/3E9Xt7zAwcSMY4Tjetj+L1DrTkC9hOxXUsDENYwrgLLeJ24HdffcQwt8hIPCY4EbvITx6Jdp+R3BsFXaqiPs8UJPpCg2WV+BsMohhwCsY7YcDf0FmSS789h7mc6KGgxeFW2ZS20P2p…\n  data decrypted (276 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zqhIo3kw2HgEk3BfDj/53rmtM41jcSnSU3cvo7fobOE5hrgTrzLISRItsnDpQsERifonj1JXRi0uk5ygYqfdqNJGRf84Gui7/vXHuvuwQKA5Ym6vDWJfv7pGxDuKBGhlfsFVc3QViHYrFeXWDlonN9A0IoT79h0peuvblQ2B9xW3Nsy1nwTnG53m5CoN4cU/f8Jlpm+nAekLgk1+PccbcqCezjTWN+G37TB7lK/EHbgbhZDSZ…\n  data decrypted (350 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zqhIo3kw2HgEk3BfDj/53rmtM41jcSnSU3cvo7fobOE5hrgTrzLISRItsnDpQsERifonj1JXRi0uk5ygYqfdqNJGRf84Gui7/vXHuvuwQKA5Ym6vDWJfv7pGxDuKBGhlfsFVc3QViHYrFeXWDlonN9A0IoT79h0peuvblQ2B9xW3Nsy1nwTnG53m5CoN4cU/f8Jlpm+nAekLgk1+PccbcqCezjTWN+G37TB7lK/EHbgbhZDSZ…\n  data decrypted (350 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zojMvRrquMI45/ggqiy2/Cl73kokk1Ce/cD9xjKJa7zEQjSEkjPlFbYdUxxlrc8rqxSAzCFiD5YME6UWjyoQqX7M2SpM3Z0WuU6sJeEmzTklLdW81LIVkWQ1JjBC1yj5oqgvGw30CYvDWJ+Vj0nIzDcWA31qB3B/3l40RLOcB/zmEQB3ub3On5/iBedhkd1Yoet0ubDpnXTXU8vItCV+2rwQA4gdAI+F6ENyHFxXYjX5NmPKT…\n  data decrypted (350 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zojMvRrquMI45/ggqiy2/Cl73kokk1Ce/cD9xjKJa7zEQjSEkjPlFbYdUxxlrc8rqxSAzCFiD5YME6UWjyoQqX7M2SpM3Z0WuU6sJeEmzTklLdW81LIVkWQ1JjBC1yj5oqgvGw30CYvDWJ+Vj0nIzDcWA31qB3B/3l40RLOcB/zmEQB3ub3On5/iBedhkd1Yoet0ubDpnXTXU8vItCV+2rwQA4gdAI+F6ENyHFxXYjX5NmPKT…\n  data decrypted (350 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zorZT6BznkHYJ7azummz8ISoKF23Mr/4LJTEVTSyy3XWp8mKvKaLJjWW1rOG1ZDdwRwQ08T8fnoOCU5lBjykKN0aL9v53n1pItVl+VGHiMeY0VV8KMubqkJqS5QYas+fYBoQHhr07kpiPmv/Y1zUyvzv6K8hYShyqtoDpOQzbLflihLWZD8BVh0d24eev4n0R6D4EDDrIGxV83SwsOzn7HnSUzev54+++iY9vFysCq9XMvnCK…\n  data decrypted (350 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zorZT6BznkHYJ7azummz8ISoKF23Mr/4LJTEVTSyy3XWp8mKvKaLJjWW1rOG1ZDdwRwQ08T8fnoOCU5lBjykKN0aL9v53n1pItVl+VGHiMeY0VV8KMubqkJqS5QYas+fYBoQHhr07kpiPmv/Y1zUyvzv6K8hYShyqtoDpOQzbLflihLWZD8BVh0d24eev4n0R6D4EDDrIGxV83SwsOzn7HnSUzev54+++iY9vFysCq9XMvnCK…\n  data decrypted (350 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zrw9PV4mX79Oh2IuLN5O5M+slILbDq7awvRVIYhGFBolzIIlov6Hi6B9b6aUyljY4ArbtzH1CL1jxMR3cbvzgW2mwlLi9SREFBzKzLZpcqRr9kLVYZNfG9M6afqF/DcQE5eiecwzJmY56g7iCo1ATO4zyBFPZRwYr1gjsQIuank8WJTAj1x2pEbxNClBWKDIfq4zEVlVg4y7F2c1UfFiBWadV8mbPC0lZbcS9hZiZJ0NyEbz6…\n  data decrypted (350 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zrw9PV4mX79Oh2IuLN5O5M+slILbDq7awvRVIYhGFBolzIIlov6Hi6B9b6aUyljY4ArbtzH1CL1jxMR3cbvzgW2mwlLi9SREFBzKzLZpcqRr9kLVYZNfG9M6afqF/DcQE5eiecwzJmY56g7iCo1ATO4zyBFPZRwYr1gjsQIuank8WJTAj1x2pEbxNClBWKDIfq4zEVlVg4y7F2c1UfFiBWadV8mbPC0lZbcS9hZiZJ0NyEbz6…\n  data decrypted (350 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zovxtj7KnZpRtCDu3SFYSjapjp+DA8ZenZMtPOJpbuEuBpBrLJWxLuWRhtfKYVRJwg2Jg6Rbs37mpveWcHWitfR6WKqDj9MkC2wRL8CBi8AlxRkhj4f+mvu3ApE50RBMzunSpArN0RqETETzM2nopxowwaUBG87kRPcFQNDLs9+KduEdkALji5R60Fbe4x3n5949IL78IfVLWzbNcQWjLzXzA4pMBjjAvk9ji1Qqzgh1tQ1ZO…\n  data decrypted (350 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zovxtj7KnZpRtCDu3SFYSjapjp+DA8ZenZMtPOJpbuEuBpBrLJWxLuWRhtfKYVRJwg2Jg6Rbs37mpveWcHWitfR6WKqDj9MkC2wRL8CBi8AlxRkhj4f+mvu3ApE50RBMzunSpArN0RqETETzM2nopxowwaUBG87kRPcFQNDLs9+KduEdkALji5R60Fbe4x3n5949IL78IfVLWzbNcQWjLzXzA4pMBjjAvk9ji1Qqzgh1tQ1ZO…\n  data decrypted (350 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zpaj7+bIi/umCdbIZ7goqaEABhU9sqzP6boeWOLrs0Dq3GtF+vdQJtVvUwsDiDWvMBrswsNaAsNBAZAMNDld0ZUbyU0ZKXijCBjR2F/3E9Xt7zAwcSMY4Tjetj+L1DrTkC9hOxXUsDENYwrgLLeJ24HdffcQwt8hIPCY4EbvITx6Jdp+R3BsFXaqiPs8UJPpCg2WV+BsMohhwCsY7YcDf0FmSS789h7mc6KGgxeFW2ZS20P2p…\n  data decrypted (276 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zpaj7+bIi/umCdbIZ7goqaEABhU9sqzP6boeWOLrs0Dq3GtF+vdQJtVvUwsDiDWvMBrswsNaAsNBAZAMNDld0ZUbyU0ZKXijCBjR2F/3E9Xt7zAwcSMY4Tjetj+L1DrTkC9hOxXUsDENYwrgLLeJ24HdffcQwt8hIPCY4EbvITx6Jdp+R3BsFXaqiPs8UJPpCg2WV+BsMohhwCsY7YcDf0FmSS789h7mc6KGgxeFW2ZS20P2p…\n  data decrypted (276 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zpaj7+bIi/umCdbIZ7goqaEABhU9sqzP6boeWOLrs0Dq3GtF+vdQJtVvUwsDiDWvMBrswsNaAsNBAZAMNDld0ZUbyU0ZKXijCBjR2F/3E9Xt7zAwcSMY4Tjetj+L1DrTkC9hOxXUsDENYwrgLLeJ24HdffcQwt8hIPCY4EbvITx6Jdp+R3BsFXaqiPs8UJPpCg2WV+BsMohhwCsY7YcDf0FmSS789h7mc6KGgxeFW2ZS20P2p…\n  data decrypted (276 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zpaj7+bIi/umCdbIZ7goqaEABhU9sqzP6boeWOLrs0Dq3GtF+vdQJtVvUwsDiDWvMBrswsNaAsNBAZAMNDld0ZUbyU0ZKXijCBjR2F/3E9Xt7zAwcSMY4Tjetj+L1DrTkC9hOxXUsDENYwrgLLeJ24HdffcQwt8hIPCY4EbvITx6Jdp+R3BsFXaqiPs8UJPpCg2WV+BsMohhwCsY7YcDf0FmSS789h7mc6KGgxeFW2ZS20P2p…\n  data decrypted (276 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zoB6W9i0se6G6J+X0f4qcmmiPeO7CqKV3HOAiXiSY8Yj4uM8CavwtMhrlOWgELif6LrV5LR3mPbj5iZz/t+z0QmwMv9amGY8DllkeUldztehn5v5FAG1wbIR84RmVHX/NyGmwVTNuMfiXydjmUZeU3GuNHM3UfCHriitAO9Ro2LDVDC2csuXGZWcucTyrJNbuUApG2kiz87/FTbpXJnoC8iNHR7RS+3bw2tpYxgcN9Ww8Hrwe…\n  data decrypted (350 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zoB6W9i0se6G6J+X0f4qcmmiPeO7CqKV3HOAiXiSY8Yj4uM8CavwtMhrlOWgELif6LrV5LR3mPbj5iZz/t+z0QmwMv9amGY8DllkeUldztehn5v5FAG1wbIR84RmVHX/NyGmwVTNuMfiXydjmUZeU3GuNHM3UfCHriitAO9Ro2LDVDC2csuXGZWcucTyrJNbuUApG2kiz87/FTbpXJnoC8iNHR7RS+3bw2tpYxgcN9Ww8Hrwe…\n  data decrypted (350 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zpEBgciNatPb37JS2LitdhGaUWrpavD+yeHe4kkHZOjFdZtH9c5lBlM6sBTqsqD0FI2cmwCUXA345/PdJK91CQ91TO+SAYCWzW52Qkjw/TaqPo+ErkI5z7iq/1jJ2gEXaJOuiUo1fkSDlGCj9CFV8gpQriPFFqTew5ecNbBm1vTPmDFKLnidK/63+WYZfTbWAi8VQavhEIP6+hIRTuCbHV3nSRmeTMgZLscjEyBP5FOjcPmn+…\n  data decrypted (350 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zpEBgciNatPb37JS2LitdhGaUWrpavD+yeHe4kkHZOjFdZtH9c5lBlM6sBTqsqD0FI2cmwCUXA345/PdJK91CQ91TO+SAYCWzW52Qkjw/TaqPo+ErkI5z7iq/1jJ2gEXaJOuiUo1fkSDlGCj9CFV8gpQriPFFqTew5ecNbBm1vTPmDFKLnidK/63+WYZfTbWAi8VQavhEIP6+hIRTuCbHV3nSRmeTMgZLscjEyBP5FOjcPmn+…\n  data decrypted (350 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zpaj7+bIi/umCdbIZ7goqaEABhU9sqzP6boeWOLrs0Dq3GtF+vdQJtVvUwsDiDWvMBrswsNaAsNBAZAMNDld0ZUbyU0ZKXijCBjR2F/3E9Xt7zAwcSMY4Tjetj+L1DrTkC9hOxXUsDENYwrgLLeJ24HdffcQwt8hIPCY4EbvITx6Jdp+R3BsFXaqiPs8UJPpCg2WV+BsMohhwCsY7YcDf0FmSS789h7mc6KGgxeFW2ZS20P2p…\n  data decrypted (276 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zpaj7+bIi/umCdbIZ7goqaEABhU9sqzP6boeWOLrs0Dq3GtF+vdQJtVvUwsDiDWvMBrswsNaAsNBAZAMNDld0ZUbyU0ZKXijCBjR2F/3E9Xt7zAwcSMY4Tjetj+L1DrTkC9hOxXUsDENYwrgLLeJ24HdffcQwt8hIPCY4EbvITx6Jdp+R3BsFXaqiPs8UJPpCg2WV+BsMohhwCsY7YcDf0FmSS789h7mc6KGgxeFW2ZS20P2p…\n  data decrypted (276 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zpaj7+bIi/umCdbIZ7goqaEABhU9sqzP6boeWOLrs0Dq3GtF+vdQJtVvUwsDiDWvMBrswsNaAsNBAZAMNDld0ZUbyU0ZKXijCBjR2F/3E9Xt7zAwcSMY4Tjetj+L1DrTkC9hOxXUsDENYwrgLLeJ24HdffcQwt8hIPCY4EbvITx6Jdp+R3BsFXaqiPs8UJPpCg2WV+BsMohhwCsY7YcDf0FmSS789h7mc6KGgxeFW2ZS20P2p…\n  data decrypted (276 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zpaj7+bIi/umCdbIZ7goqaEABhU9sqzP6boeWOLrs0Dq3GtF+vdQJtVvUwsDiDWvMBrswsNaAsNBAZAMNDld0ZUbyU0ZKXijCBjR2F/3E9Xt7zAwcSMY4Tjetj+L1DrTkC9hOxXUsDENYwrgLLeJ24HdffcQwt8hIPCY4EbvITx6Jdp+R3BsFXaqiPs8UJPpCg2WV+BsMohhwCsY7YcDf0FmSS789h7mc6KGgxeFW2ZS20P2p…\n  data decrypted (276 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zpaj7+bIi/umCdbIZ7goqaEABhU9sqzP6boeWOLrs0Dq3GtF+vdQJtVvUwsDiDWvMBrswsNaAsNBAZAMNDld0ZUbyU0ZKXijCBjR2F/3E9Xt7zAwcSMY4Tjetj+L1DrTkC9hOxXUsDENYwrgLLeJ24HdffcQwt8hIPCY4EbvITx6Jdp+R3BsFXaqiPs8UJPpCg2WV+BsMohhwCsY7YcDf0FmSS789h7mc6KGgxeFW2ZS20P2p…\n  data decrypted (276 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zpaj7+bIi/umCdbIZ7goqaEABhU9sqzP6boeWOLrs0Dq3GtF+vdQJtVvUwsDiDWvMBrswsNaAsNBAZAMNDld0ZUbyU0ZKXijCBjR2F/3E9Xt7zAwcSMY4Tjetj+L1DrTkC9hOxXUsDENYwrgLLeJ24HdffcQwt8hIPCY4EbvITx6Jdp+R3BsFXaqiPs8UJPpCg2WV+BsMohhwCsY7YcDf0FmSS789h7mc6KGgxeFW2ZS20P2p…\n  data decrypted (276 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zpaj7+bIi/umCdbIZ7goqaEABhU9sqzP6boeWOLrs0Dq3GtF+vdQJtVvUwsDiDWvMBrswsNaAsNBAZAMNDld0ZUbyU0ZKXijCBjR2F/3E9Xt7zAwcSMY4Tjetj+L1DrTkC9hOxXUsDENYwrgLLeJ24HdffcQwt8hIPCY4EbvITx6Jdp+R3BsFXaqiPs8UJPpCg2WV+BsMohhwCsY7YcDf0FmSS789h7mc6KGgxeFW2ZS20P2p…\n  data decrypted (276 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zp7tWBPJHRcwQNiHlCIillJONlr8Z4eP25yJ0rjz1bV8s6gpMP7U7C6Z5UEq7cNZrL8zjoCqAUrEoqB1+NfhE8L5ZIzdzxpYxOwWH3PgnaB8GR0ycFQBxqXzJwuGQUqumfK4PXAlmo1Sx0IhdhSYPWEImcBgnCbBOiVdfrLC0vhOlKGMzzlp+yO0ZeWAsC6n0jd/COuuHW0R5GtFVCgaTf8lpcOrirW77Q06URgFN/+bQSKap…\n  data decrypted (350 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zpaj7+bIi/umCdbIZ7goqaEABhU9sqzP6boeWOLrs0Dq3GtF+vdQJtVvUwsDiDWvMBrswsNaAsNBAZAMNDld0ZUbyU0ZKXijCBjR2F/3E9Xt7zAwcSMY4Tjetj+L1DrTkC9hOxXUsDENYwrgLLeJ24HdffcQwt8hIPCY4EbvITx6Jdp+R3BsFXaqiPs8UJPpCg2WV+BsMohhwCsY7YcDf0FmSS789h7mc6KGgxeFW2ZS20P2p…\n  data decrypted (276 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zpaj7+bIi/umCdbIZ7goqaEABhU9sqzP6boeWOLrs0Dq3GtF+vdQJtVvUwsDiDWvMBrswsNaAsNBAZAMNDld0ZUbyU0ZKXijCBjR2F/3E9Xt7zAwcSMY4Tjetj+L1DrTkC9hOxXUsDENYwrgLLeJ24HdffcQwt8hIPCY4EbvITx6Jdp+R3BsFXaqiPs8UJPpCg2WV+BsMohhwCsY7YcDf0FmSS789h7mc6KGgxeFW2ZS20P2p…\n  data decrypted (276 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zpaj7+bIi/umCdbIZ7goqaEABhU9sqzP6boeWOLrs0Dq3GtF+vdQJtVvUwsDiDWvMBrswsNaAsNBAZAMNDld0ZUbyU0ZKXijCBjR2F/3E9Xt7zAwcSMY4Tjetj+L1DrTkC9hOxXUsDENYwrgLLeJ24HdffcQwt8hIPCY4EbvITx6Jdp+R3BsFXaqiPs8UJPpCg2WV+BsMohhwCsY7YcDf0FmSS789h7mc6KGgxeFW2ZS20P2p…\n  data decrypted (276 bytes)\n→ POST https://tt-group-wpa.zaloapp.com/api/group/getmg-v2?zpw_ver=694&zpw_type=30\n  body: params=ufFy8KAtP83H8KYzSWV6b%2BtY%2Bx2kb%2FzFVuL6%2F93TTbTLj6UWVbs%2BnwM1pk4hWg0910HmjXKEvozj%2FjK9Fc6OQQHrW6rj%2BTWF4S44ziKdpScHX%2Bcs7HdyOOuKmO26Z3K0%2FFTMhW1%2F916VoefzhL8oMd4A4P0xdzH3Vb6IZMZq%2B8G%2BTLHons69k5FlOP3gkOD6OrW1z4HQiD03%2FkdovSBbWQ%3D%3D\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zp7tWBPJHRcwQNiHlCIillJONlr8Z4eP25yJ0rjz1bV8s6gpMP7U7C6Z5UEq7cNZrL8zjoCqAUrEoqB1+NfhE8L5ZIzdzxpYxOwWH3PgnaB8GR0ycFQBxqXzJwuGQUqumfK4PXAlmo1Sx0IhdhSYPWEImcBgnCbBOiVdfrLC0vhOlKGMzzlp+yO0ZeWAsC6n0jd/COuuHW0R5GtFVCgaTf8lpcOrirW77Q06URgFN/+bQSKap…\n  data decrypted (350 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/auRfO7mg2xvP99Q9keko/sSEod8dMdjnUlq3PYwOdM0KpeVNHNlgHdU6l8aNyFEi6ZLGYqVQ02w/R7hlsuH1AlVG1iuUjVgrgXC4X+OZs7rsR2UY2ZLv6YnAr2cY/FMIRql/7AlCkUPCPZn8k2ahAaHjV3GqGUult2x4s828dwQtrJK9a7M+MzqFvrsD/pdHV7D0fXKeCW65AhYCyApUXHeRPnurvVW4WZQUMaYe0HSd7Q5k//nB0xwBDUDiaoMfoo0W4mqk1GCvThjgVjSXZsQs7LN…\n  data decrypted (2909 bytes)\n→ POST https://tt-profile-wpa.chat.zalo.me/api/social/group/members?zpw_ver=694&zpw_type=30\n  body: params=DKN%2BFfOaJj8Qa9uV43uWPtxKgHvfEQyp%2BIOvJaxS8O3pKh0u0ET3GkqQcOzLQC0fxm5TMjB474AlemcZCrcFpG%2FZKRQqwhLHAWY8QvHy3vbqlsE6vTOL3aJOfkv6Tqn7J%2BAFboqPLX5OHZ%2F7O77SQKjfOgHasdj8Z09Mxt0U%2FMA%3D\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zpaj7+bIi/umCdbIZ7goqaEABhU9sqzP6boeWOLrs0Dq3GtF+vdQJtVvUwsDiDWvMBrswsNaAsNBAZAMNDld0ZUbyU0ZKXijCBjR2F/3E9Xt7zAwcSMY4Tjetj+L1DrTkC9hOxXUsDENYwrgLLeJ24HdffcQwt8hIPCY4EbvITx6Jdp+R3BsFXaqiPs8UJPpCg2WV+BsMohhwCsY7YcDf0FmSS789h7mc6KGgxeFW2ZS20P2p…\n  data decrypted (276 bytes)\n→ GET https://tt-group-cm.api.zaloapp.com/api/cm/getrecentv2?zpw_ver=694&zpw_type=30&params=<redacted>&nretry=0\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/aHV/PLbhRnShPvU2mVGn0LZc36hh42L5u7qwMZ+3l1zpaj7+bIi/umCdbIZ7goqaEABhU9sqzP6boeWOLrs0Dq3GtF+vdQJtVvUwsDiDWvMBrswsNaAsNBAZAMNDld0ZUbyU0ZKXijCBjR2F/3E9Xt7zAwcSMY4Tjetj+L1DrTkC9hOxXUsDENYwrgLLeJ24HdffcQwt8hIPCY4EbvITx6Jdp+R3BsFXaqiPs8UJPpCg2WV+BsMohhwCsY7YcDf0FmSS789h7mc6KGgxeFW2ZS20P2p…\n  data decrypted (276 bytes)\n→ POST https://tt-group-wpa.zaloapp.com/api/group/getmg-v2?zpw_ver=694&zpw_type=30\n  body: params=ufFy8KAtP83H8KYzSWV6b%2BtY%2Bx2kb%2FzFVuL6%2F93TTbTLj6UWVbs%2BnwM1pk4hWg0910HmjXKEvozj%2FjK9Fc6OQQHrW6rj%2BTWF4S44ziKdpScHX%2Bcs7HdyOOuKmO26Z3K0%2FFTMhW1%2F916VoefzhL8oMd4A4P0xdzH3Vb6IZMZq%2B8G%2BTLHons69k5FlOP3gkOD6OrW1z4HQiD03%2FkdovSBbWQ%3D%3D\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/auRfO7mg2xvP99Q9keko/sSEod8dMdjnUlq3PYwOdM0KpeVNHNlgHdU6l8aNyFEi6ZLGYqVQ02w/R7hlsuH1AlVG1iuUjVgrgXC4X+OZs7rsR2UY2ZLv6YnAr2cY/FMIRql/7AlCkUPCPZn8k2ahAaHjV3GqGUult2x4s828dwQtrJK9a7M+MzqFvrsD/pdHV7D0fXKeCW65AhYCyApUXHeRPnurvVW4WZQUMaYe0HSd7Q5k//nB0xwBDUDiaoMfoo0W4mqk1GCvThjgVjSXZsQs7LN…\n  data decrypted (2909 bytes)\n→ POST https://tt-profile-wpa.chat.zalo.me/api/social/group/members?zpw_ver=694&zpw_type=30\n  body: params=DKN%2BFfOaJj8Qa9uV43uWPtxKgHvfEQyp%2BIOvJaxS8O3pKh0u0ET3GkqQcOzLQC0fxm5TMjB474AlemcZCrcFpG%2FZKRQqwhLHAWY8QvHy3vbqlsE6vTOL3aJOfkv6Tqn7J%2BAFboqPLX5OHZ%2F7O77SQKjfOgHasdj8Z09Mxt0U%2FMA%3D\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/abJVlG4WIkDckEJhSf8xuRKH4347snN/w7veSp7l+ww69tEgYc2Oz+zVYGuwqw3rhRH1rP90vb9YaNWC/PS+sP82bR+YssjkO/U1RZDlv++CED36d7Om1bWQAesLMQKf0kTJ3pP8znd3QaLBcBpyZXbvgM27tO4Tmz4Ov6+rPtIsg4rrshJSQo8k52DMhsGH00FwVVBFCGvsizEStVHiiKtAf47grOGvpNgOE6NfFa47st9Wpa8s1aOU02Wmo06rOu3wuIAtu341NK594s4yr8iCMo0…\n  data decrypted (1452 bytes)\n→ GET https://tt-profile-wpa.chat.zalo.me/api/social/profile/getmute?zpw_ver=694&zpw_type=30&params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/abJVlG4WIkDckEJhSf8xuRKH4347snN/w7veSp7l+ww69tEgYc2Oz+zVYGuwqw3rhRH1rP90vb9YaNWC/PS+sP82bR+YssjkO/U1RZDlv++CED36d7Om1bWQAesLMQKf0kTJ3pP8znd3QaLBcBpyZXbvgM27tO4Tmz4Ov6+rPtIsg4rrshJSQo8k52DMhsGH00FwVVBFCGvsizEStVHiiKtAf47grOGvpNgOE6NfFa47st9Wpa8s1aOU02Wmo06rOu3wuIAtu341NK594s4yr8iCMo0…\n  data decrypted (1452 bytes)\n→ GET https://tt-profile-wpa.chat.zalo.me/api/social/profile/getmute?zpw_ver=694&zpw_type=30&params=<redacted>\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/a4DaKbn00wPGaIHmv15J9DS+dXw2dELNU6xsg2NIJvszcr4EbR2qapiPvs/Opoo/X9aheK6y010PdK7G+cNjyPLSvig65O/A0RiZ9mIFs9rw7dBvD3p9DR+CCW8jP86hOsENJR0m8p3PNKLA2K1sEWRZQ0tImOxcWS11p87tqm6eDhgeW8W8OOGvik9xhNKnZSPAiy49qgLdnw6+C5KmSGHpuuFPgJTZWlUeaOcOLMDK3IwmdQWhE23MzQGpW5jONE8IfI5Lkc/pO+7COQ+tiBdgz4m…\n  data decrypted (2207 bytes)\n← 200 OK {\"error_code\":0,\"error_message\":\"Successful.\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVauy4yT3qzXih+QjMrVdZZi/a4DaKbn00wPGaIHmv15J9DS+dXw2dELNU6xsg2NIJvszcr4EbR2qapiPvs/Opoo/X9aheK6y010PdK7G+cNjyPLSvig65O/A0RiZ9mIFs9rw7dBvD3p9DR+CCW8jP86hOsENJR0m8p3PNKLA2K1sEWRZQ0tImOxcWS11p87tqm6eDhgeW8W8OOGvik9xhNKnZSPAiy49qgLdnw6+C5KmSGHpuuFPgJTZWlUeaOcOLMDK3IwmdQWhE23MzQGpW5jONE8IfI5Lkc/pO+7COQ+tiBdgz4m…\n  data decrypted (2207 bytes)\n→ GET https://p4-msg-chat.zaloapp.com/?zpw_ver=694&zpw_type=30&params=<redacted>&ts=1793714266\n← 200 OK {\"error_code\":0,\"error_message\":\"\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVaux4zQ/8TsNHmp9153vYMAxKsFiBoOxdp5eminGaWpYM0/f2IUdfNJxuPv7TY9GrfFYYEE9AGuk6IEuzpMXLnUdA9WMslZZRFnlH6yj+ugLB0vScBYNQMdawz8CRhcCkYuT8d4r5Z3SW9ogIhrfE8CYNI7AeavbFxd4LySF0fqFm/4dx+GcLtAI8D1+IG+rYJY6yBJvDzkNYa2l+gMDV0tJylH+wR/JtDlhsAzKZliOMZBAVTk3yEBosMTC3MXYW1IzSvZmUwmkUYU7k3kfRSPmI/rijFPSdc4uZUrMMS10efVxG9yAWwYvkCFJ2Y…\n  data decrypted (555 bytes)\n→ GET https://p4-msg-chat.zaloapp.com/?zpw_ver=694&zpw_type=30&params=<redacted>&ts=1793714267\n← 200 OK {\"error_code\":0,\"error_message\":\"\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVaux4zQ/8TsNHmp9153vYMAxKsFiBoOxdp5eminGaWpYM0/f2IUdfNJxuPv7TY9GrfFYYEE9AGuk6IEuzpMXLnUdA9WMslZZRFnlH6yj+ugLB0vScBYNQMdawz8CRhcCkYuT8d4r5Z3SW9ogIhrfE8CYNI7AeavbFxd4LySF0fqFm/4dx+GcLtAI8D1+IG+rYJY6yBJvDzkNYa2l+gMDV0tJylH+wR/JtDlhsAzKZliOMZBAVTk3yEBosMTC3MXYW1IzSvZmUwmkUYU7k3kfRSPmI/rijFPSdc4uZUrMMS10efVxG9yAWwYvkCFJ2Y…\n  data decrypted (555 bytes)\n→ GET https://p4-msg-chat.zaloapp.com/?zpw_ver=694&zpw_type=30&params=<redacted>&ts=1793714268\n← 200 OK {\"error_code\":0,\"error_message\":\"\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVaux4zQ/8TsNHmp9153vYMAxKsFiBoOxdp5eminGaWpYM0/f2IUdfNJxuPv7TY9GrfFYYEE9AGuk6IEuzpMXLnUdA9WMslZZRFnlH6yj+ugLB0vScBYNQMdawz8CRhcCkYuT8d4r5Z3SW9ogIhrfE8CYNI7AeavbFxd4LySF0fqFm/4dx+GcLtAI8D1+IG+rYJY6yBJvDzkNYa2l+gMDV0tJylH+wR/JtDlhsAzKZliOMZBAVTk3yEBosMTC3MXYW1IzSvZmUwmkUYU7k3kfRSPmI/rijFPSdc4uZUrMMS10efVxG9yAWwYvkCFJ2Y…\n  data decrypted (555 bytes)\n→ GET https://p4-msg-chat.zaloapp.com/?zpw_ver=694&zpw_type=30&params=<redacted>&ts=1793714269\n← 200 OK {\"error_code\":0,\"error_message\":\"\",\"data\":\"9Unl2sUDeHY0ZMbz9Eq7CVpNm/poMSOYOLOjvHIVaux4zQ/8TsNHmp9153vYMAxKsFiBoOxdp5eminGaWpYM0/f2IUdfNJxuPv7TY9GrfFYYEE9AGuk6IEuzpMXLnUdA9WMslZZRFnlH6yj+ugLB0vScBYNQMdawz8CRhcCkYuT8d4r5Z3SW9ogIhrfE8CYNI7AeavbFxd4LySF0fqFm/4dx+GcLtAI8D1+IG+rYJY6yBJvDzkNYa2l+gMDV0tJylH+wR/JtDlhsAzKZliOMZBAVTk3yEBosMTC3MXYW1IzSvZmUwmkUYU7k3kfRSPmI/rijFPSdc4uZUrMMS10efVxG9yAWwYvkCFJ2Y…\n  data decrypted (555 bytes)\n\n\n(loop liên tục)\n\n```\n",
      "closedAt": "2026-11-03T14:09:34Z",
      "createdAt": "2026-11-03T14:04:52Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 82,
      "state": "CLOSED",
      "title": "bug:"
    },
    {
      "author": {
        "id": "U_kgDOCp6XwQ",
        "is_bot": false,
        "login": "Saphira8585",
        "name": "Saphira8585"
      },
      "body": "> **Trước khi mở issue:** Vui lòng kiểm tra bảng **[Tính năng & giới hạn (`docs/TINH_NANG.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/TINH_NANG.md)** và **[Lỗi đã biết (`docs/DA_BIET.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/DA_BIET.md)** để xem vấn đề đã được ghi nhận chưa.\n\n## Môi trường\n- **Hệ điều hành:** Arch Linux (ví dụ: Windows 11 23H2 / macOS 15.2 Apple Silicon / Ubuntu 24.04)\n- **Phiên bản `ctdozlite`:v0.0.7 Native ** (xem góc trên bên trái cửa sổ ứng dụng hoặc chạy `ctdozlite --version`)\n- **Chế độ chạy:** Run the Appimage file\n\n## Các bước tái hiện\n1. Send messaages normaly\nand the other User must be offline \n\n## Kết quả thực tế vs. kỳ vọng\n- **Thực tế:** Show Đã nhận\n- **Kỳ vọng:** Sent only when it really Delivered show Delivered  (Đã nhận). The app in mobile still say as Sent\n\n## Log (`--dev`)\nChạy lại với `ctdozlite --gui --dev` và dán đoạn log liên quan tại đây (nhớ xoá thông tin nhạy cảm / cookie nếu có):\n```bash\n       ctdozlite --gui --dev\nzsh: command not found: ctdozlite\n```\n",
      "closedAt": "2026-11-11T18:30:18Z",
      "createdAt": "2026-11-03T12:11:58Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 81,
      "state": "CLOSED",
      "title": "bug:"
    },
    {
      "author": {
        "id": "U_kgDOEuocxQ",
        "is_bot": false,
        "login": "phamducit2005-bot",
        "name": ""
      },
      "body": "### Summary\n\ngóp ý\n\n### Proposal\n\nmình thấy nên thêm tính năng lưu acc chứ mỗi lần đăng nhập vào phải quét lại mã nó mất khá nhiều tin nhắn\n\n### Evidence\n\n- Evidence / capture:\n- Observed error_code:\n\n\n### Context\n\n- ADR / RFC: None\n- Alternatives considered:\n",
      "closedAt": "2026-11-11T18:30:22Z",
      "createdAt": "2026-11-03T11:25:46Z",
      "labels": [],
      "number": 80,
      "state": "CLOSED",
      "title": "mình thấy nên thêm tính năng lưu acc chứ mỗi lần đăng nhập vào phải quét lại mã nó mất khá nhiều tin nhắn"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjMxODg0NDg3",
        "is_bot": false,
        "login": "hoanglong149",
        "name": ""
      },
      "body": "### Summary\n\nNhằm nâng cao trải nghiệm nhắn tin hàng ngày trên `ctdozlite` tương đương hoặc vượt trội so với Zalo PC chính thức, nhóm tính năng tương tác hội thoại cốt lõi sau đã được bổ sung & hoàn thiện:\n\n1. **Chống xoá / xem lại tin nhắn đã bị thu hồi (Anti-Recall)**: Giữ lại toàn bộ văn bản và media/ảnh/sticker của tin nhắn thay vì xoá trắng thành `[Tin nhắn đã thu hồi]`.\n2. **Chuyển tiếp tin nhắn (Forward Message)**: Hỗ trợ chuyển tiếp mượt mà cả Text, Sticker và Hình ảnh (tự động nhận diện CDN Zalo để tải về và forward đính kèm).\n3. **Gửi ảnh gốc HD (Original Quality)**: Thêm nút chuyển đổi `[HD]` trên thanh soạn thảo, gọi endpoint `/api/message/photo_original/send` giữ nguyên 100% độ phân giải & dung lượng, không bị Zalo nén mờ.\n4. **Chụp ảnh màn hình (Screenshot) & Dán ảnh Clipboard**: Phím tắt `Alt+A` / `Cmd+Shift+A` hoặc icon Camera 📷 để kéo chọn vùng màn hình macOS, nạp trực tiếp vào composer xem trước trước khi gửi.\n5. **Hiển thị trạng thái gõ \"... đang soạn thảo\" & \"Đã xem\" (Typing & Seen Receipts)**: Bắt các gói tin điều khiển `typing` và `seen` từ WebSocket & Long-polling `lp_control`, hiển thị thanh thái thời gian thực và cập nhật trạng thái `Đã xem` của tin nhắn gửi đi.\n\n---\n\n### Proposal\n\n1. **Anti-Recall (Chống thu hồi tin nhắn)**:\n   - Trong `merge_thread_into` và `GuiEvent::MessageRecalled`: khi nhận tin nhắn thu hồi, gắn tiền tố `[Đã thu hồi]` và giữ nguyên `sticker_id`, `link_url`, `link_thumb`, ảnh đính kèm.\n   - Giao diện: Hiển thị badge màu đỏ `🚫 Đã thu hồi` phía trên bubble, vẫn cho phép nhấp phóng to xem ảnh và sao chép văn bản.\n2. **Forwarding Media**:\n   - Trong `forward_message_with_client_id`: phân loại tin nhắn, nếu có Zalo CDN photo URL thì tải xuống file tạm và gửi qua `send_attachment`. Lọc bỏ tiền tố `[Đã thu hồi]`.\n3. **Gửi ảnh Original HD**:\n   - Bổ sung `send_attachment_ext(..., is_original_hd)`. Khi bật HD, gửi params `isHD: 1, hd: 1, isOriginal: 1, oriUrl: ...`.\n   - Nút `[HD]` trên toolbar composer bật/tắt linh hoạt.\n4. **Chụp ảnh màn hình tương tác**:\n   - Gọi `screencapture -i -x` (macOS), convert sang texture RGBA cho composer preview.\n   - Bổ sung phím tắt `Alt+A` / `Cmd+Shift+A` và icon Camera trong thanh công cụ.\n5. **Typing Indicator & Seen Receipts**:\n   - Trích xuất `act_type: \"typing\"` và `\"seen\"` từ `lp_control` và WebSocket payload (trước đây bị lọc bỏ do chỉ xử lý `voip`/`mute`).\n   - Hiển thị `✍ <Tên> đang soạn thảo…` ở Sidebar, Thread Header và ngay trên khung nhập tin nhắn.\n   - Debounce gửi tín hiệu `WorkerCmd::SendTyping` khi người dùng nhập nội dung.\n   - Cập nhật `DeliveryState::Read` màu xanh `17:30 · Đã xem` khi đối phương đã đọc hoặc phản hồi.\n\n---\n\n### Evidence\n\n- Đã kiểm thử thực tế và toàn bộ 194 unit tests trong thư viện đều vượt qua: `194 passed; 0 failed`.\n- Đã build và chạy mượt mà trên macOS Native app (`ctdozlite.app`).\n\n---\n\n### Context\n\n- Affected modules: `src/chat.rs`, `src/call.rs`, `src/gui.rs`, `src/gui/worker.rs`, `src/gui/workspace.rs`, `src/gui/messages.rs`, `src/gui/theme.rs`, `src/gui/modals.rs`.\n- Tham khảo mã nguồn & full diff:\n  - Nhánh: https://github.com/hoanglong149/ctdozlite/tree/v0.0.7-local\n  - Full Compare: https://github.com/ctdoteam1110/ctdozlite/compare/main...hoanglong149:ctdozlite:v0.0.7-local\n",
      "closedAt": "2026-11-06T16:03:05Z",
      "createdAt": "2026-11-03T10:32:29Z",
      "labels": [],
      "number": 79,
      "state": "CLOSED",
      "title": "feat: chống thu hồi tin nhắn, chuyển tiếp media, gửi ảnh gốc HD, chụp màn hình và hiển thị đang soạn thảo / đã xem"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjMxODg0NDg3",
        "is_bot": false,
        "login": "hoanglong149",
        "name": ""
      },
      "body": "### Summary\n\nAvatar nhóm (cả avatar tổng hợp `zavatar-multi` và ảnh đại diện nhóm tùy chỉnh) hiện đang gặp các vấn đề nghiêm trọng về mặt hình học và tỷ lệ hiển thị (aspect ratio):\n\n1. **Avatar nhóm tổng hợp (`zavatar-multi` / `multi:`) bị phình to và lồi 4 góc ra ngoài**:\n   - Trong `src/gui/theme.rs`, toạ độ của các avatar con 4 góc được tính bằng `center ± (radius - sub_r)`.\n   - Khoảng cách từ tâm đến các góc là $d = \\sqrt{2} \\cdot (R - r)$.\n   - Với $r = 0.52 R$, điểm xa nhất của avatar con đạt tới $d + r = \\sqrt{2}(0.48 R) + 0.52 R \\approx 1.20 R$.\n   - **Hậu quả**: Bốn góc của avatar nhóm bị lồi ra ngoài đường tròn tới 20%, tạo thành hình chữ thập / hoa mai 4 góc méo mó, phình to bất thường so với avatar người dùng cá nhân (vốn có bán kính chuẩn $R$).\n   - Ngoài ra, không có vòng tròn nền chuẩn bán kính $R$ ở dưới, khiến khoảng trống giữa các avatar con bị rỗng/thủng nền.\n\n2. **Ảnh nhóm tùy chỉnh (hoặc ảnh người dùng không vuông) bị kéo dẹt (méo aspect ratio) & lòi 4 góc nhọn**:\n   - `paint_single_circle_avatar` hiện đang dùng `painter.image(...)` với UV cố định `[0, 1]` vào khung vuông `Rect::from_center_size(center, Vec2::splat(radius * 2.0))`.\n   - Nếu ảnh gốc tải lên không phải tỉ lệ 1:1 vuông (ảnh ngang/dọc từ CDN), ảnh bị ép dẹt méo mó.\n   - `painter.image` trong egui vẽ một mesh chữ nhật 4 góc vuông chứ không được clip tròn, khiến các góc ảnh bị thò ra ngoài viền tròn.\n\n3. **Nhóm không có avatar thành viên bị hiển thị 3 chữ cái li ti**:\n   - Khi không có ảnh thành viên, logic fallback hiện tại tách tên nhóm thành 3 chữ cái rời rạc (hoặc thêm \"TV\", \"N\") và vẽ vào 3 góc, nhìn rất rối mắt và không chuẩn UX.\n\n### Proposal & Solution\n\nĐã có implementation hoàn chỉnh và kiểm thử thành công tại commit [82f906b](https://github.com/hoanglong149/ctdozlite/commit/82f906b83f46f345c26b5278c2e6f4a5ec40ee79) trên nhánh `v0.0.7-local`:\n\n1. **Vẽ ảnh tròn chuẩn bằng Circle Mesh & UV Center-Crop**:\n   - Tạo hàm `paint_circular_image` dùng `egui::Mesh` hình tròn (32 đỉnh) cắt tròn mượt mà.\n   - Tự động tính toán UV center-crop (cover) dựa trên tỷ lệ thực `tw` và `th` của texture, đảm bảo ảnh luôn giữ đúng tỷ lệ 1:1 chuẩn xác, không bao giờ bị méo dẹt hay lòi góc vuông.\n2. **Căn chỉnh toạ độ hình học avatar nhóm composite**:\n   - Vẽ vòng tròn nền lớn bán kính $R$ màu `border_col` bên dưới.\n   - Tính toán lại bán kính và toạ độ các avatar con để đảm bảo điểm xa nhất $d + r \\le R$:\n     - 2 avatar: `sub_r = 0.46 R`, `delta = 0.25 R`.\n     - 3 avatar: `sub_r = 0.42 R`, `top` tại $0.40 R$, 2 đáy tại $(\\pm 0.38 R, +0.34 R)$.\n     - 4 avatar: `sub_r = 0.38 R`, `delta = 0.38 R` (kèm ô đếm số `+N` nếu $> 4$).\n     - Tất cả các avatar con đều nằm lọt 100% bên trong vòng tròn bán kính $R$.\n3. **Fallback nhóm đơn**:\n   - Nếu nhóm không có ảnh thành viên nào, hiển thị một avatar tròn đơn với màu sắc thương hiệu nhóm và chữ cái đầu to rõ ràng, đồng nhất với Zalo Mobile / Web.\n\n### Context\n- File: `src/gui/theme.rs`\n- Nhánh demo / source: https://github.com/hoanglong149/ctdozlite/tree/v0.0.7-local\n",
      "closedAt": "2026-11-06T16:03:03Z",
      "createdAt": "2026-11-03T07:24:36Z",
      "labels": [],
      "number": 77,
      "state": "CLOSED",
      "title": "bug(gui): avatar nhóm bị sai tỷ lệ hình học (phình to lòi góc) và méo aspect ratio"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjMxODg0NDg3",
        "is_bot": false,
        "login": "hoanglong149",
        "name": ""
      },
      "body": "### Summary\n\nTrải nghiệm trò chuyện nhóm (Group Chat) trong ctdozlite hiện còn thiếu một số thành phần quan trọng so with Zalo PC chuẩn:\n\n1. **Avatar người gửi trong nhóm chat**: Các tin nhắn đến từ thành viên nhóm hiện chỉ hiển thị chữ cái đầu trên nền màu (fallback), chưa hiển thị ảnh đại diện thật của từng người gửi (do `avatar_by_name` hiện tại chỉ lấy từ danh bạ chat 1-1 `threads.filter(|th| !th.is_group)`, chưa lấy từ danh sách thành viên của nhóm).\n2. **Không xem được danh sách thành viên nhóm**: Khi ở trong nhóm chat, Info Drawer bên phải và Header chưa có mục xem danh sách thành viên nhóm, khiến người dùng không nắm được nhóm gồm những ai, ai là Trưởng/Phó nhóm và không thể bấm vào để nhắn tin riêng hoặc xem trang cá nhân.\n3. **Mention thành viên (@All, @Tên)**: Các đoạn mention như `@All`, `@Tất cả` hoặc `@<Tên người dùng>` đang hiển thị ở dạng plaintext thô, chưa được render thành link/chip nổi bật màu xanh Zalo (`#0068FF` / `#61AFEF`) như trên Zalo PC.\n\n### Proposal\n\nĐề xuất bổ sung & hoàn thiện giao diện nhóm chat:\n\n1. **Fetch thông tin thành viên & hiển thị avatar thật trong bubble chat**:\n   - Khi vào nhóm chat, worker gọi `fetch_group_profiles` (`POST /api/group/getmg-v2`) và `get_mini_profiles` (`POST /api/social/group/members`) để lấy danh sách thành viên nhóm kèm `displayName`, `avatar` URL và vai trò `creatorId` / `adminIds`.\n   - Ánh xạ `sender_id` (hoặc UID) của bubble tin nhắn tới avatar URL thật của thành viên nhóm và tải ảnh đại diện thật.\n2. **Bổ sung mục Danh sách thành viên nhóm trong Info Drawer & Header**:\n   - Thêm khối **Thành viên nhóm (X)** trong Info Drawer: hiển thị avatar tròn 26-28px, tên thành viên, badge \"Trưởng nhóm\" / \"Phó nhóm\", ô tìm kiếm nhanh thành viên, nút nhắn tin 1-1 trực tiếp và xem profile.\n   - Cập nhật subtitle trên Header nhóm thành `👥 X thành viên` (cho phép click để mở nhanh danh sách thành viên).\n3. **Highlight @Mention trong tin nhắn**:\n   - Phân tích cú pháp `@All`, `@Tất cả`, `@<Tên>` trong `render_markdown_inline_rich` để hiển thị chip nổi bật màu xanh Zalo bo góc nhẹ thay vì plaintext thô.\n\n### Evidence\n\n- Quan sát thực tế khi dùng nhóm chat trên ctdozlite Native: tin nhắn nhóm chỉ có avatar chữ cái, không biết danh sách thành viên nhóm, tag tên hiển thị text thông thường.\n- Đã test giải pháp gọi API nhóm `getmg-v2` và `get_mini_profiles` hoạt động ổn định và chính xác.\n\n### Context\n\n- Affected modules: `src/chat.rs`, `src/gui/worker.rs`, `src/gui/workspace.rs`, `src/gui/messages.rs`.\n- Alternatives considered: None\n",
      "closedAt": "2026-11-06T16:03:02Z",
      "createdAt": "2026-11-03T07:11:36Z",
      "labels": [],
      "number": 76,
      "state": "CLOSED",
      "title": "feat(group): hiển thị avatar người gửi, danh sách thành viên nhóm và highlight @mention"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjMxODg0NDg3",
        "is_bot": false,
        "login": "hoanglong149",
        "name": ""
      },
      "body": "## Môi trường\n\n• Phiên bản: `v0.0.6` & `v0.0.7`\n• Hệ điều hành: macOS / Linux / Windows\n• Tính năng: Gửi tập tin đính kèm (File / Document: `.pdf`, `.docx`, `.xlsx`, `.zip`, `.txt`...) & Video trong hội thoại cá nhân và nhóm.\n\n---\n\n## Mô tả lỗi & Các bước tái hiện\n\n1. Mở bất kỳ hội thoại nào (cá nhân, nhóm hoặc Cloud của tôi).\n2. Bấm vào biểu tượng kẹp ghim (Gửi File / Video) và chọn một tập tin tài liệu (ví dụ: `file_test.pdf` hoặc `.docx`).\n3. **Thực tế:** Ứng dụng báo lỗi đỏ:\n   `Lỗi gửi 'file_test.pdf': HTTP 405 Method Not Allowed from https://tt-files-wpa.zaloapp.com/api/message/sharefile_full`\n   và tập tin không được gửi đi.\n4. **Kỳ vọng:** Tập tin được tải lên máy chủ Zalo và tin nhắn đính kèm xuất hiện trong khung chat với đầy đủ tên file, dung lượng và liên kết tải về.\n\n---\n\n## Phân tích nguyên nhân gốc (Root Cause)\n\nKhi phân tích gói tin giao thức thực tế của Zalo PC/Web:\n\n1. **Giai đoạn 1 (`/api/message/asyncfile/upload`):**\n   - App upload các chunk thành công và nhận được phản hồi chứa `fileId` (ví dụ `{\"chunkId\": 1, \"clientFileId\": 179367..., \"fileId\": \"453216...\", \"finished\": 1}`).\n   - **Tuy nhiên, máy chủ Zalo không trả về link tải file (`fileUrl`) trong HTTP response của endpoint này.**\n\n2. **Giai đoạn 2 (`/api/message/asyncfile/msg`):**\n   - Zalo xử lý lưu trữ tệp ở backend bất đồng bộ. Sau khi nhận đủ chunk, máy chủ phát một thông báo thời gian thực qua kênh **WebSocket** (`cmd = 601`, `subCmd = 0`, `act_type = \"file_done\"`), trong đó mang theo `data.url` (link CDN chính thức, ví dụ: `https://f39-zfcloud.zdn.vn/...`).\n   - Sau đó khi gọi `/api/message/asyncfile/msg`, Zalo **bắt buộc phải nhận trường `\"fileUrl\"`** mang link CDN này. Do app cũ lấy `fileUrl` từ HTTP response của bước 1 (bị rỗng), máy chủ Zalo từ chối với lỗi:\n     `error_code: 114: \"Tham số không hợp lệ\"`\n   - Ngoài ra, payload của Zalo quy định tên trường phần mở rộng là `\"extention\"` (lỗi chính tả lịch sử của Zalo API) và yêu cầu các trường: `\"fdata\": \"{}\"`, `\"checksumSha\": \"\"`, `\"fileCount\": 0`, `\"zsource\": -1`, `\"ttl\": 0`.\n\n3. **Giai đoạn 3 (Fallback cũ bị HTTP 405):**\n   - Do bước 2 thất bại với mã 114, code rơi vào nhánh fallback gọi multipart lên `/api/message/sharefile_full` (hoặc `/api/group/sharefile_full`).\n   - Endpoint `sharefile_full` này là API cũ đã bị Zalo khai tử hoàn toàn (trả về `HTTP 405 Method Not Allowed`), dẫn đến việc gửi file hoàn toàn tê liệt.\n\n---\n\n## Giải pháp đề xuất (Đã kiểm chứng & chạy thử thành công 100%)\n\nTrong hàm `send_attachment` (`src/chat.rs`):\n\n1. **Khởi tạo bộ lắng nghe WebSocket** trước khi upload chunk:\n   ```rust\n   let mut ws_rx = crate::socket::spawn(s.clone());\n   ```\n2. **Sau khi upload chunk hoàn tất, chờ sự kiện `file_done`** qua WebSocket để lấy `fileUrl`:\n   ```rust\n   let mut file_url = pick_str(data_node, &[\"fileUrl\", \"url\", \"normalUrl\"]).unwrap_or_default();\n   if file_url.is_empty() {\n       if let Some(ref fid) = file_id {\n           let deadline = tokio::time::Instant::now() + tokio::time::Duration::from_secs(6);\n           while tokio::time::Instant::now() < deadline {\n               if let Ok(Some(ev)) = tokio::time::timeout(tokio::time::Duration::from_millis(500), ws_rx.recv()).await {\n                   let ev_data = ev.payload.get(\"data\").unwrap_or(&ev.payload);\n                   if let Some(controls) = ev_data.get(\"controls\").and_then(|c| c.as_array()) {\n                       for c in controls {\n                           if c.get(\"content\").and_then(|cnt| cnt.get(\"act_type\")).and_then(|a| a.as_str()) == Some(\"file_done\") {\n                               let c_fid = c.get(\"content\").and_then(|cnt| cnt.get(\"fileId\")).map(|v| v.to_string()).unwrap_or_default();\n                               let fid_clean = c_fid.trim_matches('\"');\n                               if fid_clean == fid || fid_clean == fid.as_str() || c_fid.is_empty() {\n                                   if let Some(u) = c.get(\"content\").and_then(|cnt| cnt.get(\"data\")).and_then(|d| d.get(\"url\")).and_then(|u| u.as_str()) {\n                                       file_url = u.to_string();\n                                       break;\n                                   }\n                               }\n                           }\n                       }\n                   }\n                   if !file_url.is_empty() {\n                       break;\n                   }\n               }\n           }\n       }\n   }\n   ```\n3. **Đóng gói đầy đủ payload chuẩn Zalo** cho `/api/message/asyncfile/msg` (và `/api/group/asyncfile/msg`):\n   ```rust\n   let mut msg_obj = serde_json::Map::new();\n   msg_obj.insert(\"fileId\".into(), json!(fid));\n   msg_obj.insert(\"checksum\".into(), json!(checksum));\n   msg_obj.insert(\"checksumSha\".into(), json!(\"\"));\n   msg_obj.insert(\"extention\".into(), json!(ext));\n   msg_obj.insert(\"extension\".into(), json!(ext));\n   msg_obj.insert(\"totalSize\".into(), json!(total_size));\n   msg_obj.insert(\"fileName\".into(), json!(file_name));\n   msg_obj.insert(\"clientId\".into(), client_file_id);\n   msg_obj.insert(\"fType\".into(), json!(if is_video { 2 } else { 1 }));\n   msg_obj.insert(\"fileCount\".into(), json!(0));\n   msg_obj.insert(\"fdata\".into(), json!(\"{}\"));\n   if is_group {\n       msg_obj.insert(\"grid\".into(), json!(clean_target));\n   } else {\n       msg_obj.insert(\"toid\".into(), json!(clean_target));\n   }\n   msg_obj.insert(\"fileUrl\".into(), json!(file_url));\n   msg_obj.insert(\"zsource\".into(), json!(-1));\n   msg_obj.insert(\"ttl\".into(), json!(0));\n   ```\n4. **Loại bỏ nhánh fallback `sharefile_full`** do máy chủ đã vô hiệu hoá endpoint này.\n\nĐã kiểm chứng thực tế: gửi thành công các loại file tài liệu (`.pdf`, `.docx`, `.zip`, `.txt`...) tới cả tài khoản cá nhân, Cloud của tôi và Nhóm chat, Zalo trả về `error_code: 0` và tin nhắn hiển thị chuẩn xác.\n",
      "closedAt": "2026-11-04T15:31:40Z",
      "createdAt": "2026-11-03T06:25:33Z",
      "labels": [],
      "number": 75,
      "state": "CLOSED",
      "title": "bug(chat): gửi tập tin đính kèm (PDF, DOCX, ZIP...) thất bại với HTTP 405 do asyncfile/msg thiếu fileUrl từ WebSocket file_done"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjMwMTcxMjM3",
        "is_bot": false,
        "login": "amm119",
        "name": "Anbui"
      },
      "body": "> **Trước khi mở issue:** Vui lòng kiểm tra bảng **[Tính năng & giới hạn (`docs/TINH_NANG.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/TINH_NANG.md)** và **[Lỗi đã biết (`docs/DA_BIET.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/DA_BIET.md)** để xem vấn đề đã được ghi nhận chưa.\n\n## Môi trường\n- **Hệ điều hành:** macOS 26.5.2 Apple Silicon\n- **Phiên bản `ctdozlite`:** 0.0.6\n- **Chế độ chạy:** GUI\n\n## Các bước tái hiện\nNhận file từ người dùng khác lưu về (tôi gặp vấn đề với file .doc\n\n## Kết quả thực tế vs. kỳ vọng\n- **Thực tế:** renewMedia.doc\n- **Kỳ vọng:** tên theo file người gửi\n\n## Log (`--dev`)\nChạy lại với `ctdozlite --gui --dev` và dán đoạn log liên quan tại đây (nhớ xoá thông tin nhạy cảm / cookie nếu có):\n```text\n\n```\n",
      "closedAt": "2026-11-03T05:45:23Z",
      "createdAt": "2026-11-03T03:47:23Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 74,
      "state": "CLOSED",
      "title": "bug: Lưu file file bị đổi tên"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjc2NDgwNzYz",
        "is_bot": false,
        "login": "nguyenphuong10102003",
        "name": ""
      },
      "body": "Tôi đăng nhập nhưng tin nhắn không có đồng bộ lên app và app ko gửi lệnh đồng bộ lên app zalo trên đt",
      "closedAt": "2026-11-03T05:45:21Z",
      "createdAt": "2026-11-03T03:34:58Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 73,
      "state": "CLOSED",
      "title": "bug: Lỗi đồng bộ tin nhắn"
    },
    {
      "author": {
        "id": "U_kgDOBtD1oQ",
        "is_bot": false,
        "login": "slothteabar",
        "name": ""
      },
      "body": "### Summary\n\nTrên bản Windows 11 (Native v0.0.6), tin nhắn hình ảnh trong khung chat hiện tại đang hiển thị giống dạng thẻ link preview hơn là tin nhắn media chuẩn, gây bất tiện khi theo dõi:\n1. Kích thước ảnh thu nhỏ (thumbnail) quá bé.\n2. Hiển thị thừa phần text đường dẫn CDN (`photo-stal-...zdn.vn/...`) và tên miền ở ngay dưới bức ảnh, gây rối mắt.\n3. Khi click vào ảnh thì bị đẩy ra trình duyệt web bên ngoài thay vì xem trực tiếp được trong ứng dụng.\n\n<img width=\"294\" height=\"239\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/73a7f8aa-6dca-40de-9f3c-d6a2be17b24a\" />\n\n<img width=\"275\" height=\"520\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/ac5f3442-a515-4a9e-94cf-865d61788fff\" />\n\n### Proposal\n\nĐề xuất tinh chỉnh lại UI/UX cho phần hiển thị tin nhắn hình ảnh\n\n### Evidence\n\n- OS: Windows 11\n- App version: ctdozlite Native v0.0.6\n\n### Context\n\n- ADR / RFC: None\n- Alternatives considered:\n",
      "closedAt": "2026-11-03T05:45:19Z",
      "createdAt": "2026-11-03T03:25:34Z",
      "labels": [],
      "number": 72,
      "state": "CLOSED",
      "title": "feat(gui): cải thiện hiển thị ảnh trong chat (tăng kích thước, ẩn raw link, thêm trình xem ảnh in-app)"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjMxODg0NDg3",
        "is_bot": false,
        "login": "hoanglong149",
        "name": ""
      },
      "body": "## Môi trường\n- **Hệ điều hành:** macOS (Apple Silicon M-series & Intel, macOS 14 Sonoma / macOS 15 Sequoia)\n- **Bộ gõ:** Bộ gõ tiếng Việt mặc định của macOS (Vietnamese Simple Telex / Telex)\n- **Phiên bản `ctdozlite`:** v0.0.6\n- **Chế độ chạy:** GUI (`--gui`)\n\n## Mô tả lỗi & Các bước tái hiện\n1. Mở `ctdozlite` trên macOS và chọn bộ gõ tiếng Việt mặc định của hệ điều hành.\n2. Tại ô nhập tin nhắn (hoặc ô tìm kiếm), gõ một câu tiếng Việt bình thường (ví dụ: `xin chào các bạn`).\n3. **Thực tế:** Các từ bị dính liền vào nhau thành `xinchàocácbạn` (hoặc phải bấm phím Space 2 lần liên tiếp thì mới có được 1 dấu cách).\n4. **Kỳ vọng:** Khi gõ xong một từ và ấn phím Space, từ được chốt dấu và có đúng 1 khoảng trắng theo sau (`xin chào các bạn`).\n\n## Phân tích nguyên nhân gốc (Root Cause)\nKhi gõ tiếng Việt trên macOS qua bộ gõ mặc định, mỗi từ được soạn trong trạng thái Marked Text (preedit). Khi người dùng ấn phím Spacebar để hoàn thành từ:\n1. **`CGEventSourceKeyState(0, 49)`**: Khi người dùng gõ phím ở tốc độ tự nhiên, thời gian ấn-nhả phím cách chỉ khoảng 30–60ms. Tại thời điểm egui gọi frame `update()`, ngón tay đã nhấc khỏi phím cách nên `CGEventSourceKeyState` trả về `false`.\n2. **`space_in_events`**: Trên macOS, upstream `winit` chủ động chặn (suppress) sự kiện `KeyboardInput` (KeyDown) khi IME thực hiện `insertText` để commit từ. Do đó `space_in_events` trong egui cũng luôn là `false`.\n3. Vì cả hai đều là `false`, điều kiện kiểm tra phím cách không bao giờ được thoả mãn khi gõ nhanh → `text.push(' ')` không chạy → gây ra hiện tượng nuốt dấu cách (\"dính dấu cách\").\n\n## Giải pháp đề xuất (Đã kiểm chứng)\nTrong `src/gui.rs`, khi nhận `ImeEvent::Commit`:\n- Nếu trước đó đang trong trạng thái gõ dở (`was_composing = true`) và sự kiện Commit diễn ra mà:\n  - **Không có phím Enter** (`!enter_pressed`)\n  - **Không có ký tự văn bản/dấu câu theo sau** (`!non_space_text`)\n  - **Không có click chuột** (`!mouse_clicked`)\n- Thì trên macOS với bộ gõ Telex, 100% hành vi này là do người dùng bấm phím **Spacebar** để kết thúc từ.\n- Chỉ cần xác nhận `space_committed` dựa trên điều kiện này và tự động thêm `' '` + debounce sự kiện Space trùng lặp trong frame.\n- Ngoài ra trong `src/gui/workspace.rs`, kiểm tra `!is_ime_busy` trước khi kích hoạt `enter_pressed` gửi tin nhắn để tránh việc ấn Enter chốt từ IME bị gửi tin đi sớm.\n\nĐã test thực tế trên macOS: gõ văn bản tiếng Việt mượt mà, không dính chữ và không bị nhảy thừa 2 dấu cách.\n",
      "closedAt": "2026-11-03T05:45:16Z",
      "createdAt": "2026-11-03T02:49:51Z",
      "labels": [],
      "number": 71,
      "state": "CLOSED",
      "title": "bug(gui): lỗi nuốt dấu cách / dính chữ khi gõ bộ gõ tiếng Việt Telex mặc định trên macOS"
    },
    {
      "author": {
        "id": "U_kgDOCW-5TA",
        "is_bot": false,
        "login": "Zqw1k",
        "name": "Zuqikee"
      },
      "body": "### Summary\n\nhiện tại project chỉ đồng bộ từ cloud, không có chức năng sync từ điện thoại như web/pc\n\n### Proposal\n\nthêm tính năng đồng bộ từ điện thoại\n\n### Evidence\n\n\n\n\n### Context\n\n- ADR / RFC: None\n- Alternatives considered:\n",
      "closedAt": "2026-11-02T14:06:16Z",
      "createdAt": "2026-11-02T09:31:12Z",
      "labels": [],
      "number": 68,
      "state": "CLOSED",
      "title": "feat: thêm tính năng đồng bộ tin nhắn từ điện thoại"
    },
    {
      "author": {
        "id": "U_kgDOET8fvw",
        "is_bot": false,
        "login": "Harry-Huy",
        "name": ""
      },
      "body": "### Summary\n\nActual: Notification mute/unmute state is not synchronized between ctdozlite and official Zalo PC.\n\nExpected: Both clients should reflect the same notification mute/unmute state for the same conversation.\n\nThe affected direction(s), timing, and behavior after restarting have not been confirmed.\n\n### Reproduction\n\n1. Sign in to the same Zalo account in ctdozlite and official Zalo PC.\n2. Change a conversation's notification setting (mute/unmute) in one client.\n3. Check the same conversation's notification state in the other client.\n\nReported result: the states do not synchronize. These are suggested verification steps; the exact original sequence and whether both directions are affected are unconfirmed.\n\n### Area\n\nchat\n\n### Server error_code (if any)\n\n_No response_\n\n### Environment\n\n- OS: Windows 11\n- ctdozlite: v0.0.1 (reported; exact build/tag not independently verified)\n- Official Zalo PC version: not provided\n- Interface: not confirmed\n- Logs/screenshots: not provided\n\n### Secret check\n\n- [x] I confirm no session tokens, cookies, IMEI, phone numbers or raw captures are included.",
      "closedAt": "2026-11-02T14:06:13Z",
      "createdAt": "2026-11-02T06:48:59Z",
      "labels": [],
      "number": 67,
      "state": "CLOSED",
      "title": "fix(notifications): mute/unmute state does not sync with official Zalo PC"
    },
    {
      "author": {
        "id": "U_kgDOD9scpQ",
        "is_bot": false,
        "login": "dangvusns",
        "name": ""
      },
      "body": "> **Trước khi mở issue:** Vui lòng kiểm tra bảng **[Tính năng & giới hạn (`docs/TINH_NANG.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/TINH_NANG.md)** và **[Lỗi đã biết (`docs/DA_BIET.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/DA_BIET.md)** để xem vấn đề đã được ghi nhận chưa.\n\n## Môi trường\n- **Hệ điều hành:** Windows 11 23H2\n- **Phiên bản `ctdozlite`:** Native 0.0.5\n- **Chế độ chạy:** GUI trên Windows\n\nHiện tại mình không gõ tiếng Việt được, mình đang sử dụng Unikey\n",
      "closedAt": "2026-11-02T06:44:58Z",
      "createdAt": "2026-11-02T06:31:52Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 66,
      "state": "CLOSED",
      "title": "bug:"
    },
    {
      "author": {
        "id": "U_kgDOE2ErUA",
        "is_bot": false,
        "login": "lhbkhanh801-a11y",
        "name": ""
      },
      "body": "### Summary\n\nMong team thêm proxy vào để sử dụng được nhiều Zalo trên PC\n\n### Proposal\n\nMong team thêm proxy vào để sử dụng được nhiều Zalo trên PC\n\n### Evidence\n\n- Evidence / capture:\n- Observed error_code:\n\n\n### Context\n\n- ADR / RFC: None\n- Alternatives considered:\n",
      "closedAt": "2026-11-02T14:06:18Z",
      "createdAt": "2026-11-02T05:10:10Z",
      "labels": [],
      "number": 64,
      "state": "CLOSED",
      "title": "feat(<scope>): <summary>Mong team thêm proxy vào để sử dụng được nhiều Zalo trên PC"
    },
    {
      "author": {
        "id": "U_kgDOCKk8dg",
        "is_bot": false,
        "login": "buildertv",
        "name": "BuilderTV"
      },
      "body": "> **Trước khi mở issue:** Vui lòng kiểm tra bảng **[Tính năng & giới hạn (`docs/TINH_NANG.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/TINH_NANG.md)** và **[Lỗi đã biết (`docs/DA_BIET.md`)](https://github.com/ctdoteam1110/ctdozlite/blob/main/docs/DA_BIET.md)** để xem vấn đề đã được ghi nhận chưa.\n\n## Môi trường\n- **Hệ điều hành:** Windows 11 25H2\n- **Phiên bản `ctdozlite`:** 0.0.4\n- **Chế độ chạy:** GUI\n\n## Các bước tái hiện\n1. ấn vào tên 1 người vừa nhận tin nhắn\n2. tự động nhảy ra 1 cửa số chat của người khác trùng tên\n\n```\n\n\n",
      "closedAt": "2026-11-02T03:06:48Z",
      "createdAt": "2026-11-02T02:19:49Z",
      "labels": [
        {
          "id": "LA_kwDOVRw7ac8AAAAC-X74yg",
          "name": "bug",
          "description": "Something isn't working",
          "color": "d73a4a"
        }
      ],
      "number": 63,
      "state": "CLOSED",
      "title": "bug: lỗi khi tên zalo trùng nhau"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjgwMTUzMjk3",
        "is_bot": false,
        "login": "thedtvn",
        "name": "The DT"
      },
      "body": "### Summary\n\nRight after downloading the Windows setup installer from the official v0.0.3 GitHub release, Microsoft Defender (Windows Security) blocked and removed it as a **Severe Trojan**. The installer never ran.\n\n**Defender alert details**\n\n| Field | Value |\n|---|---|\n| Alert level | Severe |\n| Status | Removed |\n| Date | 02/11/2026 00:51 (UTC+7) |\n| Category | Trojan |\n| Details | \"This program is dangerous and executes commands from an attacker.\" |\n| Affected file | `C:\\Users\\<user>\\Downloads\\ctdozlite-0.0.3-setup-windows-x86_64.exe` |\n| Webfile | `https://release-assets.githubusercontent.com/github-production-release-asset/1427913577/...` (signed URL removed) |\n\n**Expected:** The official installer downloads and runs without being flagged.\n**Actual:** Defender quarantines/removes the installer, so the app can't be installed on Windows.\n\n<img width=\"683\" height=\"847\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/59541ac3-2eb2-4c9f-b253-c42d0e9054bf\" />\n\n### Reproduction\n\n1. Open https://github.com/ctdoteam1110/ctdozlite/releases/tag/v0.0.3\n2. Download `ctdozlite-0.0.3-setup-windows-x86_64.exe` on Windows 11 with Microsoft Defender real-time protection enabled\n3. Defender detects the file as a Trojan as soon as the download completes and removes it\n\n\n### Area\n\nother\n\n### Server error_code (if any)\n\n_No response_\n\n### Environment\n\n- OS / Version: Win 11\n\n### Secret check\n\n- [x] I confirm no session tokens, cookies, IMEI, phone numbers or raw captures are included.",
      "closedAt": "2026-11-01T19:07:22Z",
      "createdAt": "2026-11-01T17:57:59Z",
      "labels": [],
      "number": 58,
      "state": "CLOSED",
      "title": "bug(windows): Microsoft Defender flags ctdozlite-0.0.3-setup-windows-x86_64.exe as Trojan (Severe) and removes it"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjQ1NDc1MDg=",
        "is_bot": false,
        "login": "heroes1412",
        "name": ""
      },
      "body": "### Summary\n\n<img width=\"633\" height=\"235\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/4bff49cd-0448-41ea-ba76-7a9090e826e8\" />\nmình là ng dùng bình thường, k phải marketing hay gì mà nhiều group nhưng v0.0.2 thấy chiếm 400mb ram, 0.0.3 chiếm 2gb luôn\n\n### Reproduction\n\n1.\n2.\n3.\n\n\n### Area\n\nauth\n\n### Server error_code (if any)\n\n_No response_\n\n### Environment\n\n- OS / Version:\n- ctdozlite version:\n- Interface: GUI / TUI\n- Logs (redacted):\n\n\n### Secret check\n\n- [x] I confirm no session tokens, cookies, IMEI, phone numbers or raw captures are included.",
      "closedAt": "2026-11-01T15:46:32Z",
      "createdAt": "2026-11-01T15:42:01Z",
      "labels": [],
      "number": 56,
      "state": "CLOSED",
      "title": "app chiếm quá nhiều ram trong windows server 2019"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjg2MzIzNDI2",
        "is_bot": false,
        "login": "vuduc810",
        "name": ""
      },
      "body": "### Summary\n\nỨng dụng hiện tại chưa hỗ trợ lập lịch gửi tin nhắn vào một thời điểm cụ thể trong tương lai. \n\nTính năng này giúp người dùng chủ động gửi tin nhắn công việc, đặt lịch nhắc nhở hoặc nhắn tin ngoài giờ làm việc mà không làm phiền người nhận ngay lập tức (tương tự như trải nghiệm hẹn giờ gửi trên app tin nhắn của Google trên Android).\n\n### Proposal\n\nThêm tính năng \"Hẹn giờ gửi tin nhắn\" (Schedule Send) tương tự app tin nhắn của Google trên Android:\n\n- Luồng thao tác: Nhấn giữ nút \"Gửi\" (hoặc qua một icon hẹn giờ cạnh khung nhập liệu) để mở menu chọn thời gian.\n- Tùy chọn thời gian: Gợi ý sẵn các mốc nhanh (ví dụ: *Sáng mai 08:00*, *Chiều nay 18:00*) hoặc cho phép chọn ngày & giờ tùy chỉnh.\n- Quản lý tin nhắn chờ: \n  - Hiển thị tin nhắn đã hẹn giờ trong khung chat dưới dạng trạng thái chờ gửi (pending/scheduled UI).\n  - Cho phép người dùng chỉnh sửa nội dung, thay đổi thời gian gửi hoặc hủy/xóa tin nhắn đã hẹn giờ trước khi đến mốc phát tin.\n\n### Evidence\n\n- Evidence / capture: Tham khảo tính năng Hẹn giờ gửi (Schedule send) trên ứng dụng tin nhắn mặc định của Google trên Android (nhấn giữ nút gửi để chọn mốc thời gian phát tin).\n- Observed error_code: N/A (Feature request)\n\n\n### Context\n\n- ADR / RFC: None\n- Alternatives considered: Đặt nhắc nhở thủ công hoặc báo thức để gửi tin nhắn sau (khá bất tiện và dễ quên).",
      "closedAt": "2026-11-01T19:07:24Z",
      "createdAt": "2026-11-01T11:16:06Z",
      "labels": [],
      "number": 38,
      "state": "CLOSED",
      "title": "feat(<scope>): <summary> Hẹn giờ gửi tin nhắn (Schedule message)"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjM1MjQ5MTA1",
        "is_bot": false,
        "login": "congnvp",
        "name": ""
      },
      "body": "Mình đang rất kỳ vọng dự án này có thể phát triển mở các tính năng tích hợp công cụ thứ ba vào để quản lý tin nhắn, trả lời tự động... Và có phương thức trả dữ liệu/đồng bộ dữ liệu cho công cụ thứ ba để dễ quản lý và lưu trữ. ",
      "closedAt": "2026-11-01T19:07:26Z",
      "createdAt": "2026-11-01T10:48:21Z",
      "labels": [],
      "number": 35,
      "state": "CLOSED",
      "title": "Bổ sung REST API, MCP"
    },
    {
      "author": {
        "id": "U_kgDOBs78nQ",
        "is_bot": false,
        "login": "OngAnBaNgan",
        "name": "Ông An Bà Ngân"
      },
      "body": "Chào tác giả và đội ngũ phát triển,\n\nTrước tiên, cảm ơn các bạn vì đã tạo ra một dự án mã nguồn mở rất thú vị và chất lượng bằng Rust. Tuy nhiên, với tư cách là một người quan tâm đến sự phát triển lâu dài của dự án, mình muốn đóng góp một vài ý kiến quan trọng liên quan đến **rủi ro pháp lý** từ phía VNG/Zalo. \n\nCác công ty công nghệ lớn như VNG thường có chính sách bảo vệ thương hiệu và Điều khoản sử dụng (ToS) rất khắt khe đối với các ứng dụng bên thứ ba (đặc biệt là hành vi reverse engineering API). Để tránh việc kho lưu trữ (repo) này bị report đánh gậy bản quyền (DMCA Takedown) hoặc tác giả gặp rắc rối pháp lý, mình đề xuất dự án nên điều chỉnh một số điểm sau:\n\n### 1. Rebranding (Xóa bỏ các yếu tố vi phạm thương hiệu)\nHiện tại dự án đang sử dụng trực tiếp tên \"Zalo\" trong tên repo, tên file thực thi (`zalo-rust`), và database (`zalo_history.db`). \n- **Đề xuất:** Hãy đổi tên dự án thành một cái tên trung lập (ví dụ: `ctdo-client` hoặc `z-lite-client`). Thay đổi tên các biến, file thực thi và tên cơ sở dữ liệu để không chứa từ khóa bảo hộ của VNG.\n- Thay thế các icon/logo hiện tại bằng thiết kế riêng không gây liên tưởng trực tiếp đến thương hiệu gốc.\n\n### 2. Thêm Tuyên bố miễn trừ trách nhiệm (Disclaimer)\nTrong file `README.md`, dự án cần có một mục Disclaimer lớn và rõ ràng ngay từ đầu để bảo vệ chính tác giả.\n- **Đề xuất bổ sung:** \n  > *\"Dự án này là một nghiên cứu độc lập mã nguồn mở vì mục đích giáo dục (Educational Purposes Only). Dự án KHÔNG có bất kỳ mối liên hệ, tài trợ hay bảo trợ nào từ VNG Corporation hay Zalo. Người dùng tự chịu mọi trách nhiệm và rủi ro (bao gồm rủi ro bị khóa tài khoản gốc) khi sử dụng phần mềm này.\"*\n\n### 3. Tài nguyên đồ họa & Mã nguồn\n- Đảm bảo trong source code và file cài đặt không đóng gói (bundle) bất kỳ tài nguyên có bản quyền nào của VNG (âm thanh thông báo đặc trưng, bộ sticker, emoji độc quyền). Các tài nguyên này nếu cần thiết chỉ nên được fetch trực tiếp từ server thông qua token của người dùng cuối.\n\n### 4. Cam kết phi thương mại\n- Để tránh bị đội ngũ pháp lý của họ nhắm tới, dự án nên giữ vững cam kết 100% mã nguồn mở, không thu phí, không gắn quảng cáo và không nhận donate gắn liền với các tính năng mở rộng.\n\nMong tác giả xem xét các góp ý này để dự án có thể tồn tại và phát triển an toàn trong cộng đồng open-source. Cảm ơn công sức của các bạn!",
      "closedAt": "2026-11-01T19:07:29Z",
      "createdAt": "2026-11-01T09:46:02Z",
      "labels": [],
      "number": 34,
      "state": "CLOSED",
      "title": "Rủi ro pháp lý về thương hiệu & bản quyền từ VNG và đề xuất hướng khắc phục"
    },
    {
      "author": {
        "id": "U_kgDOBfQ3Iw",
        "is_bot": false,
        "login": "maiminhdung",
        "name": "maiminhdung"
      },
      "body": "Thông tin hệ điều hàng:\n+ OS: Nobara Linux 44 (Fedora 44)\n+ DE: GNOME 50.6\n+ WM: Mutter (Wayland)\n\nLỗi xảy ra ở phiên bản 0.0.2 của app.\n\n<img width=\"1016\" height=\"703\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/98561c10-1a73-4453-bbb1-0a986c54223c\" />\n<img width=\"1016\" height=\"703\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/191c7fc3-aefa-4708-8437-ea7db3f3455a\" />",
      "closedAt": "2026-11-01T10:47:00Z",
      "createdAt": "2026-11-01T09:32:42Z",
      "labels": [],
      "number": 33,
      "state": "CLOSED",
      "title": "Lỗi phông chữ ở mục đăng nhập Zalo, không hiển thị trên tên/logo app ở chuyển tab"
    },
    {
      "author": {
        "id": "U_kgDOCbz4Lg",
        "is_bot": false,
        "login": "ngxccc",
        "name": "Ngoc Tran"
      },
      "body": "## Context\n- Báo cáo gốc: #31 (mục 3, macOS, bản đầu): *\"Giao diện luôn hiển thị mặc định những người dùng là đang online dù không hề biết state online của người đó\"*.\n- Cùng triệu chứng xuất hiện trong ảnh của #8 (`Nhóm đang hoạt động`) và trong ảnh #31 (`Chưa kết bạn trên Zalo` vẫn có chấm xanh).\n\n## Current behaviour\nApp chưa từng gọi bất kỳ API/field presence nào — toàn bộ chỉ báo online là hằng số trong code giao diện:\n\n- `src/gui/workspace.rs:804` — mọi hội thoại không phải nhóm (`!t.is_group`) đều được vẽ chấm xanh trong danh sách chat.\n- `src/gui/workspace.rs:1493` — avatar ở khung chat luôn vẽ chấm xanh, không có điều kiện nào.\n- `src/gui/workspace.rs:1581` — hội thoại 1-1 là bạn bè → phụ đề `\"Đã kết bạn · Vừa mới truy cập\"`.\n- `src/gui/workspace.rs:1165` — cùng nội dung ở drawer phải: `\"✓ Đã kết bạn · Trực tuyến\"`.\n- `src/gui/workspace.rs:1575` / `:1159` — mọi nhóm: `\"Đang hoạt động\"` / `\"Nhóm đang hoạt động\"`.\n- `src/gui.rs:241` — `show_online_status: true` (toggle ở `src/gui/modals.rs:966`) chỉ bật/tắt chấm của **chính mình** ở dock trái (`src/gui/workspace.rs:53-57`), không liên quan tới người khác.\n\n## Root cause\nPresence chưa được triển khai: không có struct, field hay endpoint nào cho trạng thái online trong `src/` (không có `is_online`, `last_online`, `presence`). `FriendshipStatus::Friend` đang bị dùng thay cho trạng thái online.\n\n## Scope (chọn 1 trước khi code)\n- **A. Bỏ chỉ báo cho tới khi có nguồn thật:** xoá chấm xanh và các chuỗi `\"Trực tuyến\"` / `\"Đang hoạt động\"` / `\"Vừa mới truy cập\"` (thay bằng `\"Đã kết bạn\"`), giữ `show_online_status` cho chấm của chính mình. Rẻ, hết nói dối người dùng.\n- **B. Implement presence thật:** lấy trạng thái qua WS/API của Zalo, chỉ vẽ khi có dữ liệu; chưa có thì không hiển thị gì. Cần xác minh nguồn dữ liệu trước (chưa có trong repo).\n\n## Acceptance\n- [ ] Không còn chấm xanh / chuỗi \"Trực tuyến\", \"Đang hoạt động\", \"Vừa mới truy cập\" khi app không có dữ liệu presence.\n- [ ] Nếu chọn B: phân biệt được 3 trạng thái (online / offline / chưa rõ), có đường dữ liệu thật và test cho trạng thái \"chưa rõ\".\n- [ ] Docs feature parity (#20) ghi rõ mục \"trạng thái online\".\n\n## Verification\n- [ ] Mở app khi một liên hệ chắc chắn offline (đã tắt app trên điện thoại) → không hiện chấm xanh / không hiện \"Trực tuyến\".\n\nTrùng/liên quan: #31 (mục 3), #8.\n",
      "closedAt": "2026-11-01T10:46:58Z",
      "createdAt": "2026-11-01T09:24:29Z",
      "labels": [],
      "number": 32,
      "state": "CLOSED",
      "title": "fix(ui): trạng thái online đang đặt cứng — mọi hội thoại 1-1 đều hiện chấm xanh"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjYyMzc4NTM1",
        "is_bot": false,
        "login": "trungnt1000vn",
        "name": "Trung Nguyễn "
      },
      "body": "<img width=\"1395\" height=\"760\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/0ed69a73-0fd5-456c-a9e1-12ca7c5112c2\" />\n\n<img width=\"1397\" height=\"791\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/0dbb06f0-3ff9-4e7c-9f35-9f28c91af493\" />\n\nList các vấn đề : \n- Danh sách chat đang bị sai, hiển thị sai cả về mặt thời gian, thứ tự \n- Chưa sync được lịch sử tin nhắn, hoàn toàn không có gì dù đã đồng ý chuyển tin nhắn sang.\n- Giao diện luôn hiển thị mặc định những người dùng là đang online dù không hề biết state online của người đó\n- Nhóm đang hiển thị ID của nhóm chứ chưa hiển thị tên của nhóm là gì.\n- Lỗi hiển thị cho phần Setting",
      "closedAt": "2026-11-01T10:47:14Z",
      "createdAt": "2026-11-01T09:05:23Z",
      "labels": [],
      "number": 31,
      "state": "CLOSED",
      "title": "Issue trên MacOS version đầu"
    },
    {
      "author": {
        "id": "U_kgDOCbz4Lg",
        "is_bot": false,
        "login": "ngxccc",
        "name": "Ngoc Tran"
      },
      "body": "## Context\n\n- Parent: #3 (item 2 — domain 404)\n- Follows: #22 (Fix A: hằng số đã verify thắng `zpw_service_map_v3`)\n- Blocked by: None\n- Nhãn đề xuất: `tech-debt` (repo chưa có nhãn này; account mở issue không có quyền `push` để tạo nhãn upstream)\n\n## Scope\n\nFix A khiến mỗi endpoint chỉ còn **một** host đã verify. Đúng cho lỗi 404 hiện tại, nhưng đổi lại: khi Zalo xoay host đã verify (`tt-chat3` → `tt-chat4`, `p5-msg-chat` → `p6-…`), client bám hằng số cũ và 404 cho tới khi có bản build mới. Đợt này bỏ giả định \"một host đúng cho mỗi endpoint\":\n\n- Mỗi endpoint giữ **danh sách candidate** theo thứ tự ưu tiên: hằng số đã verify trước, host `zpw_service_map_v3` trả về sau, rồi tới host trong `session.json` cũ.\n- Một helper dùng chung thử lần lượt cho tới khi có response dùng được, và nhớ host thắng cho các request sau trong cùng phiên (không ghi vào `session.json`).\n- Chỉ chuyển candidate khi lỗi transport hoặc 404. Lỗi nghiệp vụ (`600`, `-102`) dừng ngay — giữ nguyên hành vi đã chốt ở #21.\n- `chat::get_friends` bỏ vòng lặp fallback riêng, gọi helper chung.\n\nNgoài phạm vi: không đổi shape `session.json`; không đụng `lp_chat` / `lp_control` / `ws` — ba host này Zalo xoay sẵn và đã lấy từ server mỗi lần login.\n\n## Acceptance\n\n- [ ] `Domains` giữ được nhiều hơn một host cho mỗi endpoint.\n- [ ] Candidate đầu fail (404 hoặc timeout) → tự thử candidate kế tiếp, không cần sửa code.\n- [ ] Host thắng được ghi nhớ và dùng lại cho request sau trong cùng phiên.\n- [ ] `chat::get_friends` dùng helper chung; vòng lặp `for host in hosts` cục bộ bị xoá.\n- [ ] Unit test chứng minh thứ tự thử và việc ghi nhớ host thắng (thêm seam transport nếu cần).\n- [ ] Lỗi `600` / `-102` không kích hoạt failover.\n",
      "closedAt": "2026-11-01T19:07:31Z",
      "createdAt": "2026-11-01T07:44:25Z",
      "labels": [],
      "number": 27,
      "state": "CLOSED",
      "title": "task(net): try multiple host candidates per endpoint instead of one verified constant"
    },
    {
      "author": {
        "id": "U_kgDODE5a1Q",
        "is_bot": false,
        "login": "DEWE-ex",
        "name": ""
      },
      "body": "Lỗi:\n\n<img width=\"1026\" height=\"66\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/26c6025b-13da-44a9-b5e7-e488d7165acd\" />\n- Phần border khi thu nhỏ cửa sổ/upfile bị lỗi\n- Không có tin nhắn group\n- Files từ điện thoại lên pc bị chuyển thành link (từ laptop về điện thoại bình thường)\nĐề xuất tính năng (về cơ bản là mấy cái tính năng sẵn có của zalo thôi)\n- Xóa tin nhắn\n- Xóa cuộc trò truyện\n- Lưu file về máy\n- Ẩn tin nhắn =)))\n- Nhắc hẹn\nDaika cứ làm dần, cuối tuần sau em rảnh em test tiếp cho daika",
      "closedAt": "2026-11-01T19:07:34Z",
      "createdAt": "2026-11-01T07:15:42Z",
      "labels": [],
      "number": 24,
      "state": "CLOSED",
      "title": "Lỗi (windows) và đề xuất tính năng"
    },
    {
      "author": {
        "id": "U_kgDOCbz4Lg",
        "is_bot": false,
        "login": "ngxccc",
        "name": "Ngoc Tran"
      },
      "body": "## Context\nParent: #3 (item 3 — \"Bấm nút đồng bộ chỉ có một toast notification hiện lên, không có pop-up tiến trình, và không có gì xảy ra\").\n\nTrùng: #10 (nội dung \"chưa thể đồng bộ toàn bộ tin nhắn\"), #8 (không đồng bộ được đoạn chat cũ).\n\n## Root cause\nNút gửi `WorkerCmd::RefreshThreads` (`src/gui/modals.rs:73`). Handler (`src/gui/worker.rs:609`) chỉ:\n- validate session + `sync_profile_me`\n- `get_friends`\n- `poll_new_messages` → ~50 tin gần nhất mỗi nhóm / Cloud\n\n**Không** tải lịch sử đầy đủ, **không** phát `GuiEvent` tiến trình, và lỗi bị nuốt bởi `unwrap_or_default()`. Toast đang nói dối người dùng.\n\nSync lịch sử thật đã tách sang #19.\n\n## Scope\n- Đổi nhãn nút thành **\"Làm mới hội thoại\"**, toast đúng sự thật (\"Đã làm mới danh sách hội thoại\").\n- Thêm `GuiEvent::RefreshProgress { done, total }` + `RefreshFinished { threads, messages, errors }`.\n- Hiện lỗi thật (ví dụ \"Không tải được danh sách bạn bè: HTTP 404 …\") thay vì nuốt.\n- Nếu `RefreshThreads` chạy do mở app, không hiện toast — chỉ hiện khi người dùng bấm.\n- Thêm dòng \"Muốn tải toàn bộ lịch sử? Xem #19\" trong UI (hoặc trong docs) để người dùng biết giới hạn.\n\n## Acceptance\n- [ ] Không còn chuỗi \"Đồng bộ tin nhắn\" ở bất kỳ đâu dùng cho hành vi refresh.\n- [ ] Refresh thất bại → có thông báo lỗi cụ thể, không im lặng.\n- [ ] Có tiến trình hiển thị khi refresh chạy lâu.\n- [ ] Không có toast khi refresh tự động lúc khởi động.\n",
      "closedAt": "2026-11-01T19:06:37Z",
      "createdAt": "2026-11-01T07:10:13Z",
      "labels": [],
      "number": 23,
      "state": "CLOSED",
      "title": "fix(ui): \"Đồng bộ tin nhắn\" only refreshes previews — rename, show real progress and errors"
    },
    {
      "author": {
        "id": "U_kgDOCbz4Lg",
        "is_bot": false,
        "login": "ngxccc",
        "name": "Ngoc Tran"
      },
      "body": "## Context\nParent: #3 (item 2 — `HTTP 404 Not Found from https://tt-friend-wpa.zaloapp.com/api/...`).\n\n## Root cause\nĐịa chỉ API đến từ 3 tầng, và thứ tự ưu tiên hiện tại là **sai**:\n\n| Tầng | Nguồn | Hiện tại |\n|---|---|---|\n| 1 | `endpoints::SERVICE_MAP_V3` — đã verify thật 2026-10-21 | bị ghi đè |\n| 2 | `zpw_service_map_v3` từ server lúc login | **thắng tuyệt đối** (`apply_server_info`, `src/session.rs:535`) |\n| 3 | `session.json` lưu lại | lưu vĩnh viễn cái sai |\n\n`load_from_file()` (`src/session.rs`) chỉ \"heal\" 3/13 domain (`chat`, `conversation`, `sticker`). `friend`, `profile`, `group`, `file`, `cloud_msg`, `voice_call`… giữ nguyên host sai từ tầng 2.\n\nChuỗi nhân quả: server trả `tt-friend-wpa.zaloapp.com` (phải là `.chat.zalo.me`) → lưu vào `session.json` → gọi bạn bè 404 → `validate_session()` (gọi `profile`, cũng có thể sai) fail → bị coi là \"hết hạn\" → xoá session (#21).\n\n## Scope — Fix A (chốt)\n1. `SERVICE_MAP_V3` thắng cho các key đã verify; `zpw_service_map_v3` **chỉ điền key chưa biết**.\n2. Heal **cả 13** domain trong `load_from_file()`, không chỉ 3.\n3. `get_friends`: đổi thứ tự fallback thành `friend` trước, `profile` sau (hiện đang ngược), và **surface lỗi** thay vì nuốt.\n4. Unit test: server map trả `.zaloapp.com` cho `friend` → assert vẫn dùng `.chat.zalo.me`.\n\n## Follow-up (không làm trong issue này)\nFix B: mỗi domain giữ **danh sách candidate**, thử lần lượt đến khi 200 — bền với việc Zalo xoay host (`tt-chat3` → `tt-chat4`). Cần refactor mọi call site đang ghép URL từ `s.domains.X`. Tách issue riêng, label `tech-debt`.\n\n## Acceptance\n- [ ] `session.json` chứa `tt-friend-wpa.zaloapp.com` → sau khi load, `domains.friend` là `tt-friend-wpa.chat.zalo.me`.\n- [ ] Unit test cho precedence: verified > server; server chỉ điền key trống.\n- [ ] Unit test heal: fixture session.json cũ → cả 13 domain đúng sau `load_from_file`.\n- [ ] `get_friends` lỗi → có log + lỗi hiện ra UI, không im lặng.\n",
      "closedAt": "2026-11-01T10:46:51Z",
      "createdAt": "2026-11-01T07:10:12Z",
      "labels": [],
      "number": 22,
      "state": "CLOSED",
      "title": "fix(net): zpw_service_map_v3 overrides verified domains — TUI friend list 404s on tt-friend-wpa.zaloapp.com"
    },
    {
      "author": {
        "id": "U_kgDOCbz4Lg",
        "is_bot": false,
        "login": "ngxccc",
        "name": "Ngoc Tran"
      },
      "body": "## Context\nParent: #3 (item 1 — \"Phiên đăng nhập trong session.json đã hết hạn … tài khoản được log out ngay khi đóng app\").\n\n## Root cause\n`Session::validate_session()` (`src/session.rs:242`) trả `false` cho **mọi** `Err`, kể cả lỗi mạng, HTTP 404/500, sai domain:\n\n```rust\nmatch self.get(&url).await {\n    Ok(resp) => resp.error_code == 0,\n    Err(_) => false,          // network error, 404, parse error -> \"expired\"\n}\n```\n\nCaller rồi xoá thẳng credentials:\n\n| Caller | Hành vi khi `false` |\n|---|---|\n| `src/main.rs:146` (TUI startup) | xoá `session.json` + `threads_cache.json`, `logout()` |\n| `src/gui/worker.rs:617` (`RefreshThreads`) | xoá `session.json`, `logout()`, `SessionExpired`, mở QR |\n| `src/gui.rs:564` | gửi `RefreshThreads` **ngay khi GUI khởi động** |\n\nNghĩa là: đóng app → mở lại → GUI tự `RefreshThreads` → bất kỳ lỗi mạng nào → xoá session + logout. Khớp chính xác báo cáo.\n\nThêm: `WorkerCmd::Logout` (`src/gui/worker.rs:421`) còn xoá `zalo_history.db` → đăng xuất làm mất toàn bộ lịch sử cục bộ.\n\n## Scope\n- Thay `bool` bằng `SessionStatus { Valid, Expired, Unreachable }` (hoặc enum tương đương).\n- `Expired` **chỉ** khi `error_code` là `600`/`-102` (và các mã auth rõ ràng). Mạng/HTTP/parse lỗi → `Unreachable`.\n- `Unreachable`: **giữ nguyên** `session.json`, hiện retry, không mở QR, không logout.\n- `Expired`: không xoá ngay — đánh dấu expired, hiện prompt đăng nhập lại, **chỉ ghi đè `session.json` sau khi login mới thành công**.\n- Logout chỉ xoá credentials. Xoá `zalo_history.db` là hành động riêng, có xác nhận.\n- TUI (`src/main.rs:149-150`) áp dụng cùng chính sách.\n\n## Acceptance\n- [ ] Kill mạng / chặn domain → mở app → session còn nguyên, có thông báo \"không kết nối được\", không mở QR.\n- [ ] Server trả `error_code 600` → hiện prompt đăng nhập lại; `session.json` chỉ bị ghi đè sau khi login thành công.\n- [ ] Logout không xoá `zalo_history.db`.\n- [ ] Unit test cho mapping `error_code`/lỗi mạng → `SessionStatus`.\n- [ ] Không còn `Err(_) => false` trong codebase.\n",
      "closedAt": "2026-11-01T10:45:59Z",
      "createdAt": "2026-11-01T07:10:10Z",
      "labels": [],
      "number": 21,
      "state": "CLOSED",
      "title": "fix(session): network/HTTP errors are treated as \"expired\" and delete session.json"
    },
    {
      "author": {
        "id": "U_kgDOCbz4Lg",
        "is_bot": false,
        "login": "ngxccc",
        "name": "Ngoc Tran"
      },
      "body": "## Context\nParent: #3 (item 4 — \"làm một trang docs về feature parity\").\n\nNgười dùng cuối không đọc GitHub. Docs chỉ nằm trong repo là sai chỗ. Nhưng hiện tại **không có đường ống deploy**:\n\n- `ctdozlite.ctdo.net` là HTML tĩnh, host **Hostinger (hPanel)**, sau Cloudflare.\n- Chỉ có `index.html`, `contributors.html`, `privacy.html`, `disclaimer.html`. Không có trang docs, không có `sitemap.xml`.\n- Repo chỉ có 1 workflow (`ci-release.yml`), không đụng tới website.\n- Người báo issue / contributor **không có credential Hostinger, không sửa được DNS**.\n\nVì vậy issue này **chỉ làm phần sinh docs**, không chờ quyền deploy.\n\n## Scope\n- Nguồn sự thật: **Markdown trong `docs/`**. HTML/docs site phải được **sinh ra**, không viết tay (tránh hai nguồn lệch nhau).\n- Thêm mdBook (hoặc VitePress) sinh site từ `docs/`, build được offline bằng `mdbook build`, output vào `target/` hoặc `book/` (gitignored).\n- Bốn trang:\n  1. `Tính năng` — feature matrix: `Tính năng | Trạng thái | Ghi chú | Issue`. Trạng thái: `Hoạt động / Một phần / Chưa hỗ trợ / Đã biết lỗi / Không kế hoạch`. Kèm dòng \"kiểm chứng tại commit `<sha>`\".\n  2. `Cài đặt` — trỏ về `docs/CAI_DAT.md` (SSOT hiện có).\n  3. `Khắc phục sự cố` — gồm session/404 (#21, #22), đăng nhập lại, mất dữ liệu.\n  4. `Đã biết` — **sinh tự động** từ GitHub Issues (label `bug` + `known-issue`) lúc build, để reporter thấy \"đang fix rồi\" trước khi mở issue trùng.\n- Link tới docs ở 3 chỗ: trong app (`Trợ giúp → Tính năng & giới hạn`), release notes, và issue template.\n\n## Out of scope (chặn bởi quyền)\n- Upload lên Hostinger, cấu hình DNS, GitHub Pages.\n\n## Phương án deploy (đích: **C**)\n- **A** (loại): viết tay HTML trong site hiện tại → hai nguồn sự thật, lệch sau 2 release.\n- **B**: GitHub Pages ở `docs.ctdozlite.ctdo.net` → cần sửa DNS.\n- **C** (đích): CI build mdBook rồi upload qua FTP/SFTP lên `public_html/docs/` → cần maintainer tạo secret `FTP_*`/`SFTP_*`.\n\nTrong lúc chờ C: build artifact đính kèm release, maintainer upload tay 1 lần/release.\n\n## Acceptance\n- [ ] `mdbook build` chạy được từ repo, không cần mạng.\n- [ ] `docs/TINH_NANG.md` là nguồn duy nhất của feature matrix.\n- [ ] Trang \"Đã biết\" sinh từ GitHub Issues, có fallback khi không có token/mạng.\n- [ ] Có link từ app, release notes, issue template.\n- [ ] PR mô tả ghi rõ: deploy hiện là thủ công, cần chuyển sang phương án C khi có quyền.\n",
      "closedAt": "2026-11-01T19:07:38Z",
      "createdAt": "2026-11-01T07:10:08Z",
      "labels": [],
      "number": 20,
      "state": "CLOSED",
      "title": "docs: generate user docs site from docs/*.md (mdBook) — deploy blocked on Hostinger access"
    },
    {
      "author": {
        "id": "U_kgDOCbz4Lg",
        "is_bot": false,
        "login": "ngxccc",
        "name": "Ngoc Tran"
      },
      "body": "## Context\n- Parent: #3 (item 3)\n- Duplicate report: #10 (\"chỉ mới đồng bộ tin nhắn của My document\")\n\n## Current behaviour\nThe \"Đồng bộ tin nhắn\" button sends `WorkerCmd::RefreshThreads`, which only:\n- validates the session and refreshes the profile,\n- calls `get_friends` to reload the thread list,\n- calls `poll_new_messages`, which merges ~50 recent messages per group / Cloud thread.\n\nIt does **not** download full history, emits **no progress events**, and swallows errors via `unwrap_or_default()`. The toast therefore claims a sync that never happened.\n\n## Scope (design first, implement after approval)\n- Define sync semantics: full history vs last N messages, per-thread on demand vs bulk.\n- Paginate cloud history (`/api/cm/getoldv2` and `/api/cm/getrecentv2`) with a resumable cursor per thread.\n- Cover 1-1 and group threads, including \"Cloud của tôi\".\n- Deduplicate by `msg_id` and persist to `zalo_history.db`.\n- Add `GuiEvent::SyncProgress { done, total, thread }` and `SyncFinished { added, skipped, errors }`.\n- UI: progress modal with cancel, error summary, and a truthful final state.\n- Rate-limit / concurrency cap to avoid getting the account flagged.\n\n## Non-goals\n- Do not fake progress or mark the current refresh as \"sync\".\n- Do not block the #3 bug fixes on this feature.\n\n## Acceptance\n- [ ] Sync downloads history beyond the latest 50 messages for at least one 1-1 and one group thread.\n- [ ] Progress is visible and cancellable; cancel leaves the DB consistent.\n- [ ] Re-running sync does not duplicate messages.\n- [ ] Errors are surfaced per thread, not swallowed.\n- [ ] Scope/semantics documented in an ADR or design note before implementation.\n\n## Open questions\n- Full history or a bounded window (e.g. last 1,000 messages / 1 year)?\n- Background sync on login, or only when the user asks?\n- One thread at a time (safe) or a small parallel pool (faster, riskier)?\n\n## Verification\n- [ ] Compare DB message count for one thread against Zalo Web before/after.\n- [ ] Kill the app mid-sync and confirm resume does not corrupt or duplicate.",
      "closedAt": "2026-11-01T19:07:41Z",
      "createdAt": "2026-11-01T06:57:36Z",
      "labels": [],
      "number": 19,
      "state": "CLOSED",
      "title": "feat(sync): full message history sync from Zalo cloud"
    },
    {
      "author": {
        "id": "U_kgDOCbz4Lg",
        "is_bot": false,
        "login": "ngxccc",
        "name": "Ngoc Tran"
      },
      "body": "## Context\n- Parent: #2\n- Release notes now list **merged PRs only**. Direct commits to `main` appear only via the Full Changelog link, so they are invisible in the release description.\n\n## Problem\nNothing currently prevents direct pushes to `main`, and the repository history up to `v0.0.2` was pushed directly. As long as that is allowed, auto-generated release notes will silently miss changes.\n\nThe current maintainer account used for this work has **pull-only access**, so this cannot be enabled from here.\n\n## Scope (maintainer, admin permission)\nEnable a branch protection rule or repository ruleset on `main`:\n\n- [ ] Require a pull request before merging.\n- [ ] Require status checks to pass before merging (at minimum the `prepare` job; ideally a lightweight `test` job — see note below).\n- [ ] Block force pushes.\n- [ ] Block branch deletion.\n- [ ] Optionally require linear history / squash merges to keep the PR list clean.\n\n## Note on status checks\nThe workflow currently runs `cargo test` only inside `linux-packages`, which is a full packaging job. Requiring it makes every PR pay the full multi-platform build cost. Consider adding a dedicated fast `test` job first, then require that check instead.\n\n## Acceptance\n- [ ] Direct push to `main` is rejected.\n- [ ] A PR with failing required checks cannot be merged.\n- [ ] A merged PR appears in the next release notes.\n\n## Verification\n- [ ] `gh api repos/ctdoteam1110/ctdozlite/branches/main/protection` returns the rule\n- [ ] Attempt a direct push to `main` from a non-admin account and confirm rejection",
      "closedAt": "2026-11-01T19:07:44Z",
      "createdAt": "2026-11-01T06:29:56Z",
      "labels": [],
      "number": 17,
      "state": "CLOSED",
      "title": "chore(repo): require pull requests on main (branch protection)"
    },
    {
      "author": {
        "id": "U_kgDOCbz4Lg",
        "is_bot": false,
        "login": "ngxccc",
        "name": "Ngoc Tran"
      },
      "body": "## Context\n- Parent: #2\n- Release notes are now auto-generated by GitHub from merged PRs (`generate_release_notes`).\n- The hand-maintained changelog section in `README.md` was removed to avoid duplication; the README now points to GitHub Releases.\n\n## Problem\nThe repository has **no `CHANGELOG.md`**. Release history exists only on the Releases page, and it is:\n- generated **once** at release creation (it does not update later);\n- **per release**, not cumulative;\n- **PR-only** (direct commits appear only via the Full Changelog link).\n\nSome contributors and downstream packagers expect a versioned changelog file in the repository.\n\n## Options\n- **(a) Releases-only (status quo).** Zero maintenance, single source of truth on the Releases page. No file in the repo.\n- **(b) `git-cliff` (or similar) generates `CHANGELOG.md` from Conventional Commits in CI.** Covers direct commits too, but requires a commit-back step or shipping the file only as a release asset, and a previous-tag range.\n- **(c) `release-please`.** Auto-opens a PR that bumps the version and updates `CHANGELOG.md`. Conflicts with the current decision that version bumps and tags stay manual (see #2), so it would need `skip-versioning`-style configuration or a rethink.\n\n## Decision needed\nPick one option and record it in an ADR if (b) or (c) is chosen.\n\n## Acceptance\n- [ ] Option chosen and documented.\n- [ ] If (a): README/docs link to Releases is the accepted canonical history.\n- [ ] If (b)/(c): generated changelog covers both PRs and direct commits, and does not introduce a second version source.\n- [ ] No manual per-release changelog editing remains.\n\n## Verification\n- [ ] Generate notes/changelog for the range `v0.0.2..<next tag>` and compare with the published release body.",
      "closedAt": "2026-11-01T19:07:47Z",
      "createdAt": "2026-11-01T06:26:06Z",
      "labels": [],
      "number": 16,
      "state": "CLOSED",
      "title": "docs(release): decide whether to keep a cumulative CHANGELOG.md in the repository"
    },
    {
      "author": {
        "id": "U_kgDOCbz4Lg",
        "is_bot": false,
        "login": "ngxccc",
        "name": "Ngoc Tran"
      },
      "body": "## Context\n- Parent: #2\n- `packaging/windows/installer.iss` and `packaging/macos/Info.plist` no longer contain a real version. They default to the sentinel `0.0.0`.\n- CI injects the real version from `Cargo.toml`:\n  - Windows: `iscc /DMyAppVersion=<version> packaging/windows/installer.iss`\n  - macOS: `plutil -replace CFBundleShortVersionString -string <version> .../Info.plist`\n\n## Problem\nA local packaging run without the CI flags produces installers labelled `0.0.0`. That is safe (it cannot be mistaken for a release) but inconvenient: contributors cannot verify a local build end-to-end without remembering the manual flags, and the result looks broken.\n\n## Scope\nAdd one local packaging entry point that reads the version from `Cargo.toml` and injects it into a **temporary copy** of the packaging templates:\n\n- Read the version once via `cargo metadata --no-deps` (same source as CI).\n- Windows: pass `/DMyAppVersion=<version>` to `iscc` (preferred) instead of editing the tracked file.\n- macOS: `plutil -replace` on the assembled app bundle, or render a temp plist.\n- Fail loudly if the version cannot be resolved.\n\n## Non-goals\n- Do not put a real version literal back into `installer.iss` / `Info.plist`.\n- Do not add a second version source; `Cargo.toml` stays the SSOT.\n- Do not change the CI release path (it already injects the version).\n\n## Acceptance\n- [ ] A documented local command builds `.exe` / `.dmg` with the `Cargo.toml` version.\n- [ ] The tracked templates still contain only the `0.0.0` sentinel.\n- [ ] CI behaviour is unchanged.\n- [ ] The local command is referenced from `docs/CAI_DAT.md` or the README build section.\n\n## Verification\n- [ ] Build locally with `Cargo.toml` at some `x.y.z`; installer metadata reports `x.y.z`.\n- [ ] `grep -R \"0.0.0\" packaging/` still shows only the intended sentinels.",
      "closedAt": "2026-11-01T19:07:50Z",
      "createdAt": "2026-11-01T06:26:06Z",
      "labels": [],
      "number": 15,
      "state": "CLOSED",
      "title": "build(packaging): inject Cargo.toml version into local installer.iss / Info.plist builds"
    },
    {
      "author": {
        "id": "U_kgDOCbz4Lg",
        "is_bot": false,
        "login": "ngxccc",
        "name": "Ngoc Tran"
      },
      "body": "## Context\n- Parent: #2\n- Related: release notes now rely on `generate_release_notes`, so merged PRs are listed automatically.\n\n## Problem\nGitHub reads `.github/release.yml` to group auto-generated release notes into categories. Categories are matched by **PR labels**, but the upstream repository currently has **no labels configured** (a label taxonomy exists only on the fork). Adding `release.yml` before labels exist would push every PR into a single catch-all category, which is no better than GitHub's default `What's Changed` section.\n\n## Scope\nDo this only after labels are synced to `ctdoteam1110/ctdozlite`:\n1. Sync the label taxonomy to upstream.\n2. Apply labels to future PRs (and optionally backfill a few recent ones).\n3. Add `.github/release.yml` with categories such as:\n   - `Features` — `feature`, `enhancement`\n   - `Fixes` — `bug`, `fix`\n   - `Documentation` — `documentation`\n   - `CI / Build` — `ci`, `build`\n   - `Other Changes` — `*` (catch-all, must exist so PRs are never silently dropped)\n4. Exclude noise labels (for example `skip-changelog`) only if that convention is adopted.\n\n## Acceptance\n- [ ] Labels exist on the upstream repository.\n- [ ] `.github/release.yml` has a catch-all `*` category.\n- [ ] A test release (or `gh api .../releases/generate-notes`) shows PRs grouped as expected.\n- [ ] PRs without labels still appear under `Other Changes`.\n\n## Verification\n- [ ] `gh api repos/ctdoteam1110/ctdozlite/releases/generate-notes -f tag_name=vX.Y.Z` output inspected",
      "closedAt": "2026-11-01T19:07:52Z",
      "createdAt": "2026-11-01T06:21:09Z",
      "labels": [],
      "number": 14,
      "state": "CLOSED",
      "title": "task(release): categorize auto-generated release notes with .github/release.yml"
    },
    {
      "author": {
        "id": "U_kgDOCbz4Lg",
        "is_bot": false,
        "login": "ngxccc",
        "name": "Ngoc Tran"
      },
      "body": "## Context\n- Parent: #2\n- Blocked by: None\n\n## Scope\nAdd a `workflow_dispatch` entry point that builds and packages all platforms without creating a GitHub Release, so release plumbing can be validated without pushing a real `v*` tag.\n\n- Input `version` (optional, default from `Cargo.toml`).\n- Input `publish` (default `false`); the release job is skipped when false.\n- Reuse the single version-propagation mechanism introduced for #2; do not duplicate version logic.\n\n## Acceptance\n- [ ] Manual run from Actions produces `.deb`, `.rpm`, `.AppImage`, `.tar.gz`, `.exe` and `.dmg` named with the input version.\n- [ ] No GitHub Release and no tag are created when `publish=false`.\n- [ ] `publish=true` still requires a `v*` tag and the tag/version equality guard.\n- [ ] Workflow YAML validated by a successful dry run.\n\n## Verification\n- [ ] Manual dry run on `main`\n- [ ] Confirm artifact names in the run summary",
      "closedAt": "2026-11-01T19:07:55Z",
      "createdAt": "2026-11-01T06:05:41Z",
      "labels": [],
      "number": 13,
      "state": "CLOSED",
      "title": "task(ci): add dry-run packaging workflow_dispatch"
    },
    {
      "author": {
        "id": "U_kgDOCbz4Lg",
        "is_bot": false,
        "login": "ngxccc",
        "name": "Ngoc Tran"
      },
      "body": "## Context\n- Parent: #2\n- Blocked by: None\n\n## Evidence\n- Release `v0.0.2` contains `latest.json` (`version: 0.0.2`, `sha256`, `repository`, `notes`) and `ctdozlite-0.0.2-portable-windows-x86_64.exe`.\n- Neither artifact exists in the tagged tree (`git grep latest.json v0.0.2` returns nothing) nor in any workflow.\n- `src/` contains no self-updater code, yet the `v0.0.2` release notes claim in-place auto-update.\n- `latest.json` has 7 downloads, so something may consume it.\n- The other 9 assets in `v0.0.2` are still named `0.0.1`.\n\n## Scope\nDecide and implement one of:\n- (a) Merge the updater source, then generate `latest.json` in CI from the tagged build (version, tag, SHA-256 of the canonical asset, notes).\n- (b) Remove the auto-update claim and delete `latest.json`; document that updates are manual.\nEither way, CI must become the only writer of release assets.\n\n## Out of Scope\n- Rewriting published `v0.0.2` assets. They stay frozen because they are downloaded and referenced by checksums.\n\n## Acceptance\n- [ ] No release asset is uploaded outside CI.\n- [ ] Manifest version and SHA-256 match the tagged build, or the manifest no longer exists.\n- [ ] Release notes match shipped functionality.\n- [ ] Documented in an ADR if the updater is kept.\n\n## Verification\n- [ ] `git grep latest.json <tag>` returns the generator or nothing by design\n- [ ] Downloaded asset hash matches the manifest",
      "closedAt": "2026-11-01T19:07:57Z",
      "createdAt": "2026-11-01T06:05:41Z",
      "labels": [],
      "number": 12,
      "state": "CLOSED",
      "title": "task(release): resolve self-updater manifest and manual release assets"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjg3MzI4Mzkx",
        "is_bot": false,
        "login": "T1st-isme",
        "name": "Hà Trung Triển"
      },
      "body": "Chưa thể đồng bộ toàn bộ tin nhắn từ zalo, chỉ mới đồng bộ tin nhắn của \"My document\"",
      "closedAt": "2026-11-01T19:08:00Z",
      "createdAt": "2026-11-01T05:23:17Z",
      "labels": [],
      "number": 10,
      "state": "CLOSED",
      "title": "Đồng bộ tin nhắn"
    },
    {
      "author": {
        "id": "U_kgDOCL1Wrw",
        "is_bot": false,
        "login": "Khoasoma",
        "name": "khoasoma"
      },
      "body": "<img width=\"1019\" height=\"473\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/d8e4d51d-8a44-443a-a07a-da718456655a\" />\n\n\ngửi ảnh hiện url cdn ( nên sửa )\n<img width=\"373\" height=\"546\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/fe2e9f8e-8891-4450-b478-399e7cea2872\" />\n\n\n\nDự án hiện tại đã khá hoàn chỉnh , phần cốt lõi như giao nhận tin nhắn đã hoàn thành , chỉ cần cải thiện về mặt giao diện là được",
      "closedAt": "2026-11-01T19:08:02Z",
      "createdAt": "2026-11-01T03:39:56Z",
      "labels": [],
      "number": 9,
      "state": "CLOSED",
      "title": "Cập nhật thứ tự nhắn tin sai , không hiện nhóm , tin nhắn được ghim lên đầu nên thêm tự import tin nhắn từ folder có sẵn ( migrate )"
    },
    {
      "author": {
        "id": "U_kgDOBtD1oQ",
        "is_bot": false,
        "login": "slothteabar",
        "name": ""
      },
      "body": "<img width=\"1220\" height=\"349\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/a36e3289-2329-4852-85c7-ab30ce09fe33\" /> \n\n<img width=\"1224\" height=\"615\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/e2af67ba-1e1b-4063-9ddd-ab1724c32f52\" />",
      "closedAt": "2026-11-01T10:47:11Z",
      "createdAt": "2026-11-01T03:15:47Z",
      "labels": [],
      "number": 8,
      "state": "CLOSED",
      "title": "Không có tên nhóm, không đồng bộ được đoạn chat cũ + Lỗi UI cài đặt. Win11"
    },
    {
      "author": {
        "id": "U_kgDOBX3z_w",
        "is_bot": false,
        "login": "Shigure-Ruiseki",
        "name": "Ruiseki"
      },
      "body": "Mình muốn hỏi liệu dự án có dự định cung cấp API để cộng đồng có thể tự phát triển plugin hoặc theme trong tương lai không?\nMình nghĩ nếu có API chính thức thì sẽ rất hữu ích, vì mọi người có thể tự mở rộng tính năng hoặc tùy chỉnh giao diện theo nhu cầu mà không cần phải chỉnh sửa trực tiếp mã nguồn của dự án.\nVí dụ: Custom Sticker Pack sync bằng GD hoặc cloud nào đó, theme sync với gtk và qt",
      "closedAt": "2026-11-01T19:08:05Z",
      "createdAt": "2026-11-01T03:14:54Z",
      "labels": [],
      "number": 7,
      "state": "CLOSED",
      "title": "[Feature Request] Thêm API hỗ trợ Dev Plugin và Theme"
    },
    {
      "author": {
        "id": "U_kgDOBipDdA",
        "is_bot": false,
        "login": "Nghia323223",
        "name": ""
      },
      "body": "Mình dùng nó không hiện được chữ việt, hay tàu, ảnh là link và bấm cũng không làm được hay xem được\nmình chạy trên Fedora 44.\n\n<img width=\"1023\" height=\"696\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/f8b96f48-0504-4815-9dc8-22946c974be6\" />",
      "closedAt": "2026-11-01T10:47:03Z",
      "createdAt": "2026-11-01T02:21:03Z",
      "labels": [],
      "number": 6,
      "state": "CLOSED",
      "title": "Không thể hiển thị được chữ"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjQ1NDc1MDg=",
        "is_bot": false,
        "login": "heroes1412",
        "name": ""
      },
      "body": "chữ tàu thì hiện ô vuông, ảnh thì là 1 cái link, nhưng bấm vào link cũng k xem dc ảnh.\n group nhiều thành viên bị chớp chớp, kiểu bị refresh hay sao đó\nmình chạy trên windows",
      "closedAt": "2026-11-01T19:08:07Z",
      "createdAt": "2026-11-01T02:18:23Z",
      "labels": [],
      "number": 5,
      "state": "CLOSED",
      "title": "k hiển thị dc chữ tàu, k hiển thị dc ảnh, group nhiều thành viên bị chớp chớp???"
    },
    {
      "author": {
        "id": "U_kgDOCbrkvQ",
        "is_bot": false,
        "login": "tinzisreal",
        "name": "Nguyen Huu Tung"
      },
      "body": "<img width=\"877\" height=\"716\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/6f3f2921-d8ff-4e4a-b70d-d01a3126fb48\" />",
      "closedAt": "2026-11-01T10:47:08Z",
      "createdAt": "2026-11-01T01:37:39Z",
      "labels": [],
      "number": 4,
      "state": "CLOSED",
      "title": "Hiện không đúng giờ"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjU5Mjg0NTQ5",
        "is_bot": false,
        "login": "shinishiho",
        "name": "Đỗ Quang Minh"
      },
      "body": "1. `⚠️ Phiên đăng nhập trong session.json đã hết hạn. Đang chuyển sang đăng nhập mới…` (trên TUI). Tương tự trong GUI, tài khoản được log out ngay khi đóng app.\n2. TUI trống, không tải được danh sách chat: `Lỗi tải danh sách bạn bè: HTTP 404 Not Found from https://tt-friend-wpa.zaloapp.com/api/...`\n3. Đồng bộ tin nhắn trên GUI: Bấm nút đông bộ chỉ có một toast notification hiện lên, không có pop-up tiến trình, và không có gì xảy ra.\n\nBtw, mình biết phần mềm đang \"beta\", nên nếu được thì bạn hãy làm một trang docs về feature parity nhé, như vậy sẽ tiện hơn cho mn biết cái nào đang hoạt động cái nào không (đã biết đối với dev). Chứ để report issues trùng lặp hoặc những cái đã biết và đang fix thì cũng bất tiện.",
      "closedAt": "2026-11-01T10:47:05Z",
      "createdAt": "2026-10-31T23:34:40Z",
      "labels": [],
      "number": 3,
      "state": "CLOSED",
      "title": "`session.json`, TUI và đồng bộ tin nhắn"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjU5Mjg0NTQ5",
        "is_bot": false,
        "login": "shinishiho",
        "name": "Đỗ Quang Minh"
      },
      "body": "CI release đang hardcode version là `0.0.1` thì phải, kết quả là release cho v0.0.2 mà filename vẫn ghi `0.0.1`.\nTrên góc trái GUI hoặc `zalo-rust --version` cũng đều hiện `v0.0.1`.\n\nEdit: có vẻ từ tag 0.0.1 -- 0.0.2 chỉ có commit về CI và docs.",
      "closedAt": "2026-11-01T10:46:00Z",
      "createdAt": "2026-10-31T23:24:27Z",
      "labels": [],
      "number": 2,
      "state": "CLOSED",
      "title": "CI release version"
    },
    {
      "author": {
        "id": "U_kgDOCMTSTg",
        "is_bot": false,
        "login": "duyan0",
        "name": "Võ Duy Ân"
      },
      "body": "<img width=\"1026\" height=\"638\" alt=\"Image\" src=\"https://github.com/user-attachments/assets/f2c6946f-e15d-40ea-9f69-0f95763a351b\" />",
      "closedAt": "2026-10-31T17:24:00Z",
      "createdAt": "2026-10-31T15:58:20Z",
      "labels": [],
      "number": 1,
      "state": "CLOSED",
      "title": "Bug UI"
    }
  ],
  "prs": [
    {
      "author": {
        "id": "U_kgDOBkRHDA",
        "is_bot": false,
        "login": "ziq3",
        "name": "Nguyen Tuan Khoi"
      },
      "body": "## What\r\n- Stop showing OS banners again, on every launch, for reactions that were already shown in an earlier run.\r\n- Persist, per account, how far reaction banners have been handled (`accounts/<uin>/reaction_banner_mark`): a mark (highest server id handled) plus pending ids whose target message was not loaded yet. Show a banner, and count the reaction as unread, only for reactions that are not done yet.\r\n\r\n## Why\r\n- Each connect pulls `610_1`/`611_1` with `lastId: 1`, so the server replays every reaction it still retains.\r\n- The dedup in `ReactionInbox` is in RAM only.\r\n- `ReactionEvent::historical` is always `false`, so the existing banner gate never fires.\r\n- Result: the same 4 reaction banners appeared on every launch (reproduced with `dbus-monitor`). Reading the messages did not help.\r\n\r\n## How\r\n- `reactions::{banner_seq, BannerMark, load_banner_mark, save_banner_mark}`.\r\n  - The id is `ReactionEvent::event_id` (`msgId`), which equals the queue `actionId` and increases in server order. `ts` does not follow that order (see the capture).\r\n  - Events without a numeric id are never suppressed.\r\n  - The path is only built for `is_safe_uin`, under `account_dir`, so it is deleted together with the account.\r\n- In the `GuiEvent::ReactionEventsReceived` handler:\r\n  - Load the mark once per uin.\r\n  - Gate both the banner loop and the reaction-unread loop on the mark.\r\n  - Advance the mark to the max id in the batch. A peer reaction whose target message is not loaded yet has no banner decision, so it stays in `pending` (capped at 256) and a later replay can still banner it. Save on a background thread. Each save takes a generation number on the UI thread, and the writer drops any save older than the last one written for that account, so a late writer cannot put back a lower mark.\r\n- The replay is kept on purpose: `ReactionInbox` needs it for correct counts, because history snapshots are stale. No socket or protocol change.\r\n- Rejected: persisting the queue cursor. The server accepts it, but it removes the replay that counts rely on and loses control events for passive accounts.\r\n- Capture (`ZALO_REACTION_TRACE=1`): the `610_1` answer held 5 reacts with `msgId` = `actionId` 8491736742795 … 8492421889729, in increasing order, while `ts` was out of order on two of them.\r\n\r\n## Verification\r\n- [x] `cargo test --locked`\r\n- [ ] `cargo clippy --all-targets -- -D warnings`: the baseline `1ff6ee2` already has warnings. No new warnings on the changed lines.\r\n- [ ] `cargo fmt --all --check`: the baseline `1ff6ee2` already has diffs. The changed code is rustfmt-clean.\r\n- [x] Manual: GUI, Linux (CachyOS), existing session (no re-login). `dbus-monitor` watching `org.freedesktop.Notifications.Notify`:\r\n  - Run 1 (no mark yet): the 4 old reaction banners appear once, then the mark is written (`8492421889729`).\r\n  - Run 2, with `ZALO_REACTION_TRACE=1` confirming the server replayed the same 5 reactions: 0 banners.\r\n  - Reaction counts on the affected messages are unchanged (❤️×3, ❤️×1, 👍×1).\r\n  - After the pending-ids fix: relaunched with the previous one-line mark file, the replay arrived and 0 banners were shown (old format still loads).\r\n- [x] Unit tests: `banner_gate_uses_server_seq_not_ts`, `unresolved_reaction_stays_eligible_until_its_message_loads` (early reaction stays eligible, newer resolved one is not re-bannered, pending cap) and `older_banner_save_never_replaces_a_newer_one`.\r\n\r\n## Risk & Rollback\r\n- Risk: two-way door.\r\n- Blast radius: reaction banners and the reaction-unread badge only. Counts, socket, messages and the other control consumers are unchanged.\r\n  - If a live push carried a different id, that reaction would be bannered once more after a restart.\r\n  - If a future `603_1` reaction used its own id range, its banner would be suppressed.\r\n- Rollback: revert the commit. Deleting `accounts/<uin>/reaction_banner_mark` resets the mark, so old reactions are bannered once and the mark is saved again.\r\n\r\n## Context\r\n- Resolves: None\r\n- Relates to: None\r\n- ADR / RFC: None\r\n\r\n🤖 Generated with [Claude Code](https://claude.com/claude-code)\r\n\n\n<!-- This is an auto-generated comment: release notes by coderabbit.ai -->\n\n<!-- coderabbit-ai-generated:v1 -->\n\n## Summary by CodeRabbit\n\n* **Bug Fixes**\n  * Reaction banners and per-thread unread counts now exclude reactions that have already been processed, preventing them from appearing again.\n  * Reactions whose target message is not yet available remain eligible to appear later, so delayed message resolution does not hide them permanently.\n  * Processing progress is maintained separately for each account and preserved across app restarts.\n\n<!-- end of auto-generated comment: release notes by coderabbit.ai -->",
      "closedAt": null,
      "createdAt": "2026-11-15T16:03:53Z",
      "labels": [],
      "number": 154,
      "state": "OPEN",
      "title": "fix(chat): stop re-showing reaction banners on every launch"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjc3MTQyMDUw",
        "is_bot": false,
        "login": "nguyenhoclaptrinh",
        "name": "Nguyen Truong"
      },
      "body": "## What\n- Align Sync V2 handshake payload with upstream Zalo PC two-phase flow to trigger mobile confirmation prompt.\n- Replace HTTP GET query string with POST form submission on command 12888 to avoid URI line length limits.\n- Resolve Windows sync diagnostic log path under AppData.\n\n## Why\n- Initiating Sync V2 with massive message partition query lists causes HTTP 414 URI Too Long on upstream reverse proxies and causes mobile clients to reject the unconfirmed session.\n- Logging to Unix `/tmp` on Windows silently discards diagnostic logs.\n\n## How\n- Split Sync V2 into an initiation phase (`req.type: \"conversation\"`) and a post-confirmation query phase (`req.type: \"message\"`).\n- Switched `request_phone_sync_v2_http` to `post_form`.\n- Configured Windows diagnostic logs to write to `%APPDATA%/ctdozlite/sync_debug.log`.\n- Added unit test `sync_v2_initiation_payload_uses_conversation_partition`.\n- Documented upstream findings in `re-notes/38_sync_v2_handshake_and_windows_fixes.md` and updated ADR 0001.\n\n## Verification\n- [x] `cargo test --lib sync_v2_initiation_payload_uses_conversation_partition` passed.\n- [x] `cargo check --lib` passed with zero errors.\n- [x] Upstream payload contract verified against Zalo PC 26.11.11 `app.asar`.\n\n## Risk & Rollback\n- Transport fallback remains backward compatible with active sessions.\n- Rollback: Revert commit on `fix/sync-v2-mobile-handshake`.\n\nFixes #151\n\n\n<!-- This is an auto-generated comment: release notes by coderabbit.ai -->\n\n<!-- coderabbit-ai-generated:v1 -->\n\n## Summary by CodeRabbit\n\n* **Bug Fixes**\n  * Improved phone message sync by separating conversation setup from message retrieval. Message partitions are requested after phone confirmation, helping the sync flow work across devices.\n  * Sync setup now uses a direct POST request, improving handling of larger requests.\n  * Sync diagnostic logs are stored in platform-appropriate locations, including on Windows and macOS.\n\n<!-- end of auto-generated comment: release notes by coderabbit.ai -->",
      "closedAt": "2026-11-15T09:00:01Z",
      "createdAt": "2026-11-15T06:06:52Z",
      "labels": [],
      "number": 153,
      "state": "CLOSED",
      "title": "fix(net): resolve sync v2 mobile handshake failure and windows sync logger path"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjc3MTQyMDUw",
        "is_bot": false,
        "login": "nguyenhoclaptrinh",
        "name": "Nguyen Truong"
      },
      "body": "## What\n- Suppress console window flicker when loading video thumbnails and native dialogs on Windows.\n\n## Why\n- Subprocesses executed via `std::process::Command::new` without `CREATE_NO_WINDOW` allocate a console host window on Windows, causing visual flicker during chat browsing.\n\n## How\n- Attached `CREATE_NO_WINDOW` (`0x0800_0000`) flag to ffmpeg and powershell subprocess calls on Windows via `std::os::windows::process::CommandExt`.\n- Documented upstream findings in `re-notes/38_sync_v2_handshake_and_windows_fixes.md` and updated ADR 0001.\n\n## Verification\n- [x] `cargo check --lib` passed with zero errors.\n- [x] Verified on Windows: thumbnail extraction runs silently without cmd window flashing.\n\n## Risk & Rollback\n- Subprocess window flags are guarded with `#[cfg(target_os = \"windows\")]` and introduce no API changes on Linux or macOS.\n- Rollback: Revert commit `e172917`.\n\nFixes #150\n",
      "closedAt": "2026-11-15T08:59:57Z",
      "createdAt": "2026-11-15T05:59:46Z",
      "labels": [],
      "number": 152,
      "state": "CLOSED",
      "title": "fix(ui): suppress console window flicker on windows media thumbnails"
    },
    {
      "author": {
        "id": "U_kgDOClROHw",
        "is_bot": false,
        "login": "Fin12n",
        "name": "Trần Đức Thịnh"
      },
      "body": "feat(ui,media,sync): redesign file cards, fix video playback, polish composer & sync v2 protobuf\n\n<!--\nTier guide (see docs/standards/git-flow-and-pr-matrix.md):\n- Tier 1 (docs/typo/deps): keep What + Why, delete the rest.\n- Tier 2 (feature/bugfix/refactor): fill every section, tick the checks you ran.\n- Tier 3 (protocol/schema/crypto/breaking): add ADR link, evidence and rollback detail.\nUse concise bullets, not paragraphs. Never paste session.json, cookies, zpw_sek/zpw_enk, IMEI or raw captures.\n-->\n\n## What\n- Redesign thẻ tập tin (File Card): bổ sung badge icon 40x40px phân loại màu theo định dạng (.pdf, .docx, .xlsx, .zip...), chuẩn hóa kích thước nút `[ Mở ]` (64x28px) và `[ Lưu về máy ]` (96x28px), thêm khoảng đệm 14px chống dính timestamp footer.\n- Sửa lỗi phát video: bổ sung nhận diện đường dẫn file local (`file://`, `C:\\...`) trong `open_external_url_with_monitor`, ngăn chặn việc tự thêm tiền tố `https://` gây lỗi DNS, kích hoạt trực tiếp video player mặc định của hệ điều hành qua `ShellExecuteW`.\n- Tối ưu modal xem video: thêm chu kỳ repaint 250ms trong lúc tải video để tự động phát ngay khi hoàn tất tải mà không bị đông cứng ở frame đầu tiên.\n- Tinh chỉnh thanh soạn thảo (Composer): tăng chiều cao input lên 38px, tăng khoảng cách toolbar lên 8px, căn giữa nút gửi và mở rộng không gian nhập liệu (tối thiểu 108px).\n- Nâng cấp parser Protobuf Sync V2: bóc tách Field 8 (`MetaData` -> repeated `Attach`), ánh xạ chính xác `msg_type` 18 (`chat.video.msg`) để lấy CDN URL, thumbnail và tiêu đề thay vì hiển thị chuỗi token hash 40 ký tự (`2aOboRCN...`).\n- Tự động hydrate tin nhắn cũ trong SQLite: tự phát hiện và nâng cấp các bản ghi video lưu dạng file thô về lại `MessageKind::Video` kèm thumbnail preview.\n\n## Why\n- Xử lý các lỗi UI/UX: nút bấm mở/lưu tệp bị bẹp dí sát giờ gửi, thanh soạn thảo chật chội gây cảm giác gò bó.\n- Sửa lỗi video không chạy được và bị kẹt cứng ở thumbnail (frame 1) do sai giao thức mở tệp trên Windows.\n- Ngăn chặn tin nhắn video bị suy thoái thành thẻ file thô chứa chuỗi hash vô nghĩa do thiếu parser Protobuf Field 8.\n\n## How\n- `src/gui/messages.rs`: tái cấu trúc `render_file_bubble_content`, chuẩn hóa layout container với `min_width` 280px và padding 14px; sửa `open_external_url_with_monitor` bổ sung cờ `is_local_file` trước khi xử lý mở tệp.\n- `src/gui/modals.rs`: thêm `ctx.request_repaint_after(Duration::from_millis(250))` trong `render_video_player_modal` khi `!viewer.is_downloaded`.\n- `src/gui/workspace.rs`: tăng chiều cao và padding của composer bar, căn giữa nút gửi.\n- `src/history_sync.rs`: bổ sung struct `ParsedProtoAttach`, bóc tách wire types cho Field 8 và chuyển đổi thành metadata attachment cho `ChatMessage`.\n- `src/store.rs`: thêm hàm `is_video_url` trong `hydrate_threads` để tự động nâng cấp record video cũ khi tải từ SQLite.\n\n## Verification\n- [x] `cargo test --locked` (318/318 tests passed)\n- [x] `cargo clippy --all-targets -- -D warnings`\n- [x] `cargo fmt --all --check`\n- [x] Manual: GUI, Windows 11 x64, kiểm tra phát video mượt qua OS player, thẻ file thoáng mắt, composer thoải mái, tin nhắn video từ sync hiển thị đúng thumbnail và tên.\n\n## Risk & Rollback\n- Risk: two-way door (toàn bộ thay đổi nằm ở tầng UI render, parser protobuf và hydration dữ liệu đọc từ SQLite, không làm thay đổi hay phá vỡ cấu trúc schema SQLite).\n- Blast radius: Giao diện thẻ tin nhắn (`messages.rs`), thanh soạn thảo (`workspace.rs`), modal phát video (`modals.rs`) và parser đồng bộ tin nhắn (`history_sync.rs`).\n- Rollback: `git revert 9c7fae9`\n\n## Context\n- Resolves: None\n- Relates to: Commit `9c7fae9`, tài liệu `Diff.md`\n- ADR / RFC: None\n\n<!-- This is an auto-generated comment: release notes by coderabbit.ai -->\n\n<!-- coderabbit-ai-generated:v1 -->\n\n## Summary by CodeRabbit\n\n* **New Features**\n  * Added in-app video playback, improved media and attachment previews, and clearer message status indicators.\n  * Added configurable animation settings, more responsive layouts, and improved theme and accessibility options.\n  * Added group-leaving controls and hidden password entry in the terminal.\n  * Improved chat history syncing and message handling for media, links, and other message types.\n  * Improved sticker loading with retry options, and added script management tools.\n\n* **Improvements**\n  * Refined navigation, search, dialogs, and message composition across desktop and terminal interfaces.\n  * Updated design documentation and change log.\n\n<!-- end of auto-generated comment: release notes by coderabbit.ai -->",
      "closedAt": "2026-11-15T02:33:53Z",
      "createdAt": "2026-11-14T04:54:31Z",
      "labels": [],
      "number": 148,
      "state": "MERGED",
      "title": "feat(ui,media,sync): redesign file cards, fix video playback, polish …"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjc1OTMxMzE=",
        "is_bot": false,
        "login": "gr219",
        "name": "Tuyến Trần"
      },
      "body": "## What\n- \"Đồng bộ tin nhắn từ điện thoại\" now finds the Sync V2 WASM (`libzproto_wasm_bg.*.wasm`) inside an installed Zalo PC's `resources/app.asar`, not only in plain folders.\n- When the phone (V2) step cannot open a session, the sync no longer idles in the 90 s \"waiting for confirmation\" loop; it moves straight on to the pinned / queue / Cloud / group phases.\n- The final summary keeps the reason the phone step failed (\"Không đồng bộ được từ điện thoại: …\") instead of overwriting it.\n- Clearer error when no Zalo PC is installed; the \"Cài Zalo PC…\" hint is no longer printed twice.\n\n## Why\n- On Windows, Zalo PC 26.11.11 packs `pc-dist/libs/libzproto_wasm_bg.5ea0fb2c….wasm` inside `app.asar`; `app.asar.unpacked` only holds native libs. `find_wasm_path` only globs directories, so every phone sync failed with \"Không sinh được khóa Sync V2: Không tìm thấy libzproto_wasm_bg.*.wasm\" even with Zalo PC installed.\n- With no session opened (no `590` sent), no confirmation can ever arrive, but the UI showed a spinner for 90 s at 20 % — it looked hung.\n\n## How\n- `zproto_host.rs`: `ZprotoHost::load` keeps the existing lookup order (`CTDOZLITE_ZPROTO_WASM` → plain-folder glob → `scripts/assets`) and then falls back to `find_wasm_in_zalo_asar`:\n  - candidates: `%LOCALAPPDATA%\\Programs\\Zalo\\Zalo-<ver>\\resources\\app.asar` (newest version first, numeric compare) and `%ProgramFiles%\\Zalo`; `/Applications/Zalo.app/…/app.asar` and `~/Applications` on macOS;\n  - parses the asar index (header pickle + JSON), finds the packed `libzproto_wasm_bg.*.wasm` (skips `libzproto_backup_wasm_bg`, skips entries flagged `unpacked`), seeks and reads only that entry (size-bounded, `\\0asm` magic checked);\n  - new `ZprotoHost::load_from_bytes`; `load_from(path)` delegates to it. Nothing is written to disk and the WASM is still never committed.\n- `history_sync.rs`: records the phone-step failure (`last_phone_sync_error`) and uses a zero confirm window when `v2_sync_id` is `None`.\n- `gui.rs`: `phone_sync_completion_message` gains `phone_error` and shows it in place of \"Lịch sử điện thoại chưa được tải vào ứng dụng\".\n- `chat.rs`: `wake_phone_sync_v2` no longer appends a second install hint to the loader's error.\n- No protocol change: params, signing, frame ids and the `590`/`591` flow are untouched.\n\n## Verification\n- [x] `cargo test --locked` — 313 lib tests pass (6 new: asar extraction, unpacked/missing entries skipped, non-WASM/garbage rejected, numeric version ordering, opt-in real-archive test, phone-error summary) plus all integration tests.\n- [x] `cargo clippy --all-targets` — diffed against `main`: no new warnings (`main` itself is not `-D warnings` clean).\n- [x] `cargo check --all-targets --target x86_64-pc-windows-gnu`.\n- [x] Real archive: `CTDOZLITE_TEST_ASAR=<Zalo-26.11.11>/resources/app.asar` → module read from `/pc-dist/libs/libzproto_wasm_bg.5ea0fb2c9af4a45bc7882b71307b6f83.wasm`, instantiates, generates a 33/32-byte keypair.\n- [x] Windows 11 host with Zalo PC 26.11.11 installed (no env vars): the lib test binary run natively finds the WASM via `app.asar`; key generation, uniqueness and agreement tests run instead of skipping.\n- [ ] Manual end to end: click \"Đồng bộ tin nhắn từ điện thoại ngay\" with the phone online and confirm on the phone.\n- [ ] macOS `app.asar` path (code path compiles; not run).\n\n## Risk & Rollback\n- Read-only access to Zalo PC's installation; only the index and one ~0.3 MB entry are read. A malformed archive is logged and skipped.\n- Without Zalo PC the phone step still cannot run (the WASM also performs Sync2 decryption). The public `stc-zlogin.zdn.vn` WASM was checked and lacks the `zprotoSync2*` exports, so it is not a substitute.\n- Rollback: revert this commit; behaviour returns to plain-folder lookup and the 90 s wait.\n\n\n<!-- This is an auto-generated comment: release notes by coderabbit.ai -->\n<!-- coderabbit-ai-generated:v1 -->\n\n## Summary by CodeRabbit\n\n* **New Features**\n  * Zproto can now be loaded from supported Zalo PC installations when it isn’t available as a separate file.\n* **Bug Fixes**\n  * Phone-sync startup errors are now included in the completion message, separately from overall sync failures.\n  * Phone syncing no longer waits for confirmation if a sync session cannot be started, allowing later sync steps to proceed.\n  * Key-generation failures are reported without the previous setup instructions.\n<!-- end of auto-generated comment: release notes by coderabbit.ai -->",
      "closedAt": "2026-11-15T02:33:53Z",
      "createdAt": "2026-11-13T08:37:15Z",
      "labels": [],
      "number": 145,
      "state": "MERGED",
      "title": "fix(sync): load Sync V2 WASM from Zalo PC app.asar"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjMxODg0NDg3",
        "is_bot": false,
        "login": "hoanglong149",
        "name": ""
      },
      "body": "## What\nTUI thành mini-workspace per-thread (không đụng bridge):\n- Console `/`: note/watch/stats/script/sum/help/find/privacy/model/provider + popup gợi ý (↑↓/Tab/Enter/Esc), Ctrl+F tìm trong chat (n/N, highlight vàng), sort recency + ghim theo GUI, footer 1 dòng + model indicator, splash boot animation, focus ring + sidebar Ctrl+B, linkify + OSC8 + attachment lines.\n- LLM đa provider (OpenAI-compatible): resolve env → opencode.json → ollama local; registry 20+ hãng (deepseek/minimax/openai/anthropic/google/qwen/moonshot/zhipu/groq/together/fireworks/deepinfra/openrouter/cerebras/novita/siliconflow/xai/mistral/volcengine/stepfun/baidu/tencent/iflytek/internlm/opencode-go); `fetch_models` live; `/model` + `/provider` chọn trong TUI, sync vào settings.\n- `src/auth.rs`: Keychain macOS (`zlite --auth`) + env + settings; không đọc app khác.\n- `src/agent.rs`: loop decide-act-log per-thread, dry-run mặc định, fail-closed 4 verb, reply gate (chỉ khi có tin operator), anti-injection (`<TIN_NHAN>` + red-team tests).\n- MCP: `--thread` lock tools theo thread (`wrong_thread`), chặn autoreply global trong bound mode, `zalo_privacy` tool, stdio sạch JSON.\n- Privacy gate M1: attachment chỉ vào context khi consent per-thread (TUI/GUI/MCP), audit log đầy đủ.\n- Data dir chung macOS (GUI/TUI/agent/MCP cùng Application Support); `zlite` binary rename.\n\n## Why\nBiến zlite thành nền tảng: chat + console + AI + agent trong 1 TUI; MCP cho agent ngoài (omp/Claude/Cursor) lái theo thread. Cộng đồng chỉ cần thêm provider vào registry là chạy.\n\n## Verification\n- [x] `cargo test --locked`: 315 passed (12 TUI + 5 agent + 4 llm/auth mới)\n- [x] `cargo clippy --all-targets`: không warning mới ở file chạm\n- [x] Smoke: `/sum` tóm tắt thật qua opencode-go local proxy; `/provider opencode-go` + `/model` đổi live\n\n## Risk & Rollback\n- Không đụng protocol/schema/bridge. Rollback = revert 1 commit.\n- Key không vào code/log (mask + Keychain 0600). Dry-run mặc định cho agent.\n\n<!-- This is an auto-generated comment: release notes by coderabbit.ai -->\n\n<!-- coderabbit-ai-generated:v1 -->\n\n## Summary by CodeRabbit\n\n* **New Features**\n  * Added an interactive terminal interface with chat search, slash commands, pinned threads, provider and model selection, and improved link and attachment display.\n  * Added LLM provider settings, model discovery, and API-key management, including secure keychain storage on macOS.\n  * Added per-thread controls for whether AI can read attachments, plus an optional AI agent mode with dry-run behavior by default.\n  * Added offline startup and the ability to open a specific thread from the command line.\n  * Updated the application’s command-line binary name to `zlite`.\n\n<!-- end of auto-generated comment: release notes by coderabbit.ai -->",
      "closedAt": "2026-11-15T08:59:21Z",
      "createdAt": "2026-11-13T08:28:56Z",
      "labels": [],
      "number": 144,
      "state": "MERGED",
      "title": "feat(tui): console workspace + LLM providers + agent loop + MCP thread lock + privacy gate"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjMxODg0NDg3",
        "is_bot": false,
        "login": "hoanglong149",
        "name": ""
      },
      "body": "## What\nBridge 2 chiều text-only Zalo ⇄ Telegram qua topic map (auto-provision forum topic):\n- `src/tg_bridge.rs` mới: poll Zalo → queue gửi Telegram (`TgJob` + retry backoff);\n  long-poll Telegram → gửi Zalo. Batch flush 1 lần/poll giữ thứ tự.\n- Quota LRU 10GB (`--bridge-quota-gb`), từ chối file đơn quá quota, bảng `bridge_files` + `bridge_sent` (echo-guard riêng, `zalo_privacy` tool).\n- CLI: `--tg-bridge --tg-map --tg-owner [--dry-run] [--media]`. Mặc định text-only; media PR sau.\n- Skip `get_server_info` khi `--offline` (rabbit review).\n\n## Why\nChat với người dùng Zalo từ Telegram; file/ảnh lưu vĩnh viễn bên TG, local chỉ đệm chuyển tiếp. Học zalo-tg/node: topic-per-thread, msg-id 2 chiều, graceful degradation.\n\n## Verification\n- [x] `cargo test --locked`: 319 passed (4 bridge tests: topic-map, sanitize, quota LRU, echo-guard)\n- [x] `cargo clippy --all-targets`: không warning mới ở file chạm\n- [x] Smoke `--dry-run`: poll 2 chiều sống, heartbeat log, không gửi\n\n## Risk & Rollback\n- Module mới + 2 bảng mới (CREATE IF NOT EXISTS). Không đụng protocol/schema hiện có. Rollback = revert.\n- Token qua env, không vào code/log. Owner gate mặc định tắt (chỉ bật khi dùng thật).\n\n<!-- This is an auto-generated comment: release notes by coderabbit.ai -->\n\n<!-- coderabbit-ai-generated:v1 -->\n\n## Summary by CodeRabbit\n\n* **New Features**\n  * Added a Telegram–Zalo bridge with message and optional media forwarding, topic mapping, and configurable storage limits.\n  * Added an AI assistant with configurable provider settings, per-thread controls, and attachment privacy settings.\n  * Expanded the terminal interface with in-chat search, slash commands, model selection, pinned threads, and richer message display.\n  * Added offline startup and thread-bound MCP tools.\n* **Improvements**\n  * Added macOS keychain support for API keys and moved default macOS data storage to Application Support.\n  * Renamed the default executable to `zlite`.\n* **Documentation**\n  * Added Vietnamese setup and usage guidance for the Telegram–Zalo bridge.\n\n<!-- end of auto-generated comment: release notes by coderabbit.ai -->",
      "closedAt": "2026-11-15T08:59:21Z",
      "createdAt": "2026-11-13T07:45:06Z",
      "labels": [],
      "number": 143,
      "state": "MERGED",
      "title": "feat(bridge): Telegram ↔ Zalo text relay + quota LRU + echo-guard"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjMzMjQxMDI0",
        "is_bot": false,
        "login": "ducseul",
        "name": "ducseul"
      },
      "body": "## What\r\n- Avatar menu lists the signed-in accounts (unread badge per inactive account, check mark on the current one), switches on click, and has a \"Thêm tài khoản\" entry.\r\n- \"Thêm tài khoản\" opens the existing login screen (QR or phone) with a \"Thoát thêm tài khoản\" button that returns to the previous account and cancels any pending QR/password login.\r\n- Inactive accounts keep a passive `zpw_ws` socket: their messages are stored in their own history database and raise OS banners labelled `sender → account`.\r\n- Media is separated per account: image cache in `accounts/<uin>/media_cache`, downloads in `<download_dir>/<uin>/`.\r\n- Saved accounts are labelled by profile name, then phone number, never by the raw account id.\r\n- SQLite connections now use WAL, `synchronous=NORMAL` and a 5 s busy timeout.\r\n\r\n## Why\r\n- Only one account could be live at a time; the others were silent until switched to.\r\n- The saved-account list showed the numeric id because the session never stored a name or avatar.\r\n- After signing in a new account the window froze in multi-second steps and ignored clicks on the avatar: UI-thread SQLite reads waited behind the worker's first-sync writes.\r\n\r\n## How\r\n- `background_accounts.rs` (new): a supervisor in the worker runtime reconciles one listener per saved account other than the connected one, on a 3 s cadence plus an explicit wake-up on switch.\r\n- `socket.rs`: `spawn_passive` never registers as the outbound target; any socket now closes as soon as its receiver is dropped, so a switched account does not keep a second connection.\r\n- `store.rs`: `configure_connection` applies the pragmas required by `docs/standards/database-and-migrations.md`; `persist_received_messages` writes messages without overwriting stored vanish settings or read-points from a stale thread snapshot.\r\n- `session.rs`: `sync_profile_me` accepts the profile under `data.profile`, `data.info` or bare `data`, no longer drops everything when `userId` is absent, and falls back to the existing `getprofiles` call on the own uid. `LoginInfo` gains an optional `phone` field.\r\n- `account_manager.rs`: active-account id, per-account paths (ids validated before use in paths), account labels, snapshot refresh from `session.json`.\r\n- `gui/worker.rs`: `SwitchAccount` aborts pending logins, resets poll state and releases the session lock before touching it; QR login snapshots the account and keeps the name/avatar reported at scan time; startup refresh now persists avatar and phone.\r\n- No new endpoint, host, parameter, signing or frame id; no schema change.\r\n\r\n## Verification\r\n- [ ] `cargo test --locked` — not run as written: debug builds fail to link `audiopus_sys` on the test machine (debug CRT mismatch). Ran `cargo test --release --locked --lib` instead: 327 passed, 1 ignored.\r\n- [ ] `cargo clippy --all-targets -- -D warnings` — not clean on `main` already; `cargo clippy --release --lib` reports no lints in the new code.\r\n- [ ] `cargo fmt --all --check` — not run; it would reformat unrelated legacy code.\r\n- [x] Manual: GUI, Windows 11, release build. Signed in a second account via the add-account flow; both accounts now store name, avatar and phone; the avatar menu opens immediately after login (previously 10–15 s).\r\n- Not verified manually: background OS banners for an inactive account, the unread badge, and per-account download folders.\r\n\r\n## Risk & Rollback\r\n- Risk: two-way door, with one persistent side effect — a database opened by this build stays in WAL mode (older builds read it fine).\r\n- Blast radius: account switching, login, media cache location, notifications, every SQLite connection.\r\n- Behaviour changes users will notice: downloads land in a `<uin>` subfolder; the old shared `stickers_cache/` is no longer used once signed in (images re-download once); after an explicit logout no account notifies.\r\n- Rollback: revert the branch. Per-account media folders can be deleted; they only hold cache.\r\n\r\n## Context\r\n- Resolves: None\r\n- Relates to: #103, #130 (saved accounts / add-account flow)\r\n- ADR / RFC: None\r\n- Known gaps: an expired background session retries every 30 s without telling the user; a poll already in flight during a switch can still deliver the previous account's messages (pre-existing).\r\n\n\n<!-- This is an auto-generated comment: release notes by coderabbit.ai -->\n\n<!-- coderabbit-ai-generated:v1 -->\n\n## Summary by CodeRabbit\n\n* **New Features**\n  * Manage multiple saved accounts from the profile menu or login screen, with account-switching controls and unread-message badges.\n  * Receive notifications for new messages in inactive accounts; muted conversations and outgoing messages are excluded.\n  * Account names and phone numbers appear in account lists, with clearer account labels and subtitles.\n  * Media and attachment downloads are separated by account.\n* **Bug Fixes**\n  * Profile details are retained when a refresh omits them, and older saved sessions remain compatible with phone-number details.\n\n<!-- end of auto-generated comment: release notes by coderabbit.ai -->",
      "closedAt": "2026-11-15T02:33:53Z",
      "createdAt": "2026-11-13T06:41:42Z",
      "labels": [],
      "number": 142,
      "state": "MERGED",
      "title": "Feature/multi account switcher"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjMxODg0NDg3",
        "is_bot": false,
        "login": "hoanglong149",
        "name": ""
      },
      "body": "# feat(bridge): Telegram ↔ Zalo text relay + quota LRU + echo-guard (PR1)\n\n## What\nCầu chat 2 chiều text giữa Zalo thread và Telegram topic:\n- `src/tg_bridge.rs` mới: poll Zalo (tái dùng `poll_new_messages`) → forward sang\n  Telegram (`sendMessage` + `reply_parameters`), Telegram long-poll → gửi về\n  Zalo (`send_message`/`send_group_message`).\n- Topic map `--tg-map \"<tg_chat>[/topic]:<zalo_tid>[,...]\"`, owner gate\n  `--tg-owner`, quota `--bridge-quota-gb` (mặc định 10), `--dry-run`.\n- SQLite: bảng `bridge_files` (msg↔path↔tg_file_id↔zalo_url↔offset) +\n  `bridge_sent(tg_key)` echo-guard riêng (không dùng KV hack).\n- Docs: `docs/CAU_TELEGRAM_ZALO.md`.\n\n## Why\nChat với người dùng Zalo đầu kia hoàn toàn từ Telegram; Telegram giữ file/ảnh\nvĩnh viễn, local chỉ là bộ đệm chuyển tiếp. Học từ `zalo-tg`/`zca-bridge`:\ntopic-per-thread, msg-id 2 chiều, graceful degradation — bỏ phần thừa\n(poll sync, reaction mapping đầy đủ).\n\n## How\n- Text trước, media sau (PR2): attachment hiện chỉ forward URL chữ, chưa tải file.\n- Echo-guard bảng riêng, không lẫn logic note/watch.\n- Quota LRU + `ensure_quota` đã có + unit test; file đơn > quota trống → từ chối ngay.\n\n## Verification\n- [x] `cargo test --locked`: tg_bridge 4 passed (topic-map, split target, sanitize, quota LRU) — full suite 319 passed trước khi tách (1 test cũ fail do schema SQL hỏng đã fix, re-run pass)\n- [x] `cargo clippy --all-targets`: không warning mới ở file chạm\n- [x] `cargo build --locked` pass\n- [ ] Smoke `--tg-bridge --dry-run` với nhóm test (cần TELEGRAM_BOT_TOKEN thật — chưa chạy ở đây)\n\n## Risk & Rollback\n- Module mới hoàn toàn + 2 bảng mới (CREATE IF NOT EXISTS), không đụng\n  protocol/threads hiện có. Rollback = revert 1 commit.\n- Token qua env, không vào code/log. Owner gate mặc định tắt (0) — bật khi dùng thật.\n- PR tiếp theo: PR2 media (album buffer 500ms + resume + local Bot API doc), PR3 recall/reply-chain đầy đủ.\n\n\n<!-- This is an auto-generated comment: release notes by coderabbit.ai -->\n\n<!-- coderabbit-ai-generated:v1 -->\n\n## Summary by CodeRabbit\n\n* **New Features**\n  * Added a Telegram–Zalo bridge for forwarding messages and media, with topic mapping, configurable storage quota, and dry-run support.\n  * Added an optional per-thread AI agent and settings for LLM endpoint, API key, and model.\n  * Added in-chat search, slash commands, thread sorting, and model/provider selection in the TUI.\n  * Added an attachment-reading privacy control for each conversation; it is off by default.\n  * Added offline mode and API-key management options.\n* **Documentation**\n  * Added Vietnamese setup and usage guidance for the Telegram–Zalo bridge.\n\n<!-- end of auto-generated comment: release notes by coderabbit.ai -->",
      "closedAt": "2026-11-15T09:00:19Z",
      "createdAt": "2026-11-13T04:12:35Z",
      "labels": [],
      "number": 140,
      "state": "CLOSED",
      "title": "feat(bridge): Telegram ↔ Zalo text relay + quota LRU + echo-guard (PR1)"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjMxODg0NDg3",
        "is_bot": false,
        "login": "hoanglong149",
        "name": ""
      },
      "body": "## What\nTUI thành mini-workspace per-thread: console lệnh `/` (note/watch/stats/script/sum/help/find/privacy/model/provider) với popup gợi ý, Ctrl+F tìm trong chat (n/N, highlight), sort recency + ghim theo GUI, footer hiện model đang dùng, splash boot animation.\n\nLLM đa provider (OpenAI-compatible): resolve env → opencode.json → ollama local; registry hãng (deepseek/minimax/openai/anthropic/google/qwen/moonshot/zhipu/groq/together/xAI/Mistral/Nga+TQ...); `fetch_models` live từ server; key qua Keychain (`zlite --auth`), không đọc auth file app khác.\n\nAgent loop `--agent --thread`: decide→act→log, dry-run mặc định, fail-closed 4 verb + reply gate (chỉ khi có tin operator).\n\nMCP: `--thread` lock + tool `zalo_privacy`, stdio sạch JSON.\n\nPrivacy: opt-in per-thread (TUI/GUI/MCP), M1 attachment gate + audit log.\n\n## Why\nBiến zlite thành nền tảng: chat + console + AI + agent trong 1 TUI, MCP cho agent ngoài (omp/Claude) lái theo thread. Cộng đồng chỉ cần thêm provider vào registry là chạy.\n\n## How\n- TUI slash dispatch + popup + find-mode; sort/pin dùng fingerprint cache\n- `src/llm.rs`, `src/auth.rs`, `src/agent.rs` mới; GUI settings section + workspace toggle\n- Chuẩn repo: comment EN, string VI; worker-thread cho I/O\n\n## Verification\n- [x] `cargo test --locked`: 315 passed, 0 failed (+12 test mới: console/find/sort/pin/privacy/injection)\n- [x] `cargo clippy --all-targets`: không warning mới ở file chạm\n- [x] Smoke: `/sum` tóm tắt thật qua opencode-go, `/model` đổi provider live\n\n## Risk & Rollback\n- TUI-only + additive modules; GUI cũ giữ nguyên path. Rollback: revert 1 commit.\n- Key không bao giờ vào code/log (mask + Keychain 0600).\n- Anti-injection: tin nhắn bọc `<TIN_NHAN>`, reply gate, rule fallback chỉ note/silence — có red-team test.\n\n<!-- This is an auto-generated comment: release notes by coderabbit.ai -->\n\n<!-- coderabbit-ai-generated:v1 -->\n\n## Summary by CodeRabbit\n\n* **New Features**\n  * Added configurable AI assistant support, including provider/model selection, API key management, connection checks, and optional per-thread automation.\n  * Added controls for whether AI features can read a thread’s attachments.\n  * Added offline mode and thread-specific MCP access.\n  * Added chat search, pinned threads, adjustable sidebar modes, slash commands, and model/provider selection in the TUI.\n  * Added attachment labels, clickable links, and search-result highlighting.\n* **Improvements**\n  * The executable is now named `zlite`.\n  * macOS data is stored in the app’s Application Support folder.\n\n<!-- end of auto-generated comment: release notes by coderabbit.ai -->",
      "closedAt": "2026-11-15T08:59:21Z",
      "createdAt": "2026-11-12T10:36:00Z",
      "labels": [],
      "number": 137,
      "state": "MERGED",
      "title": "feat(tui): console workspace + LLM providers + MCP thread lock + privacy gate"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjMxODg0NDg3",
        "is_bot": false,
        "login": "hoanglong149",
        "name": ""
      },
      "body": "## What\n- Worker poll 1.2s gửi `ThreadsUpdated` vô điều kiện → relayout toàn bộ message list mỗi tick kể cả khi không có tin mới.\n- Sidebar filter+sort ~1300 threads chạy lại mỗi frame repaint.\n- Thêm fingerprint gate (id/preview/len/last-msg/unread) và sidebar cache 500ms.\n\n## Why\nFixes #135 — bản 0.1.1 ăn 100% CPU trên máy yếu (Air 2020 i3). Đo trên Mac: idle 43% → 20%, RAM 864MB → 421MB.\n\n## How\n- `src/gui/worker.rs`: fingerprint trước khi send, bỏ tick không đổi.\n- `src/gui/workspace.rs` + `src/gui.rs`: cache filtered+sorted indices (`ThreadListCache`, `MsgHits`).\n\n## Verification\n- [x] `cargo test --locked` — 233 passed\n- [x] `cargo clippy --all-targets` — không warning mới\n- [x] Manual: idle 1 phút không chạm chuột, Activity Monitor 43% → 20%\n\n## Risk & Rollback\n- Pure UI perf, không đụng protocol/schema/DB. Rollback = revert commit.\n\n<!-- This is an auto-generated comment: release notes by coderabbit.ai -->\n\n<!-- coderabbit-ai-generated:v1 -->\n\n## Summary by CodeRabbit\n\n* **Improvements**\n  * Sidebar filtering, sorting, and message-search results can be reused briefly, helping the thread list feel more responsive.\n  * The thread list now refreshes when its displayed content or unread state changes, rather than refreshing unnecessarily.\n  * If search-result processing encounters an error, the app continues to recover by clearing the query and showing a notification.\n\n<!-- end of auto-generated comment: release notes by coderabbit.ai -->",
      "closedAt": "2026-11-15T02:33:53Z",
      "createdAt": "2026-11-12T10:25:50Z",
      "labels": [],
      "number": 136,
      "state": "MERGED",
      "title": "fix(gui): gate poll-driven relayout + cache sidebar filter/sort"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjgwOTM2MjAw",
        "is_bot": false,
        "login": "cuongdm75",
        "name": ""
      },
      "body": "- In send_link_message_with_client_id: when sendlink fails, text fallback retains URL even if caption omitted it\n- In forward_message_with_client_id: add rich-link card forwarding flow and robust text fallback\n- In parse_chat_message: prioritize link cards over photo CDN thumbnails and correctly classify MessageKind::Link\n- Add comprehensive regression tests in tests/test_message_forward.rs\n\n<!--\nTier guide (see docs/standards/git-flow-and-pr-matrix.md):\n- Tier 1 (docs/typo/deps): keep What + Why, delete the rest.\n- Tier 2 (feature/bugfix/refactor): fill every section, tick the checks you ran.\n- Tier 3 (protocol/schema/crypto/breaking): add ADR link, evidence and rollback detail.\nUse concise bullets, not paragraphs. Never paste session.json, cookies, zpw_sek/zpw_enk, IMEI or raw captures.\n-->\n\n## What\n-\n\n## Why\n-\n\n## How\n-\n\n## Verification\n- [ ] `cargo test --locked`\n- [ ] `cargo clippy --all-targets -- -D warnings`\n- [ ] `cargo fmt --all --check`\n- [ ] Manual: <GUI/TUI, OS, flow, observed error_code>\n\n## Risk & Rollback\n- Risk: two-way door / one-way door\n- Blast radius:\n- Rollback:\n\n## Context\n- Resolves: None\n- Relates to:\n- ADR / RFC: None\n\n\n<!-- This is an auto-generated comment: release notes by coderabbit.ai -->\n<!-- coderabbit-ai-generated:v1 -->\n\n## Summary by CodeRabbit\n\n* **New Features**\n  * Forwarding supports richer content, including media, captions, and link cards, with a text fallback when a link card can’t be sent.\n  * Recipients can be filtered by thread name or ID, and the forwarding flow supports dismissal and selection-state handling.\n* **Bug Fixes**\n  * Failed link sends now preserve the link URL in the text fallback.\n  * Link cards are distinguished from photos, including when a photo thumbnail contains an image URL.\n<!-- end of auto-generated comment: release notes by coderabbit.ai -->",
      "closedAt": "2026-11-15T02:33:53Z",
      "createdAt": "2026-11-12T01:38:10Z",
      "labels": [],
      "number": 134,
      "state": "MERGED",
      "title": "fix(chat): preserve url in rich link forward text fallback"
    },
    {
      "author": {
        "id": "U_kgDOClROHw",
        "is_bot": false,
        "login": "Fin12n",
        "name": "Trần Đức Thịnh"
      },
      "body": "<!--\nTier guide (see docs/standards/git-flow-and-pr-matrix.md):\n- Tier 1 (docs/typo/deps): keep What + Why, delete the rest.\n- Tier 2 (feature/bugfix/refactor): fill every section, tick the checks you ran.\n- Tier 3 (protocol/schema/crypto/breaking): add ADR link, evidence and rollback detail.\nUse concise bullets, not paragraphs. Never paste session.json, cookies, zpw_sek/zpw_enk, IMEI or raw captures.\n-->\n\n## What\n-\n\n## Why\n-\n\n## How\n-\n\n## Verification\n- [x] `cargo test --locked`\n- [x] `cargo clippy --all-targets -- -D warnings`\n- [x] `cargo fmt --all --check`\n- [x] Manual: <GUI/TUI, OS, flow, observed error_code>\n\n## Risk & Rollback\n- Risk: two-way door / one-way door\n- Blast radius:\n- Rollback:\n\n## Context\n- Resolves: None\n- Relates to:\n- ADR / RFC: None\n\n\n<!-- This is an auto-generated comment: release notes by coderabbit.ai -->\n<!-- coderabbit-ai-generated:v1 -->\n\n## Summary by CodeRabbit\n\n* **New Features**\n  * Added searchable Contacts and Social Feed pages, with friend and group views, timelines, image viewing, and packet tools.\n  * Added scheduled-message management and history import feedback in the app.\n  * Added safer Windows data storage selection and migration for existing app data.\n\n* **Improvements**\n  * Improved the login screen, title bar, icons, tooltips, and window resizing.\n  * Windows shortcuts now launch the app from its installation directory.\n\n* **Bug Fixes**\n  * Improved preservation of account history and handling of local conversation data during account changes and shutdown.\n<!-- end of auto-generated comment: release notes by coderabbit.ai -->",
      "closedAt": "2026-11-11T18:29:18Z",
      "createdAt": "2026-11-11T07:40:28Z",
      "labels": [],
      "number": 132,
      "state": "CLOSED",
      "title": "Fix UI ux"
    },
    {
      "author": {
        "id": "U_kgDOBeZLAA",
        "is_bot": false,
        "login": "Leoaxo098",
        "name": ""
      },
      "body": "## What\n\n- Replace FFmpeg call-video discovery, capture, preview and decoding with native camera APIs and bundled source-built OpenH264.\n- Rebase onto current upstream v0.1.0, preserving native Wasmtime Sync v2, persisted read points and upstream UI behavior.\n- Retain the reviewed account/history isolation, camera-off, reaction replay and read-receipt fixes.\n\n## Why\n\n- Call video should work without an FFmpeg executable/library or runtime codec download.\n- Integrate the feature without reverting the newer sync, read-point and UI changes already on main.\n\n## How\n\n- Pin Nokhwa 0.10.11 and source-linked OpenH264 0.9.9. Keep upstream Wasmtime 50.0.1 and regenerate the combined lockfile with Cargo.\n- Reflect Wasmtime's existing Rust 1.97 requirement in the manifest and setup docs; retain edition 2021. Verification used Rust 1.99, not the historical video-only 1.89 baseline.\n- Share camera permission and bounded ordered video queues across call legs, use fresh-IDR recovery and coalesce generation-tagged frames.\n- Preserve upstream native Sync v2 while routing its imports to account-owned stores/caches. Require verified ownership for automatic legacy migration; unmarked files require explicit Settings import. Preserve last-read IDs and scheduler rows, serialize migration, and preflight unsupported import files before writes.\n- Bind queued actions to the visible account and generation at enqueue time, snapshot accepted action sessions, and reject/clean up late calls. Independent work generations let history wipes cancel old writes/events while active calls retain their account lifetime.\n- Preserve cached history during authentication restarts, clear reply/forward state on owner changes, and restore usable command scopes after failed switches/wipes. TUI logout retains its retry identity until disk cleanup succeeds.\n- Recover abandoned schedules under exclusive delivery ownership, warning that delivery may have completed. Current-schema history reads avoid the writer lock.\n- Fix device-discovery completion ordering and primary-video SSRC recovery; use direct signalling/SRTP for per-member group fallbacks. Reconcile queued reactions against late-loaded persisted event identities.\n- Keep embedded dependency notices. No Cisco binary-module royalty coverage is claimed for this source build.\n\n## Verification\n\n- [x] `cargo +stable test --locked --no-run` — all targets/tests compile on Windows x64, Rust 1.99.\n- [x] `cargo +stable test --locked` — 385 passed, zero failed, one ignored live-network test.\n- [x] Upstream WASM tests: framing and missing-module handling pass. Key generation/agreement self-skip because the Zalo WASM module is absent; no real phone-sync claim.\n- [x] Final rebuilt v0.1.0 debug binary with PATH limited to Windows system tools: bundled native codec encoded/decoded 10/10 frames at 320×240, without external tools. This is a codec smoke check, not a real-call latency measurement.\n- [x] Latest main is an ancestor; rebase conflicts resolved; `git diff --check` passes.\n- [x] Final `coderabbit review --plain` against `origin/main`: no new findings. The retained historical reply/forward finding is fixed and verified against the current owner-transition handlers.\n- [x] Regressions cover wipe barriers, stale generations, cached-history ownership, failed-switch recovery, failed TUI logout/retry, group fallback cancellation, interleaved video SSRCs and current-schema reads during a writer.\n- [x] Prior legacy-import concurrency stress: 10/10 repeated runs pass; account fixture suite passes five repeated runs. Focused formatting and patch whitespace checks pass.\n- [ ] Strict Clippy and repository-wide formatting — Clippy stops at the unchanged `build.rs:123` `manual_is_multiple_of` lint under Rust 1.99. Existing formatting findings remain; no unrelated cleanup included.\n- [ ] Dependency audit — earlier advisories remain documented; this is not a clean release gate.\n- [ ] Physical camera checks, real-peer direct/relay/group calls, latency, repeated calls, Linux and both macOS architectures.\n\n## Risk & Rollback\n\n- Risk: two-way door for runtime changes; back up a local profile before evaluating account-history migration.\n- Blast radius: call video, worker lifetime, account/history storage and message presentation. Attachment playback and screen sharing remain outside the video change.\n- The upstream merge-conflict blocker is resolved. Hardware, interoperability, cross-platform and distribution checks remain release gates; this work does not claim release readiness.\n- Rollback: revert the PR changes and restore the pre-test profile backup when necessary; do not copy live SQLite/WAL files or reassign account-owned history.\n\n## Context\n\n- Resolves: the merge-conflict blocker on this PR; no unrelated issue closure requested.\n- ADRs: [native video](https://github.com/Leoaxo098/ctdozlite/blob/codex/native-video-v009/docs/adr/0005-native-call-video.md), [account ownership](https://github.com/Leoaxo098/ctdozlite/blob/codex/native-video-v009/docs/adr/0004-account-history-and-sync-failure-policy.md), [v0.1.0 integration](https://github.com/Leoaxo098/ctdozlite/blob/codex/native-video-v009/docs/adr/0006-native-video-on-v010.md).\n- Earlier v0.0.9 verification remains historical. The integration ADR records the rebased baseline; `re-notes/40` and `re-notes/41` distinguish historical results from the current review fixes and verification.\n\n\n<!-- This is an auto-generated comment: release notes by coderabbit.ai -->\n<!-- coderabbit-ai-generated:v1 -->\n\n## Summary by CodeRabbit\n\n* **New Features**\n  * Video calls now use native camera capture and integrated H.264 encoding and decoding; FFmpeg is no longer required.\n  * Chat history, scheduled messages, and account data are isolated by account, with safer account switching and history import.\n  * Added clearer camera setup and troubleshooting guidance, plus video-library license notices.\n* **Bug Fixes**\n  * Improved recovery from interrupted video streams and handling of camera permissions and unavailable devices.\n  * Prevented history and previews from changing outgoing message delivery status.\n  * Improved scheduled-message handling and prevented access to account history when logged out.\n* **Documentation**\n  * Updated build requirements to Rust 1.97+ and added architecture, migration, and release-readiness notes.\n<!-- end of auto-generated comment: release notes by coderabbit.ai -->",
      "closedAt": "2026-11-15T09:01:04Z",
      "createdAt": "2026-11-10T07:22:49Z",
      "labels": [],
      "number": 129,
      "state": "CLOSED",
      "title": "feat(media): replace FFmpeg call video with native capture and bundled H.264"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjc1OTMxMzE=",
        "is_bot": false,
        "login": "gr219",
        "name": "Tuyến Trần"
      },
      "body": "## What\n- Composer attachment queue (up to 50 files, same limit as Zalo PC): paste, drag-and-drop, screenshots and pickers now **append** instead of replacing the previous file.\n- `Ctrl+V` pastes every copied file — Explorer `CF_HDROP` (all entries, any type), Finder (NSURL via JXA), Linux `text/uri-list` — not only the first image.\n- Attach button opens **Chọn File / Chọn Thư mục**; file and image dialogs are multi-select; folder pick queues the visible top-level files (skips hidden, empty, subfolders).\n- Queue strip above the composer: thumbnails or file-type tiles, click a tile to drop it, `Esc` / \"Hủy tất cả\" to clear, `Enter` to send.\n- Two or more still photos in one send go out as a single **album** (shared `groupLayoutId`).\n- Incoming / stored album photos are drawn as one **grid bubble** (2–3 columns, center-cropped, click a tile to zoom).\n- Settings → download folder now uses a real folder picker instead of \"pick a file, take its parent\".\n\n## Why\n- Official Zalo lets users paste/attach many files at once; ctdozlite kept a single `Option` slot, so each paste overwrote the last one and only one file could be picked.\n- Photos sent one by one show as separate bubbles on the recipient's phone; albums match the official client.\n\n## How\n- New `src/gui/attachments.rs`: native multi-select/folder dialogs (Win32 `GetOpenFileNameW` + `OFN_ALLOWMULTISELECT`, `SHBrowseForFolderW`; `osascript`; zenity/kdialog), clipboard file lists, folder listing, preview preparation, album slot planning. The old single-file `pick_native_file_dialog` and the `CF_HDROP` branch of the clipboard-image reader are removed.\n- `WorkerCmd::PickAndSendAttachment` → `PickAttachments` / `PrepareAttachments` (thumbnails on a blocking worker thread, never the UI thread) / `SendAttachments` (sequential, preserves order; caption on the first file).\n- Album send: `chat::MediaGroupLayout` adds `groupLayoutId`, `isGroupLayout`, `idInGroup`, `totalItemInGroup`, `extMsgProp` to `photo_original/send` — field set from zca-js `sendMessage.ts`. Upload stage unchanged. GIFs, videos and files stay standalone. `send_attachment_ext` keeps its signature (delegates with no layout).\n- Album display: `MediaGroupLayout::from_content` reads `group_layout_id` / `is_group_layout` / `id_in_group` from incoming `content.params`; stored as `ChatMessage::media_group` (serde-optional) and persisted in the `raw_json` metadata column — no schema change. `album_run_lengths` folds consecutive same-sender photos of one album into the first bubble.\n- Evidence and field sources: `re-notes/38_photo_album_group_layout.md`.\n\n## Verification\n- [x] `cargo test --locked` — 252 lib tests pass (15 new: dialog-buffer/uri-list parsing, folder listing, queue cap/dedupe, album slots, layout params in/out, album runs, grid columns/crop), plus all integration tests.\n- [x] `cargo check --all-targets --target x86_64-pc-windows-gnu` passes.\n- [ ] `cargo clippy --all-targets -- -D warnings` — not clean on `main` today; diffed before/after: no new warning kinds. `render_message_body` / `render_chat_bubble` were already over `too_many_arguments` and gained one parameter each.\n- [x] Manual (Windows 11, portable gnu build): multi-select files, folder pick, repeated `Ctrl+V` of Explorer files, queue + send.\n- [x] Manual: several photos sent from ctdozlite show as **one album** on the official Zalo mobile client.\n- [ ] Manual: album grid rendering inside ctdozlite (not yet clicked through).\n- [ ] macOS (JXA clipboard, `choose file … multiple selections`) and Linux (zenity `--multiple`, `wl-paste`/`xclip` uri-list) paths compile but are untested.\n\n## Risk & Rollback\n- Behaviour change: pickers and drag-and-drop now queue files instead of sending immediately (press `Enter` to send) — intentional, lets users review before sending a whole folder.\n- Protocol: album fields are additive on `photo_original/send`; not yet confirmed with a Zalo PC capture. Rollback = stop passing a layout in `SendAttachments` (single call site) to return to per-photo sends.\n- Photos stored before this change have no `media_group` and keep rendering one by one.\n- `cargo fmt` was not run repo-wide: `main` is not fmt-clean and it would touch ~30 unrelated files.\n\n\n<!-- This is an auto-generated comment: release notes by coderabbit.ai -->\n<!-- coderabbit-ai-generated:v1 -->\n\n## Summary by CodeRabbit\n\n* **New Features**\n  * Select multiple images or files, attach files from a folder, or add items from the clipboard or by dragging and dropping. Review or remove queued attachments, clear the queue, or cancel it with Escape. Queues support up to 50 files.\n  * Send eligible image batches as albums with captions on the first image. Received albums appear in an organized thumbnail grid.\n* **Bug Fixes**\n  * Pasting multiple files now adds them to the queue without replacing earlier attachments, including non-image files.\n<!-- end of auto-generated comment: release notes by coderabbit.ai -->",
      "closedAt": "2026-11-15T08:59:21Z",
      "createdAt": "2026-11-09T03:35:53Z",
      "labels": [],
      "number": 127,
      "state": "MERGED",
      "title": "feat(ui): attach many files and send photos as one album"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjc1OTMxMzE=",
        "is_bot": false,
        "login": "gr219",
        "name": "Tuyến Trần"
      },
      "body": "## What\n- The friend list (\"Danh sách bạn bè\") and the sidebar now show the alias (tên gợi nhớ) the user set for a contact on the phone or official Zalo app, the same name \"Nhật Ký\" already shows.\n- `SidebarPresentation::contact_name` marks names from the friend list (`getfriends` `displayName`). These names win in `merge_thread_into` and are no longer overridden by `hydrate_threads`.\n- `upsert_thread` now stores the latest real name. Placeholders (`''`, `Người dùng …`, `Nhóm …`, all-digit IDs) never overwrite a stored name.\n- `displayName` is preferred over `zaloName` in `resolve_thread_names`, the group avatar slots and the Zalo PC `Index.db` import.\n\n## Why\n- `getfriends` already returns the alias: `displayName` is the alias and `zaloName` is the owner's own name. But three layers dropped it:\n  1. `upsert_thread` never replaced a stored real name, so the first name ever saved for a thread (e.g. the sender's `dName` from a message) stuck forever.\n  2. `hydrate_threads` then overwrote the fresh `getfriends` name with that stale stored name.\n  3. In the GUI, `ThreadsUpdated` → `merge_thread_into` only replaced placeholder names, so the name loaded from the cache never got updated either.\n- \"Nhật Ký\" was correct because it uses `displayName` from the feed API directly, without going through the DB.\n\n## How\n- Friend-list names are authoritative for 1-1 threads: `parse_thread` sets `contact_name`, and the worker merges that copy `ft.name` also copy the flag. Every other source keeps the old rule (fill placeholders only), so message, cache and preview names still can't overwrite a real name.\n- The DB now mirrors the in-memory name, so stale names self-heal on the next friend-list load. No migration is needed.\n\n## Verification\n- [x] `cargo test --locked`: all pass (lib: 236 passed, 1 ignored), on Linux (Docker, rust 1.97). New tests:\n  - `friend_list_alias_wins_over_stale_names`: `parse_thread` and `merge_thread_into`; placeholders are still filled by non-contact names.\n  - `stored_thread_name_tracks_renames_but_never_overrides_friend_list_names`: `upsert_thread` and `hydrate_threads`.\n- [x] `cargo clippy --all-targets -- -D warnings`: no errors on the changed lines. The existing repo-wide errors are unchanged.\n- [x] `cargo fmt --check`: no diffs on the changed lines. The two hunks reported in `chat.rs` are pre-existing on `main`.\n- [x] Manual, live account (a read-only probe on a copy of the data dir, not included in this PR):\n  - For a contact with an alias, `getfriends` returned `displayName` = alias and `zaloName` = owner's name.\n  - `chat::get_friends` returned the owner's name before this change and the alias after it.\n- [x] Windows: a MinGW cross-build of this branch started on the same data dir (`--version` OK). I haven't clicked through the friend list in that build.\n- [ ] Not run on macOS or with the official MSVC build.\n\n## Risk & Rollback\n- Risk: two-way door. No schema change: `contact_name` is a new `#[serde(default)]` field, so old caches load with `false`.\n- Blast radius: thread and contact names in the sidebar, the contact list and stored threads. A group or contact rename now also updates the stored name; previously the first stored name stuck.\n- Rollback: revert this commit.\n\n## Context\n- Resolves: None (no existing issue)\n- Relates to: None\n- ADR / RFC: None\n\n\n<!-- This is an auto-generated comment: release notes by coderabbit.ai -->\n<!-- coderabbit-ai-generated:v1 -->\n\n## Summary by CodeRabbit\n\n* **Bug Fixes**\n  * Friend lists and conversations now show a contact’s display name when available, rather than an outdated name from a message or saved data.\n  * Real contact names are preserved when conversation details refresh, and saved names can be updated when a real name becomes available.\n  * Generic placeholder names no longer replace real contact names.\n  * Profile names now prefer the display name, falling back to the alternate name when no display name is available.\n<!-- end of auto-generated comment: release notes by coderabbit.ai -->",
      "closedAt": "2026-11-11T18:29:11Z",
      "createdAt": "2026-11-09T02:21:25Z",
      "labels": [],
      "number": 125,
      "state": "CLOSED",
      "title": "fix(chat): show contact aliases in the friend list"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjg4NzMzNTAy",
        "is_bot": false,
        "login": "TIPC1110",
        "name": "ctdo"
      },
      "body": "Fixes #119.\n\n- Add last_read_msg_id column to threads table with migration and ON CONFLICT preservation\n- Persist read-point to active account SQLite on thread selection\n- Restore last_seen_id during thread hydration\n- Don't clear unread counts on session refresh under the same account\n- Only count incoming messages newer than last_seen_id for newly loaded threads\n- Add unit test verifying read-point roundtrip and re-upsert preservation\n\n<!-- This is an auto-generated comment: release notes by coderabbit.ai -->\n\n<!-- coderabbit-ai-generated:v1 -->\n\n## Summary by CodeRabbit\n\n* **New Features**\n  * Read progress is now saved per conversation, so reopening the app preserves where you last read.\n  * Selecting a conversation with messages updates its saved read position.\n* **Bug Fixes**\n  * Unread counts now exclude incoming messages at or before the saved read position; outgoing messages are not counted.\n  * Refreshing a logged-in session preserves conversations, unread counts, and the selected conversation instead of clearing them.\n  * Conversations without a saved read position continue to count all incoming messages as unread.\n\n<!-- end of auto-generated comment: release notes by coderabbit.ai -->",
      "closedAt": "2026-11-08T12:46:41Z",
      "createdAt": "2026-11-08T12:41:34Z",
      "labels": [],
      "number": 124,
      "state": "MERGED",
      "title": "fix(unread): persist thread read-point to prevent recount on app reopen (#119)"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjMxODg0NDg3",
        "is_bot": false,
        "login": "hoanglong149",
        "name": ""
      },
      "body": "# PR body — feat/reaction-noti-preview → ctdoteam1110:main\n\n## feat: OS banner, reaction unread, preview file, vanish, voice, UI Zalo + fix 13 issues (trên v0.0.9)\n\n15 commit, đối chiếu hành vi Zalo PC chính thức 26.11.10. Giữ nguyên incoming_popup + chime + reaction_inbox của upstream.\n\n### Đợt 1 — notification & preview\n1. **Preview file đa dạng** (`c11b030`): video thumbnail inline (CDN/ffmpeg); voice bubble ▶ + duration, mở player ngoài; doc-card phân loại PDF/Word/Excel/PPT + Mở/Lưu. Không dep mới.\n2. **OS banner ngoài app** (`2b72e46`, `74f183c`): module `os_notify` dùng chung; tin mới (kể cả thread mới toanh) → unread + popup in-app (upstream) + OS banner + chime. Fix thread mới `push` trơ không đếm.\n3. **Reaction** (`2b72e46`): banner riêng \"X đã bày tỏ cảm xúc…\" + `reaction_unread_counts` với pill ❤+n sidebar, clear khi mở thread.\n\n### Đợt 2 — vanish, voice, UI\n4. **Tin tự xoá local-only** (`899ef66`→`e8652bc`): bật/tắt theo thread + 1h/1d/7d/14d; badge ⏳ + countdown; hết hạn → \"[Tin nhắn đã biến mất]\"; persist SQLite + sweep rẻ. LOCAL-ONLY (không có API server).\n5. **Gửi ghi âm** (`5cbe668`): nút mic 🎤 + strip thu (timer/meter/Gửi/Huỷ); cpal → MP3 rusty_mp3 CBR 64k; upload đúng flow `sendVoice` (`voicemp3`/`sendmp3`). Dep mới duy nhất: `rusty_mp3`.\n6. **UI token Zalo PC** (`d538fbd`, visual-only): light #0068FF/#E5F1FF, dark #1D2025/#3384FF; bubble 12; bỏ indicator xanh; badge unread xanh; nút Gửi pill.\n\n### Đợt 3 — fix issues cộng đồng\n7. **Folder picker** (`cd4f81a`, #106 #91): `pick_native_folder_dialog` riêng (Win/macOS/Linux) cho download dir + nút migrate cả thư mục.\n8. **Multi-account** (`37c9a39`, #103): nút \"Hủy, quay lại tài khoản trước\" ở login; add-account không gửi Logout; Logout không xóa `zalo_history.db`.\n9. **Download reveal** (`bdf426c`, #102): tải xong mở thư mục chứa file (open -R / explorer /select / xdg-open cha).\n10. **Mở link** (`925b6ca`, #107): check ShellExecuteW + fallback `cmd /C start`, normalize link không scheme, toast lỗi.\n11. **Sync V2 ngắn hạn** (`08b2f7d`, #108 #116 #87): xóa khóa giả (fail rõ thay vì request rác); viết lại helper JS (syntax, path động, multi-chunk, unpack zstd/gzip/zlib); session gate theo sync_id; decompress Rust đa chuẩn. Giới hạn: cần glue WASM đúng/wasmtime mới chạy end-to-end.\n12. **Link/ảnh/unread** (`856aca9`, #111 #118 #119): link card hiện URL rút gọn click được + copy; ảnh ưu tiên msgType + mở rộng CDN (jxl/heic/avif, zpsocial); unread persist read-point SQLite + bỏ clear-recount khi refresh.\n\n### Verify\n- `cargo test`: 241 lib + 14 integration passed, 0 failed.\n- `cargo clippy --all-targets`: +1 (too-many-args 13→14 do param vanish — chấp nhận).\n- GUI khởi chạy OK. `node --check` helper JS OK.\n\n\n<!-- This is an auto-generated comment: release notes by coderabbit.ai -->\n<!-- coderabbit-ai-generated:v1 -->\n\n## Summary by CodeRabbit\n\n- **New Features**\n  - Record, send, and play voice messages, with microphone-level and recording-time indicators.\n  - Set local disappearing-message timers for conversations, with expiry labels and remaining-time indicators.\n  - Translate messages and choose a preferred translation language.\n  - Add group notes and keyword watches, review matching messages, and view group activity statistics.\n  - View desktop notifications for new messages and reactions, and see reaction unread counts.\n  - Import chat history from a selected data folder; open downloaded files in the system file manager.\n  - See richer attachment previews and actions for audio, video, documents, and links.\n  - Cancel adding an account and return to the previous account.\n\n- **Improvements**\n  - Improved history-sync compatibility with additional compression formats and more reliable processing.\n  - Updated chat colors, layouts, and unread indicators.\n  - Chat history is now kept separate for each account.\n  - Incoming calls remain passive until answered.\n<!-- end of auto-generated comment: release notes by coderabbit.ai -->",
      "closedAt": "2026-11-11T18:28:24Z",
      "createdAt": "2026-11-08T04:42:48Z",
      "labels": [],
      "number": 121,
      "state": "MERGED",
      "title": "feat: OS banner, reaction unread, preview file, vanish, voice, UI Zalo + fix 13 issues"
    },
    {
      "author": {
        "id": "U_kgDOCvFoNQ",
        "is_bot": false,
        "login": "lebad280324",
        "name": "Lê Bá Dũng"
      },
      "body": "<!--\nTier guide (see docs/standards/git-flow-and-pr-matrix.md):\n- Tier 1 (docs/typo/deps): keep What + Why, delete the rest.\n- Tier 2 (feature/bugfix/refactor): fill every section, tick the checks you ran.\n- Tier 3 (protocol/schema/crypto/breaking): add ADR link, evidence and rollback detail.\nUse concise bullets, not paragraphs. Never paste session.json, cookies, zpw_sek/zpw_enk, IMEI or raw captures.\n-->\n\n## What\n-\n\n## Why\n-\n\n## How\n-\n\n## Verification\n- [ ] `cargo test --locked`\n- [ ] `cargo clippy --all-targets -- -D warnings`\n- [ ] `cargo fmt --all --check`\n- [ ] Manual: <GUI/TUI, OS, flow, observed error_code>\n\n## Risk & Rollback\n- Risk: two-way door / one-way door\n- Blast radius:\n- Rollback:\n\n## Context\n- Resolves: None\n- Relates to:\n- ADR / RFC: None\n\n\n<!-- This is an auto-generated comment: release notes by coderabbit.ai -->\n<!-- coderabbit-ai-generated:v1 -->\n\n## Summary by CodeRabbit\n\n* **New Features**\n  * Added a free community build for Apple Silicon Macs, with a refreshed glass-style interface.\n  * Added global search, conversation details, emoji and recent-sticker pickers, group member and reaction views, contact aliases, pinned topics, and group mentions.\n  * Added previews for supported attachments, access to media already downloaded by Zalo PC, richer Official Account content, and native macOS video support.\n  * Phone-history sync remains experimental and may import only part of the history.\n* **Bug Fixes**\n  * Improved message recall, update package selection, and recovery when an app update fails.\n  * Kept automatic history refresh separate from phone sync to reduce interference.\n* **Documentation**\n  * Added macOS installation, update, and troubleshooting guidance.\n<!-- end of auto-generated comment: release notes by coderabbit.ai -->",
      "closedAt": "2026-11-15T09:01:00Z",
      "createdAt": "2026-11-06T17:40:40Z",
      "labels": [],
      "number": 113,
      "state": "CLOSED",
      "title": "Codex/macos arm64 public release"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjgwOTM2MjAw",
        "is_bot": false,
        "login": "cuongdm75",
        "name": ""
      },
      "body": "## What\n- **R1 (Download Directory)**: Replaces file picker with native Windows COM folder dialog (`IFileOpenDialog` with `FOS_PICKFOLDERS`, `SHBrowseForFolderW` / PowerShell fallbacks). Auto-creates missing target directories and displays full directory tooltips.\n- **R2 (UI Customization)**: Adds direct visual theme customizer in Appearance settings:\n  - Font family selector (`Be Vietnam Pro`, `Segoe UI`, `Arial`, `Tahoma`) with full Vietnamese glyph fallbacks.\n  - UI scaling slider (80%..=140%) with quick preset buttons (90%, 100%, 110%, 125%).\n  - Native color pickers (`color_edit_button_srgba`) for primary, accent, and chat bubble background colors with 7 one-click presets.\n  - Live chat bubble preview box with automatic WCAG AAA contrast ratio calculation.\n  - Immediate live reload without manual JSON editing, persisted to `app_settings.json`.\n- **R3 (Message Quote & Reply)**:\n  - Supports double-click on message bubbles or right-click context menu (\"Trả lời\") to initiate quote-reply.\n  - Auto-focuses composer text input; quick cancel via `Escape` key or `✕` button.\n  - Composer banner shows sender name and smart media indicators (`[Hình ảnh]`, `[Nhãn dán]`, `[Tập tin]`).\n  - Chat bubble renders an elegant quote preview block with accent border and jump-to-original scroll on click.\n  - Syncs quote metadata (`reply_to_id`, `reply_to_sender`, `reply_to_text`) through Zalo API and local SQLite storage.\n- **R4 (Message Forwarding)**:\n  - Adds context menu \"Chuyển tiếp\" opening a dedicated thread/contact selection modal.\n  - Case-insensitive real-time search across contacts and group chats with empty state feedback.\n  - Pressing `Escape` cancels/dismisses the modal.\n  - Preserves original media URLs (`msg.link_url`) in `src/chat.rs` when forwarding photos, videos, and files instead of sending placeholder text.\n  - Shows Toast confirmation upon successful forwarding.\n- **R5 (Testing & Verification)**:\n  - Added 11 automated test suites in `tests/` covering download settings, theme live updates, quote-reply interactions, and message forwarding stress tests.\n\n## Why\n- Resolves issue where users could not modify the default download folder from the General Settings modal due to path truncation and file-selection dialog.\n- Enables end-users to customize fonts, scaling, and theme colors directly from the GUI without manually editing `app_settings.json`.\n- Fulfills essential messaging capabilities: quoting messages with direct reply context and forwarding text/media to other users or groups.\n\n## How\n- **src/gui/modals.rs**: Implemented pick_native_folder_dialog(), wired directory update logic with create_dir_all, added font/color controls to Settings modal, and built the Forward modal with search filtering.\n- **src/gui/theme.rs**: Extended ThemeConfig and AppSettings with font family, UI scale, and custom RGBA colors; added WCAG luminance calculation for auto text contrast.\n- **src/gui/messages.rs & src/gui/workspace.rs**: Implemented quote reply state management, double-click message handling, composer banner preview, jump-to-message scrolling, and keybind listener (Escape).\n- **src/chat.rs**: Fixed media forwarding logic to retain file and image download URLs in forward payloads.\n- **\tests/**: Added 11 comprehensive unit and adversarial integration test suites.\n\n## Verification\n- [x] cargo test --locked (305+ tests passed, 0 failed)\n- [x] cargo clippy --all-targets -- -D warnings (clean, 0 warnings)\n- [x] cargo check --locked (clean)\n- [x] Manual: Tested on Windows 11 — folder picker dialog opens and persists; theme color/font changes apply live; double-click quotes and reply scrolls to origin; forwarding forwards actual media attachments with toast notifications.\n\n## Risk & Rollback\n- Risk: Two-way door. All modifications are additive UI enhancements and backward-compatible with existing ^Gpp_settings.json schemas (gracefully falling back to defaults if keys are missing).\n- Blast radius: GUI layer (src/gui/) and message forwarding logic in src/chat.rs. Core protocol cipher, auth sessions, and SQLite store schemas remain untouched.\n- Rollback: Revert the commit on this branch.\n\n## Context\n- Resolves: None\n- Relates to: #105\n- ADR / RFC: None\n\n<!-- This is an auto-generated comment: release notes by coderabbit.ai -->\n<!-- coderabbit-ai-generated:v1 -->\n\n## Summary by CodeRabbit\n\n* **New Features**\n  * Customize fonts, interface scaling, and theme colors, with a live message-bubble preview.\n  * Search conversations by name or ID when forwarding messages; previews reflect content such as photos, files, and links.\n  * Choose a download folder with a native folder picker and receive a warning if it isn’t writable.\n* **Improvements**\n  * Replies show clearer quote details and sender names, with easier reply controls.\n  * Forwarded messages better preserve media, stickers, links, and message text.\n* **Bug Fixes**\n  * Improved message-bubble double-click handling and quote display when message details are incomplete.\n<!-- end of auto-generated comment: release notes by coderabbit.ai -->",
      "closedAt": "2026-11-11T18:29:15Z",
      "createdAt": "2026-11-05T08:18:55Z",
      "labels": [],
      "number": 105,
      "state": "CLOSED",
      "title": "feat(ui): add native download folder picker, direct UI customizer, message quote-reply, and forward dialog"
    },
    {
      "author": {
        "id": "U_kgDOCvFoNQ",
        "is_bot": false,
        "login": "lebad280324",
        "name": "Lê Bá Dũng"
      },
      "body": "<!--\nTier guide (see docs/standards/git-flow-and-pr-matrix.md):\n- Tier 1 (docs/typo/deps): keep What + Why, delete the rest.\n- Tier 2 (feature/bugfix/refactor): fill every section, tick the checks you ran.\n- Tier 3 (protocol/schema/crypto/breaking): add ADR link, evidence and rollback detail.\nUse concise bullets, not paragraphs. Never paste session.json, cookies, zpw_sek/zpw_enk, IMEI or raw captures.\n-->\n\n## What\n-\n\n## Why\n-\n\n## How\n-\n\n## Verification\n- [ ] `cargo test --locked`\n- [ ] `cargo clippy --all-targets -- -D warnings`\n- [ ] `cargo fmt --all --check`\n- [ ] Manual: <GUI/TUI, OS, flow, observed error_code>\n\n## Risk & Rollback\n- Risk: two-way door / one-way door\n- Blast radius:\n- Rollback:\n\n## Context\n- Resolves: None\n- Relates to:\n- ADR / RFC: None\n\n\n<!-- This is an auto-generated comment: release notes by coderabbit.ai -->\n\n<!-- coderabbit-ai-generated:v1 -->\n\n## Summary by CodeRabbit\n\n* **New Features**\n  * macOS now offers an optional Liquid Glass appearance, with translucent windows and interface elements.\n* **Bug Fixes**\n  * Improved mobile backup imports by better identifying outgoing messages and correcting message direction when reliable sender information is available.\n  * Sync operations now report an error when valid keys cannot be generated, rather than continuing with placeholder keys.\n  * Improved compatibility when locating the components needed to load sync data, including in packaged macOS apps.\n\n<!-- end of auto-generated comment: release notes by coderabbit.ai -->",
      "closedAt": "2026-11-15T09:00:57Z",
      "createdAt": "2026-11-05T06:29:01Z",
      "labels": [],
      "number": 104,
      "state": "CLOSED",
      "title": "Feat/liquid glass"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjc1OTMxMzE=",
        "is_bot": false,
        "login": "gr219",
        "name": "Tuyến Trần"
      },
      "body": "## What\n- `native_notifications::invoke` now spawns the helper through a new `spawn_helper`. It retries up to 5 times (10, 20, … 50 ms backoff) when spawning fails with `ErrorKind::ExecutableFileBusy` (`ETXTBSY`).\n- Every other spawn error still maps to `HelperUnavailable`.\n\n## Why\n- The `Cargo Test Suite (Linux x86_64)` job failed on `main` at `477d6de` (a README-only commit) in [run 41862925699](https://github.com/ctdoteam1110/ctdozlite/actions/runs/41862925699):\n  - `mock_executable_timeout_and_missing_helper` returned `Err(HelperUnavailable)` where `Err(Timeout)` was expected.\n  - The same code passed on `7e409ee`, which points to a flaky test.\n- Root cause: the mock-helper tests write a shell script and execute it immediately. If another test thread forks at that moment, the child briefly inherits the script's write handle until it reaches `exec`. During that window, `execve` on the script fails with `ETXTBSY` (\"Text file busy\").\n- Reproduced locally by logging the spawn error: `Os { code: 26, kind: ExecutableFileBusy, message: \"Text file busy\" }`, 2 failures in 300 runs of `native_notifications::tests::mock`. Any of the `mock_executable_*` tests can hit it, not just the timeout one.\n\n## How\n- Retry only on `ExecutableFileBusy`, which is a transient condition. A real missing or broken helper still fails on the first attempt.\n- Behaviour for the packaged macOS helper is otherwise unchanged; the retry only adds up to about 150 ms in the busy case.\n\n## Verification\n- [x] `cargo test --locked`: all pass (lib: 204 passed, 1 ignored). Run on Linux (Docker, rust 1.91).\n- [x] Stress test: `native_notifications::tests::mock` run 1000×, 0 failures (before: 2/300). Full lib test binary run 30×, 0 failures.\n- [x] `cargo clippy`: no warnings in the changed code. The repo-wide `-D warnings` errors are pre-existing on `main`.\n- [x] `rustfmt --check src/native_notifications.rs`: clean.\n- [ ] Manual: not applicable (test harness / process-spawn path only). Not run on macOS.\n\n## Risk & Rollback\n- Risk: two-way door.\n- Blast radius: `native_notifications` helper spawning only (`status` / `request_if_user_action`).\n- Rollback: revert this commit.\n\n## Context\n- Resolves: None\n- Relates to: #85 (the CI on `main` has been red since v0.0.8, which also blocks a release that includes the #75 fix)\n- ADR / RFC: None\n\n\n<!-- This is an auto-generated comment: release notes by coderabbit.ai -->\n\n<!-- coderabbit-ai-generated:v1 -->\n\n## Summary by CodeRabbit\n\n* **Bug Fixes**\n  * Notification requests now retry when the notification service is temporarily busy, with a short wait between attempts. This can help notifications proceed during brief periods of launch contention. Requests still report the service as unavailable if it cannot be started after the retries or encounters another startup error; notification output and completion handling are unchanged.\n\n<!-- end of auto-generated comment: release notes by coderabbit.ai -->",
      "closedAt": "2026-11-06T16:19:09Z",
      "createdAt": "2026-11-05T00:39:34Z",
      "labels": [],
      "number": 99,
      "state": "MERGED",
      "title": "fix(harness): retry notification helper spawn on ETXTBSY"
    },
    {
      "author": {
        "id": "U_kgDOBeZLAA",
        "is_bot": false,
        "login": "Leoaxo098",
        "name": ""
      },
      "body": "## What\n- Respect group posting restrictions and administrator roles.\n- Show a localized notice when members cannot send messages.\n\n## Why\n- Match Zalo’s group chat behavior when only the owner and deputies can post.\n\n## How\n- Read `lockSendMsg` and group administrator metadata during group refresh.\n- Hide the composer for restricted members; show a Vietnamese or English notice based on the selected language.\n\n## Verification\n- [x] Library tests: 190 passed, 1 ignored.\n- [ ] `cargo test --locked` — full suite not completed.\n- [ ] `cargo clippy --all-targets -- -D warnings`\n- [ ] `cargo fmt --all --check`\n- [ ] Manual: Windows GUI, restricted group; verify members see the notice and owners/deputies can still compose. Live verification not performed.\n\n## Risk & Rollback\n- Risk: two-way door; restriction display depends on group metadata.\n- Blast radius: group chat composer in the desktop GUI.\n- Rollback: revert the commit.\n\n## Context\n- Resolves: None\n- Relates to: Zalo’s `lockSendMsg` group setting.\n- ADR / RFC: None\n\n<!-- This is an auto-generated comment: release notes by coderabbit.ai -->\n\n<!-- coderabbit-ai-generated:v1 -->\n\n## Summary by CodeRabbit\n\n* **New Features**\n  * Group posting restrictions are now reflected in the chat workspace. When a group allows only owners and deputies to send messages, other members see a notice and the message composer is hidden.\n  * Owners and deputies retain access to the composer. Restricted members also no longer see contextual sticker suggestions or link previews in that workspace.\n\n<!-- end of auto-generated comment: release notes by coderabbit.ai -->",
      "closedAt": "2026-11-06T16:48:50Z",
      "createdAt": "2026-11-04T14:27:49Z",
      "labels": [],
      "number": 97,
      "state": "MERGED",
      "title": "feat(chat): lock composer for restricted group members"
    },
    {
      "author": {
        "id": "U_kgDOBeZLAA",
        "is_bot": false,
        "login": "Leoaxo098",
        "name": ""
      },
      "body": "## What\n\n- Add local Zalo Desktop V2 backup import: decrypt and validate archives, merge messages, and restore matched media in conversations.\n- Recover account keys locally; preserve existing history and make repeat imports safe.\n\n## Why\n\n- Let users recover backed-up Zalo history and attachments in ctdozlite without exposing account keys or private backup data.\n\n## How\n\n- Stream and validate encrypted archives before a transactional SQLite merge.\n- Match conversations and messages by identity while retaining richer existing records.\n- Recover keys locally and connect matched media to conversation display and file-opening flows.\n- Add coverage for archive safety, identity matching, duplicate imports, preservation, and rollback.\n\n## Verification\n\n- [ ] `cargo test --locked`\n- [ ] `cargo clippy --all-targets -- -D warnings`\n- [ ] `cargo fmt --all --check`\n- [x] Manual: Windows, F: verification build; imported the supplied backup, reopened conversations, and checked sampled recovered images.\n\n## Risk & Rollback\n\n- Risk: two-way door; importer writes recovered history and media to the selected local profile.\n- Blast radius: local ctdozlite profile and imported media only; installed C: executable and remote Zalo account are unaffected.\n- Rollback: use the pre-import profile backup or restore the profile from its saved copy; remove the imported F: build if needed.\n\n## Context\n\n- Resolves: None\n- Relates to: None\n- ADR / RFC: None",
      "closedAt": "2026-11-15T09:00:55Z",
      "createdAt": "2026-11-04T13:29:01Z",
      "labels": [],
      "number": 96,
      "state": "CLOSED",
      "title": "feat(chat): import Zalo Desktop backups"
    },
    {
      "author": {
        "id": "U_kgDOCCmn_Q",
        "is_bot": false,
        "login": "TraiNguyenVan",
        "name": "Nguyễn Văn Trãi"
      },
      "body": "## Nội dung\n- Chuyển tiếp sự kiện preedit và commit của bộ gõ tiếng Việt trên Linux qua bản vá tích hợp `egui-winit`.\n- Xem sự kiện IME `Enabled` là thông báo tiêu điểm nhập liệu, không phải trạng thái đang ghép chữ.\n\n## Lý do\n- Trước đây, nội dung từ bộ gõ Linux không được chuyển đúng vào các ô nhập liệu; thao tác Enter cũng có thể bị chặn khi IME chỉ vừa được bật.\n\n## Cách thực hiện\n- Đưa `egui-winit 0.29.1` vào repository và chỉnh sửa để chuyển tiếp các sự kiện ghép chữ trên Linux.\n- Cập nhật trạng thái IME phía ứng dụng để chỉ bắt đầu ghép chữ khi nhận được preedit.\n\n### Trước khi sửa\n\nhttps://github.com/user-attachments/assets/71067ab6-b3d0-4c9b-8ffc-6db1d15c67b2\n\n### Sau khi sửa\n\nhttps://github.com/user-attachments/assets/2ddc9be3-ca7c-4b2a-b345-d9f92f206d36\n\n## Kiểm tra\n- [x] `cargo build --locked`\n- [x] `cargo test --locked` (239 test thành công, 1 test bị bỏ qua; chạy trước khi tách commit)\n- [x] Kiểm tra thủ công: người dùng xác nhận gõ tiếng Việt bằng IME trên Linux hoạt động.\n- [ ] `cargo clippy --all-targets -- -D warnings`\n\n## Rủi ro và hoàn tác\n- Thay đổi chuyển tiếp sự kiện chỉ áp dụng trên Linux; xử lý IME trên các hệ điều hành khác không đổi.\n- Hoàn tác commit này sẽ khôi phục hành vi `egui-winit` ban đầu.",
      "closedAt": "2026-11-11T18:28:24Z",
      "createdAt": "2026-11-04T08:19:47Z",
      "labels": [],
      "number": 93,
      "state": "MERGED",
      "title": "fix(gui): restore Linux IME composition events"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjYyNzk3",
        "is_bot": false,
        "login": "bqqhuy",
        "name": "henryb"
      },
      "body": "## What\nHarden the Windows `CF_DIB` / `CF_DIBV5` clipboard fallback with a small pure-Rust decoder and eight synthetic regression tests.\n\n## Why\nThe current reader advertises DIBV5 support but only probes/requests format 8 (`CF_DIB`), so a clipboard exposing only format 17 is missed. It also treats bitfield pixels as fixed BGRA, which swaps colors for alternative masks and mishandles 10-bit channels/alpha. Dimensions, planes and compression are not fully validated before allocating the destination bitmap.\n\n## How\n- Add a checked layout parser: at most 80 MiB input, 8192 per edge and 16 million pixels; checked stride/offset arithmetic, valid planes/compression, nonoverlapping contiguous masks and complete pixel data.\n- Handle top-down/bottom-up rows, padding, external/embedded masks, alpha normalization and optional truecolor palettes.\n- Prefer validated DIBV5, fall back to DIB, and unlock/close the clipboard before pixel conversion. Invalid format-17 data can fall back to format 8.\n- Keep PNG, copied-file and non-Windows behavior intact; no dependency/lockfile, protocol, schema, updater or packaging changes.\n\nThe decoder is adapted from the Windows ARM64 fork (`bqqhuy/ctdozlite`, v0.003-arm) and isolated from its platform/release choices. It also validates/skips optional palettes. This is complementary to #90: that PR addresses macOS/Linux paste detection and a macOS backend; this one changes the Windows DIB fallback.\n\n## Verification\n- [x] `cargo test --locked`: **247 passed, 0 failed, 1 ignored** on Linux, Rust 1.99.1.\n- [x] Eight new tests cover BGRX, bottom-up 24-bit padding, DIBV5 channel masks, 10-bit color/2-bit alpha, external masks, optional palettes, truncation/extreme dimensions and invalid compression/masks.\n- [x] Cross-compile check of the changed Win32 reader and decoder using an isolated `aarch64-pc-windows-msvc` harness. This was a component check, not a native Windows app build.\n- [x] New decoder passes rustfmt; `git diff --check` is clean.\n- [ ] Strict Clippy is not clean on current upstream: **139 errors on both unchanged bcecc40 and this branch**, with identical error-message multisets on Rust 1.99.1. No new errors are introduced; no unrelated lint sweep included.\n- [ ] Native clipboard test: paste a Windows screenshot, then a producer exposing DIBV5-only / non-BGRA masks; confirm preview/colors, cancellation and subsequent pastes.\n\n## Risk & Rollback\nWindows DIB fallback only. Malformed, oversized or unsupported bitmaps are rejected. This does not move the existing clipboard invocation off the UI thread, or bound the separate PNG/file decoders; those remain separate work. No proprietary resources or account data are included. Revert the commit to roll back.\n\n\n<!-- This is an auto-generated comment: release notes by coderabbit.ai -->\n\n<!-- coderabbit-ai-generated:v1 -->\n\n## Summary by CodeRabbit\n\n- **Bug Fixes**\n  - Improved Windows clipboard image support, including images in the DIBV5 format.\n  - Added safeguards for oversized or malformed bitmap data and improved handling of color, transparency, and image orientation.\n\n<!-- end of auto-generated comment: release notes by coderabbit.ai -->",
      "closedAt": "2026-11-06T17:02:51Z",
      "createdAt": "2026-11-04T06:36:40Z",
      "labels": [],
      "number": 92,
      "state": "CLOSED",
      "title": "fix/upstream-clipboard-dibfix(ui): validate Windows clipboard DIB layouts"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjQwNTUzMjgz",
        "is_bot": false,
        "login": "zt-hky",
        "name": "Kyle Nguyễn"
      },
      "body": "## What\n- `Cmd+V` dán ảnh từ Clipboard vào khung chat trên macOS (ảnh chụp màn hình, \"Copy Image\" từ trình duyệt/Preview, file ảnh copy trong Finder) → hiện thanh xem trước có sẵn, nhập chú thích, `Enter` để gửi.\n- Menu chuột phải khung chat hiển thị `Cmd+V` trên macOS.\n\n## Why\n- `egui-winit` 0.29.1 (`on_keyboard_input`) chỉ phát `Event::Paste` khi Clipboard có **text**, rồi `return` — sự kiện nhấn `V` bị nuốt, chỉ còn sự kiện nhả. Với Clipboard chỉ chứa ảnh, `is_clipboard_paste_triggered` không bao giờ kích hoạt ngoài Windows (Windows đã có workaround `GetAsyncKeyState`).\n- `read_clipboard_image_to_png` không có backend macOS (chỉ gọi `wl-paste`/`xclip`) nên luôn trả `None`.\n\n## How\n- `PasteKeyTracker` (`src/gui/messages.rs`): phát hiện sự kiện nhả `V` không có cặp nhấn khi `command` đang giữ (hoặc vừa nhả ≤ 0,5 s). Điều kiện `command` bắt buộc để IME Telex (#71) nuốt `v` thường không dán nhầm ảnh. Áp dụng cho mọi nền tảng non-Windows → Linux `Ctrl+V` ảnh cũng hoạt động.\n- Nhánh macOS dùng `arboard` (chỉ target macOS, feature `image-data`; arboard 3.7 đã có sẵn trong lock qua egui-winit): ưu tiên file ảnh từ Finder (`file_list`), sau đó bitmap TIFF trên pasteboard.\n- Gom code tạo thumbnail vào `clipboard_image_preview`, dùng chung cho Windows `CF_HDROP`.\n- `Cargo.lock`: chỉ thêm các crate macOS-only (`tiff`, `objc2-core-graphics`, ...), không đổi phiên bản crate nào.\n\n## Verification\n- [x] `cargo test --locked` (Linux, Docker `rust:1-bookworm`): toàn bộ pass, gồm 6 test mới `clipboard_paste_tests` (Cmd+V bị nuốt, gõ `v` thường, nhả `v` từ IME, không dán 2 lần khi đã paste text, nhả Cmd ngay trước `V`, nhả Cmd đã quá hạn).\n- [x] Cross-build `aarch64-apple-darwin`, `x86_64-apple-darwin`, `universal2-apple-darwin` (cargo-zigbuild + macOS 11.3 SDK): build & link thành công.\n- [x] `cargo clippy --all-targets -- -D warnings`: không có cảnh báo trong code thay đổi; toolchain stable mới báo các lint có sẵn ở chỗ khác trong repo.\n- [x] `rustfmt --check` sạch cho `src/gui/messages.rs`, `src/gui/workspace.rs`.\n- [x] Manual (macOS): screenshot → `Cmd+V` → gửi; Copy Image từ Safari/Chrome; file `.png` từ Finder (không dán tên file); text vẫn dán 1 lần; gõ Telex chứa `v` khi Clipboard có ảnh không hiện banner; menu chuột phải.\n  - Đã kiểm chứng trên macOS (bản DMG universal): `Cmd+V` ảnh → hiện thanh xem trước → gửi thành công, người nhận hiển thị ảnh.\n\n  ![macOS: dán ảnh bằng Cmd+V và gửi thành công](https://raw.githubusercontent.com/zt-hky/ctdozlite/pr-assets/pr90/macos-paste-sent.png)\n\n## Risk & Rollback\n- Risk: two-way door; không đổi protocol/params/signing.\n- Blast radius: phát hiện phím tắt dán trên macOS/Linux; đọc Clipboard trên macOS. Windows không đổi hành vi (chỉ refactor thumbnail).\n- Rollback: revert commit.\n\n## Context\n- Resolves: None\n- Relates to: #71\n- ADR / RFC: None\n\n",
      "closedAt": "2026-11-15T09:00:23Z",
      "createdAt": "2026-11-04T05:02:38Z",
      "labels": [],
      "number": 90,
      "state": "CLOSED",
      "title": "fix(ui): paste clipboard images into composer on macOS"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjc1OTMxMzE=",
        "is_bot": false,
        "login": "gr219",
        "name": "Tuyến Trần"
      },
      "body": "## What\r\n- `send_attachment` now waits for the `act_type: \"file_done\"` control event to get the `fileUrl` before calling `asyncfile/msg` (file, document and video sends).\r\n- New `fileId`-keyed waiter registry in `src/chat.rs`. It is fed by the GUI WebSocket loop (`src/gui/worker.rs`) and the `lp_control` poller (`src/call.rs`). An event that arrives before its waiter registers is kept for 120s.\r\n- Chunk upload failures and non-zero `error_code` from `asyncfile/upload` are now returned as errors. Previously the loop hit `break` silently.\r\n\r\n## Why\r\n- Resolves #75: picking a file with the 📎 button looks like it does nothing, and the status bar shows `HTTP 405 … sharefile_full`.\r\n- `asyncfile/upload` only returns `fileId`. Zalo pushes `fileUrl` later as `file_done` (WebSocket `cmd 601 sub 0`; also seen on `cmd 603` when offline events are pulled). Without `fileUrl`, `asyncfile/msg` is rejected, and the `sharefile_full` fallback answers 405.\r\n\r\n## How\r\n- `dispatch_file_done_controls` parses `controls[].content` (`fileId`, plus `data.url` as an object or a JSON string) and resolves a `tokio::sync::oneshot` waiter.\r\n- The wait is capped at 90s.\r\n- The wait only runs once a control listener has been seen, so the TUI (which has no listener) keeps its current behaviour instead of stalling.\r\n- Compared with the approach sketched in #75, this reuses the session's existing WebSocket and doesn't open a second connection per send.\r\n- The extra `asyncfile/msg` fields mentioned in #75 (`extention`, `zsource`, `ttl`) were not needed: sends succeed without them.\r\n\r\n## Verification\r\n- [x] `cargo test --locked`: 194 passed, 1 ignored (`main`: 192). Two new tests:\r\n  - `extract_file_done_controls_parses_object_and_string_data`\r\n  - `file_done_resolves_waiters_registered_before_and_after_the_event`\r\n- [x] `cargo clippy --all-targets -- -D warnings`: 70 errors, the same count as current `main`. None in the new code.\r\n- [x] `cargo fmt --all --check`: no diffs in the changed files. The remaining diffs are pre-existing on `main`.\r\n- [x] Manual: a live account sending to \"Cloud của tôi\" (1-1), using a throwaway probe binary that runs `socket::spawn` → `dispatch_file_done_controls` → `send_attachment` (not included in this PR). Run on Linux (Docker, rust 1.91):\r\n  - `.pdf`, `.txt` and `.mp4` each received `file_done` within about 0.1s, then `asyncfile/msg` returned `error_code: 0`, and the messages appeared on the phone.\r\n  - With `file_done` dispatch disabled (the old behaviour): `.txt` fails with `HTTP 405 Method Not Allowed … /api/message/sharefile_full`.\r\n- [ ] Not tested: group sends; clicking the 📎 button in a patched Windows GUI build.\r\n\r\n## Risk & Rollback\r\n- Risk: two-way door. There are no protocol, schema or crypto changes; it only orders existing calls and adds a wait.\r\n- Blast radius: non-image attachments in `send_attachment`. Image sends (`photo_original/*`) are untouched.\r\n- Rollback: revert this commit.\r\n\r\n## Context\r\n- Resolves: #75\r\n- Relates to: None\r\n- ADR / RFC: None\r\n\r\n",
      "closedAt": "2026-11-04T15:31:38Z",
      "createdAt": "2026-11-04T03:14:32Z",
      "labels": [],
      "number": 85,
      "state": "MERGED",
      "title": "fix(chat): wait for file_done before sending async file attachments"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjMxODg0NDg3",
        "is_bot": false,
        "login": "hoanglong149",
        "name": ""
      },
      "body": "## Mô tả các tính năng & sửa lỗi (Resolves #76, Resolves #77, Resolves #79)\n\nPR này bao gồm toàn bộ các tính năng hoàn thiện trải nghiệm trò chuyện, quản lý nhóm và các tương tác cốt lõi trên `ctdozlite`:\n\n### 1. Hiển thị Avatar thật & Tên người gửi trong nhóm chat (Closes #76)\n• Bổ sung `WorkerCmd::FetchGroupMembers`: gọi `fetch_group_profiles` (`getmg-v2`) và `get_mini_profiles` (`/api/social/group/members`) để lấy thông tin thành viên (`displayName`, `avatar` URL, vai trò `creator`/`admin`).\n• Xây dựng map avatar & tên người gửi trong nhóm chat để bubble tin nhắn hiển thị avatar tròn thật và tên hiển thị chuẩn (thay vì UID số thô).\n\n### 2. Danh sách thành viên nhóm trong Info Drawer & Header (Closes #76)\n• Thêm khối **Thành viên nhóm (X)** trong Info Drawer bên phải: danh sách cuộn mượt mà có avatar, tên, huy hiệu \"Trưởng nhóm\" (vàng cam), \"Phó nhóm\" (xanh dương), ô tìm kiếm nhanh, nút nhắn tin 1-1 trực tiếp và xem profile.\n• Header hiển thị `👥 X thành viên`, click vào mở ngay danh sách thành viên.\n\n### 3. Highlight Mention Chip & Gợi ý `@mention` thông minh (Closes #76)\n• Nhận diện cú pháp `@All`, `@Tất cả`, `@<Tên>` trong `render_markdown_inline_rich` và hiển thị chip bo góc mềm mại màu xanh Zalo (`#0068FF` / `#61AFEF`).\n• **Autocomplete Popover**: Gõ `@` trong ô soạn thảo hiển thị popup danh sách thành viên nhóm / bạn bè, hỗ trợ lọc theo tên, điều hướng bằng phím mũi tên `↑`/`↓`, `Enter`/`Tab` để chèn tag tên dạng link `zalo://mention/...`.\n\n### 4. Sửa lỗi tỷ lệ hình học avatar nhóm & chống méo ảnh (Closes #77)\n• Dùng Circle Mesh (32 vertices) và UV center-crop (cover) trong `paint_circular_image`: cắt tròn mượt mà, giữ nguyên tỉ lệ 1:1, không bị kéo dẹt hay thò 4 góc vuông.\n• Căn chỉnh lại toạ độ hình học avatar nhóm composite (`multi:`): vẽ vòng tròn nền chuẩn bán kính $R$, đặt các avatar con nằm lọt 100% bên trong vòng tròn bán kính $R$.\n\n### 5. Preview hình ảnh Zalo\n• Bao quát toàn diện các domain Zalo CDN (`photo-stal`, `zpcloud`, `zfcloud`,...).\n• Ẩn hoàn toàn raw CDN URL thô kệch, hiển thị thumbnail ảnh to, sắc nét, có trạng thái thông báo ảnh hết hạn lịch sự.\n\n### 6. Chống thu hồi, Chuyển tiếp media, Ảnh gốc HD, Chụp màn hình & Typing/Seen (Closes #79)\n• **Chống xoá / xem lại tin nhắn thu hồi (Anti-recall)**: Giữ nguyên văn bản, media, link ảnh, sticker của tin nhắn đã thu hồi; hiển thị badge đỏ `🚫 Đã thu hồi` và cho phép bấm phóng to ảnh như bình thường.\n• **Chuyển tiếp tin nhắn (Forward)**: Hỗ trợ forward cả text, sticker và hình ảnh (tự động nhận diện CDN URL tải về gửi đính kèm sang hội thoại đích).\n• **Gửi ảnh gốc HD (Original Quality)**: Nút bật/tắt `[HD]` trên toolbar composer; gửi qua endpoint `/api/message/photo_original/send` giữ nguyên 100% chất lượng ảnh gốc không nén.\n• **Chụp ảnh màn hình (Screenshot) & Clipboard**: Phím tắt `Alt+A` / `Cmd+Shift+A` hoặc icon 📷 Camera kéo chọn vùng màn hình macOS, nạp ngay vào khung xem trước composer; hỗ trợ dán ảnh `Cmd+V`.\n• **Trạng thái đang soạn thảo (Typing) & Đã xem (Seen)**: Bắt các gói tin điều khiển `typing` và `seen` từ WebSocket & `lp_control`; hiển thị `✍ <Tên> đang soạn thảo…` ở Sidebar, Header và ô nhập; tự động đồng bộ trạng thái `Đã xem` màu xanh khi tin nhắn được đọc.\n\n### 7. Thanh cảm xúc nổi (Hover Reaction Bar) & Menu chuột phải chuẩn Zalo PC\n• **Thanh reaction capsule**: Hover chuột qua bong bóng chat hiển thị thanh nổi 6 emoji cảm xúc cùng nút thao tác nhanh (Trả lời `”`, Chia sẻ `↷`). Áp dụng cơ chế kiểm tra toạ độ màn hình tuyệt đối (`hover_zone`), loại bỏ hoàn toàn hiện tượng thanh reaction bị biến mất khi lia chuột giữa bóng chat và thanh reaction.\n• **Menu chuột phải (Context Menu)**: Hỗ trợ cả chuột phải (`Secondary Click`) lẫn `Control + Click` trên macOS trackpad. Render nổi độc lập ở tầng `Order::Foreground` với 9 mục chuẩn Zalo PC (*Trả lời, Chia sẻ, Copy tin nhắn, Ghim/Bỏ ghim, Đánh dấu sao, Xem ảnh phóng to, Lưu tập tin, Thu hồi, Xóa chỉ ở phía tôi*). Tự động đóng khi click thao tác, click ra ngoài hoặc nhấn phím `Esc`.\n\n---\n\n**Kiểm thử tự động**: Đã vượt qua 100% bộ kiểm thử tự động (**196 passed; 0 failed**).\n\n\n<!-- This is an auto-generated comment: release notes by coderabbit.ai -->\n<!-- coderabbit-ai-generated:v1 -->\n\n## Summary by CodeRabbit\n\n* **New Features**\n  * Added typing indicators, read receipts, `@mentions`, group-member search, and member profile and chat actions.\n  * Added screenshot capture, original-quality attachment sending, and clearer forwarding progress and results.\n  * Added message hover actions, clickable reactions, delivery states, and expanded conversation menu options.\n  * Pinned conversations now appear in the sidebar and sync from phone history.\n  * Added a command-line option for sending an attachment.\n* **Bug Fixes**\n  * Improved preservation of conversation names and display settings during refresh and sync.\n  * Improved recalled-message display, photo previews, and group-avatar rendering.\n<!-- end of auto-generated comment: release notes by coderabbit.ai -->",
      "closedAt": "2026-11-06T16:03:00Z",
      "createdAt": "2026-11-03T07:24:58Z",
      "labels": [],
      "number": 78,
      "state": "MERGED",
      "title": "feat(chat): nhóm chat, chống thu hồi, forward media, gửi ảnh gốc HD, chụp màn hình & typing/seen"
    },
    {
      "author": {
        "id": "U_kgDOCuAYQQ",
        "is_bot": false,
        "login": "quangthoai1985",
        "name": "Dương Quang Thoại"
      },
      "body": "<!--\nTier guide (see docs/standards/git-flow-and-pr-matrix.md):\n- Tier 1 (docs/typo/deps): keep What + Why, delete the rest.\n- Tier 2 (feature/bugfix/refactor): fill every section, tick the checks you ran.\n- Tier 3 (protocol/schema/crypto/breaking): add ADR link, evidence and rollback detail.\nUse concise bullets, not paragraphs. Never paste session.json, cookies, zpw_sek/zpw_enk, IMEI or raw captures.\n-->\n\n## What\n-\n\n## Why\n-\n\n## How\n-\n\n## Verification\n- [ ] `cargo test --locked`\n- [ ] `cargo clippy --all-targets -- -D warnings`\n- [ ] `cargo fmt --all --check`\n- [ ] Manual: <GUI/TUI, OS, flow, observed error_code>\n\n## Risk & Rollback\n- Risk: two-way door / one-way door\n- Blast radius:\n- Rollback:\n\n## Context\n- Resolves: None\n- Relates to:\n- ADR / RFC: None\n",
      "closedAt": "2026-11-15T09:00:52Z",
      "createdAt": "2026-11-02T13:30:57Z",
      "labels": [],
      "number": 70,
      "state": "CLOSED",
      "title": "Claude/v0.1.2"
    },
    {
      "author": {
        "id": "U_kgDOCuAYQQ",
        "is_bot": false,
        "login": "quangthoai1985",
        "name": "Dương Quang Thoại"
      },
      "body": "<!--\nTier guide (see docs/standards/git-flow-and-pr-matrix.md):\n- Tier 1 (docs/typo/deps): keep What + Why, delete the rest.\n- Tier 2 (feature/bugfix/refactor): fill every section, tick the checks you ran.\n- Tier 3 (protocol/schema/crypto/breaking): add ADR link, evidence and rollback detail.\nUse concise bullets, not paragraphs. Never paste session.json, cookies, zpw_sek/zpw_enk, IMEI or raw captures.\n-->\n\n## What\n-\n\n## Why\n-\n\n## How\n-\n\n## Verification\n- [ ] `cargo test --locked`\n- [ ] `cargo clippy --all-targets -- -D warnings`\n- [ ] `cargo fmt --all --check`\n- [ ] Manual: <GUI/TUI, OS, flow, observed error_code>\n\n## Risk & Rollback\n- Risk: two-way door / one-way door\n- Blast radius:\n- Rollback:\n\n## Context\n- Resolves: None\n- Relates to:\n- ADR / RFC: None\n",
      "closedAt": "2026-11-15T09:00:27Z",
      "createdAt": "2026-11-02T13:30:43Z",
      "labels": [],
      "number": 69,
      "state": "CLOSED",
      "title": "Claude/a8 media"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjkxMjQ4NDU2",
        "is_bot": false,
        "login": "esvm4",
        "name": "Thanh Ngoc"
      },
      "body": "- Add a persisted Linux-only close-to-tray preference.\\n- Add a StatusNotifier tray item for restore and quit actions.\\n- Preserve Ctrl+Q as an explicit application exit.\n\nCo-Authored-By: Claude Opus 5 <noreply@anthropic.com>\n",
      "closedAt": "2026-11-15T09:00:49Z",
      "createdAt": "2026-11-02T05:52:00Z",
      "labels": [],
      "number": 65,
      "state": "CLOSED",
      "title": "feat(ui): add Linux close-to-tray setting"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjM0NjA1OTE=",
        "is_bot": false,
        "login": "thuongtin",
        "name": "Ethan"
      },
      "body": "## Nội dung và mục tiêu\n\nPR này khôi phục các phần tính năng còn lại từ nhánh phát triển cũ lên upstream v0.0.4, lấy `20e742c` làm nền. Mục tiêu là giữ lại tính năng đã xây dựng nhưng nối chúng vào kiến trúc worker, mô hình tin nhắn và vòng đời hiện tại của upstream, thay vì chép đè nguyên GUI archive.\n\nThay đổi có cả phần người dùng nhìn thấy như **emoji, reaction, mention, ảnh và điều hướng lịch sử**, lẫn nền tảng cần thiết để các phần đó giữ đúng dữ liệu khi tải lại, đổi tài khoản, gửi lỗi hoặc hủy đồng bộ.\n\n## Thay đổi cụ thể và lý do\n\n### 1. Emoji inline, Unicode và nội dung sao chép\n\n**Thay đổi:**\n- Thêm renderer emoji inline cho alias trong tin nhắn; giữ nguyên raw alias trong dữ liệu lưu, selection và nội dung copy.\n- Xử lý theo grapheme để giữ nguyên tổ hợp màu da, cờ và chuỗi ZWJ, thay vì tách từng code point.\n- Bổ sung font Noto fallback và helper CoreText trên macOS cho glyph màu; worker xử lý bất đồng bộ, có giới hạn tác vụ và cache.\n- Artwork Zalo là pack local opt-in qua `ZL_EMOJI_PACK`, được kiểm bounds, PNG và hash. Default build không chứa hoặc tự tải artwork. 88 mục không có Unicode canonical giữ raw alias thay vì đoán hình thay thế.\n\n**Vì sao:** Emoji cần hiển thị đúng mà không làm biến đổi nội dung tin nhắn hoặc tạo ô trống khi helper/pack chưa có. Việc giữ raw text cũng tránh tình trạng copy rồi gửi lại cho ra nội dung khác bản gốc; giới hạn cache/tác vụ tránh decode ảnh và font trên UI thread.\n\n**Phần chính:** `src/gui/emoji_text.rs`, `src/gui/emoji_text/visual_tests.rs`, `src/gui/theme.rs`, `src/assets/emojis/zalo/`, `src/assets/fonts/`, `build.rs`, `docs/zalo-message-emoji.md`.\n\n### 2. Reaction bền vững và replay đúng tin nhắn\n\n**Thay đổi:**\n- Thêm journal SQLite theo tài khoản; ghi reaction trước khi phát sự kiện lên GUI.\n- Replay reaction cho đúng định danh tin nhắn, kể cả tin chưa nằm trong phần lịch sử đang nạp.\n- Giữ số đếm, trạng thái clear và nguồn snapshot/ledger; biểu diễn giới hạn dưới khi chưa đủ bằng chứng về số đếm đầy đủ.\n- Thêm import cache reaction được người dùng chọn rõ ràng, kiểm schema/chủ tài khoản/nguồn dữ liệu trước khi đưa vào cache quản lý riêng. Tệp nguồn không bị sửa.\n\n**Vì sao:** Chỉ cập nhật bubble đang hiển thị sẽ làm mất reaction sau restart hoặc khi tin nằm ngoài viewport. Reaction cũng không được hợp nhất bằng nội dung/thời gian, vì hai tin giống nhau vẫn có thể là hai tin khác nhau. Import có kiểm tra tránh đưa cache của tài khoản khác vào hội thoại hiện tại.\n\n**Phần chính:** `src/reaction_journal.rs`, `src/original_reactions.rs`, `src/gui/messages.rs`, worker consumers; `tests/reaction_journal_regressions.rs`, `tests/original_reaction_import_regressions.rs`.\n\n### 3. Mention incoming và mention trong composer\n\n**Thay đổi:**\n- Đọc span mention theo UTF-16, kiểm range/grapheme và UID trước khi hiển thị hoặc mở hồ sơ.\n- Click mention đi qua luồng profile có target UID và request version, không suy người dùng từ tên hiển thị.\n- Thêm picker thành viên cho composer; giữ UID/range đã chọn cùng raw text, reply và nháp bền.\n- Kiểm metadata mention khi đọc nháp và trước gửi; giữ client ID đã stage khi gửi text/reply/mention.\n\n**Vì sao:** Tên hiển thị có thể trùng, thay đổi hoặc bị gõ tay. Emoji trước mention cũng khiến offset UTF-16 khác byte offset. Cần giữ đúng danh tính và range xuyên suốt render, click, lưu nháp và gửi, đồng thời chặn phản hồi profile cũ ghi đè lựa chọn mới.\n\n**Phần chính:** `src/composer_mentions.rs`, `src/message_navigation.rs`, `src/gui/composer.rs`, `src/gui/workspace.rs`, `src/chat.rs`; bằng chứng/fixture tại `re-notes/36_composer_group_mentions.md`, `tests/fixtures/composer_group_mentions.json`.\n\n### 4. Metadata, hợp nhất chính xác và bảo vệ tin đã Recall\n\n**Thay đổi:**\n- Giữ metadata ảnh, sticker, link, reply, mention, reaction, delivery và raw JSON qua lưu/đọc SQLite; cô lập dữ liệu malformed thay vì làm hỏng toàn lượt hydrate.\n- Hợp nhất bằng định danh đã xác minh, không gộp thread bằng tên/avatar và không coi text/timestamp là danh tính tin nhắn.\n- Khi import/hydrate cache, chỉ bổ sung phần còn thiếu, không tua ngược nội dung hoặc thời gian live.\n- Chặn cache khôi phục nội dung và attachment của tin đã Recall.\n\n**Vì sao:** Chỉ lưu text làm tính năng trông như hoạt động trong phiên hiện tại nhưng mất sau restart. Ghép theo thông tin hiển thị có thể nhập nhầm hội thoại; cache cũ cũng không được phép hồi sinh nội dung đã bị thu hồi.\n\n**Phần chính:** `src/store.rs`, `src/chat.rs`, `src/gui.rs`; `tests/store_metadata_regressions.rs`.\n\n### 5. Kho lịch sử theo tài khoản, tìm kiếm và điều hướng ngoài viewport\n\n**Thay đổi:**\n- Thêm `AccountStore` tại `accounts/<sha256(account_id)>/history-v2.sqlite3`, tách khỏi DB legacy đang chứa scheduler/sync/API độc lập.\n- Dùng định danh tin theo thread; chuyển GUI sang adapter worker cho đọc/ghi, trạng thái chưa đọc, mute và xóa cục bộ.\n- Đọc trang cục bộ theo keyset cursor, tối đa 200 tin; tìm kiếm literal hỗ trợ tiếng Việt không dấu; mở trích dẫn cả khi tin chưa có trong viewport.\n- Gắn revision/request ID cho page, search và quote để kết quả trễ không ghi đè thread mới hoặc làm tin vừa xóa xuất hiện lại.\n- Import dữ liệu cũ là thao tác rõ ràng, đọc nguồn chỉ đọc và ghi đích theo transaction; không tự nhận kho legacy làm dữ liệu của tài khoản đang đăng nhập.\n\n**Vì sao:** Một DB/cache toàn cục dễ lẫn dữ liệu giữa tài khoản. Tăng dần LIMIT hoặc chỉ tìm trong phần đang nạp không giải quyết lịch sử dài. Cần vừa tải có giới hạn, vừa giữ dữ liệu ngoài viewport và đúng trạng thái sau restart.\n\n**Phần chính:** `src/store_v2.rs`, `src/gui/history.rs`, `src/gui/worker.rs`, `src/main.rs`; `tests/store_v2_regressions.rs`, `docs/LICH_SU_THEO_TAI_KHOAN.md`, ADR-0004.\n\n### 6. Dấu xóa schema 3 và xung đột client/server ID\n\n**Thay đổi:**\n- Nâng schema 2 lên 3, giữ thêm `server_id` đã xác minh trong tombstone và index theo thread/server ID.\n- Khi xóa tin/hội thoại, lưu provenance trước khi xóa hàng dữ liệu.\n- Chặn replay của đúng tin máy chủ đã xóa, nhưng giữ tin mới có server ID khác dù dùng lại client ID.\n- Tombstone cũ thiếu provenance hoặc khóa đụng một danh tính khác được báo lỗi mơ hồ rõ ràng, không đoán hoặc ghi đè.\n\n**Vì sao:** Canonical ID trong dữ liệu cũ có thể là client ID. Tombstone chỉ có khóa đó dễ xóa nhầm tin mới hoặc cho tin cũ xuất hiện lại khi đổi canonical key. Migration giữ nguyên dữ liệu cũ thay vì dựng lại provenance đã mất.\n\n**Phần chính:** `src/store_v2.rs`, `tests/schema_migrations.rs`, `tests/store_v2_regressions.rs`, ADR-0005. Tên DB mới vẫn là `history-v2.sqlite3`; số schema hiện tại là 3.\n\n### 7. Nháp bền, stage trước gửi và khôi phục gửi dở\n\n**Thay đổi:**\n- Lưu nháp theo tài khoản/thread, gồm raw text, reply, mention và attachment paths; coalesce bằng `LatestSnapshot`, một wake không mang payload, worker debounce rồi ghi.\n- Nút chọn tệp/ảnh, drag/drop và clipboard thêm vào nháp; thao tác gửi diễn ra khi người dùng nhấn Gửi.\n- Ghi outbox và cấp client ID decimal trước network; text và từng attachment có ID riêng. Persisted floor/khóa interprocess bảo vệ allocator của durable store.\n- Chỉ xóa nháp trên UI sau stage ACK và khi nháp vẫn khớp bản đã gửi; không xóa nội dung người dùng vừa sửa trong lúc chờ.\n- Ghi ACK text và từng tệp độc lập, phân biệt cả hai attachment có cùng path. Text thành công không bị hạ trạng thái vì tệp sau đó lỗi.\n- Sau restart/hủy phiên, phần pending trở thành chưa rõ kết quả; người dùng mở phần chưa xác nhận để sửa, không tự gọi lại transport.\n\n**Vì sao:** Enqueue command không đồng nghĩa đã lưu hoặc server đã nhận. Xóa nháp quá sớm làm mất nội dung; gửi lại toàn lượt khi chỉ một tệp lỗi có thể tạo bản trùng. Client ID và trạng thái từng phần cần sống qua restart để khôi phục có căn cứ.\n\n**Thay đổi hành vi có chủ đích:** Text/reply/mention gửi trước các tệp; tệp không lặp caption. Không thêm field mention/reply chưa có bằng chứng vào attachment protocol.\n\n**Phần chính:** `src/composer_store.rs`, `src/gui/durable_composer.rs`, `src/latest_snapshot.rs`, composer/workspace/worker và allocator trong `src/chat.rs`; `docs/durable-composer.md`.\n\n### 8. Giữ trang chờ ghi, backpressure và flush khi đóng/đổi phiên\n\n**Thay đổi:**\n- Đặt chỗ trước khi đọc nguồn tiếp theo, tối đa 8 lượt; giữ toàn bộ trang mạng tối đa 128 MiB trong mailbox trước await ghi đầu tiên.\n- Chia SQL thành chunk tối đa 32 MiB, chỉ tăng tiến độ sau SQL ACK. Giữ cả tail chưa ghi nếu DB lỗi hoặc waiter bị hủy giữa chừng.\n- Snapshot GUI thay thế có giới hạn riêng 1 MiB; không dùng snapshot 200 tin để ghi đè delta chứa các trang chưa nạp.\n- Giữ dữ liệu ghi lỗi để thử lại; chỉ công bố kết quả confirmed lên GUI sau ACK kho lịch sử.\n- Flush lịch sử và nháp dưới cùng hàng rào vòng đời trước xác nhận đóng/đăng xuất/đổi phiên. Lỗi flush chặn xác nhận hoàn tất và giữ dữ liệu để thử lại.\n- ACK đồng bộ nhóm không chặn socket loop, để lệnh hủy/control tiếp tục được xử lý khi SQLite lỗi.\n\n**Vì sao:** Một queue chỉ giữ chunk hiện tại vẫn mất phần còn lại khi future bị hủy. Snapshot mới nhất cũng không thay thế được immutable delta. Giới hạn hàng đợi tạo backpressure thay vì để tác vụ/dữ liệu tăng vô hạn; flush phải có kết quả thật, không chỉ gửi command đóng.\n\n**Ranh giới:** Mailbox bộ nhớ không bảo đảm sống qua crash trước khi ghi đĩa. Một tin/metadata riêng vượt 32 MiB hoặc trang vượt 128 MiB bị từ chối rõ, không được ACK là đã lưu. Composer có durable intent trước transport; legacy buttons/scheduler chưa có cùng bảo đảm đó và không được mô tả là exactly-once.\n\n**Phần chính:** `src/gui/history_retention.rs`, `src/latest_snapshot.rs`, `src/gui/history.rs`, `src/gui/worker.rs`, `src/gui/confirmed_history.rs`.\n\n### 9. Epoch fence cho event, side effect và auth commit\n\n**Thay đổi:**\n- Gắn generation tại lúc enqueue; task/blocking task và GUI event giữ epoch gốc, không tự nhận tài khoản hiện tại sau await.\n- Kiểm lại scope ở ranh giới ghi storage, dispatch request, cập nhật cache và chạy helper native.\n- Chặn cả A/B/A hoặc đăng nhập lại cùng UID và cùng khóa phiên; không dùng mỗi UID để xác định một incarnation.\n- Drain công việc đã bắt đầu trước cleanup; auth commit blocking chỉ cập nhật ticket task khởi tạo, không chuyển quyền cho task con cũ.\n- Cho phép flush dữ liệu immutable đã ràng buộc owner trong callback cleanup đồng bộ được kiểm intent; thao tác thông thường vẫn từ chối owner sai.\n\n**Vì sao:** So UID sau khi await không phát hiện trường hợp quay lại cùng tài khoản. Task cũ có thể phát sự kiện, ghi lại DB vừa wipe hoặc mở helper sau logout. Cần phân biệt phiên thực thi, không chỉ danh tính tài khoản, và giữ việc flush có chủ đích tách khỏi quyền chạy tiếp network.\n\n**Phần chính:** `src/runtime_scope.rs`, worker/GUI event bridge, `src/session.rs`, call/audio/scheduler và các media/reaction consumers.\n\n### 10. Receipt giao tin và đọc theo vùng thực sự hiển thị\n\n**Thay đổi:**\n- Parse/apply delivery receipt bằng định danh wire đã xác minh, không dựng từ display ID hoặc timestamp.\n- Chỉ nhận ứng viên đọc khi hàng tin thực sự nằm trong vùng clip, ở cuối chat không lọc, cửa sổ foreground/không minimized và không bị overlay che; yêu cầu dwell liên tục 500 ms.\n- Ticket hiển thị hết hạn sau 250 ms thiếu sample; kho kiểm lại ticket trước khi ghi unread, Session kiểm epoch trước HTTP.\n- Tách kết quả đọc cục bộ khỏi seen lên máy chủ. Chỉ live ledger đủ wire identity mới đủ điều kiện gửi seen, không gửi vì vừa hydrate/import lịch sử.\n- Mỗi request tối đa 50 identity; không tự retry kết quả transport chưa rõ. Attempt ledger tối đa 4096 identity/epoch, đạt giới hạn thì báo rõ và dừng wire seen mới, đọc cục bộ vẫn hoạt động.\n\n**Vì sao:** Đã tải tin không có nghĩa đã đọc. Công việc chờ trong queue có thể chạy sau khi người dùng đổi cửa sổ/thread; còn tự retry write chưa rõ kết quả sẽ thay đổi ngữ nghĩa transport.\n\n**Ngoại lệ giao thức cần review:** Bằng chứng client gốc cho hai seen-v2 endpoint dùng session AES JSON/form `params`, cookie, timeout 10 giây, retry 0; không có signing tag ParamCipher. PR giữ đúng hợp đồng đó trong Session, không tự đặt tag `seenv2`. ADR-0006 ở trạng thái đề xuất; nguồn và primary excerpts nằm trong `re-notes/visible-read-seen-v2*`.\n\n**Phần chính:** `src/delivery_receipts.rs`, `src/read_receipts.rs`, `src/session.rs`, workspace/worker/history; `re-notes/strict-delivery-receipts.md`.\n\n### 11. Preview ảnh, bản gốc và trình xem native\n\n**Thay đổi:**\n- Khi thumbnail lỗi, cho phép lấy bản gốc qua cùng pipeline, giữ nguyên signed URL; có retry/HD rõ ràng.\n- Cache/tác vụ/texture có giới hạn, key theo tài khoản và định danh ảnh đã xác minh; không gộp hai ảnh chỉ vì client ID đụng nhau.\n- Decode ảnh và orientation qua worker/helper, không làm trên UI thread.\n- Mở viewer macOS bằng tệp private và argv, không dùng shell để ghép URL; quản lý một viewer và dọn lease khi đóng/lỗi/hủy.\n\n**Vì sao:** Thumbnail lỗi không nên làm mất đường xem ảnh. Decode lớn và cache không giới hạn gây giật/tăng bộ nhớ; kết quả từ tài khoản cũ cũng không được xuất hiện trong tài khoản mới. Lease private giúp quản lý tài nguyên và vòng đời helper rõ ràng.\n\n**Phần chính:** `src/gui/media_preview.rs`, `src/gui/native_photo_viewer.rs`, `src/media_resources.rs`, worker/media consumers.\n\n### 12. Thông báo native và presence không bị phản hồi cũ chi phối\n\n**Thay đổi:**\n- Hiển thị trạng thái quyền thông báo macOS riêng với kết quả gửi thông báo.\n- Lọc thông báo theo tài khoản, enabled/mute/foreground và server ID ổn định; chống lặp, không thông báo lại toàn lịch sử bootstrap.\n- Kiểm cấu trúc lỗi/quyền riêng tư/timestamp trong phản hồi presence; bỏ phản hồi profile cũ bằng target/request version và epoch.\n- Giữ ngưỡng online hiện có, không đổi chỉ để giống implementation archive.\n\n**Vì sao:** Khởi động app không nên tạo hàng loạt thông báo cũ, và helper chạy được không đồng nghĩa hệ điều hành đã cấp quyền. Presence sai timestamp hoặc response trễ có thể làm người dùng hiểu sai trạng thái của người đang xem.\n\n**Phần chính:** `src/native_notifications.rs`, `src/presence.rs`, GUI/worker/profile handlers.\n\n### 13. Đồng bộ điện thoại/nhóm và giữ khả năng resume HTTP của upstream\n\n**Thay đổi:**\n- Bổ sung provider socket dùng các command 511/590/591/592, coordinator, progress và cancel; giữ luồng HTTP history/resume hiện có.\n- Download archive có giới hạn/kiểm host, redirect và kích thước; blob download không mang cookie phiên. Decode qua helper có timeout và cleanup.\n- Giữ raw page theo owner trước lookup UID. Nếu sender còn opaque, không đoán UID để ép chuyển thành tin nhắn.\n- Lưu raw page private tại `phone-pending-v1`; chỉ resume khi người dùng bắt đầu đồng bộ rõ ràng, trước yêu cầu transfer mới. Chỉ unlink sau khi toàn bộ chunk đã nhận AccountStore ACK.\n- Wipe xóa dữ liệu chờ đúng tài khoản, giữ tài khoản khác/session/legacy. Raw spool giới hạn 8 tệp, 256 MiB tổng và 128 MiB/trang.\n- Với HTTP sync, reserve trước mỗi fetch, capture page đồng bộ trước future ghi; chỉ tiến cursor sau ACK. Fallback HTTP cũng giữ kết quả mỗi response trước await tiếp.\n- Merge archive theo semantics cache: bổ sung lịch sử nhưng không tua ngược body/clock/reaction/Recall/unread của dữ liệu live.\n\n**Vì sao:** Hủy lúc đang lookup UID hoặc ghi chunk đầu có thể làm mất cả shard đã tải. Đoán mapping lại gây sai danh tính. Spool và cursor ACK cho phép tiếp tục có căn cứ, đồng thời tránh vô tình thay thế tính năng resume đã có của upstream.\n\n**Phần chính:** `src/gui/phone_history.rs`, `src/gui/phone_pending.rs`, `src/gui/group_history.rs`, `src/socket.rs`, `src/history_sync.rs`, `src/endpoints.rs`; `re-notes/phone-history-pipeline.md`.\n\n### 14. API HTTP/MCP nhúng đọc đúng kho của GUI\n\n**Thay đổi:**\n- Pin account, root và epoch lúc listener chấp nhận connection; kiểm account khớp `Session.login.uin`, hủy connection cũ khi đổi phiên.\n- Chuyển threads/messages/search/resources/export/status của API nhúng sang AccountStore, query có giới hạn; không fallback sang DB/cache global khi kho account mới trống hoặc lỗi.\n- Giữ hình dạng payload hiện có; search V2 là literal text tiếng Việt không dấu, không giả lập là legacy substring/sender-name search.\n- CLI/TUI, API chạy độc lập và bảng scheduler giữ đường legacy tương ứng; PR không thêm API xóa lịch sử/mark-read mới.\n\n**Vì sao:** GUI đọc kho mới nhưng API vẫn đọc global legacy sẽ trả dữ liệu khác màn hình hoặc lẫn tài khoản. Request đã nhận trước A/B/A cũng không được nhận quyền của phiên mới.\n\n**Phần chính:** `src/api_mcp.rs`, `docs/verification/account-scoped-gui-api.md`; test qua HTTP/MCP loopback thật với dữ liệu A/B/A và legacy poison row.\n\n## Cách tích hợp và phạm vi upstream\n\n### 15. Tài liệu, phạm vi upstream và cách đọc PR\n\n**Thay đổi và lý do:**\n- Thêm hướng dẫn tính năng, history/composer/emoji, ADR-0004/0005/0006 và protocol notes để reviewer truy được thay đổi schema/giao thức, giới hạn và quyết định thiết kế; cập nhật mục lục docs.\n- Giữ scheduler, feed, updater, HTTP sync và TUI của upstream thay vì đưa lại phiên bản cũ từ archive, nhằm tránh mất các cải tiến đã merge.\n- PR chứa ba thay đổi cũng đang có trong #59, #60 và #61. Chúng được giữ thành commit độc lập; cần chọn thứ tự merge/rebase để tránh áp lại cùng patch. PR này không tự đóng ba PR đó.\n- Có thể đọc theo các lớp: metadata/identity → emoji/reaction/mention → AccountStore/schema → runtime fence → composer/outbox → retention và các producer. Các lớp liên quan dữ liệu cần được review cùng nhau, không coi renderer hoặc utility riêng lẻ là toàn bộ tính năng.\n\nTổng quan nằm tại `docs/KHOI_PHUC_TINH_NANG.md`. Phần kiểm chứng và hoàn tác bên dưới tách kết quả đã chạy khỏi nghiệm thu tài khoản thật.\n\n### 16. Dọn cảnh báo runtime của GitHub Actions\n\n**Thay đổi:** Trong `ci-release.yml` và `docs.yml`, cập nhật `actions/checkout@v4` lên `@v5`, `actions/upload-artifact@v4` lên `@v6`, và action tải artifact của release từ `actions/download-artifact@v4` lên `@v7`.\n\n**Vì sao:** CI trước đã pass nhưng còn warning Node.js 20 bị deprecated. Các phiên bản được chọn có `runs.using: node24` trong metadata chính thức. Chỉ thay phiên bản action; giữ nguyên triggers, permissions, điều kiện bỏ qua package/release trên PR, các input artifact và lệnh build/test. Chưa kích hoạt phát hành hoặc merge PR.\n\n**Kiểm chứng:** CI tại `8f92787` đã pass Resolve version, Build docs và Cargo Test Suite Linux; annotations của cả ba job không còn warning/error. Linux chạy `cargo test --locked`: **555 pass, 0 fail, 2 ignored**. Thông báo migration của nhãn `ubuntu-latest` còn ở mức notice, không phải lỗi job. Job đóng gói/release skip đúng điều kiện PR, không được tính là đã kiểm thử package đa nền tảng.\n\n## Kiểm chứng\n\n- [x] `cargo test --locked`: **556 pass, 0 fail, 4 ignored**, trên commit `d404292`.\n- [x] Baseline integration `8de5b36`: **232 pass, 0 fail, 1 ignored**.\n- [x] Rollback patch trên worktree disposable: **78 paths phục hồi**, sau đó chạy lại baseline **232 pass, 0 fail, 1 ignored**. Source tích hợp vẫn giữ thay đổi.\n- [x] Review độc lập đóng các finding về multi-chunk cancellation, raw phone resume, owner/epoch, GUI admission và thứ tự ACK.\n- [x] Không có diagnostic clippy mới khi đối chiếu baseline theo file/code/function: **40 baseline, 39 modified, 0 mới**.\n- [ ] `cargo clippy --locked --all-targets -- -D warnings` chưa xanh toàn repo: exit **101** do lỗi nền upstream trên toolchain hiện tại. Không tắt warning để đạt pass.\n- [ ] Kiểm thử GUI tương tác/tài khoản thật trước release.\n\n- Helper macOS: build và self-test emoji, media, codec, notifier, viewer đã chạy; test native emoji worker tới inline mesh đã pass.\n- Startup/close process disposable chạy bằng bundle background-only, không giành focus. Chưa coi đó là GUI rendering thông thường.\n- Test dùng fixture, loopback và tài khoản giả; chưa đánh dấu pass cho gửi/seen/phone sync trên tài khoản thật hoặc toast hệ điều hành.\n- Default build không tải artwork/codec assets hoặc chứa session, cookies, DB và capture của tài khoản thật.\n\n## Rủi ro và hoàn tác\n\n- Đây là thay đổi Tier 3 về storage/runtime/protocol, cần human audit và kiểm tra thủ công các flow tài khoản thật trước release.\n- Schema 3 nâng cấp từ schema 2 giữ provenance của tombstone. Tệp có phiên bản mới hơn hoặc chủ khác bị từ chối; không hạ schema tại chỗ.\n- Rollback source bằng revert các commit sau integration base; giữ nguyên dữ liệu người dùng và dùng bản sao khi kiểm migration/rollback.\n- Không tự gửi lại request chưa rõ kết quả. Giới hạn wire seen 4096 identity/epoch được báo rõ; đọc cục bộ vẫn tiếp tục, exclusion chưa bền qua epoch.\n- Standalone API/TUI/scheduler giữ kho legacy; adapter account store trong PR này áp dụng cho GUI và API nhúng.\n",
      "closedAt": "2026-11-15T09:00:45Z",
      "createdAt": "2026-11-02T00:38:31Z",
      "labels": [],
      "number": 62,
      "state": "CLOSED",
      "title": "feat(chat): khôi phục bộ tính năng chat trên upstream v0.0.4"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjM0NjA1OTE=",
        "is_bot": false,
        "login": "thuongtin",
        "name": "Ethan"
      },
      "body": "## Thay đổi\n- Hợp nhất tin nhắn bằng định danh chính xác trong thread, preview và lịch sử nhóm; giữ các tin khác nhau dù trùng nội dung và thời gian.\n- Giữ client ID xuyên suốt thao tác gửi và phản hồi worker, tránh nhân đôi khi server push đến trước acknowledgement.\n- Giữ thời gian server khi acknowledgement cục bộ đến muộn, không đảo thứ tự cuộc trò chuyện.\n\n## Lý do\n- So khớp theo nội dung hoặc thời gian có thể làm mất tin hợp lệ; chỉ thêm tin cục bộ sau phản hồi cũng có thể tạo bản sao.\n- Đây là phần nền được tách để tiếp tục đóng góp các tính năng đã xây mà không kéo theo toàn bộ nhánh tùy biến.\n\n## Cách thực hiện\n- Dùng chỉ mục định danh, từ chối trường hợp mơ hồ, khác hướng gửi hoặc xung đột định danh server đã xác minh.\n- Giữ thứ tự chọn canonical ID của parser hiện tại để tương thích cache SQLite cũ; bổ sung metadata wire/client mà không đổi schema.\n- Các hàm gửi công khai giữ chữ ký; biến thể nội bộ nhận một client ID xuyên suốt các đường fallback có sẵn.\n- Không thêm trường request, không đổi host, mã hóa, ký yêu cầu hay cookie. Căn cứ và hợp đồng được ghi tại `re-notes/35_exact_message_identity.md`.\n\n## Kiểm chứng\n- [x] `CMAKE_POLICY_VERSION_MINIMUM=3.5 cargo test --locked`: 213 kiểm thử thành công, 0 thất bại, 1 bỏ qua trên macOS arm64.\n- [x] 14 regression tích hợp, gồm replay, xung đột định danh và persist/hydrate thực qua SQLite trong bộ nhớ.\n- [x] Regression server push trước acknowledgement và acknowledgement đến muộn giữ chronology; ba regression review thất bại trước sửa và thành công sau sửa.\n- [x] Kiểm 10.000 client ID tăng đơn điệu và chín request/fallback bằng HTTP fixture loopback, giải mã body để kiểm client ID. Không gọi Zalo thật.\n- [x] Review độc lập, `git diff --check` và hoàn tác bốn file nguồn trên bản sao về đúng hash ban đầu.\n- [ ] `cargo clippy --locked --all-targets -- -D warnings`: upstream đã thất bại; so sánh JSON có 39 chẩn đoán khác nhau ở cả hai bản, không phát sinh chẩn đoán mới.\n- [ ] Chưa kiểm tài khoản thật hoặc giao diện desktop; chưa kiểm Windows/Linux tại máy cục bộ.\n\n## Rủi ro và hoàn tác\n- Không bao gồm migration định danh/schema. Với hai global server ID khác nhau nhưng cùng cliMsgId và không có msgId, memory merge giữ hai tin nhưng khóa msg_id của SQLite legacy vẫn có thể va chạm như baseline. Cần PR migration riêng, không coi trường hợp đó đã được sửa bền vững.\n- Client ID tăng đơn điệu trong một process, chưa phải outbox bền sau restart.\n- Hoàn tác commit khôi phục thuật toán hợp nhất và wiring gửi cũ, không xóa dữ liệu người dùng.\n",
      "closedAt": "2026-11-02T04:32:29Z",
      "createdAt": "2026-11-01T22:29:16Z",
      "labels": [],
      "number": 61,
      "state": "MERGED",
      "title": "fix(chat): hợp nhất tin nhắn theo định danh chính xác"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjM0NjA1OTE=",
        "is_bot": false,
        "login": "thuongtin",
        "name": "Ethan"
      },
      "body": "## Thay đổi\n- Nhấn phần trích dẫn để cuộn tới và làm nổi bật tin nhắn gốc đã nạp trong cuộc trò chuyện.\n- Định vị theo định danh chính xác, không đoán theo nội dung hay thời gian. Báo rõ khi chưa nạp hoặc định danh không duy nhất.\n\n## Lý do\n- Upstream đã hiển thị trích dẫn nhưng chưa có thao tác quay về tin gốc. Đây là phần được tách từ nhánh tính năng đã xây, không gửi lại nền dữ liệu đã được nhận.\n\n## Cách thực hiện\n- Phân giải tham chiếu server/client theo namespace và từ chối kết quả mơ hồ.\n- Mở rộng cửa sổ dựng tin nhắn khi cần, cuộn tới hàng đích và làm nổi bật trong ba giây.\n- Gắn trạng thái với cuộc trò chuyện, phân giải lại chỉ số và vô hiệu hóa bám đáy trong lúc điều hướng để hoạt ảnh không kéo về cuối.\n\n## Kiểm chứng\n- [x] `CMAKE_POLICY_VERSION_MINIMUM=3.5 cargo test --locked`: 206 kiểm thử thành công, 0 thất bại, 1 bỏ qua trên macOS arm64.\n- [x] 11 kiểm thử tập trung, gồm nhấn trích dẫn trong bong bóng gửi/nhận bằng egui, định danh xung đột, đích ngoài cửa sổ dựng và hoạt ảnh cuộn mặc định.\n- [x] Regression bám đáy thất bại trước sửa và thành công sau sửa; review độc lập xác nhận đã xử lý.\n- [x] `git diff --check` và hoàn tác trên bản sao, khôi phục hash nguồn ban đầu.\n- [ ] `cargo clippy --locked --all-targets -- -D warnings`: upstream đã thất bại; so sánh JSON có 39 chẩn đoán khác nhau ở cả hai bản, không có chẩn đoán mới.\n- [ ] Chưa chạy giao diện desktop với tài khoản thật; chưa kiểm Windows/Linux tại máy cục bộ.\n\n## Rủi ro và hoàn tác\n- Chỉ điều hướng trong các tin đã nạp, chưa tìm lịch sử từ cơ sở dữ liệu và chưa triển khai nhấn mention.\n- Không đổi giao thức, dữ liệu tài khoản hoặc schema. Hoàn tác commit trả phần trích dẫn về hiển thị tĩnh như upstream.\n",
      "closedAt": "2026-11-02T04:32:36Z",
      "createdAt": "2026-11-01T22:28:26Z",
      "labels": [],
      "number": 60,
      "state": "MERGED",
      "title": "feat(ui): điều hướng tới tin nhắn trích dẫn đã nạp"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjM0NjA1OTE=",
        "is_bot": false,
        "login": "thuongtin",
        "name": "Ethan"
      },
      "body": "## Thay đổi\n- Thêm phần hiển thị trạng thái quyền thông báo macOS và nút yêu cầu quyền riêng trong Cài đặt.\n- Chỉ bổ sung phần quản lý quyền của cầu nối thông báo native đã có trong bản tùy biến. Chưa gửi thông báo native, để không bỏ qua kiểm tra tài khoản, tắt tiếng và trạng thái cửa sổ.\n\n## Lý do\n- Helper `DailyNotifier.app` đã được đóng gói, nhưng công tắc thông báo của ứng dụng chưa thể hiện quyền do hệ điều hành cấp.\n- Không tự xin quyền khi khởi động hoặc đăng nhập; bật công tắc thông báo không đồng nghĩa với yêu cầu cấp quyền macOS.\n\n## Cách thực hiện\n- Worker gọi helper bất đồng bộ, giới hạn đầu ra ở 4096 byte, giới hạn thời gian chờ và kiểm tra mã thoát, UTF-8 cùng giá trị trạng thái trả về.\n- Mã thứ tự yêu cầu ngăn gọi trùng, bỏ qua kết quả cũ và kết thúc trạng thái đang tải khi lỗi để người dùng thử lại.\n- Tìm helper theo vị trí gói ứng dụng, kể cả sau khi di chuyển, hoặc theo đường dẫn phát triển. Không đổi cấu trúc dữ liệu hay giao thức Zalo.\n- Chỉ bật thêm tính năng `process` của Tokio; `Cargo.lock` không nâng phiên bản thư viện.\n\n## Kiểm chứng\n- [x] `CMAKE_POLICY_VERSION_MINIMUM=3.5 cargo test --locked`: 203 kiểm thử thành công, 0 thất bại, 1 bỏ qua trên macOS arm64.\n- [x] `cargo check --locked --all-targets`: thành công.\n- [x] 8 kiểm thử tập trung: helper giả lập, tham số lệnh, UTF-8, đầu ra quá lớn, quá thời gian, thiếu helper, mã thoát lỗi, yêu cầu trùng, kết quả cũ và thử lại sau lỗi.\n- [x] Helper thật `DailyNotifier --status` trả về `notDetermined`, mã thoát 0. Không chạy `--request` thật.\n- [x] `git diff --check`, review độc lập và hoàn tác mã nguồn trên bản sao dùng cho kiểm thử.\n- [ ] `cargo clippy --locked --all-targets -- -D warnings`: upstream trước thay đổi đã thất bại. So sánh chẩn đoán JSON, bỏ qua thay đổi số dòng: cả hai bản đều có 39 chẩn đoán khác nhau, không phát sinh chẩn đoán mới.\n- [ ] Chưa kiểm giao diện desktop trực tiếp, hộp thoại xin quyền thật, bản dựng Windows/Linux hoặc chạy gói ứng dụng đã di chuyển.\n\n## Rủi ro và hoàn tác\n- PR này chưa triển khai gửi thông báo native; giao diện nói rõ điều đó.\n- Không đổi dữ liệu tài khoản, phiên đăng nhập, ký yêu cầu, logic tắt tiếng hoặc các cài đặt thông báo hiện hữu.\n- Hoàn tác commit sẽ gỡ phần giao diện và bộ chuyển tiếp mới, đồng thời trả tính năng Tokio về bản gốc. Kiểm tra trên bản sao đã xác nhận khôi phục đúng hash mã nguồn ban đầu.\n",
      "closedAt": "2026-11-02T04:32:41Z",
      "createdAt": "2026-11-01T22:16:31Z",
      "labels": [],
      "number": 59,
      "state": "MERGED",
      "title": "feat(ui): hiển thị trạng thái quyền thông báo macOS"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjM0NjA1OTE=",
        "is_bot": false,
        "login": "thuongtin",
        "name": "Ethan"
      },
      "body": "## Nội dung\n- Tách model tin nhắn và trạng thái thành PR độc lập: loại tin, wire identity, delivery state, unread policy, reaction summary/replay ledger và sidebar presentation.\n- Bổ sung các pure helper `message_model`, `reactions`, `unread`, export cho library/binary và giá trị mặc định tại các constructor hiện có.\n\n## Lý do\n- Giữ định danh và metadata gốc qua các lần merge; tránh đánh đồng unread với kích thước history batch hoặc xóa reaction khi nhận snapshot cũ.\n- Cung cấp nền nhỏ nhất cho các PR adapter tiếp theo mà không phụ thuộc PR format hoặc mang toàn bộ UI/network thay đổi vào cùng lúc.\n\n## Cách thực hiện\n- Nền trực tiếp upstream `main` mới nhất tại `1345f5d0cfd24fdb7d54e5caf3a88e3e3b2acd4a`, đã gồm PR #40/#41; không kéo commit format vào nhánh này và giữ các sửa build/performance upstream.\n- Port patch model `d1dc2a2` so với `82d8941`, tương ứng delta `7da4fa1` so với `54437ca` với các constructor bổ sung trên upstream mới. Các file `message_model.rs`, `reactions.rs`, `unread.rs` giữ nguyên byte của source cũ.\n- `ChatMessage` nhận metadata có `serde(default)` và helper merge; `Thread` nhận `presentation`/`unread`. Các constructor GUI/worker/TUI/store/probe chỉ thêm default để build tương thích, chưa nối network/GUI adapter mới.\n- Mở rộng clock helper cho normalize/sidebar/day, đồng thời giữ lại cả ba test timezone/invalid timestamp upstream cùng test mới, tránh giảm coverage.\n- Không đổi host, cipher, signing, outbound request hay schema SQLite; không thêm dependency hoặc sửa lockfile.\n\n## Kiểm chứng\n- Nền cuối `1345f5d0cfd24fdb7d54e5caf3a88e3e3b2acd4a`; rebase sạch, range-diff xác nhận delta tính năng không đổi. Giữ binary rename `ctdozlite` và các sửa avatar/media upstream mới.\n- Đạt: `CMAKE_POLICY_VERSION_MINIMUM=3.5 CARGO_TARGET_DIR=/Users/ethan/VibeCoding/zl/target cargo check --locked --all-targets` (exit 0).\n- Đạt: cùng biến môi trường, `cargo test --locked --lib -- message_model reactions unread clock message_state_defaults`: 22 passed, 0 failed (exit 0).\n- Đạt: `git diff --check` (exit 0). Affected-file rustfmt đạt trên ba module mới, `clock.rs` và `lib.rs`. Bảy file legacy có format fail trên cả baseline upstream mới và bản sửa; giữ minimal diff, không mang format sweep vào PR.\n- 22 test gồm pure model/reaction/unread helpers, 4 test clock (giữ cả 3 test timezone upstream) và 2 test backward-compatible defaults cho `ChatMessage`/`Thread`.\n- Evidence nền cuối: `model-1345-check.log`, `model-1345-test.log`, `model-1345-fmt-*.log`, `model-1345-range-diff.txt` và `format-results-1345.json` trong `dist/pr-rebase-20261101/`. Kết quả nền cũ giữ riêng trong `base-6cfad22/`.\n- Chỉ kiểm local macOS 27.0.1 arm64 bằng fixture; không chạy app, thiết bị, live GUI, dữ liệu thật hay Pixel. UI chưa được tối ưu; chưa nghiệm thu adapter network/GUI, workflow người dùng hoặc Linux/Windows.\n\n## Rủi ro và hoàn tác\n- Thêm public fields ảnh hưởng downstream Rust struct literals; các constructor hiện có trong repository đã được cập nhật. Dữ liệu serde cũ dùng mặc định, nhưng chưa nghiệm thu đồng bộ metadata mới qua adapter/persistence workflow đầy đủ.\n- Pure mappings/replay/unread helpers giữ lại hành vi source đã tách, không tự xác nhận semantics server live. Chưa kiểm Linux/Windows hoặc hiệu năng UI.\n- Hoàn tác commit để gỡ model/helper/export và default constructors; không có schema migration hoặc thay đổi dữ liệu thật.\n\n### Tái kiểm sau khi làm sạch package artifacts\n- Chạy lại `cargo clean -p ctdozlite`, `cargo check --locked --all-targets` và các scoped tests đã liệt kê trên đúng SHA `c12d9d29feb6dae8a2bc3acf4286a792948ea65b`: tất cả exit 0; số test và filtered count được đối chiếu để tránh nhầm artifact giữa worktree. Không đổi source trong lượt tái kiểm.\n",
      "closedAt": "2026-11-01T15:18:16Z",
      "createdAt": "2026-11-01T14:14:31Z",
      "labels": [],
      "number": 55,
      "state": "MERGED",
      "title": "feat(chat): bảo toàn định danh và trạng thái tin nhắn"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjM0NjA1OTE=",
        "is_bot": false,
        "login": "thuongtin",
        "name": "Ethan"
      },
      "body": "## Nội dung\n- Thêm module độc lập để tải release có giới hạn dung lượng/deadline, kiểm tên asset, size, SHA-256 tùy chọn và chữ ký định dạng executable.\n- Cài đặt giao dịch qua sibling staging/backup, launcher callback và hoàn tác nếu launch thất bại; hỗ trợ portable deferred stage/abort/drop.\n\n## Lý do\n- Ngăn ghi gói chưa xác thực lên bản đang chạy và giữ được bản cũ khi thay thế hoặc khởi chạy thất bại.\n\n## Cách thực hiện\n- Nền độc lập mới nhất: upstream `main` tại `1345f5d0cfd24fdb7d54e5caf3a88e3e3b2acd4a`, đã bao gồm các PR upstream #40 và #41. Giữ nguyên thay đổi build/performance và các phần không liên quan của upstream.\n- Không thêm dependency, không đổi `Cargo.toml`/`Cargo.lock`, không đổi giao thức, models, chat hay GUI.\n- Giữ nguyên module và test từ `3d824c2bdbfcd5ad03a61db5e985976916b4efa0`, chỉ thêm export `updater` vào `src/lib.rs` của upstream.\n- Caller truyền HTTP client riêng không cookie và đích cài đặt rõ ràng; không suy ra đường dẫn đích từ metadata từ xa.\n- Staging riêng qua `create_new`, flush trước thay thế; giữ quyền executable portable. Callback launch được tiêm từ adapter, không chạy process thật trong test. Nếu rollback lỗi, trả lỗi rõ và giữ backup cho phục hồi.\n\n## Kiểm chứng\n- Nền cuối `1345f5d0cfd24fdb7d54e5caf3a88e3e3b2acd4a`; rebase sạch, range-diff xác nhận delta tính năng không đổi. Giữ binary rename `ctdozlite` và các sửa avatar/media upstream mới.\n- Đạt: `CMAKE_POLICY_VERSION_MINIMUM=3.5 CARGO_TARGET_DIR=/Users/ethan/VibeCoding/zl/target cargo check --locked --all-targets` (exit 0).\n- Đạt: cùng biến môi trường, `cargo test --locked --test updater_regressions`: 20 passed, 0 failed (exit 0).\n- Đạt: `git diff --check` (exit 0). Affected-file rustfmt đạt trên toàn bộ file ảnh hưởng (exit 0).\n- Fixture loopback/disposable kiểm HTTP status, độ dài/stream overflow, timeout, SHA-256, traversal, executable signature, atomic replacement/rollback, symlink rejection và deferred stage/drop/abort/unwind.\n- Evidence nền cuối: `updater-1345-check.log`, `updater-1345-test.log`, `updater-1345-fmt-*.log`, `updater-1345-range-diff.txt` và `format-results-1345.json` trong `dist/pr-rebase-20261101/`. Kết quả nền cũ giữ riêng trong `base-6cfad22/`.\n- Chỉ kiểm local macOS 27.0.1 arm64 bằng fixture; không chạy app, thiết bị, live GUI, dữ liệu thật hay Pixel. UI chưa được tối ưu; chưa nghiệm thu adapter network/GUI, workflow người dùng hoặc Linux/Windows.\n\n## Rủi ro và hoàn tác\n- Đây là core transaction primitives, không phải xác nhận release distribution: chưa kiểm chữ ký/notarization/installer thật hoặc restart ứng dụng thật. SHA-256 tùy chọn không thay thế kiểm nguồn phát hành của adapter.\n- Hoàn tác commit để gỡ module/export/test; không có migration. Rollback runtime được kiểm trong sandbox, không thay bản ứng dụng hiện cài.\n\n### Tình trạng CI kế thừa\n- Windows CI của nền `6cfad22` và lần chạy PR trước rebase đều lỗi fixture đường dẫn `resolves_bundle_cli_and_explicit_paths` vì `/Users/fixture` không là absolute path trên Windows. Lần rebase này không sửa test ngoài phạm vi. Kết quả local ở trên là macOS arm64; không đồng nghĩa đã kiểm thử Windows/Linux runtime.\n\n### Tái kiểm sau khi làm sạch package artifacts\n- Chạy lại `cargo clean -p ctdozlite`, `cargo check --locked --all-targets` và các scoped tests đã liệt kê trên đúng SHA `78194805b2687726bd1b411e6dfd76220d0c1bd8`: tất cả exit 0; số test và filtered count được đối chiếu để tránh nhầm artifact giữa worktree. Không đổi source trong lượt tái kiểm.\n",
      "closedAt": "2026-11-01T15:27:08Z",
      "createdAt": "2026-11-01T14:05:14Z",
      "labels": [],
      "number": 54,
      "state": "MERGED",
      "title": "feat(core): tải cập nhật có giới hạn và hoàn tác"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjM0NjA1OTE=",
        "is_bot": false,
        "login": "thuongtin",
        "name": "Ethan"
      },
      "body": "## Nội dung\n- Thêm module độc lập cho tải media có giới hạn, định danh cache theo tài khoản/thread/message/chất lượng và lỗi có kiểu.\n- Có cancellation ticket, ghi original qua file tạm riêng, cache account-isolated, prune cache, LRU texture và admission giới hạn trước khi spawn.\n\n## Lý do\n- Cung cấp lớp tài nguyên dùng lại để hạn chế bộ nhớ, dung lượng đĩa và số job đồng thời; không trộn adapter GUI vào PR nền tảng.\n\n## Cách thực hiện\n- Nền độc lập mới nhất: upstream `main` tại `1345f5d0cfd24fdb7d54e5caf3a88e3e3b2acd4a`, đã bao gồm các PR upstream #40 và #41. Giữ nguyên thay đổi build/performance và các phần không liên quan của upstream.\n- Không thêm dependency, không đổi `Cargo.toml`/`Cargo.lock`, không đổi giao thức, models, chat hay GUI.\n- Giữ nguyên module và test từ `9dea7cd620a5d8395977778c3e0d84210ec1ef28`, chỉ thêm export `media_resources` vào `src/lib.rs` của upstream.\n- Preview tối đa 16 MiB, HTML 128 KiB, texture budget 128 MiB, disk budget 512 MiB; tối đa 4 job active và 128 pending.\n- Key SHA-256 có length prefix và chất lượng; cache từ chối account không khớp. Chỉ cache-owned regular `.media` được prune, không đụng saved originals. Drop future/file chưa commit dọn partial.\n\n## Kiểm chứng\n- Nền cuối `1345f5d0cfd24fdb7d54e5caf3a88e3e3b2acd4a`; rebase sạch, range-diff xác nhận delta tính năng không đổi. Giữ binary rename `ctdozlite` và các sửa avatar/media upstream mới.\n- Đạt: `CMAKE_POLICY_VERSION_MINIMUM=3.5 CARGO_TARGET_DIR=/Users/ethan/VibeCoding/zl/target cargo check --locked --all-targets` (exit 0).\n- Đạt: cùng biến môi trường, `cargo test --locked --test media_resources_regressions`: 9 passed, 0 failed (exit 0).\n- Đạt: `git diff --check` (exit 0). Affected-file rustfmt đạt trên toàn bộ file ảnh hưởng (exit 0).\n- Các test HTTP chỉ gọi fixture loopback và dùng filesystem disposable, kiểm account isolation, LRU, status/overflow/offline, private streaming, cancellation cleanup, queue bounds, prune và partial ownership. Hai unit admission/drop đã đạt trên nền cũ; module giữ nguyên byte, lần xác minh nền cuối chạy lại 9 integration test.\n- Evidence nền cuối: `media-1345-check.log`, `media-1345-test.log`, `media-1345-fmt-*.log`, `media-1345-range-diff.txt` và `format-results-1345.json` trong `dist/pr-rebase-20261101/`. Kết quả nền cũ giữ riêng trong `base-6cfad22/`.\n- Chỉ kiểm local macOS 27.0.1 arm64 bằng fixture; không chạy app, thiết bị, live GUI, dữ liệu thật hay Pixel. UI chưa được tối ưu; chưa nghiệm thu adapter network/GUI, workflow người dùng hoặc Linux/Windows.\n\n## Rủi ro và hoàn tác\n- Các adapter sau phải giữ network/disk ngoài UI thread và chủ động xử lý errors/cancellation; PR này chỉ cung cấp primitives, không tự thay luồng GUI đang có.\n- Chưa đo before/after trên workflow người dùng hoặc kiểm Windows/Linux. Hoàn tác commit để gỡ module/export/test; không đổi schema hay giao thức.\n\n### Tình trạng CI kế thừa\n- Windows CI của nền `6cfad22` và lần chạy PR trước rebase đều lỗi fixture đường dẫn `resolves_bundle_cli_and_explicit_paths` vì `/Users/fixture` không là absolute path trên Windows. Lần rebase này không sửa test ngoài phạm vi. Kết quả local ở trên là macOS arm64; không đồng nghĩa đã kiểm thử Windows/Linux runtime.\n\n### Tái kiểm sau khi làm sạch package artifacts\n- Chạy lại `cargo clean -p ctdozlite`, `cargo check --locked --all-targets` và các scoped tests đã liệt kê trên đúng SHA `43b12cc5ed210babffe2f703a09e55535a09c1b4`: tất cả exit 0; số test và filtered count được đối chiếu để tránh nhầm artifact giữa worktree. Không đổi source trong lượt tái kiểm.\n",
      "closedAt": "2026-11-01T15:23:04Z",
      "createdAt": "2026-11-01T14:05:09Z",
      "labels": [],
      "number": 53,
      "state": "MERGED",
      "title": "feat(media): giới hạn tài nguyên và cô lập cache"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjM0NjA1OTE=",
        "is_bot": false,
        "login": "thuongtin",
        "name": "Ethan"
      },
      "body": "## Nội dung\n- Ghi `session.json` qua file tạm riêng có quyền `0600` trên Unix, flush hoàn chỉnh rồi rename thay thế.\n- Bổ sung kiểm thử ghi lần hai, lỗi thay thế không phá đích, dọn file tạm và roundtrip định dạng phiên.\n\n## Lý do\n- Tránh cắt cụt file phiên khi ghi thất bại; giữ định dạng `SavedSession` và luồng đăng nhập hiện có.\n\n## Cách thực hiện\n- Nền độc lập mới nhất: upstream `main` tại `1345f5d0cfd24fdb7d54e5caf3a88e3e3b2acd4a`, đã bao gồm các PR upstream #40 và #41. Giữ nguyên thay đổi build/performance và các phần không liên quan của upstream.\n- Không thêm dependency, không đổi `Cargo.toml`/`Cargo.lock`, không đổi giao thức, models, chat hay GUI.\n- Port đúng phần atomic từ `ae9db507d92917e4e98704281585b2da059c0c5e`: helper `data_dir::write_private_atomic`, lời gọi trong `Session::save_to_file`, export thư viện và hai test.\n- Không mang theo refactor fixture `Domains` ngoài phạm vi của commit cũ. File tạm dùng helper `create_new` có sẵn, `sync_all` trước khi thay thế và cleanup best effort khi lỗi.\n\n## Kiểm chứng\n- Nền cuối `1345f5d0cfd24fdb7d54e5caf3a88e3e3b2acd4a`; rebase sạch, range-diff xác nhận delta tính năng không đổi. Giữ binary rename `ctdozlite` và các sửa avatar/media upstream mới.\n- Đạt: `CMAKE_POLICY_VERSION_MINIMUM=3.5 CARGO_TARGET_DIR=/Users/ethan/VibeCoding/zl/target cargo check --locked --all-targets` (exit 0).\n- Đạt: cùng biến môi trường, `cargo test --locked --lib atomic`: 2 passed, 0 failed (exit 0).\n- Đạt: `git diff --check` (exit 0). Affected-file rustfmt đạt trên `data_dir.rs`/`lib.rs`; `session.rs` exit 1 khớp chính xác baseline upstream sau chuẩn hóa path/line, không thêm diff format.\n- Evidence nền cuối: `atomic-1345-check.log`, `atomic-1345-test.log`, `atomic-1345-fmt-*.log`, `atomic-1345-range-diff.txt` trong `dist/pr-rebase-20261101/`. Kết quả nền cũ được giữ riêng trong `base-6cfad22/`.\n- Chỉ kiểm chứng local macOS 27.0.1 arm64; không dùng phiên thật, dữ liệu thật, live GUI hay Pixel. UI chưa được tối ưu và chưa nghiệm thu thao tác GUI; PR này chỉ thay lớp persistence.\n\n## Rủi ro và hoàn tác\n- Kiểm thử quyền Unix và atomic replacement chạy trên macOS arm64, chưa xác minh Linux/Windows. Không khẳng định crash-durability của directory entry hay physical filesystem ngoài fixture.\n- Hoàn tác commit này để trả `save_to_file` về cơ chế cũ; không có migration và không đổi cấu trúc file phiên.\n\n### Tình trạng CI kế thừa\n- Windows CI của nền `6cfad22` và lần chạy PR trước rebase đều lỗi fixture đường dẫn `resolves_bundle_cli_and_explicit_paths` vì `/Users/fixture` không là absolute path trên Windows. Lần rebase này không sửa test ngoài phạm vi. Kết quả local ở trên là macOS arm64; không đồng nghĩa đã kiểm thử Windows/Linux runtime.\n\n### Tái kiểm sau khi làm sạch package artifacts\n- Chạy lại `cargo clean -p ctdozlite`, `cargo check --locked --all-targets` và các scoped tests đã liệt kê trên đúng SHA `92b2445030815e8f6dda4034c491b65cd8772575`: tất cả exit 0; số test và filtered count được đối chiếu để tránh nhầm artifact giữa worktree. Không đổi source trong lượt tái kiểm.\n",
      "closedAt": "2026-11-01T15:18:19Z",
      "createdAt": "2026-11-01T14:05:04Z",
      "labels": [],
      "number": 52,
      "state": "MERGED",
      "title": "fix(session): ghi phiên bằng thay thế nguyên tử"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjM0NjA1OTE=",
        "is_bot": false,
        "login": "thuongtin",
        "name": "Ethan"
      },
      "body": "## Nội dung\n- Đóng gói độc lập 5 helper macOS theo đúng arm64, x86_64 hoặc universal của app.\n- Rebase nguyên patch lên upstream `1345f5d`; 23 file, +1349/-0, không sửa Rust hoặc giao thức.\n\n## Lý do\n- Gói macOS cần đầy đủ helper với kiến trúc đồng nhất và contract headless có thể tái lập.\n\n## Cách thực hiện\n- Compile từng Swift slice, liên kết tĩnh zstd 1.5.7 đã kiểm SHA256 và đưa BSD license vào bundle.\n- Public build không tải/đóng gói codec JS/WASM bên ngoài; missing-assets trả lỗi có hướng dẫn local opt-in.\n- Giữ tên binary mới `ctdozlite` trong workflow, validator và fixture; các CI jobs ngoài macOS giữ nguyên byte.\n- Validator kiểm kiến trúc, system-only dependencies và relocated contracts trước codesign.\n- Giữ nguyên GNU windres/MSVC resource logic và release `strip = true` của upstream.\n\n## Kiểm chứng\n- [x] macOS arm64: build đủ 5 helper universal; tất cả có `x86_64 arm64`.\n- [x] Native packaging 3/3 sau rebase mới, dùng fixture `Contents/MacOS/ctdozlite`; helper Swift byte-identical nên tái sử dụng universal build đã kiểm.\n- [x] Rollback fixture trở về byte-identical upstream `1345f5d`.\n- [x] EmojiPreview 21/21, 2 cluster distinction checks; MediaPreview 9/9.\n- [x] Disposable rollback phục hồi byte-identical upstream; nhánh rebased vẫn giữ patch.\n- [x] Shell syntax, Python AST, diff whitespace.\n- [ ] `cargo fmt --all -- --check`: upstream có formatting failure, native diff không sửa Rust.\n- [x] Native boundary `206292c`: làm sạch package artifacts rồi `cargo check --locked --all-targets`, exit 0.\n- [x] Integration tip `20a7412` chứa native patch cùng đầy đủ feature: format, Clippy all-targets `-D warnings`, full tests 318 library + 142 integration = 460 pass, 9 opt-in ignored. Đây là kiểm chứng macOS arm64, không phải runtime Intel/Windows/Linux.\n- [ ] Full release/DMG và GUI runtime chưa nghiệm thu.\n- [ ] Intel runtime, GUI/permission và visual polish chưa nghiệm thu; UI còn thô. Không đại diện test app thật.\n\n## Rủi ro và hoàn tác\n- Chỉ macOS packaging/helpers; không thay đổi protocol, schema hoặc dependency Rust.\n- Không phân phối asset codec ngoài. Intel chỉ build/metadata-checked trên host arm64.\n- Revert commit native hoặc dùng reverse patch trên fixture. Old SHA `ca23a8a40b6ab973ded6f0d3350557d7140d3c55`; new SHA `206292cf66f1f59f9722519be4390a58d1baa272`.\n- Packaging contract và manual steps: `packaging/macos/NATIVE_HELPERS.md`. Full GUI/permission/DMG cần nghiệm thu riêng.\n",
      "closedAt": "2026-11-01T15:18:05Z",
      "createdAt": "2026-11-01T13:09:23Z",
      "labels": [],
      "number": 51,
      "state": "MERGED",
      "title": "feat(ci): đóng gói và kiểm tra helper macOS độc lập"
    },
    {
      "author": {
        "id": "U_kgDOBkTUYw",
        "is_bot": false,
        "login": "GrootTheDeveloper",
        "name": "Dang Phuc An Khang"
      },
      "body": "## What\n- `[profile.release] strip = true`: Windows release exe 33.4 MB → 24.9 MB\n\n## Why\n- The release exe carried its symbol table\n\n## How\n- strip only; codegen unchanged. Thin LTO + `codegen-units = 1` reached 15.3 MB but used more CPU during the first seconds after start in my measurements, so it is left out\n\n## Verification\n- [x] `cargo test --locked` (107 passed)\n- [ ] `cargo clippy --all-targets -- -D warnings`: `main` already fails it (56 lib warnings); this branch adds none (per-lint counts compared with `main`)\n- [ ] `cargo fmt --all --check`: `main` is not fmt-clean; the touched hunks are rustfmt-formatted\n- [x] Manual: Windows 11 GNU toolchain; startup time / RAM / CPU unchanged versus the unstripped build (3 runs each)\n\n## Risk & Rollback\n- Risk: two-way door; panic backtraces lose symbol names\n- Rollback: revert\n",
      "closedAt": "2026-11-01T13:29:43Z",
      "createdAt": "2026-11-01T13:04:31Z",
      "labels": [],
      "number": 50,
      "state": "MERGED",
      "title": "chore(ci): strip symbols from release binaries"
    },
    {
      "author": {
        "id": "U_kgDOBkTUYw",
        "is_bot": false,
        "login": "GrootTheDeveloper",
        "name": "Dang Phuc An Khang"
      },
      "body": "## What\n- `cargo build` works on `x86_64-pc-windows-gnu` (MSYS2 / MinGW) again\n\n## Why\n- `build.rs` passes the raw `app_icon.res` to the linker; GNU ld cannot read `.res`, so the GNU build fails at link time\n\n## How\n- On `target_env = \"gnu\"`, `windres` (MinGW binutils) converts the `.res` to a COFF object which is linked instead; MSVC path unchanged\n\n## Verification\n- [x] `cargo test --locked` (107 passed)\n- [ ] `cargo clippy --all-targets -- -D warnings`: `main` already fails it (56 lib warnings); this branch adds none (per-lint counts compared with `main`)\n- [ ] `cargo fmt --all --check`: `main` is not fmt-clean; the touched hunks are rustfmt-formatted\n- [x] Manual: Windows 11, Rust 1.99 GNU toolchain, MSYS2 ucrt64; debug and release builds link and run\n\n## Risk & Rollback\n- Risk: two-way door, build script only\n- Blast radius: Windows GNU builds\n- Rollback: revert\n",
      "closedAt": "2026-11-01T13:29:47Z",
      "createdAt": "2026-11-01T13:04:28Z",
      "labels": [],
      "number": 49,
      "state": "MERGED",
      "title": "fix(ci): link the Windows icon on the GNU toolchain"
    },
    {
      "author": {
        "id": "U_kgDOBkTUYw",
        "is_bot": false,
        "login": "GrootTheDeveloper",
        "name": "Dang Phuc An Khang"
      },
      "body": "## What\n- Startup RAM 253 MB → 196 MB when no Han / Kana / Hangul text is present (offline fixture: fake session, 303 threads, 200 messages per 1-1 thread, 62k rows in `zalo_history.db`, network blocked); unchanged once CJK text shows up\n\n## Why\n- `setup_custom_fonts` read `msyh.ttc` (~20 MB) or another CJK font at every start and egui kept it for the whole run\n\n## How\n- Fonts are built without it; `ensure_cjk_font` rebuilds them with the same CJK fallback the first time such text appears in the loaded threads, a `ThreadsUpdated` batch or the composer\n- Same font list and fallback order as before once loaded\n\n## Verification\n- [x] `cargo test --locked` (107 passed)\n- [ ] `cargo clippy --all-targets -- -D warnings`: `main` already fails it (56 lib warnings); this branch adds none (per-lint counts compared with `main`)\n- [ ] `cargo fmt --all --check`: `main` is not fmt-clean; the touched hunks are rustfmt-formatted\n- [x] Manual: Windows 11; fixture with a Chinese/Japanese message renders it (screenshot), RAM measured with and without CJK text, 2 runs each\n\n## Risk & Rollback\n- Risk: two-way door\n- Blast radius: font setup\n- Rollback: revert\n\n## Context\n- Relates to: #5, #6\n",
      "closedAt": "2026-11-01T13:29:55Z",
      "createdAt": "2026-11-01T13:04:25Z",
      "labels": [],
      "number": 48,
      "state": "MERGED",
      "title": "perf(ui): load the CJK fallback font only when CJK text appears"
    },
    {
      "author": {
        "id": "U_kgDOBkTUYw",
        "is_bot": false,
        "login": "GrootTheDeveloper",
        "name": "Dang Phuc An Khang"
      },
      "body": "## What\n- Idle RAM 208 MB → 162–166 MB, idle CPU 3.0–3.4 % → 0.6–1.2 % (offline fixture: fake session, 303 threads, 200 messages per 1-1 thread, 62k rows in `zalo_history.db`, network blocked, a 2 000-message group open)\n\n## Why\n- egui laid out every bubble of the open chat each frame (2 000 galleys per repaint in a large group) and painted every thread row, downloading every avatar\n- Each group bubble scanned all threads to find the sender avatar\n- Images were decoded twice (validate, then use) on the 2-thread async runtime and kept as full-size textures (~8 MB per 1080p photo, never freed) although bubbles draw at most 240 pt\n\n## How\n- Chat view renders the newest 150 bubbles; \"Tải thêm tin nhắn cũ\" widens the window first, then asks the server. Search and the starred filter still scan everything\n- Off-screen thread rows keep their space but skip painting\n- Sender → avatar `HashMap` built once per frame\n- Image fetch: one decode on `spawn_blocking`; message images capped at 480 px; the full-screen viewer and drawer grid request a separate `hd_` texture\n\n## Verification\n- [x] `cargo test --locked` (107 passed)\n- [ ] `cargo clippy --all-targets -- -D warnings`: `main` already fails it (56 lib warnings); this branch adds none (per-lint counts compared with `main`)\n- [ ] `cargo fmt --all --check`: `main` is not fmt-clean; the touched hunks are rustfmt-formatted\n- [x] Manual: Windows 11, screenshots of the thread list and a 2 000-message group chat\n\n## Risk & Rollback\n- Risk: two-way door\n- Blast radius: chat view, thread list, image textures\n- Rollback: revert\n\n## Context\n- Relates to: #5 (large groups flicker / lag)\n",
      "closedAt": "2026-11-01T13:30:00Z",
      "createdAt": "2026-11-01T13:04:21Z",
      "labels": [],
      "number": 47,
      "state": "MERGED",
      "title": "perf(ui): render only what is on screen"
    },
    {
      "author": {
        "id": "U_kgDOBkTUYw",
        "is_bot": false,
        "login": "GrootTheDeveloper",
        "name": "Dang Phuc An Khang"
      },
      "body": "## What\n- The GUI no longer repaints every 1.2 s while idle; idle CPU 3.0–3.4 % → 2.3–2.7 % of one core (offline fixture: fake session, 303 threads, 200 messages per 1-1 thread, 62k rows in `zalo_history.db`, network blocked; the fixture itself triggers a reconnect event every ~2.4 s, so a real idle session saves more)\n\n## Why\n- The `PollMessages` trigger lived in `update()`, so `request_repaint_after(1200 ms)` re-laid out the whole UI forever just to fire a timer\n\n## How\n- A tokio interval in the worker sends `PollMessages` (the handler already skips while a long-poll is in flight or logged out)\n- Thread list for the poll is shared through a `Mutex`, republished whenever the GUI persists threads\n- `GuiTx` wraps the event `Sender` and calls `request_repaint()` on every send, so worker events still show immediately; `call.rs` / `media.rs` take `GuiTx`\n\n## Verification\n- [x] `cargo test --locked` (107 passed)\n- [ ] `cargo clippy --all-targets -- -D warnings`: `main` already fails it (56 lib warnings); this branch adds none (per-lint counts compared with `main`)\n- [ ] `cargo fmt --all --check`: `main` is not fmt-clean; the touched hunks are rustfmt-formatted\n- [x] Manual: Windows 11, idle CPU sampled over 20 s after a 15 s warm-up, 2 runs per build\n\n## Risk & Rollback\n- Risk: two-way door\n- Blast radius: GUI event delivery, message polling cadence (unchanged 1.2 s)\n- Rollback: revert\n",
      "closedAt": "2026-11-01T19:06:35Z",
      "createdAt": "2026-11-01T13:04:19Z",
      "labels": [],
      "number": 46,
      "state": "MERGED",
      "title": "perf(ui): move the message poll timer into the worker"
    },
    {
      "author": {
        "id": "U_kgDOBkTUYw",
        "is_bot": false,
        "login": "GrootTheDeveloper",
        "name": "Dang Phuc An Khang"
      },
      "body": "> Draft. Stacked on #43 (first commit). Not yet exercised against a live account (my phone camera cannot scan the login QR right now); it reuses the same `getoldv2` call and digit-only cursor rule as the existing `LoadGroupHistory` backfill. Overlaps in purpose with the history parts of #42; happy to close if that lands first.\n\n## What\n- Settings → \"Đồng bộ ngay\" pages old history for every group and \"Cloud của tôi\" into `zalo_history.db`, with progress in the status line and a \"Dừng\" button\n- Each run resumes from the oldest stored server message, so pressing it again goes further back\n- \"Tải thêm tin nhắn cũ\" now reaches further back on every click\n\n## Why\n- #23: the button only re-fetched the thread list\n- \"Tải thêm\" re-read the same newest 500 rows from SQLite, so its `getoldv2` cursor never got past them\n\n## How\n- New `history_sync.rs`: 50 messages per page, up to 20 pages per thread per run, 400 ms between pages, one thread at a time; each page persisted immediately; SQLite work on `spawn_blocking` (no connection held across an await)\n- `store::oldest_msg_id` skips local ids (`out_*`, `{thread}_{ts}`) the server rejects as cursors\n- `LoadGroupHistory { have }` reads `have + 300` rows deep\n- `upsert_thread` keeps the larger `last_ts`, so persisting older history does not move a thread down the list\n- Settings text says 1-1 history is not available over HTTP\n\n## Verification\n- [x] `cargo test --locked` (110 passed)\n- [ ] `cargo clippy --all-targets -- -D warnings`: `main` already fails it (56 lib warnings); this branch adds none (per-lint counts compared with `main`)\n- [ ] `cargo fmt --all --check`: `main` is not fmt-clean; the touched hunks are rustfmt-formatted\n- [x] Tests: cursor stops when a page brings nothing older; `oldest_msg_id` skips local ids; `last_ts` never goes back\n- [ ] Manual against a live account: pending\n\n## Risk & Rollback\n- Risk: two-way door; read-only API calls, rate kept low (one request / 400 ms)\n- Blast radius: Settings sync tab, \"Tải thêm\", `threads.last_ts`\n- Rollback: revert\n\n## Context\n- Resolves: #23\n- Relates to: #19, #10, #42\n",
      "closedAt": "2026-11-01T19:06:35Z",
      "createdAt": "2026-11-01T13:04:15Z",
      "labels": [],
      "number": 45,
      "state": "MERGED",
      "title": "feat(chat): resumable history sync for groups and Cloud"
    },
    {
      "author": {
        "id": "U_kgDOBkTUYw",
        "is_bot": false,
        "login": "GrootTheDeveloper",
        "name": "Dang Phuc An Khang"
      },
      "body": "> Stacked on #43 (first commit). Please review the last 3 commits; I will rebase once #43 lands.\n\n## What\n- Window shows in 0.61–0.70 s instead of 1.04–1.12 s with a large history (Windows 11, release build, offline fixture: fake session, 303 threads, 200 messages per 1-1 thread, 62k rows in `zalo_history.db`, network blocked)\n\n## Why\n- Every hydrate ran `DELETE FROM messages WHERE text = 'sendBubbleMessage'` (full table scan) and loaded every `(msg_id, thread_id)` row in the database into a HashMap\n- `new()` hydrated from SQLite a second time and rewrote `threads_cache.json` right after `main` had already loaded it\n- `merge_thread_into` / `merge_group_history` scanned the whole target thread for every incoming message (1 000 into 1 000 = a million comparisons)\n\n## How\n- `PRAGMA user_version` migration 1 runs the cleanup once; `persist_threads` filters those rows instead\n- Owner lookups use the primary key for messages actually in memory, inside one transaction (in autocommit each SELECT takes and drops the file lock, ~50 µs on Windows)\n- Thread cache loaded once; startup/refresh preload 20 messages per thread (`MessageStore::PRELOAD_MSGS`); opening a thread still reads 500\n- `threads_cache.json` is written compact with the last 20 messages per thread\n- Merges use HashMap indexes with the same match rules (id; or text + direction at the same minute / within 5 s); new test pins those rules\n\n## Verification\n- [x] `cargo test --locked` (108 passed)\n- [ ] `cargo clippy --all-targets -- -D warnings`: `main` already fails it (56 lib warnings); this branch adds none (per-lint counts compared with `main`)\n- [ ] `cargo fmt --all --check`: `main` is not fmt-clean; the touched hunks are rustfmt-formatted\n- [x] Manual: Windows 11, GNU toolchain, startup timed 2× fresh fixture + 2× second launch per build\n\n## Risk & Rollback\n- Risk: two-way door; migration only deletes placeholder rows that were already deleted on every start\n- Blast radius: startup, thread refresh, cache file format (still the same JSON schema)\n- Rollback: revert; an old build reads the compact cache fine\n\n## Context\n- Relates to: #5, #9\n",
      "closedAt": "2026-11-01T13:29:51Z",
      "createdAt": "2026-11-01T13:04:13Z",
      "labels": [],
      "number": 44,
      "state": "MERGED",
      "title": "perf(store): open the window faster with a large history"
    },
    {
      "author": {
        "id": "U_kgDOBkTUYw",
        "is_bot": false,
        "login": "GrootTheDeveloper",
        "name": "Dang Phuc An Khang"
      },
      "body": "## What\n- `messages_for(thread, n)` returns the newest `n` messages (still oldest-first) instead of the oldest `n`\n- `persist_threads` runs in one transaction\n\n## Why\n- `ORDER BY ts ASC LIMIT n` handed back the *oldest* rows, so any thread with more stored history than the hydrate limit (200 at startup, 500 when opened) reopened on stale messages and the newest ones were missing\n- In autocommit every inserted row was its own fsync\n\n## How\n- `SELECT ... FROM (SELECT ... ORDER BY ts DESC LIMIT ?2) ORDER BY ts ASC`\n- `BEGIN` / `COMMIT` around the existing loop, `ROLLBACK` on error\n\n## Verification\n- [x] `cargo test --locked` (107 passed)\n- [ ] `cargo clippy --all-targets -- -D warnings`: `main` already fails it (56 lib warnings); this branch adds none (per-lint counts compared with `main`)\n- [ ] `cargo fmt --all --check`: `main` is not fmt-clean; the touched hunks are rustfmt-formatted\n- [x] Store test extended: `messages_for(t, 2)` on rows ts 100/200/300 returns [200, 300]\n\n## Risk & Rollback\n- Risk: two-way door, read query only, no schema change\n- Blast radius: every SQLite history read\n- Rollback: revert the commit\n\n## Context\n- Relates to: #10, #19, #23\n",
      "closedAt": "2026-11-01T13:29:53Z",
      "createdAt": "2026-11-01T13:04:04Z",
      "labels": [],
      "number": 43,
      "state": "MERGED",
      "title": "fix(store): return the newest rows from messages_for"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjM0NjA1OTE=",
        "is_bot": false,
        "login": "thuongtin",
        "name": "Ethan"
      },
      "body": "# PR #42: chuỗi đóng góp đã rebase và chia theo dependency\n\nKhông yêu cầu review/merge monolith này. Giữ nguyên source cũ để tham chiếu; mỗi PR bên dưới có delta riêng.\n\n## Nền và trạng thái hiện tại\n\n- Upstream `main`: `1345f5d0cfd24fdb7d54e5caf3a88e3e3b2acd4a`; đã có #40/#41 và các thay đổi history/render/CJK, GNU/MSVC icon, release strip, đổi binary thành `ctdozlite`, avatar nhóm và group-call/audio mới.\n- Integration tip: `20a7412a5a78c722a709f12252ce5fe30f6a3db7`. Đã giải quyết conflict bằng cách giữ identity/account scope/paging/emoji/media của chuỗi cùng thay đổi upstream, không chọn đè toàn bộ một phía.\n- Không thay app trong Applications, phiên đăng nhập, kho dữ liệu thật hoặc Pixel trong lần rebase này.\n\n## PR độc lập đã mở upstream, ready for review\n\n| PR | Nội dung | Phạm vi |\n|---|---|---|\n| [#51](https://github.com/ctdoteam1110/ctdozlite/pull/51) | Native helpers/build/CI | 5 helper, kiến trúc và relocated contracts; đã rebase, giữ binary mới |\n| [#52](https://github.com/ctdoteam1110/ctdozlite/pull/52) | Atomic session persistence | Writer private, flush/rename, rollback khi lỗi |\n| [#53](https://github.com/ctdoteam1110/ctdozlite/pull/53) | Bounded media resources | Giới hạn bytes/cache/queue, account identity và cancellation |\n| [#54](https://github.com/ctdoteam1110/ctdozlite/pull/54) | Transactional updater engine | Download bounded/hash, stage/backup, rollback khi launch lỗi |\n| [#55](https://github.com/ctdoteam1110/ctdozlite/pull/55) | Message/reaction/unread models | Stable IDs, delivery/reaction provenance, sidebar/unread data, giữ clock tests |\n\nCác PR này dựa trực tiếp trên upstream main, không kéo theo format sweep hoặc các lớp GUI chưa đến lượt. Chỉ review/merge mỗi thay đổi một lần; bản draft tương ứng trên fork vẫn làm dependency của chuỗi.\n\n## Chuỗi delta đã rebase trên fork\n\nCác PR phụ thuộc giữ draft trên fork. Sau khi PR nền vào upstream, rebase/gửi tiếp từng lớp; không mở PR cộng dồn hàng nghìn dòng của dependency để tránh làm khó review. Chưa có PR upstream riêng cho từng lớp phụ thuộc.\n\n| Thứ tự | Draft review | Delta sau rebase |\n|---|---|---|\n| 1 | [style(core): tách định dạng Rust khỏi thay đổi chức năng](https://github.com/thuongtin/zl/pull/2) | 29 files, +2758/-1604 |\n| 2 | [feat(chat): bảo toàn model và định danh tin nhắn](https://github.com/thuongtin/zl/pull/3) | 12 files, +2219/-17 |\n| 3 | [fix(session): ghi phiên và cache bằng thay thế nguyên tử](https://github.com/thuongtin/zl/pull/4) | 3 files, +99/-16 |\n| 4 | [feat(media): giới hạn tải và cô lập cache theo tài khoản](https://github.com/thuongtin/zl/pull/5) | 3 files, +691/-0 |\n| 5 | [feat(core): tải và cài cập nhật với rollback](https://github.com/thuongtin/zl/pull/6) | 3 files, +763/-0 |\n| 6 | [fix(chat): hợp nhất message và metadata theo định danh gốc](https://github.com/thuongtin/zl/pull/7) | 9 files, +1955/-381 |\n| 7 | [feat(media): truyền tệp giới hạn và gia hạn preview cũ](https://github.com/thuongtin/zl/pull/8) | 2 files, +1012/-83 |\n| 8 | [feat(net): bổ sung sự kiện lịch sử và trạng thái socket](https://github.com/thuongtin/zl/pull/9) | 1 files, +718/-27 |\n| 9 | [feat(chat): xác thực đã xem, nhận tin và trạng thái truy cập](https://github.com/thuongtin/zl/pull/10) | 8 files, +1514/-0 |\n| 10 | [feat(store): lịch sử theo tài khoản và migration có phiên bản](https://github.com/thuongtin/zl/pull/11) | 9 files, +3567/-103 |\n| 11 | [feat(chat): bảo toàn reaction lịch sử và replay journal](https://github.com/thuongtin/zl/pull/12) | 5 files, +1191/-0 |\n| 12 | [feat(core): cô lập scope và lưu draft outbox bền vững](https://github.com/thuongtin/zl/pull/13) | 3 files, +622/-0 |\n| 13 | [feat(ui): contract thông báo macOS qua helper](https://github.com/thuongtin/zl/pull/14) | 5 files, +186/-1 |\n| 14 | [chore(core): tách sửa lint khỏi các PR chức năng](https://github.com/thuongtin/zl/pull/15) | 8 files, +131/-153 |\n| 15 | [feat(ui): hiển thị emoji và font Unicode nguyên văn](https://github.com/thuongtin/zl/pull/16) | 17 files, +3180/-27 |\n| 16 | [feat(ui): cô lập tài khoản và hoàn tất lưu trước khi đóng](https://github.com/thuongtin/zl/pull/17) | 10 files, +4229/-315 |\n| 17 | [feat(ui): nối worker lịch sử và preview tài khoản](https://github.com/thuongtin/zl/pull/18) | 6 files, +2627/-491 |\n| 18 | [feat(ui): điều khiển đồng bộ lịch sử từ điện thoại](https://github.com/thuongtin/zl/pull/19) | 3 files, +2020/-5 |\n| 19 | [feat(ui): hiển thị hội thoại reply mention và composer](https://github.com/thuongtin/zl/pull/20) | 5 files, +1993/-1217 |\n| 20 | [feat(ui): tích hợp cài đặt và hoàn tất modal hội thoại](https://github.com/thuongtin/zl/pull/21) | 2 files, +232/-685 |\n| 21 | [test(chat): harness và tài liệu kiểm chứng toàn chuỗi](https://github.com/thuongtin/zl/pull/22) | 11 files, +601/-38 |\n\n## Kiểm chứng\n\n- Bản tích hợp: 318 library unit + 142 integration = **460 tests pass**, 9 opt-in ignored; main chạy lặp unit tests không tính trùng. `cargo fmt --all -- --check` và Clippy all-targets `-D warnings` đạt.\n- Boundary: 21/21 checkpoint đã được check all-targets riêng, làm sạch package artifacts trước từng SHA. Số này được cập nhật theo kiểm tra cuối.\n- Đã sửa slicing trung gian PR #17: giữ state render-window upstream tới PR #20, nơi renderer thay bằng paging200. Final tree không đổi; không dùng stub để qua compile.\n- Regression bổ sung: indexed group merge/enrich client ID trong cùng batch; distinct ID không bị gộp theo text/time; compact legacy cache20 và newest500/tie; per-context lazy CJK giữ Unicode families; avatar UID/name ambiguity; HTTP cache miss/hit cho Preview480 và raster Full.\n- Fixture remote storage đổi sang endpoint profile nhóm của upstream mới; vẫn kiểm tên người gửi theo UID chính xác, không hydrate/ghi legacy ở GUI.\n- Native: 5 helper universal compile; packaging 3/3, emoji 21/21 và 2 cluster distinctions, media 9/9 đạt. Intel chỉ build/metadata, không phải runtime.\n- Rollback source trên clone riêng phục hồi byte-identical `22ba405`, worktree clean; hai test atomic đạt. Không down-migration database.\n- Synthetic benchmark trước/sau: 5000 tin, 200 switches yêu cầu, soak2s trên storage/window. Đây không phải GUI soak hoặc kết luận về RAM/crash của app thật.\n- Windows CI đang lỗi fixture đường dẫn có sẵn trên upstream: `/Users/fixture` không là absolute path trên Windows. Baseline run 41312405310 cũng lỗi cùng test; không gán lỗi đó cho atomic/media/updater hoặc coi CI đa nền tảng là runtime QA.\n\n## Tính năng giữ đầy đủ và giới hạn\n\nGiữ emoji/fonts ở bubble/sidebar, ảnh/video/transfer/preview renewal, lịch sử 1-1/nhóm/điện thoại, reaction/replay journal, unread/read/delivery/presence, account persistence, drafts/outbox/shutdown, reply jump, clickable mention, sidebar/avatar/người gửi cuối và native notification. Không bỏ phần UI chức năng để giảm diff.\n\n**Chỉ mới kiểm thử local trên Mac, cụ thể macOS arm64. Giao diện chưa được tối ưu/hoàn thiện về thiết kế.** Windows/Linux/Mac Intel runtime, GUI pixel/accessibility, permission, cuộc gọi thật và soak dài chưa nghiệm thu. Public history codec assets không được tự đóng gói; thiếu assets không được mô tả thành sync thành công.\n",
      "closedAt": "2026-11-01T13:37:47Z",
      "createdAt": "2026-11-01T12:30:11Z",
      "labels": [],
      "number": 42,
      "state": "CLOSED",
      "title": "docs(chat): mục lục các PR nhỏ thay thế bản tổng"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjM0NjA1OTE=",
        "is_bot": false,
        "login": "thuongtin",
        "name": "Ethan"
      },
      "body": "## Thay đổi (What)\n\nBổ sung cơ chế chọn và kiểm tra thư mục dữ liệu **trước khi mở phiên đăng nhập, cache, log hoặc cơ sở dữ liệu**:\n\n- `--data-dir <PATH>` và `--data-dir=PATH` để chọn rõ thư mục dữ liệu.\n- `--print-data-dir` để chuẩn bị, in thư mục được chọn rồi thoát trước các thao tác khác.\n- `--version`/`-V` thoát trước khi chuẩn bị nơi lưu dữ liệu, không tạo Application Support chỉ để xem phiên bản.\n\n**Phạm vi PR: fix độc lập về khởi động/lưu trữ. UI tổng thể chưa được tối ưu/hoàn thiện, không phải lý do loại các tính năng có UI khỏi đóng góp. Chỉ mới kiểm thử trên Mac (macOS 27.0.1, Apple Silicon arm64); Windows và Linux chưa được kiểm thử.**\n\n## Lý do (Why)\n\nỨng dụng macOS dạng `.app` có thể được mở từ Finder với thư mục làm việc `/`. Các đường dẫn lưu tương đối cần trỏ về một nơi ổn định và ghi được, thay vì dựa vào thư mục khởi chạy.\n\nNguồn trước sửa đã có hàm chuyển thư mục làm việc, nhưng chạy trước kiểm tra phiên bản, bỏ qua một số lỗi và chưa có tùy chọn chọn đường dẫn rõ ràng. Điều này khiến `--version` của bundle cũng có thể tạo Application Support, còn lỗi chọn nơi lưu không được phát hiện ngay ở đầu khởi động.\n\n## Cách thực hiện (How)\n\n### Chọn đường dẫn rõ ràng\n\n- Tách bộ phân giải đường dẫn thuần `data_dir::resolve`; kiểm thử được mà không thay đổi thư mục làm việc hoặc biến môi trường của tiến trình kiểm thử.\n- Ưu tiên đường dẫn được chỉ định bằng `--data-dir`; đường dẫn tương đối được tính từ thư mục làm việc ban đầu.\n- Với bundle macOS đúng cấu trúc `.app/Contents/MacOS`, chọn `~/Library/Application Support/ctdozlite`; báo lỗi khi thiếu `HOME` hoặc `HOME` không phải đường dẫn tuyệt đối.\n- Với CLI ngoài bundle, giữ thư mục làm việc hiện tại để tương thích dữ liệu đang lưu theo đường dẫn tương đối. Không tự tìm, di chuyển, nhập hoặc xóa dữ liệu ở thư mục khác.\n\n### Kiểm tra nơi lưu trước khi khởi động\n\n- Chuẩn bị thư mục explicit/bundle với quyền `0700` trên Unix; không đổi quyền của thư mục làm việc cũ của CLI.\n- Dùng tệp kiểm tra quyền ghi có tên riêng, mở bằng `create_new` và quyền `0600` trên Unix; xóa tệp kiểm tra sau khi xác nhận ghi được.\n- Trả lỗi rõ ràng nếu tạo thư mục, đặt quyền, kiểm tra quyền ghi hoặc chọn thư mục làm việc thất bại. Không tiếp tục với một nơi lưu chưa được kiểm chứng.\n- Chỉ đổi thư mục làm việc một lần, trước khi tạo worker hoặc mở trạng thái bền vững, để giữ tương thích với các nơi đang dùng đường dẫn tương đối.\n\n### Xử lý tham số và tài liệu\n\n- Báo lỗi khi `--data-dir` thiếu giá trị, giá trị rỗng hoặc bị chỉ định nhiều lần.\n- Cú pháp cách nhau bằng dấu cách từ chối giá trị giống cờ như `-d`, `-v`, `--logout`, tránh vừa hiểu thành tên thư mục vừa bật chế độ khác. Nếu thực sự muốn thư mục bắt đầu bằng dấu gạch, dùng `--data-dir=-d` hoặc `--data-dir ./-d`.\n- Giữ ưu tiên xem phiên bản toàn cục của `--version`/`-V`.\n- `--print-data-dir` thoát trước xử lý đăng xuất, kể cả khi đi kèm `--logout`.\n- Bổ sung hướng dẫn CLI, kiểm thử bộ phân giải và kiểm thử thực thi tiến trình với thư mục/HOME dùng một lần.\n\nFix nhỏ này không thay đổi schema, cơ chế ghi session, giao thức, dependency hoặc giao diện. Session atomic, kho lịch sử theo tài khoản và toàn bộ nhóm tính năng hội thoại/UI liên quan được đóng góp trong PR #42; tách ra để PR thư mục dữ liệu vẫn độc lập và dễ rà soát.\n\n## Kiểm chứng (Verification)\n\nMôi trường đã chạy: **macOS 27.0.1 arm64**. Các ca kiểm thử dùng thư mục tạm và nội dung mẫu, không dùng dữ liệu tài khoản thật hoặc gọi mạng. Bundle được kiểm bằng executable trong `.app` giả lập, không phải thao tác Finder/GUI thực tế.\n\n| Kiểm tra | Kết quả đã quan sát |\n| --- | --- |\n| Nguồn trước sửa cùng 5 kiểm thử CLI hồi quy | `0 passed, 5 failed`; mã thoát `101`. Các ca khởi động có `--logout` trên dữ liệu mẫu để tránh đi vào mạng/GUI ở bản cũ |\n| `cargo test --offline --locked --test data_directory_cli --bin zalo-rust -- --nocapture` sau sửa | Bộ binary: `112 passed, 0 failed, 1 ignored`; CLI: `6 passed, 0 failed`; mã thoát `0` |\n| Bộ phân giải và quyền thư mục | Kiểm tra đường dẫn tuyệt đối/tương đối/cú pháp `=`, bundle/CLI, thiếu `HOME`, tham số lỗi, quyền `0700`, giữ nội dung cũ và dọn tệp kiểm tra quyền ghi |\n| Bản executable được lưu riêng sau build, kiểm tra CLI bổ sung | `6/6` thành công: từ chối cờ làm giá trị, chấp nhận tên thư mục có dấu gạch bằng cú pháp tường minh, ưu tiên `-V` và không tạo `HOME` khi xem phiên bản |\n| `git diff --check` | Thành công |\n| `rustfmt --edition 2021 --check src/data_dir.rs tests/data_directory_cli.rs` | Thành công |\n| Hoàn tác trên bản sao dùng một lần | Nguồn và tài liệu gốc khớp baseline, các file mới được gỡ; executable cũ tái hiện hành vi chưa hỗ trợ `--data-dir` |\n\n- Module mới qua kiểm tra riêng bằng `clippy-driver -D warnings`.\n- `cargo clippy --offline --locked --bin zalo-rust --test data_directory_cli -- -D warnings` còn thất bại ở 50 lỗi của các module library không sửa trong PR, trước bước kiểm tra binary/integration. Không khẳng định lint toàn kho mã hoặc CI đã đạt.\n- Windows/Linux, Finder/GUI tương tác và phiên đăng nhập thật của riêng nhánh PR này chưa kiểm thử. Không dùng kết quả của bản cá nhân có thêm thay đổi khác để thay thế các kiểm tra còn thiếu.\n\n## Rủi ro và hoàn tác (Risk & Rollback)\n\n- CLI ngoài bundle chạy ở thư mục không ghi được nay báo lỗi thay vì tự chọn thư mục khác; người dùng có thể chỉ định `--data-dir` để chọn nơi lưu rõ ràng.\n- Bundle dùng cùng thư mục Application Support hiện có; dữ liệu ở thư mục khác vẫn được giữ nguyên, không tự chuyển sang nơi mới.\n- Thư mục explicit/bundle đã tồn tại được đặt lại quyền `0700` trên Unix; không sửa quyền các file bên trong. Revert code không tự khôi phục quyền thư mục đã thay đổi.\n- Có thể revert commit của PR để quay về cơ chế khởi động cũ, không cần sửa schema hoặc xóa dữ liệu. Bản cũ chưa hiểu `--data-dir`, vì vậy cần chọn đúng thư mục làm việc trước khi chạy lại.\n- Cần kiểm chứng Windows/Linux trước khi kết luận hành vi trên các nền tảng đó.\n",
      "closedAt": "2026-11-01T13:15:57Z",
      "createdAt": "2026-11-01T12:06:21Z",
      "labels": [],
      "number": 41,
      "state": "MERGED",
      "title": "fix(store): chọn thư mục dữ liệu ghi được trước khi khởi động"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjM0NjA1OTE=",
        "is_bot": false,
        "login": "thuongtin",
        "name": "Ethan"
      },
      "body": "## Thay đổi (What)\n\nSửa cách hiển thị thời gian tin nhắn và hội thoại để dùng **múi giờ của hệ điều hành**, thay vì mặc định hiển thị UTC từ phép chia epoch.\n\n**Phạm vi PR: fix độc lập về tính đúng đắn của thời gian. UI tổng thể chưa được tối ưu/hoàn thiện, không phải lý do loại các tính năng có UI khỏi đóng góp. Chỉ mới kiểm thử trên Mac (macOS 27.0.1, Apple Silicon arm64); Windows và Linux chưa được kiểm thử.**\n\n## Lý do (Why)\n\nCách tính giờ cũ lấy epoch, chia giây rồi modulo 24. Kết quả là giờ UTC chứ không phải giờ địa phương của người dùng. Ví dụ với timestamp `1793467800000` và `TZ=Asia/Ho_Chi_Minh`, kết quả cũ là `17:30`, trong khi giờ địa phương đúng là `00:30` ngày kế tiếp.\n\nNếu chỉ sửa phần parse tin nhắn mới thì lịch sử đọc lại từ SQLite và thời gian dự phòng vẫn có thể khác nhau. Vì vậy PR sửa cả các đường hiển thị liên quan, không thay timestamp được lưu.\n\n## Cách thực hiện (How)\n\n- Thêm bộ định dạng chung `clock::message_time` dựa trên `chrono::Local` để lấy múi giờ của hệ điều hành, không cộng cứng 7 giờ.\n- Chấp nhận timestamp dương ở đơn vị giây hoặc mili giây khi định dạng; dùng cùng một thời điểm để tạo chuỗi `HH:MM`.\n- Trả chuỗi rỗng khi giá trị không hợp lệ hoặc nằm ngoài miền ngày giờ mà bộ định dạng hỗ trợ. Giữ cách xử lý dự phòng hiện có cho tin nhắn thiếu timestamp.\n- Áp dụng bộ định dạng chung vào `Thread.time`, `ChatMessage.time` khi parse dữ liệu và khi khôi phục lịch sử từ SQLite.\n- Chuyển `ui::clock()` sang giờ địa phương, giữ định dạng có giây cho các nhãn trạng thái hiện tại.\n- Giữ nguyên epoch gốc trong dữ liệu; không sửa thứ tự tin nhắn, cấu trúc lưu trữ hoặc giao thức mạng.\n- Bổ sung `chrono` với các tính năng `clock` và `std`, tắt tính năng mặc định không cần thiết. Phụ thuộc này phục vụ đọc múi giờ và định dạng thời gian, không thêm yêu cầu mạng hay đo lường từ xa.\n- Bổ sung kiểm thử timestamp giây/mili giây, chuyển ngày ở ranh giới múi giờ, giá trị âm/0/ngoài miền, parse hội thoại/tin nhắn và khôi phục SQLite trong bộ nhớ.\n\nPR được tách độc lập từ `v0.0.3`. Emoji/font, preview ảnh/video, lịch sử, reaction, đọc/chưa đọc, điều hướng và các thay đổi UI chức năng được đóng góp trong PR #42; không nằm trong fix thời gian nhỏ này.\n\n## Kiểm chứng (Verification)\n\nMôi trường đã chạy: **macOS 27.0.1 arm64, Rust 1.91.1**. Dữ liệu kiểm thử là mẫu tổng hợp và SQLite trong bộ nhớ, không dùng phiên đăng nhập hay cơ sở dữ liệu thật.\n\n| Kiểm tra | Kết quả đã quan sát |\n| --- | --- |\n| Kiểm thử hồi quy trên nguồn trước sửa, timestamp `1793467800000`, `TZ=Asia/Ho_Chi_Minh` | Nhận `17:30` thay vì `00:30`; 1 kiểm thử thất bại, mã thoát `101` |\n| `TZ=Asia/Ho_Chi_Minh cargo test --offline --locked` sau sửa | Mỗi bộ library và main: `113 passed, 0 failed, 1 ignored`; mã thoát `0`. Hai bộ có phần trùng nhau, không cộng thành số kiểm thử độc lập |\n| `cargo test --offline --locked --lib local_time` với từng `TZ=Asia/Ho_Chi_Minh`, `TZ=UTC`, `TZ=America/New_York` | Mỗi lượt `3 passed, 0 failed`; mã thoát `0` |\n| `cargo build --offline --locked --bin zalo-rust` và `--version` | Thành công; phiên bản `ctdozlite v0.0.3 (zalo-rust)` |\n| `git diff --check` | Thành công |\n| Hoàn tác trên bản sao dùng một lần | Các file gốc khớp từng byte, module mới được gỡ; kiểm thử hồi quy tái hiện giờ UTC `17:30` như trước sửa |\n\nCác kiểm thử múi giờ ở trên đều chạy trên cùng máy Mac, **không phải bằng chứng đã chạy Windows/Linux**.\n\n- `cargo clippy --offline --locked --all-targets -- -D warnings`: nguồn trước và sau sửa đều thất bại với 56 tóm tắt chẩn đoán; đối chiếu không thấy thêm hoặc mất tóm tắt chẩn đoán. Không khẳng định toàn bộ lint đã đạt.\n- `cargo fmt --all --check`: nguồn trước và sau sửa đều báo khác biệt định dạng có sẵn, bắt đầu từ `build.rs`. Chỉ định dạng phần mới, không kèm đợt định dạng lại toàn kho mã.\n- Chưa kiểm thử GUI/TUI tương tác hoặc bản cài đặt riêng của nhánh PR này; chưa kiểm thử trên Windows/Linux. Các kết quả của bản cá nhân có thêm thay đổi khác không được dùng để chứng minh nhánh PR này.\n\n## Rủi ro và hoàn tác (Risk & Rollback)\n\n- Giờ hiển thị phụ thuộc vào cấu hình múi giờ của hệ điều hành. Nếu máy đặt sai múi giờ thì ứng dụng cũng sẽ hiển thị theo cấu hình đó.\n- Cách nhận diện timestamp giây/mili giây có ngưỡng rõ ràng và được kiểm thử; epoch gốc vẫn được giữ nguyên, không ghi lại theo giờ địa phương.\n- Không có thay đổi schema hoặc di chuyển dữ liệu. Có thể revert commit của PR để quay lại bộ định dạng trước đây cùng manifest/lockfile cũ; không cần xóa dữ liệu người dùng.\n- Windows/Linux và kiểm tra giao diện thực tế còn cần được xác nhận trước khi kết luận hỗ trợ đa nền tảng.\n",
      "closedAt": "2026-11-01T12:56:10Z",
      "createdAt": "2026-11-01T12:06:16Z",
      "labels": [],
      "number": 40,
      "state": "MERGED",
      "title": "fix(chat): hiển thị giờ tin nhắn theo múi giờ hệ điều hành"
    },
    {
      "author": {
        "id": "U_kgDODG0YpA",
        "is_bot": false,
        "login": "cypherdo37",
        "name": "Đỗ Thành Công"
      },
      "body": "## What\n- Thêm job `pr-guard` vào workflow CI.\n- Tự động quét và chặn commit nhầm `session.json`, database `*.db`, file log hoặc credentials.\n- Kiểm tra chuẩn tiêu đề Conventional Commits.\n- Kiểm tra độ dài mô tả PR.\n- Xuất bảng Checklist đánh giá tự động trực tiếp trên tab Summary của PR.\n\n## Why\n- Miễn phí 100%, bảo vệ repo an toàn mà không cần thêm API key nào.\n\n## Verification\n- Chạy thử nghiệm trên chính PR này.",
      "closedAt": "2026-11-01T11:21:05Z",
      "createdAt": "2026-11-01T11:19:38Z",
      "labels": [],
      "number": 39,
      "state": "CLOSED",
      "title": "ci(pr): thêm bot kiểm tra PR tự động miễn phí 100%"
    },
    {
      "author": {
        "id": "U_kgDODG0YpA",
        "is_bot": false,
        "login": "cypherdo37",
        "name": "Đỗ Thành Công"
      },
      "body": "## What\n- Thêm workflow `.github/workflows/pr-check.yml` kiểm tra tự động cho mọi Pull Request.\n- Kiểm tra bảo mật: chặn commit nhầm `session.json`, `*.db`, file log hoặc credentials.\n- Kiểm tra chuẩn tiêu đề Conventional Commits và mô tả PR.\n- Kiểm tra format mã nguồn Rust (`cargo fmt`).\n- Tự động comment bảng checklist đánh giá lên PR bằng `github-actions[bot]` mà không cần bất kỳ API key nào.\n\n## Why\n- Team nhỏ, cần tự động hóa việc review cơ bản và chặn rò rỉ dữ liệu nhạy cảm mà hoàn toàn miễn phí.\n\n## Verification\n- Workflow tự kích hoạt và chạy trên chính PR này.",
      "closedAt": "2026-11-01T11:16:37Z",
      "createdAt": "2026-11-01T11:08:00Z",
      "labels": [],
      "number": 37,
      "state": "CLOSED",
      "title": "ci(pr): thêm bot kiểm tra PR tự động miễn phí 100%"
    },
    {
      "author": {
        "id": "MDQ6VXNlcjg4NzMzNTAy",
        "is_bot": false,
        "login": "TIPC1110",
        "name": "ctdo"
      },
      "body": "Test PR to verify @gemini-code-assist bot review",
      "closedAt": "2026-11-01T11:04:21Z",
      "createdAt": "2026-11-01T11:02:25Z",
      "labels": [],
      "number": 36,
      "state": "CLOSED",
      "title": "docs: test gemini code assist bot"
    },
    {
      "author": {
        "id": "U_kgDOCbz4Lg",
        "is_bot": false,
        "login": "ngxccc",
        "name": "Ngoc Tran"
      },
      "body": "> Independent of #29, but both touch `session.rs`: #29 rewrites the domain heal path, this one the session status path. Merging #29 first keeps the conflict trivial.\n\n## What\n\n- `SessionStatus { Valid, Expired, Unreachable }` replaces `validate_session() -> bool`. Only `error_code` `600` / `-102` mean the credentials are gone; transport, HTTP, parse and unrelated codes are `Unreachable`.\n- `SessionError` carries the numeric code, so `send()` and the seven `chat.rs` call sites no longer bury it in a formatted string, and `is_session_expired_error(&str)` (substring matching) is gone.\n- `session.json` is never deleted for a failed request again. On `Expired` the file keeps every credential and gains an `expired_at` marker; the next start goes straight to the login prompt, and only a successful login rewrites the file (`apply_login_config` clears the marker).\n- TUI startup: `Expired` marks the file and asks for a login, `Unreachable` keeps the session and starts from cached threads plus local history (`r` retries). GUI: `GuiEvent::SessionUnreachable` shows the reason and keeps the user signed in; the QR prompt is only opened for a real `Expired`.\n- Logout no longer deletes `zalo_history.db`. Clearing local data is its own confirmed action: GUI menu entry behind a dialog, TUI `w` behind a confirmation.\n- TUI user-facing strings now use fully accented Vietnamese (statuses, help screen, confirmations, headers). Requested in review; log contexts, protocol strings and identifiers are untouched.\n\n## Why\n\nAny network hiccup at startup was read as \"session expired\": the client deleted `session.json`, logged out, and opened the QR screen with its local history at risk. `validate_session()` returned `false` for every `Err`, so a 404, a timeout and a real `600` were indistinguishable.\n\n## How\n\n- Status is decided by one function, and the file policy follows the status: `Expired` prompts, `Unreachable` keeps everything, `Valid` proceeds. `main.rs` only fetches the friend list when the status is `Valid`, so an offline start is quiet.\n- `expired_at` is written under `#[serde(default, skip_serializing_if)]`, so old files load unchanged and healthy sessions write no marker.\n\n## Verification\n\n```\ncargo test --locked → 107 passed; 0 failed (5 new tests)\ncargo build --locked → no warnings\n```\n\nNew tests: error-code mapping (`0` / `600` / `-102` / `111`), `anyhow` error mapping (typed `600` → `Expired`, `HTTP 404`, connection refused, decode failure → `Unreachable`), the `expired_at` round-trip keeping credentials, the marker clearing on login, and a ratatui `TestBackend` render of the wipe confirmation.\n\nReal binary, offline (`unshare -rn`) with a saved session:\n\n```\n⚠️ Không kết nối được máy chủ Zalo — giữ nguyên phiên, chạy tiếp với dữ liệu cục bộ. Bấm 'r' để thử lại.\nsession.json: UNCHANGED (byte-identical)\n```\n\nReal binary, real server answering `600`:\n\n```\n⚠️ Phiên đăng nhập đã hết hạn (máy chủ từ chối zpw_sek). Cần đăng nhập lại.\n→ expired_at = 1793521429, zpw_sek kept, login and domains intact\n```\n\nTUI rendered in a pty with the session intact, showing the accented UI (`Tài khoản`, `Chưa đọc`, `Hội thoại`, `Chọn một hội thoại để bắt đầu.`, `Phím tắt`) and the help screen listing `w  Xoá lịch sử tin nhắn cục bộ (có xác nhận)`.\n\n`zalo_history.db` is now removed in exactly two places, both behind a confirmation (`gui/worker.rs` wipe arm, `tui.rs` wipe arm); `session.json` is only deleted by an explicit logout or `--logout`.\n\nNot verified: the GUI's visual result. This machine reports `capture: false` (Wayland) and egui exports no accessibility tree, so the new menu entry and dialog are verified by compilation, the render test and code review only.\n\n## Risk & Rollback\n\n- Behaviour change: a session the server expires with a code other than `600` / `-102` now shows an error and keeps the credentials instead of prompting for a login. That is the accepted trade-off from Q3, and the failure mode is safe (data preserved, logout still available).\n- Blast radius: session handling plus the two UIs; no protocol or wire change. `session.json` gains one optional field.\n- Rollback: revert the commits. Older builds ignore the extra field, and `session.json` is re-derived every run.\n\n## Context\n\n- Resolves: #21\n- Parent: #3 (item 1)\n- Stacked on: #29\n",
      "closedAt": "2026-11-01T10:45:58Z",
      "createdAt": "2026-11-01T09:53:59Z",
      "labels": [],
      "number": 30,
      "state": "MERGED",
      "title": "fix(session): keep session.json on network errors, prompt only on a real expiry"
    },
    {
      "author": {
        "id": "U_kgDOCbz4Lg",
        "is_bot": false,
        "login": "ngxccc",
        "name": "Ngoc Tran"
      },
      "body": "## What\n\n- `Domains::heal_from_verified()` restores every endpoint listed in `endpoints::SERVICE_MAP_V3` whenever a session is loaded, so a verified constant always wins over what `session.json` kept.\n- `Domains::apply_service_map()`: `zpw_service_map_v3` now only fills keys that have no verified entry (`group_cloud_message`), or slots that are still empty.\n- `Domains::default()` heals itself, so a fresh QR login no longer starts on the `p*-msg-chat.zaloapp.com` / `p*-ctl-chat.zaloapp.com` long-polling hosts.\n- `load_from_file()` heals every verified host (was 3 of 13) after rescuing legacy `lp_chat` / `lp_control`, and refills those plus `ws` when the file has none.\n- `chat::get_friends` tries `domains.friend` before `domains.profile`, logs each failed host, and returns a real error instead of swallowing it.\n- `.gitignore`: ignore the mdBook output directory `/book/`.\n\n## Why\n\nOlder builds persisted the hosts the server handed out before they were verified — notably `tt-friend-wpa.zaloapp.com` and `tt-profile-wpa.zaloapp.com`, which nginx answers with 404. The friend list then fails, `validate_session()` fails on the same wrong `profile` host, and the client reads that as an expired session and deletes it (#21). The verified table must be the floor, not the fallback.\n\n## How\n\n- Precedence is now: verified constant → server payload for keys without one → earlier value. Accepted trade-off from #22: a rotated verified host (`tt-chat3` → `tt-chat4`) is no longer followed until the per-endpoint candidate list lands (#27).\n- `zpw_chts_new` / `zpw_ctrs_new` / `zpw_ws` stay server-owned: they have no verified entry and genuinely rotate per session.\n\n## Verification\n\n```\ncargo test\ntest result: ok. 102 passed; 0 failed; 1 ignored\n```\n\n7 tests in `src/session.rs`, including `loading_a_legacy_session_heals_every_verified_domain`, which writes a legacy `session.json` to disk, loads it, and asserts every verified host is restored while `lp_chat` / `lp_control` / `ws` / `cloud_msg` survive.\n\nReal binary against a legacy `session.json` carrying only `*.zaloapp.com` hosts:\n\n```\n=== ZALO REAL DATA FETCH TEST ===\n✓ Đã tải session.json:\n  Profile Host: https://tt-profile-wpa.chat.zalo.me\n  Convers Host: https://tt-convers-wpa.chat.zalo.me\n  Chat Host:    https://tt-chat3-wpa.api.zaloapp.com\n! getfriends lỗi (không tải được danh sách bạn bè)\n! get_groups lỗi: server error 600: zpw_sek bị thiếu hoặc không đúng (from https://tt-group-wpa.zaloapp.com/...)\n```\n\nNot verified: a live `getfriends` against a real account — this machine holds no credentials, so the smoke session was a fixture. Zalo answering `600` instead of 404 proves the host is correct and reachable, and that the new error path surfaces it.\n\n## Risk & Rollback\n\n- Behavior change: if Zalo rotates a verified host, we keep using the constant and can 404 until the next build — the trade-off accepted in #22, and the reason #27 exists.\n- Blast radius: `src/session.rs` plus one function in `src/chat.rs`; no UI, no protocol change.\n- Rollback: revert the commits. Hosts are re-derived on every load, so no stale `session.json` survives a revert.\n\n## Context\n\n- Relates to: #22 (parent #3, item 2)\n- Follow-up: #27 — per-endpoint candidate list, try hosts until one answers\n",
      "closedAt": "2026-11-01T10:45:09Z",
      "createdAt": "2026-11-01T07:51:20Z",
      "labels": [],
      "number": 29,
      "state": "MERGED",
      "title": "fix(net): let verified endpoints win over zpw_service_map_v3"
    },
    {
      "author": {
        "id": "U_kgDOEJfvow",
        "is_bot": false,
        "login": "vuhuymicrosoft-bot",
        "name": "jesse"
      },
      "body": "",
      "closedAt": "2026-11-01T10:45:58Z",
      "createdAt": "2026-11-01T07:47:58Z",
      "labels": [],
      "number": 28,
      "state": "MERGED",
      "title": "Add macOS x86_64 build to CI workflow"
    },
    {
      "author": {
        "id": "U_kgDOB4Ct9w",
        "is_bot": false,
        "login": "quangnguyenn98",
        "name": ""
      },
      "body": "Turn the macos-dmg job into an arm64/x86_64 matrix (macos-14 and macos-15-intel), suffix the .dmg and artifact names with the arch, and update the release notes and README accordingly.",
      "closedAt": "2026-11-01T10:45:58Z",
      "createdAt": "2026-11-01T07:43:51Z",
      "labels": [],
      "number": 26,
      "state": "MERGED",
      "title": "ci(macos): build native Intel (x86_64) .dmg alongside Apple Silicon"
    },
    {
      "author": {
        "id": "U_kgDOCbz4Lg",
        "is_bot": false,
        "login": "ngxccc",
        "name": "Ngoc Tran"
      },
      "body": "## What\n- Add `docs/TINH_NANG.md`: a 38-row feature matrix over login, messaging, history, friends, calls and platform.\n- Add `docs/DA_BIET.md`: a page generated from the live GitHub issue list, never hand-edited.\n- Extend `docs/KHAC_PHUC_SU_CO.md` with session expiry, friend-list 404, multi-account working directories and re-scan QR cases.\n- Add `book.toml`, `scripts/gen-known-issues.sh`, `scripts/build-docs.sh` and `.github/workflows/docs.yml` to render `docs/*.md` as a static site.\n- Link the matrix from `README.md`, `docs/README.md` and `docs/SUMMARY.md`.\n\n## Why\n- Issue #3 asks for a feature-parity page: users cannot tell which features work, so reports keep duplicating (#10 = #19, #5 = #6, #8/#9 overlap).\n- Markdown in the repo is the wrong surface for end users, but there is no deploy path from the repo to `ctdozlite.ctdo.net` yet.\n- A hand-maintained parity table goes stale in two releases; the known-issues page must come from the tracker itself.\n\n## How\n- Treat `docs/*.md` as the single source of truth and generate HTML with mdBook; never hand-write the site.\n- List only user-facing pages in `docs/SUMMARY.md`; internal architecture notes stay unlisted and unbuilt.\n- Generate `docs/DA_BIET.md` with `curl` + `jq` against the public issues API. On network failure the script keeps the existing file and exits 0, so builds never depend on GitHub.\n- Classify issues by label, falling back to the `fix(`/`feat(` title prefix because contributors cannot apply labels on this repo.\n- Keep `.github/workflows/docs.yml` build-only and upload a `docs-site` artifact; no deploy step.\n- Use the mdBook 0.5 config layout (`build-dir` under `[build]`, not `[book]`).\n- Ignore `book/` and record `TODO(#20)` in `docs/SUMMARY.md` for the pending install page.\n\n## Verification\n- [x] `mdbook v0.5.4` builds clean: `book/index.html`, `TINH_NANG.html`, `DA_BIET.html`, `HUONG_DAN_SU_DUNG.html`, `KHAC_PHUC_SU_CO.html` all emitted\n- [x] `./scripts/gen-known-issues.sh` produces 20 rows from the live tracker (3 bugs, 1 feature)\n- [x] Generator keeps the previous file when GitHub is unreachable (verified by short-circuiting the request)\n- [x] Feature matrix marks known-broken items as 🐞 instead of ✅: session restore (#21), friend list (#22), sync button (#23)\n- [ ] Content review by a real user: call rows and voice-message row are inferred from source at `73849bd`\n- [ ] Docs workflow run on this PR (fork PRs need maintainer approval first)\n\n## Risk & Rollback\n- Risk: two-way door.\n- Blast radius: documentation and one build-only workflow. No `src/` change, no runtime change, no release pipeline change.\n- Rollback: drop the two commits; nothing else references them.\n- Known conflict: #18 edits `README.md` and `docs/README.md` tables too. Resolve by keeping both rows; the docs PR does not depend on #18.\n\n## Context\n- Relates to: #20\n- Relates to: #3\n- ADR / RFC: None\n\n## Deploy is NOT part of this PR\n\n`ctdozlite.ctdo.net` is static HTML on Hostinger behind Cloudflare, deployed by hand. Contributors have no Hostinger credentials and cannot change DNS, so this PR cannot publish anything.\n\nTarget is **option C**: CI builds mdBook, then uploads over FTP/SFTP into `public_html/docs/`. That needs a maintainer-created `FTP_*`/`SFTP_*` secret. `TODO(#20)` is already at the top of `.github/workflows/docs.yml`; adding the upload step is the only change required.\n\nUntil then, the `docs-site` artifact from the workflow is a ready-to-upload build.\n\n## Still open\n\n- [ ] `docs/SUMMARY.md` points \"Cài đặt\" at `HUONG_DAN_SU_DUNG.md` because `docs/CAI_DAT.md` exists only on #18's branch. Add one line after #18 merges — `TODO(#20)` is in the file.\n- [ ] In-app docs link (Q6c): no public URL yet, and the GUI change cannot be verified without a warm build.\n",
      "closedAt": "2026-11-01T10:45:58Z",
      "createdAt": "2026-11-01T07:23:30Z",
      "labels": [],
      "number": 25,
      "state": "MERGED",
      "title": "docs: user-facing feature matrix + mdBook docs site (build only, deploy pending)"
    },
    {
      "author": {
        "id": "U_kgDOCbz4Lg",
        "is_bot": false,
        "login": "ngxccc",
        "name": "Ngoc Tran"
      },
      "body": "## What\n- Make `Cargo.toml` the single source of truth for the release version.\n- Derive every artifact name, package metadata and installer version from it in CI.\n- Replace the hand-written release body with GitHub's generated notes (merged PRs).\n\n## Why\n- Issue #2: `v0.0.2` shipped 9 assets still named `0.0.1` because the version was hardcoded in 6 places.\n- Release descriptions were maintained by hand and duplicated the README install section.\n- A tag could be pushed with a version that did not match the source.\n\n## How\n- Add a `prepare` job that resolves the version once through `cargo metadata --locked`.\n- Propagate it to all build jobs, `.deb`/`.rpm` metadata, AppImage/tar names, Inno Setup and `Info.plist`.\n- Fail the run when `v<tag>` differs from `Cargo.toml`, or when the tagged commit is not on `main`.\n- Keep `contents: read` globally and grant `contents: write` only to the release job.\n- Use `0.0.0` sentinels in `installer.iss` and `Info.plist`; CI injects the real version.\n- Set `generate_release_notes: true` and keep only a short link to the install guide in `body`.\n- Move install instructions to `docs/CAI_DAT.md`; the README links to it instead of duplicating commands.\n\n## Verification\n- [x] `cargo metadata --locked` resolves `0.0.3`\n- [x] Workflow YAML parses; every bash step passes `bash -n`\n- [x] Simulated `prepare` cases: matching tag on `main` passes; mismatched tag fails; tag off `main` fails\n- [x] `git diff --check` clean; no remaining `0.0.1` literals in CI, packaging or README\n- [ ] First real tag (`v0.0.3`) exercises the tag guard end to end\n\n## Risk & Rollback\n- Risk: two-way door.\n- Blast radius: release workflow and packaging metadata only; application code is unchanged and `env!(\"CARGO_PKG_VERSION\")` still reports the binary version.\n- Rollback: revert this PR; `v0.0.2` and its published assets stay untouched. The tag guard has not run against a real tag yet, so the first `v0.0.3` release is the residual risk.\n\n## Context\n- Resolves: #2\n- Relates to: #12, #13, #14, #15, #16, #17\n- ADR / RFC: None",
      "closedAt": "2026-11-01T10:45:58Z",
      "createdAt": "2026-11-01T06:30:43Z",
      "labels": [],
      "number": 18,
      "state": "MERGED",
      "title": "fix(release): derive version from Cargo.toml and generate release notes"
    },
    {
      "author": {
        "id": "U_kgDOCbz4Lg",
        "is_bot": false,
        "login": "ngxccc",
        "name": "Ngoc Tran"
      },
      "body": "## What\r\n- Add `AGENTS.md` with build/test commands, architecture map and non-negotiables.\r\n- Add `GLOSSARY.md` with session/protocol/chat/runtime vocabulary and `INV-1..4`.\r\n- Add nine standards under `docs/standards/`: comments, API/errors, SQLite, concurrency, testing, security, git flow, issue tracking, domain docs.\r\n- Add `docs/adr/README.md` template + index; link the standards from `docs/README.md`.\r\n- Add GitHub issue forms (bug, feature, task, sub-issue) and ignore local agent tooling artifacts.\r\n\r\n## Why\r\n- There was no single source of truth for comment style, error handling, SQLite migrations, concurrency ownership, testing, secret handling or PR flow.\r\n- Agents and contributors need enforceable pointers before touching protocol, session and storage code.\r\n- Issue forms now require secret redaction and capture the server `error_code`, which is essential for protocol bugs.\r\n\r\n## How\r\n- Single-context docs: one glossary, one ADR log.\r\n- Standards written in English; Vietnamese product docs and UI strings unchanged.\r\n- Content adapted to the real stack (egui/ratatui, Tokio worker, SQLite, Zalo HTTP/WebSocket).\r\n- Branch commits are sliced: chore(ignore), docs(standards), ci(issue forms).\r\n\r\n## Verification\r\n- [x] Markdown relative links validated: 0 broken across 12 new/updated files.\r\n- [x] No Rust sources touched; CI runs `cargo test --locked` on Linux, Windows and macOS.\r\n- [x] `git status` clean; no `session.json`, `*.db`, captures or logs staged.\r\n\r\n## Risk & Rollback\r\n- Docs and repo tooling only; no runtime code path changes. Two-way door.\r\n- Rollback: revert the three commits on the branch.\r\n\r\n## Context\r\n- Resolves: None\r\n- Standards index: `docs/standards/issue-tracker.md`",
      "closedAt": "2026-11-01T10:45:05Z",
      "createdAt": "2026-11-01T05:43:08Z",
      "labels": [],
      "number": 11,
      "state": "MERGED",
      "title": "docs(harness): add engineering standards and issue workflow"
    }
  ]
};
