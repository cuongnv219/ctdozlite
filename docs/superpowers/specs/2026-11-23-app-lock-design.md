# Thiết kế tính năng Khoá bảo mật ứng dụng (`App Lock` — Issue #110 & #157)

- **Ngày:** 2026-11-23
- **Liên kết Issue:** [#110](https://github.com/ctdoteam1110/ctdozlite/issues/110) (`@LKW1989-kw`), [#157](https://github.com/ctdoteam1110/ctdozlite/issues/157) (`@minhquan99990`)
- **Trạng thái:** Đã qua vòng rà soát kỹ thuật — Chờ duyệt để lập kế hoạch triển khai (*Chưa triển khai mã nguồn*)

---

## 1. Ý định & Lý do (Intent & Why)

Gộp hai yêu cầu bổ trợ nhau của người dùng thành một tính năng bảo vệ riêng tư hoàn chỉnh cho ứng dụng desktop `ctdozlite`:

1. **Từ Issue #110:** Đặt mã khoá (mặc định **PIN 4 chữ số** hoặc **mật khẩu tuỳ chỉnh**) và khoá nhanh bằng phím tắt **`Ctrl + L`** (`Cmd + L` trên macOS).
2. **Từ Issue #157 & làm rõ với chủ dự án:** Tránh sự bất tiện của Zalo PC gốc (lúc nào cũng bắt thêm một thao tác bấm khoá thủ công), tối ưu theo thói quen sử dụng hằng ngày:
   - **Mở app lên** (và khi mở lại từ khay hệ thống sau khi đóng cửa sổ `X` trên Linux) → **tự động yêu cầu nhập mã khoá ngay từ khung hình đầu tiên**.
   - Trong lúc đang dùng, chỉ khoá trở lại khi:
     - **Máy tính ngủ (`sleep`)** hoặc **không thao tác quá ngưỡng cài đặt (`idle`)**, hoặc
     - Người dùng **chủ động khoá nhanh** qua **nút khoá `🔒` trên app** hoặc phím tắt **`Ctrl + L` / `Cmd + L`**.

---

## 2. Mục tiêu, Phi mục tiêu & Điều cấm

### Mục tiêu (Goals)
- **Hai chế độ mã khoá:**
  - `Pin4`: đúng 4 chữ số ASCII (`0–9`), gõ đủ 4 số **tự động xác thực & mở khoá ngay** (không cần nhấn Enter).
  - `Custom`: mật khẩu tự do 4–32 ký tự (hỗ trợ chữ/số/ký tự đặc biệt, so sánh theo số lượng ký tự `chars().count()`), nhấn **Enter** hoặc nút **Mở khoá**.
- **Lưu trữ & mật mã học an toàn (0 dependency mới):**
  - Tách riêng module logic thuần non-GUI `src/app_lock.rs`, chỉ dùng `sha2` (HMAC-SHA256 / PBKDF2-HMAC-SHA256) và `getrandom` đã có sẵn trong `Cargo.toml`.
  - Băm **PBKDF2-HMAC-SHA256 (`100_000` vòng)** với **salt ngẫu nhiên 16 byte** sinh qua CSPRNG (`getrandom`), kiểm tra hash bằng so sánh **constant-time**.
  - Lưu trạng thái khoá ở tệp riêng `app_lock.json` (tách khỏi `app_settings.json`) qua `crate::data_dir::write_private_atomic` (quyền `0600` trên Unix), đồng thời bổ sung `"app_lock.json"` vào `data_dir::is_data_entry` / `data_mtime` để tự di trú XDG trên Linux.
  - Tuyệt đối không lưu hoặc ghi log mã PIN/mật khẩu dạng văn bản rõ (`ctdozlite --dev` và `logger.rs` không bao giờ chạm vào chuỗi mã).
- **Chống dò mã thô (Brute-force cooldown):**
  - Nhập sai **5 lần liên tiếp** → khoá nhập liệu **`30 giây`**, hiển thị đồng hồ đếm ngược ngay trên màn hình khoá.
  - Lưu `failed_attempts` và `lockout_until_unix` vào `app_lock.json` để đóng/mở lại app vẫn không lách được bộ đếm chờ; reset về `0` ngay khi mở khoá thành công hoặc tắt/đổi mã.
- **Đầy đủ 4 điểm kích hoạt khoá (Lock Triggers):**
  1. **Khi mở app (`Cold start`):** Nếu `app_lock.json` đang bật (`enabled == true`) và trong thư mục dữ liệu có phiên đăng nhập hoạt động (`session.json`) **hoặc** bất kỳ tài khoản đã lưu nào (`accounts/*.json`), `ZaloGuiApp::new` khởi tạo sẵn `app_locked = true` ở frame 0.
  2. **Khi đóng cửa sổ ẩn xuống khay hệ thống trên Linux (`hides_to_tray()`):** Tại cả nhánh `ViewportCommand::CancelClose` và nút đóng `X` trên thanh tiêu đề tuỳ chỉnh (`src/gui.rs`), gọi `lock_now()` trước khi ẩn `Visible(false)`.
  3. **Khi máy ngủ (`sleep`) hoặc không thao tác (`idle`):**
     - *Sleep (đa nền tảng, chạy được cả trên Windows & khi thu nhỏ/ẩn tray):* Vì `std::time::Instant` (`QueryPerformanceCounter`) trên Windows vẫn chạy tiếp lúc S3 sleep / Modern Standby nên không thể chỉ dựa vào chênh lệch `unix_now()` và `ctx.input(|i| i.time)`. Thay vào đó, đặt **watchdog đồng hồ tường nhẹ trong `src/gui/worker.rs`** (dùng `tokio::time::interval(Duration::from_secs(5))` đối chiếu `unix_now()` giữa các nhịp tick — nhịp `tokio` đóng băng khi OS ngủ trên cả 3 HĐH Windows/Linux/macOS mà không cần thêm dependency hay FFI). Nếu bước nhảy `>= 45 giây` và `lock_on_sleep == true` → phát `GuiEvent::SystemWakeDetected` + `ctx.request_repaint()` để khoá ngay khi tỉnh dậy (kết hợp kiểm tra dự phòng `unix_now()` vs `ctx.input(|i| i.time)` ở đầu `update_frame()` trước nhánh thu nhỏ `scr_check < 64.0`).
     - *Idle:* Theo dõi mốc tương tác chuột/phím qua `ctx.input(|i| i.time)` (`last_activity_ui_time`). Nếu bật ngưỡng idle (`5 / 15 / 30 phút`) và đã quá hạn không tương tác (kiểm tra trong `update_frame()` và qua nhịp watchdog 5s) → gọi `lock_now()`. Hoàn toàn không dùng `std::time::Instant::now()` mới trong `src/gui/` (tuân thủ `R7`).
  4. **Khoá chủ động trên app:**
     - Hiện nút **Khoá ứng dụng (`🔒`)** ngay trên thanh tiêu đề trên cùng (`src/gui.rs:render_custom_title_bar`), nằm liền kề bên trái nút đổi giao diện Sáng/Tối (`[🔒] [🌙] [-] [□] [×]`, kích thước `36×32` và cập nhật `buttons_left_x = lock_rect.left()` để không bị vùng kéo cửa sổ nuốt click; ẩn đi khi đang khoá `self.app_locked == true`).
     - Phím tắt toàn cục **`Ctrl + L`** (Windows/Linux) và **`Cmd + L`** (macOS) khi đang mở app ở trạng thái đã mở khoá (và có `session.json` hoặc tài khoản đã lưu).
- **Che màn hình & bịt kín đường rò nội dung khi đang khoá:**
  - Khi `app_locked == true`: `update_frame()` **không gọi `render_chat_workspace()`**, không gọi `render_login_screen()`, bỏ qua mọi phím tắt khác (trừ nhập mã khoá và `Ctrl + Q`; các nút khung cửa sổ thu nhỏ/phóng to/đóng hoặc ẩn tray vẫn hoạt động). Khi `incoming_popup.is_some()`, vẫn gọi `render_toast_overlay()` trong nhánh đang khoá để popup góc màn hình trên Windows hoạt động cả khi cửa sổ chính đang mở.
  - Gọi `lock_now()` tự động xoá popup góc màn hình đang hiện (`incoming_popup = None`), xoá toast đang mở, đóng mọi modal phụ (`settings_modal_open`, `contacts_view_open`, `feed_modal_open`, `saved_modal_open`, `account_menu_open`...) và reset ô nhập mã khoá.
  - **Tách riêng quy tắc bảo mật `app_locked` khỏi tuỳ chọn ẩn chữ `hide_notifications_when_locked`:**
    - Bất kể `hide_notifications_when_locked` bật hay tắt, hễ `app_locked == true` thì **bắt buộc ẩn nút `"Mở chat"` và tắt thanh trả lời nhanh (`reply_draft`)** trên `render_toast_overlay()` để không thể gửi tin hoặc nhảy vào hội thoại sau màn hình khoá.
    - Khi `app_locked && hide_notifications_when_locked` (`true` mặc định): cả 4 luồng thông báo chat (`RealtimeEvent::Message`, polling thread merge, reaction banners, `handle_background_account_activity`) ẩn tên người gửi/nội dung thành `"ctdozlite"` / `"Bạn có tin nhắn mới"` (hoặc `"{n} tin nhắn mới"`), `watch_toasts` / `GuiEvent::ReminderDue` / `GuiEvent::ScriptToast` ẩn nội dung tin nhắn, và trên Windows `render_toast_overlay` thay avatar/chữ cái đầu người gửi bằng biểu tượng khoá trung tính. Số tổng chưa đọc trên tiêu đề cửa sổ & khay Linux (`tray.rs`) giữ nguyên dạng số đếm không chứa nội dung.
    - Âm thanh thông báo (`play_notification_chime` / `play_notification_sound`) vẫn giữ theo cài đặt `play_notification_sound`.
  - **Với cuộc gọi VoIP/Video (`zalo_dedicated_call_window` & `GuiEvent::IncomingCall`) — Phương án An toàn tối đa:**
    - *Cuộc gọi đang diễn ra khi khoá giữa chừng:* Không kích hoạt `hangup_clicked` ở `modals.rs:5709`; ẩn/tạm tắt cửa sổ viewport riêng `zalo_dedicated_call_window` trong lúc khoá nhưng giữ nguyên pipeline audio/video ở worker, hiển thị dải trạng thái tối giản `"📞 Cuộc gọi đang diễn ra"` kèm nút **"Kết thúc"** trên `render_lock_screen()`, và khôi phục lại cửa sổ cuộc gọi ngay sau khi mở khoá. Tạm dừng bộ đếm `idle` trong lúc đang gọi để không tự khoá giữa cuộc trò chuyện.
    - *Cuộc gọi đến (`GuiEvent::IncomingCall`) khi đang khoá:* Không mở cửa sổ `zalo_dedicated_call_window`; ẩn `caller_name`/`status_msg` thành `"📲 Cuộc gọi đến — mở khoá để xem & trả lời"`, không hiển thị nút nghe hay Quick Reply trên màn hình khoá, chỉ cung cấp nút **"Từ chối"** (`WorkerCmd::AnswerCall { accept: false, cam_start_off: false }`); muốn xem ai gọi hoặc nghe máy phải nhập mã mở khoá trước.
- **Luồng khôi phục "Quên mã khoá? Đăng xuất" & vòng đời Đăng xuất / Thêm tài khoản:**
  - Trên màn hình khoá có liên kết **"Quên mã khoá? Đăng xuất"**, mở hộp thoại xác nhận rõ: thao tác này sẽ đăng xuất **toàn bộ phiên tài khoản đang lưu trên máy** để xoá mã khoá, nhưng **giữ nguyên lịch sử tin nhắn SQLite (`accounts/*_history.db`)**.
  - Dùng lệnh riêng `WorkerCmd::ForgetLockAndSignOutAll` (KHÔNG dùng `WorkerCmd::Logout` thông thường, vì `WorkerCmd::Logout` cố tình giữ snapshot `accounts/<uin>.json` cho màn hình đăng nhập 1-click):
    1. Dừng mọi listener nền (`background_accounts`), huỷ các task QR/Password đang chạy.
    2. Xoá `session.json`, xoá toàn bộ tệp credential snapshot `accounts/<uin>.json` (giữ lại `accounts/*_threads.json`, `accounts/*_history.db`, thư mục `accounts/<uin>/`), xoá `app_lock.json`, xoá bộ nhớ thread đang giữ trong RAM (`KNOWN_THREADS`), reset session/poll state, chuyển về màn hình đăng nhập QR sạch.
  - **Đăng xuất thông thường (`WorkerCmd::Logout`) & `"Thêm tài khoản"` (`begin_add_account`) khi đang mở khoá:** Vì người dùng đã xác thực mở khoá rồi mới thao tác, không khoá ngược lại ngay tại thời điểm bấm chuyển màn hình QR/thêm tài khoản; `app_lock.json` vẫn giữ `enabled = true` và ở bất kỳ lần kích hoạt khoá tiếp theo nào (mở lại app, tắt khay Linux, sleep/idle, `Ctrl+L`), nếu còn ít nhất 1 `session.json` hoặc `accounts/*.json` thì màn hình khoá che toàn bộ `render_login_screen()` (gồm cả `render_saved_accounts_on_login`). Nếu người dùng tự xoá hết sạch mọi tài khoản đã lưu (không còn `session.json` và `accounts/*.json` nào) thì app hiện thẳng màn hình QR sạch mà không hỏi mã, và `app_lock.json` tự kích hoạt lại ngay khi có phiên mới.
- **Giao diện quản lý trong Cài đặt → `Quyền riêng tư` (`src/gui/modals.rs`):**
  - Mục **Khoá bảo mật (`App Lock`)**:
    - Bật / Tắt khoá ứng dụng: khi bật cho chọn loại (`PIN 4 số` hoặc `Mật khẩu tuỳ chỉnh 4–32 ký tự`), nhập + nhập lại xác nhận; khi đổi mã hoặc tắt khoá **bắt buộc nhập mã hiện tại** để xác nhận.
    - Toggle **Tự khoá khi máy ngủ (sleep)** (mặc định bật).
    - ComboBox **Tự khoá khi không thao tác**: `Tắt`, `5 phút`, `15 phút`, `30 phút` (mặc định `Tắt`).
    - Toggle **Ẩn nội dung thông báo khi đang khoá** (mặc định bật).
    - Nút **Khoá ngay (`Ctrl+L`)** xem nhanh ngay trong tab.

### Không nằm trong phạm vi (Non-Goals)
- **Không mã hoá lại SQLite / toàn bộ thư mục dữ liệu trên đĩa** (người dùng đã chọn Hướng 1; bảo mật ổ đĩa vẫn dựa vào BitLocker / FileVault / LUKS và quyền `0700`/`0600`).
- **Không bổ sung System Tray cho Windows & macOS** trong phạm vi tính năng này (giữ nguyên hành vi khay hệ thống hiện tại ở `src/gui/tray.rs` dành cho Linux).
- **Không hỗ trợ mã khoá riêng từng tài khoản** (khoá theo cấp cửa sổ / thư mục dữ liệu hiện tại).
- **Không tích hợp sinh trắc học HĐH** (Windows Hello / Touch ID) ở phiên bản này.
- **Không áp dụng App Lock cho chế độ headless CLI / TUI / MCP Server / REST API.**

### Điều cấm (Anti-Goals)
- Không được nhá khung chat dù 1 frame khi mở app lúc `app_locked == true`.
- Không được để phím tắt hoặc thao tác chuột lọt xuống workspace khi đang khoá.
- Không được rò rỉ tên người gửi, nội dung tin nhắn, từ khoá group-watch hay nút trả lời nhanh qua popup/notification khi đang bật che thông báo lúc khoá.
- Không được để người lạ vượt App Lock bằng "Đăng xuất" rồi bấm 1-click vào tài khoản khác, hoặc dùng "Quên mã khoá" để giữ phiên/nhảy vào tài khoản khác mà chưa xác thực lại.
- Không vi phạm bất kỳ quy tắc nào trong `scripts/check_invariants.py` (`R1`–`R8`).

---

## 3. Phương án kiến trúc đã cân nhắc (Approaches)

1. **Cách A — ĐÃ CHỌN:** Tách riêng `src/app_lock.rs` (non-GUI, PBKDF2-HMAC-SHA256 + `getrandom`, 0 crate mới) + tệp `app_lock.json` ghi atomic `0600` qua `data_dir::write_private_atomic` + màn hình khoá tách hoàn toàn khỏi `render_chat_workspace()`.
   - *Lý do chọn:* Tách biệt rõ domain bảo mật khỏi `src/gui.rs`, dễ viết unit/property test độc lập, không lộ phím tắt/focus xuyên modal, kiểm soát chặt quyền file `0600` và không trộn secrets-hash vào `app_settings.json`.
2. **Cách B (Loại):** Gộp thẳng trường hash vào `AppSettings` (`app_settings.json`) và vẽ `egui::Area` đè lên trên workspace.
   - *Lý do loại:* `app_settings.json` đang ghi thường không set `0600`; render modal đè vẫn phải chạy layout workspace phía sau và dễ lọt shortcut/scroll/focus.
3. **Cách C (Loại):** Hook sự kiện Sleep cấp kernel của từng HĐH + Keychain/Credential Manager native.
   - *Lý do loại:* Kéo thêm nhiều crate FFI đặc thù từng HĐH, dễ vướng popup quyền Keychain trên macOS/Linux và tăng rủi ro gãy build cross-platform CI.

---

## 4. Ràng buộc kỹ thuật (Constraints)

- **Zero dependency mới:** Chỉ sử dụng `sha2` (0.10) và `getrandom` (0.3) đã có sẵn trong `Cargo.toml`.
- **Tuân thủ `scripts/check_invariants.py`:**
  - `src/app_lock.rs` được thêm vào `STRICT_NON_GUI_MODULES` (cấm import `crate::gui`).
  - `app_lock.json` được thêm vào `.gitignore` và kiểm tra `R5` (`no-tracked-secrets`) + `data_dir::is_data_entry`.
  - Không dùng `.unwrap()`/`.expect()` trong `src/gui/` (`R1`), không `.clamp` động mất an toàn (`R2`), không `thread::sleep` trên UI thread (`R4`), không thêm `Instant::now()` mới trong `src/gui/` (`R7`), không tăng số comment `//` quá trần `R6`.
- **Giao diện đồng bộ `docs/DESIGN.md`:** Dùng toàn bộ token màu `ThemePalette` (`bg_chat_canvas`, `bg_elevated`, `border`, `primary`, `danger_text`, `text_primary`, `text_secondary`), chữ `Be Vietnam Pro` / font đang chọn của app, và icon vector từ `lucide` (`theme.rs`).

---

## 5. Phân tách rõ: Người dùng đã chốt vs Mặc định đề xuất & Phần để lại cho Builder

### Quyết định người dùng đã chốt (User Decisions)
- Chọn **Hướng 1 (mức bảo vệ):** Khoá giao diện + che nội dung thông báo, không mã hoá SQLite trên đĩa.
- Chọn **hành vi kích hoạt:** Hỏi mật khẩu khi mở app (và mở lại từ khay Linux), đang dùng thì không làm phiền trừ khi máy **sleep/idle** hoặc người dùng bấm **icon khoá trên app** / **`Ctrl+L` (`Cmd+L`)**.
- Khi kích hoạt khoá: **che toàn bộ app** bằng màn hình nhập mật khẩu (không phải chỉ khoá ô chat).
- Chọn kiến trúc **Cách A** (`src/app_lock.rs` + `app_lock.json` riêng + màn hình khoá tách rời).

### Đề xuất mặc định của bản thiết kế (Designer's Calls — sếp kiểm tra lại)
1. **PBKDF2-HMAC-SHA256 `100_000` vòng + salt 16 byte** bằng `sha2` + `getrandom` (giãn chậm chống thử vét cạn offline nhưng vẫn mở khoá dưới ~30ms trên desktop).
2. **Thử sai 5 lần chờ 30 giây**, lưu `failed_attempts` + `lockout_until_unix` vào `app_lock.json` để khởi động lại app cũng không reset bộ đếm.
3. **Watchdog phát hiện sleep & idle đa nền tảng = `5 giây` trong `src/gui/worker.rs`** (ngưỡng nhảy đồng hồ sleep **`45 giây`**) thay vì chỉ đo `wall_delta - ui_delta` trên UI thread, vì `Instant` của Windows không dừng khi sleep; `idle_timeout` mặc định **`Tắt`** (có `5 / 15 / 30 phút`).
4. **Mặc định bật `Ẩn nội dung thông báo khi đang khoá`**, giữ tiếng chuông báo, giữ số tổng chưa đọc ở title/khay Linux không kèm nội dung, và **luôn khoá `"Mở chat"` + Quick Reply trên popup** mọi lúc `app_locked == true`.
5. **Nút `Quên mã khoá? Đăng xuất`** dùng `WorkerCmd::ForgetLockAndSignOutAll` xoá `session.json`, toàn bộ credential snapshot `accounts/<uin>.json` và `app_lock.json` (nhưng **giữ nguyên `accounts/*_history.db`**); khi đăng xuất thông thường lúc đã mở khoá thì vẫn ở lại màn hình đăng nhập ở phiên hiện tại, chỉ che `render_saved_accounts_on_login` ở các lần khoá kế tiếp.
6. **Cuộc gọi VoIP/Video khi đang khoá (`Phương án An toàn tối đa`):** Tạm ẩn cửa sổ `zalo_dedicated_call_window` không cúp máy và tạm dừng đếm `idle` khi đang gọi dở; với cuộc gọi đến lúc đang khoá, ẩn tên người gọi thành `"Cuộc gọi đến — mở khoá để xem & trả lời"` và chỉ cho phép bấm **"Từ chối"** trên màn hình khoá (muốn nghe phải nhập mã trước).

### Phần để lại cho bước lập kế hoạch & triển khai (Left to Builder)
- Kích thước padding, bo góc, bố cục cụ thể của `render_lock_screen` và thẻ cài đặt `Quyền riêng tư` theo `docs/DESIGN.md` (`ThemePalette`).
- Cấu trúc JSON chi tiết của `app_lock.json` (có `version: 1`, `mode`, `salt_b64`, `hash_b64`, `iterations`, `lock_on_sleep`, `idle_timeout_secs`, `hide_notifications_when_locked`, `failed_attempts`, `lockout_until_unix`).
- Cách thiết kế helper nội bộ, định dạng chấm tròn PIN 4 số / ô nhập mật khẩu masked (`egui::TextEdit::password(true)`), cũng như danh sách unit tests (`src/app_lock.rs`), property/invariant tests và kịch bản kiểm chứng giao diện.
