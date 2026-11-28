use std::time::Duration;

use eframe::egui::{
    self, Align, Align2, Color32, CursorIcon, FontId, Id, Key, Layout, Margin, Pos2, Rect,
    RichText, Rounding, Sense, Stroke, Vec2,
};

use super::theme::{
    draw_lucide_icon, font_family_semibold, lucide, render_toggle_switch, styled_input_frame,
    ThemePalette,
};
use super::worker::{set_idle_lock_timeout_secs, WorkerCmd, WORKER_TX};
use super::{AppLockSettingsAction, ZaloGuiApp};
use crate::app_lock::{
    validate_passcode, AppLockConfig, ChangePasscodeOutcome, PasscodeMode, VerifyOutcome,
    IDLE_PRESETS_SECS,
};

const APP_LOCK_SETUP_STATE_KEY: &str = "app_lock_setup_modal_state";
const APP_LOCK_SETTINGS_UI_KEY: &str = "app_lock_settings_form_ui";
const APP_LOCK_UNLOCK_SHOW_KEY: &str = "app_lock_unlock_show_passcode";
const APP_LOCK_UNLOCK_PIN4_ID: &str = "app_lock_unlock_pin4";
const APP_LOCK_UNLOCK_CUSTOM_ID: &str = "app_lock_unlock_custom";
const APP_LOCK_SETUP_NEW_ID: &str = "app_lock_setup_new";
const APP_LOCK_SETUP_CONFIRM_ID: &str = "app_lock_setup_confirm";
const APP_LOCK_ENABLE_NEW_ID: &str = "app_lock_enable_new";
const APP_LOCK_ENABLE_CONFIRM_ID: &str = "app_lock_enable_confirm";
const APP_LOCK_CHANGE_CURRENT_ID: &str = "app_lock_change_current";
const APP_LOCK_CHANGE_NEW_ID: &str = "app_lock_change_new";
const APP_LOCK_CHANGE_CONFIRM_ID: &str = "app_lock_change_confirm";
const APP_LOCK_DISABLE_CURRENT_ID: &str = "app_lock_disable_current";

/// Visual tone for live field validation pills.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ValidationTone {
    Muted,
    Pending,
    Ok,
    Warn,
}

/// Live validation indicator rendered beside a persistent field label.
#[derive(Debug, Clone, PartialEq, Eq)]
struct FieldStatus {
    text: String,
    tone: ValidationTone,
}

/// Result returned by rendering a single passcode or PIN input row.
#[derive(Debug, Clone, Copy, Default)]
struct FieldOutput {
    changed: bool,
    enter_pressed: bool,
    has_focus: bool,
    toggle_clicked: bool,
    #[cfg_attr(not(test), allow(dead_code))]
    first_box_rect: Option<Rect>,
}

/// Transient state for the dedicated quick-setup modal stored in `egui::Context` temp data.
#[derive(Debug, Clone)]
struct AppLockSetupModalState {
    open: bool,
    mode: PasscodeMode,
    new_passcode: String,
    confirm_passcode: String,
    show_passcode: bool,
    lock_on_sleep: bool,
    idle_timeout_secs: u64,
    hide_notifications_when_locked: bool,
    auto_options_expanded: bool,
    error: Option<String>,
    focus_step: Option<u8>,
    last_focused_step: Option<u8>,
}

impl Default for AppLockSetupModalState {
    fn default() -> Self {
        Self {
            open: false,
            mode: PasscodeMode::Pin4,
            new_passcode: String::new(),
            confirm_passcode: String::new(),
            show_passcode: false,
            lock_on_sleep: true,
            idle_timeout_secs: 0,
            hide_notifications_when_locked: true,
            auto_options_expanded: false,
            error: None,
            focus_step: None,
            last_focused_step: None,
        }
    }
}

/// Transient UI toggles and keyboard focus targets for Settings Privacy App Lock forms.
#[derive(Debug, Clone, Copy, Default)]
struct AppLockSettingsFormUi {
    last_action: AppLockSettingsAction,
    show_passcode: bool,
    focus_step: Option<u8>,
    last_focused_step: Option<u8>,
}

/// Human-readable Vietnamese label for an idle auto-lock timeout preset in seconds.
pub(super) fn idle_timeout_label(secs: u64) -> &'static str {
    match secs {
        0 => "Tắt",
        300 => "5 phút",
        900 => "15 phút",
        1800 => "30 phút",
        _ => "Tuỳ chỉnh",
    }
}

/// Filter a 4-digit PIN input string to ASCII digits only, capped at 4 digits.
pub(super) fn sanitize_pin4_input(raw: &str) -> String {
    raw.chars().filter(char::is_ascii_digit).take(4).collect()
}

/// Sanitize a passcode draft according to [`PasscodeMode`].
fn sanitize_passcode_draft(mode: PasscodeMode, raw: &str) -> String {
    match mode {
        PasscodeMode::Pin4 => sanitize_pin4_input(raw),
        PasscodeMode::Custom => raw.chars().take(32).collect(),
    }
}

/// Submission decision for a 4-digit PIN entry frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pin4SubmitAction {
    None,
    Incomplete,
    Submit,
}

/// Classify a `PasscodeMode::Pin4` unlock field frame without calling `verify_attempt` on short input.
fn classify_pin4_submit(input: &str, enter_pressed: bool) -> Pin4SubmitAction {
    if input.len() == 4 {
        Pin4SubmitAction::Submit
    } else if enter_pressed && !input.is_empty() {
        Pin4SubmitAction::Incomplete
    } else {
        Pin4SubmitAction::None
    }
}

/// Return which of the 4 PIN boxes is the active next digit slot while focused (`0..=3`).
fn active_pin4_box_index(entered_len: usize, has_focus: bool) -> Option<usize> {
    if has_focus && entered_len < 4 {
        Some(entered_len)
    } else {
        None
    }
}

/// Return `true` only when a `Pin4` draft just reached 4 digits on this edit frame.
fn should_auto_advance_pin4(mode: PasscodeMode, prev_chars: usize, current_val: &str) -> bool {
    mode == PasscodeMode::Pin4 && prev_chars < 4 && current_val.chars().count() == 4
}

/// Compute live validation pill text and tone for an existing-passcode input field.
fn evaluate_current_field_status(mode: PasscodeMode, current: &str) -> FieldStatus {
    let len = current.chars().count();
    match mode {
        PasscodeMode::Pin4 => {
            if len == 0 {
                FieldStatus {
                    text: "0/4 chữ số".to_string(),
                    tone: ValidationTone::Muted,
                }
            } else if len < 4 {
                FieldStatus {
                    text: format!("{len}/4 chữ số"),
                    tone: ValidationTone::Pending,
                }
            } else {
                FieldStatus {
                    text: "✓ Đủ 4 số".to_string(),
                    tone: ValidationTone::Ok,
                }
            }
        }
        PasscodeMode::Custom => {
            if len == 0 {
                FieldStatus {
                    text: "Bắt buộc".to_string(),
                    tone: ValidationTone::Muted,
                }
            } else if len < 4 {
                FieldStatus {
                    text: format!("{len}/4+ ký tự"),
                    tone: ValidationTone::Pending,
                }
            } else {
                FieldStatus {
                    text: "✓ Đủ độ dài".to_string(),
                    tone: ValidationTone::Ok,
                }
            }
        }
    }
}

/// Compute live validation pill text and tone for a new-passcode input field.
fn evaluate_new_field_status(mode: PasscodeMode, new_val: &str) -> FieldStatus {
    let len = new_val.chars().count();
    match mode {
        PasscodeMode::Pin4 => {
            if len == 0 {
                FieldStatus {
                    text: "0/4 chữ số".to_string(),
                    tone: ValidationTone::Muted,
                }
            } else if len < 4 {
                FieldStatus {
                    text: format!("{len}/4 chữ số"),
                    tone: ValidationTone::Pending,
                }
            } else {
                FieldStatus {
                    text: "✓ Đủ 4 số".to_string(),
                    tone: ValidationTone::Ok,
                }
            }
        }
        PasscodeMode::Custom => {
            if len == 0 {
                FieldStatus {
                    text: "4–32 ký tự".to_string(),
                    tone: ValidationTone::Muted,
                }
            } else if len < 4 {
                FieldStatus {
                    text: format!("{len}/4+ ký tự"),
                    tone: ValidationTone::Pending,
                }
            } else {
                FieldStatus {
                    text: format!("✓ Hợp lệ ({len}/32)"),
                    tone: ValidationTone::Ok,
                }
            }
        }
    }
}

/// Compute live confirmation status comparing `confirm_val` against `new_val`.
fn evaluate_confirm_field_status(
    mode: PasscodeMode,
    new_val: &str,
    confirm_val: &str,
) -> FieldStatus {
    let new_ok = validate_passcode(mode, new_val).is_ok();
    if confirm_val.is_empty() {
        return FieldStatus {
            text: if new_ok {
                "Đang chờ xác nhận…".to_string()
            } else {
                "Nhập mã mới trước".to_string()
            },
            tone: ValidationTone::Muted,
        };
    }
    if new_val.is_empty() {
        return FieldStatus {
            text: "Nhập mã mới trước".to_string(),
            tone: ValidationTone::Warn,
        };
    }
    let confirm_len = confirm_val.chars().count();
    if mode == PasscodeMode::Pin4 && confirm_len < 4 {
        if new_val.starts_with(confirm_val) {
            FieldStatus {
                text: format!("{confirm_len}/4 chữ số"),
                tone: ValidationTone::Pending,
            }
        } else {
            FieldStatus {
                text: "Chưa khớp mã PIN".to_string(),
                tone: ValidationTone::Warn,
            }
        }
    } else if new_ok && new_val == confirm_val {
        FieldStatus {
            text: "✓ Khớp mã khoá".to_string(),
            tone: ValidationTone::Ok,
        }
    } else if new_val == confirm_val {
        FieldStatus {
            text: "Mã mới chưa đủ hợp lệ".to_string(),
            tone: ValidationTone::Pending,
        }
    } else {
        FieldStatus {
            text: "Chưa khớp mã khoá".to_string(),
            tone: ValidationTone::Warn,
        }
    }
}

/// Localized prompt when the new passcode field is submitted empty.
fn empty_new_passcode_msg(mode: PasscodeMode) -> &'static str {
    match mode {
        PasscodeMode::Pin4 => "Vui lòng nhập mã PIN mới (4 chữ số)",
        PasscodeMode::Custom => "Vui lòng nhập mật khẩu mới (4-32 ký tự)",
    }
}

/// Validate Setup form inputs in field order, returning `(failing_step_index, message)`.
fn validate_setup_submit(
    mode: PasscodeMode,
    new_val: &str,
    confirm_val: &str,
) -> Result<(), (u8, &'static str)> {
    if new_val.is_empty() {
        return Err((0, empty_new_passcode_msg(mode)));
    }
    if let Err(msg) = validate_passcode(mode, new_val) {
        return Err((0, msg));
    }
    if confirm_val.is_empty() {
        return Err((1, "Vui lòng nhập lại mã khoá để xác nhận"));
    }
    if new_val != confirm_val {
        return Err((1, "Mã xác nhận không khớp"));
    }
    Ok(())
}

/// Validate Change form inputs in field order, returning `(failing_step_index, message)`.
fn validate_change_submit(
    current_mode: PasscodeMode,
    current_val: &str,
    new_mode: PasscodeMode,
    new_val: &str,
    confirm_val: &str,
) -> Result<(), (u8, &'static str)> {
    if current_val.is_empty() {
        return Err((0, "Vui lòng nhập mã khoá hiện tại"));
    }
    if validate_passcode(current_mode, current_val).is_err() {
        return Err((
            0,
            match current_mode {
                PasscodeMode::Pin4 => "Mã PIN hiện tại phải gồm đúng 4 chữ số",
                PasscodeMode::Custom => "Mật khẩu hiện tại phải có từ 4 đến 32 ký tự",
            },
        ));
    }
    if new_val.is_empty() {
        return Err((1, empty_new_passcode_msg(new_mode)));
    }
    if let Err(msg) = validate_passcode(new_mode, new_val) {
        return Err((1, msg));
    }
    if confirm_val.is_empty() {
        return Err((2, "Vui lòng nhập lại mã khoá mới để xác nhận"));
    }
    if new_val != confirm_val {
        return Err((2, "Mã xác nhận không khớp"));
    }
    Ok(())
}

/// Validate Disable form current-passcode input before verifying against stored hash.
fn validate_disable_submit(current_mode: PasscodeMode, current_val: &str) -> Result<(), &'static str> {
    if current_val.is_empty() {
        return Err("Vui lòng nhập mã khoá hiện tại");
    }
    if validate_passcode(current_mode, current_val).is_err() {
        return Err(match current_mode {
            PasscodeMode::Pin4 => "Mã PIN hiện tại phải gồm đúng 4 chữ số",
            PasscodeMode::Custom => "Mật khẩu hiện tại phải có từ 4 đến 32 ký tự",
        });
    }
    Ok(())
}

/// Return `None` when Setup submit is ready, or a concise Vietnamese tooltip hint when incomplete.
fn setup_readiness_hint(
    mode: PasscodeMode,
    new_val: &str,
    confirm_val: &str,
) -> Option<&'static str> {
    validate_setup_submit(mode, new_val, confirm_val)
        .err()
        .map(|(_, msg)| msg)
}

/// Return `None` when Change submit is ready, or a concise Vietnamese tooltip hint when incomplete.
fn change_readiness_hint(
    current_mode: PasscodeMode,
    current_val: &str,
    new_mode: PasscodeMode,
    new_val: &str,
    confirm_val: &str,
) -> Option<&'static str> {
    validate_change_submit(current_mode, current_val, new_mode, new_val, confirm_val)
        .err()
        .map(|(_, msg)| msg)
}

/// Return `None` when Disable submit is ready, or a concise Vietnamese tooltip hint when incomplete.
fn disable_readiness_hint(current_mode: PasscodeMode, current_val: &str) -> Option<&'static str> {
    validate_disable_submit(current_mode, current_val).err()
}

/// Summarize quick auto-lock preferences for the collapsible setup modal defaults bar.
fn quick_defaults_summary(
    lock_on_sleep: bool,
    idle_timeout_secs: u64,
    hide_notifications: bool,
) -> String {
    format!(
        "Ngủ: {} · Chờ: {} · Thông báo: {}",
        if lock_on_sleep { "Bật" } else { "Tắt" },
        idle_timeout_label(idle_timeout_secs),
        if hide_notifications { "Ẩn" } else { "Hiện" },
    )
}

/// Persist `app_lock.json` atomically and log any storage failure without panicking.
fn persist_app_lock_config(cfg: &AppLockConfig) -> Result<(), String> {
    cfg.save_default().map_err(|err| {
        let msg = err.to_string();
        crate::logger::log_error("app_lock", &msg);
        msg
    })
}

/// Format a user-facing verification failure message for both lock screen and Settings prompts.
fn format_verify_error(outcome: VerifyOutcome) -> String {
    match outcome {
        VerifyOutcome::Unlocked => String::new(),
        VerifyOutcome::Wrong { remaining_attempts } => {
            format!("Mã khoá không đúng (còn {remaining_attempts} lần thử)")
        }
        VerifyOutcome::LockedOut { remaining_secs } => {
            format!("Quá nhiều lần nhập sai — thử lại sau {remaining_secs} giây")
        }
    }
}

/// Resolve inline Settings error text while keeping lockout countdowns live and clearing
/// stale lockout notices once the cooldown expires.
fn resolve_settings_error_text(
    cfg: &AppLockConfig,
    settings_error: &mut Option<String>,
    now_unix: i64,
) -> Option<(String, bool)> {
    let lockout_rem = cfg.remaining_lockout_secs(now_unix);
    if lockout_rem > 0 {
        let msg = format_verify_error(VerifyOutcome::LockedOut {
            remaining_secs: lockout_rem,
        });
        *settings_error = Some(msg.clone());
        Some((msg, true))
    } else {
        if settings_error
            .as_deref()
            .is_some_and(|s| s.starts_with("Quá nhiều lần nhập sai"))
        {
            *settings_error = None;
        }
        settings_error.clone().map(|msg| (msg, false))
    }
}

/// Load a typed temporary UI state from `egui::Context` data store without holding nested locks.
fn load_ctx_temp<T: Clone + Default + Send + Sync + 'static>(
    ctx: &egui::Context,
    key: &str,
) -> T {
    ctx.data(|d| d.get_temp::<T>(Id::new(key)).unwrap_or_default())
}

/// Store a typed temporary UI state in `egui::Context` data store.
fn save_ctx_temp<T: Clone + Send + Sync + 'static>(
    ctx: &egui::Context,
    key: &str,
    val: T,
) {
    ctx.data_mut(|d| d.insert_temp(Id::new(key), val));
}

/// Scrub all transient app-lock UI state (`setup_modal`, `settings_form_ui`, `unlock_show`) on lock.
fn scrub_transient_ctx_state(ctx: &egui::Context) {
    ctx.data_mut(|d| {
        d.remove_temp::<AppLockSetupModalState>(Id::new(APP_LOCK_SETUP_STATE_KEY));
        d.remove_temp::<AppLockSettingsFormUi>(Id::new(APP_LOCK_SETTINGS_UI_KEY));
        d.remove_temp::<bool>(Id::new(APP_LOCK_UNLOCK_SHOW_KEY));
    });
}

/// Render inline verification or lockout errors for Settings `Changing` / `Disabling` panels.
fn render_settings_panel_error(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    pal: &ThemePalette,
    cfg: &AppLockConfig,
    settings_error: &mut Option<String>,
) {
    let now_unix = crate::login::unix_now();
    if let Some((err, active_lockout)) = resolve_settings_error_text(cfg, settings_error, now_unix) {
        if active_lockout {
            ctx.request_repaint_after(Duration::from_millis(500));
        }
        ui.add_space(6.0);
        ui.label(RichText::new(err).color(pal.danger_text).size(12.2));
    }
}

/// Render a localized subtle `"Hiện"` / `"Ẩn"` passcode visibility toggle button.
fn render_show_toggle_button(
    ui: &mut egui::Ui,
    pal: &ThemePalette,
    show_passcode: &mut bool,
) -> bool {
    let label = if *show_passcode { "Ẩn" } else { "Hiện" };
    let active_fg = if pal.is_dark {
        pal.primary_hover
    } else {
        pal.primary
    };
    let btn = egui::Button::new(
        RichText::new(label)
            .size(11.4)
            .color(if *show_passcode {
                active_fg
            } else {
                pal.text_secondary
            })
            .strong(),
    )
    .fill(if *show_passcode {
        pal.primary.linear_multiply(0.14)
    } else {
        pal.bg_hover.linear_multiply(0.8)
    })
    .stroke(Stroke::new(1.0_f32, pal.border.linear_multiply(0.75)))
    .rounding(Rounding::same(6.0))
    .min_size(Vec2::new(42.0, 22.0));
    let resp = ui
        .add(btn)
        .on_hover_cursor(CursorIcon::PointingHand)
        .on_hover_text(if *show_passcode {
            "Ẩn mã khoá"
        } else {
            "Hiện mã khoá để kiểm tra"
        });
    if resp.clicked() {
        *show_passcode = !*show_passcode;
        ui.ctx().request_repaint();
        true
    } else {
        false
    }
}

/// Render a persistent field label and optional right-aligned live validation pill.
fn render_field_header(
    ui: &mut egui::Ui,
    pal: &ThemePalette,
    label: &str,
    status: Option<&FieldStatus>,
) {
    ui.horizontal(|ui| {
        ui.label(
            RichText::new(label)
                .font(FontId::new(12.2, font_family_semibold()))
                .color(pal.text_primary),
        );
        if let Some(st) = status {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let (fg, bg) = match st.tone {
                    ValidationTone::Ok => (
                        pal.accent_online,
                        pal.accent_online.linear_multiply(0.14),
                    ),
                    ValidationTone::Pending => (
                        if pal.is_dark {
                            pal.primary_hover
                        } else {
                            pal.primary
                        },
                        pal.primary.linear_multiply(0.12),
                    ),
                    ValidationTone::Warn => (pal.danger_text, pal.danger.linear_multiply(0.14)),
                    ValidationTone::Muted => (pal.text_muted, Color32::TRANSPARENT),
                };
                if bg == Color32::TRANSPARENT {
                    ui.label(RichText::new(&st.text).color(fg).size(11.3));
                } else {
                    egui::Frame::none()
                        .fill(bg)
                        .rounding(Rounding::same(6.0))
                        .inner_margin(Margin::symmetric(7.0, 2.0))
                        .show(ui, |ui| {
                            ui.label(RichText::new(&st.text).color(fg).size(11.3).strong());
                        });
                }
            });
        }
    });
    ui.add_space(4.0);
}

/// Render a segmented pill selector for switching between `PIN 4 số` and `Mật khẩu tuỳ chỉnh`.
fn render_segmented_mode_selector(
    ui: &mut egui::Ui,
    pal: &ThemePalette,
    label: &str,
    mode: &mut PasscodeMode,
) -> bool {
    let prev = *mode;
    ui.label(
        RichText::new(label)
            .font(FontId::new(12.2, font_family_semibold()))
            .color(pal.text_primary),
    );
    ui.add_space(4.0);
    egui::Frame::none()
        .fill(pal.bg_input)
        .stroke(Stroke::new(1.0_f32, pal.border))
        .rounding(Rounding::same(10.0))
        .inner_margin(Margin::same(3.0))
        .show(ui, |ui| {
            let gap = 4.0_f32;
            let seg_w = ((ui.available_width() - gap) * 0.5).max(88.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = gap;
                for (candidate, title) in [
                    (PasscodeMode::Pin4, "PIN 4 số"),
                    (PasscodeMode::Custom, "Mật khẩu tuỳ chỉnh"),
                ] {
                    let active = *mode == candidate;
                    let (rect, resp) =
                        ui.allocate_exact_size(Vec2::new(seg_w, 30.0), Sense::click());
                    let bg = if active {
                        pal.primary
                    } else if resp.hovered() {
                        pal.bg_hover
                    } else {
                        Color32::TRANSPARENT
                    };
                    if bg != Color32::TRANSPARENT {
                        ui.painter().rect_filled(rect, Rounding::same(7.0), bg);
                    }
                    let fg = if active {
                        pal.primary_fg
                    } else if resp.hovered() {
                        pal.text_primary
                    } else {
                        pal.text_secondary
                    };
                    let font = if active {
                        FontId::new(12.4, font_family_semibold())
                    } else {
                        FontId::proportional(12.4)
                    };
                    ui.painter()
                        .text(rect.center(), Align2::CENTER_CENTER, title, font, fg);
                    if resp.on_hover_cursor(CursorIcon::PointingHand).clicked()
                        && *mode != candidate
                    {
                        *mode = candidate;
                    }
                }
            });
        });
    *mode != prev
}

/// Render a single-line passcode field with integrated `"Hiện"`/`"Ẩn"` button inside a themed frame.
fn render_passcode_field(
    ui: &mut egui::Ui,
    pal: &ThemePalette,
    id_salt: &str,
    buf: &mut String,
    mode: PasscodeMode,
    hint: &str,
    show_passcode: &mut bool,
    request_focus: bool,
) -> FieldOutput {
    let field_id = Id::new(id_salt);
    let had_focus = ui.memory(|m| m.has_focus(field_id));
    let enter_in_frame = ui.input(|i| i.key_pressed(Key::Enter));
    let mut out = FieldOutput::default();
    let frame_border_focus = had_focus || request_focus;

    styled_input_frame(pal, frame_border_focus).show(ui, |ui| {
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            let toggle_clicked = render_show_toggle_button(ui, pal, show_passcode);
            let input_w = ui.available_width().max(64.0);
            let r = ui.add(
                egui::TextEdit::singleline(buf)
                    .id(field_id)
                    .frame(false)
                    .password(!*show_passcode)
                    .hint_text(hint)
                    .desired_width(input_w),
            );
            if request_focus {
                r.request_focus();
            }
            if r.changed() {
                let cleaned = sanitize_passcode_draft(mode, buf);
                if cleaned != *buf {
                    *buf = cleaned;
                }
                out.changed = true;
            }
            if r.lost_focus() && enter_in_frame {
                out.enter_pressed = true;
            }
            out.toggle_clicked = toggle_clicked;
            out.has_focus = r.has_focus() || request_focus;
        });
    });

    out
}

/// Sync the backing `TextEdit` cursor to the end of `buf` so clicking a PIN box appends cleanly.
fn sync_pin4_cursor_to_end(ctx: &egui::Context, field_id: Id, buf: &str) {
    if let Some(mut st) = egui::TextEdit::load_state(ctx, field_id) {
        let cc = egui::text::CCursor::new(buf.chars().count());
        st.cursor
            .set_char_range(Some(egui::text::CCursorRange::one(cc)));
        st.store(ctx, field_id);
    }
}

/// Paint 4 disabled PIN boxes when brute-force lockout cooldown is active on the lock screen.
fn render_pin4_lockout_boxes(ui: &mut egui::Ui, pal: &ThemePalette) {
    ui.horizontal(|ui| {
        let box_w = 44.0_f32;
        let box_h = 48.0_f32;
        let gap = 10.0_f32;
        let total_w = 4.0 * box_w + 3.0 * gap;
        let lead = ((ui.available_width() - total_w) * 0.5).max(0.0);
        ui.add_space(lead);
        ui.spacing_mut().item_spacing.x = gap;
        for _ in 0..4 {
            let (rect, _) = ui.allocate_exact_size(Vec2::new(box_w, box_h), Sense::hover());
            ui.painter()
                .rect_filled(rect, Rounding::same(9.0), pal.bg_input.linear_multiply(0.75));
            ui.painter().rect_stroke(
                rect,
                Rounding::same(9.0),
                Stroke::new(1.0_f32, pal.border.linear_multiply(0.7)),
            );
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                "○",
                FontId::proportional(16.0),
                pal.text_muted.linear_multiply(0.5),
            );
        }
    });
}

/// Render unified clickable 4-box PIN input surface backed by an invisible `TextEdit` for keyboard/IME.
fn render_pin4_entry(
    ui: &mut egui::Ui,
    pal: &ThemePalette,
    id_salt: &str,
    buf: &mut String,
    show_pin: &mut bool,
    request_focus: bool,
    centered_lock_layout: bool,
) -> FieldOutput {
    let field_id = Id::new(id_salt);
    let had_focus = ui.memory(|m| m.has_focus(field_id));
    let enter_in_frame = ui.input(|i| i.key_pressed(Key::Enter));
    let mut any_box_clicked = false;
    let mut toggle_clicked = false;
    let entered_chars: Vec<char> = buf.chars().take(4).collect();
    let entered_len = entered_chars.len();
    let visual_focus = had_focus || request_focus;
    let active_idx = active_pin4_box_index(entered_len, visual_focus);

    let (box_w, box_h, gap) = if centered_lock_layout {
        (44.0_f32, 48.0_f32, 10.0_f32)
    } else {
        (40.0_f32, 42.0_f32, 8.0_f32)
    };
    let mut first_box_rect: Option<Rect> = None;

    ui.horizontal(|ui| {
        if centered_lock_layout {
            let total_w = 4.0 * box_w + 3.0 * gap;
            let lead = ((ui.available_width() - total_w) * 0.5).max(0.0);
            ui.add_space(lead);
        }
        ui.spacing_mut().item_spacing.x = gap;
        for idx in 0..4 {
            let (rect, raw_resp) =
                ui.allocate_exact_size(Vec2::new(box_w, box_h), Sense::click());
            if first_box_rect.is_none() {
                first_box_rect = Some(rect);
            }
            let resp = raw_resp
                .on_hover_cursor(CursorIcon::Text)
                .on_hover_text("Nhấn để nhập 4 chữ số PIN");
            if resp.clicked() || resp.is_pointer_button_down_on() {
                any_box_clicked = true;
            }
            let is_active = active_idx == Some(idx);
            let digit = entered_chars.get(idx).copied();

            ui.painter()
                .rect_filled(rect, Rounding::same(9.0), pal.bg_input);
            if is_active {
                ui.painter().rect_filled(
                    rect,
                    Rounding::same(9.0),
                    pal.primary.linear_multiply(if pal.is_dark { 0.14 } else { 0.08 }),
                );
            }

            let stroke = if is_active {
                Stroke::new(1.6_f32, pal.primary)
            } else if digit.is_some() {
                Stroke::new(
                    1.3_f32,
                    if visual_focus {
                        pal.primary.linear_multiply(0.75)
                    } else {
                        pal.border
                    },
                )
            } else if resp.hovered() {
                Stroke::new(1.2_f32, pal.primary_hover)
            } else {
                Stroke::new(1.0_f32, pal.border)
            };
            ui.painter().rect_stroke(rect, Rounding::same(9.0), stroke);

            if let Some(ch) = digit {
                if *show_pin {
                    ui.painter().text(
                        rect.center(),
                        Align2::CENTER_CENTER,
                        ch.to_string(),
                        FontId::new(18.0, font_family_semibold()),
                        pal.text_primary,
                    );
                } else {
                    ui.painter().text(
                        rect.center(),
                        Align2::CENTER_CENTER,
                        "●",
                        FontId::proportional(17.0),
                        pal.primary,
                    );
                }
            } else if is_active {
                let c = rect.center();
                ui.painter().line_segment(
                    [
                        Pos2::new(c.x, c.y - 8.0),
                        Pos2::new(c.x, c.y + 8.0),
                    ],
                    Stroke::new(1.8_f32, pal.primary),
                );
            } else {
                ui.painter().text(
                    rect.center(),
                    Align2::CENTER_CENTER,
                    "○",
                    FontId::proportional(16.0),
                    pal.text_muted.linear_multiply(0.55),
                );
            }
        }

        if !centered_lock_layout {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                toggle_clicked = render_show_toggle_button(ui, pal, show_pin);
            });
        }
    });

    if centered_lock_layout {
        ui.add_space(7.0);
        let helper_txt = if visual_focus {
            "Gõ trực tiếp 4 chữ số PIN"
        } else {
            "Nhấn vào ô PIN để nhập"
        };
        let helper_galley = ui.painter().layout_no_wrap(
            helper_txt.to_string(),
            FontId::proportional(11.8),
            pal.text_muted,
        );
        let strip_w = helper_galley.size().x + 8.0 + 42.0;
        ui.horizontal(|ui| {
            let lead = ((ui.available_width() - strip_w) * 0.5).max(0.0);
            ui.add_space(lead);
            ui.spacing_mut().item_spacing.x = 8.0;
            let helper_resp = ui
                .add(
                    egui::Label::new(
                        RichText::new(helper_txt)
                            .color(pal.text_muted)
                            .size(11.8),
                    )
                    .sense(Sense::click()),
                )
                .on_hover_cursor(CursorIcon::PointingHand);
            if helper_resp.clicked() {
                any_box_clicked = true;
            }
            toggle_clicked = render_show_toggle_button(ui, pal, show_pin);
        });
    }

    let anchor_pos = first_box_rect.map_or_else(|| ui.cursor().min, |r| r.left_top());
    let backing_rect = Rect::from_min_size(anchor_pos, Vec2::splat(2.0));
    let mut out = FieldOutput {
        toggle_clicked,
        first_box_rect,
        ..Default::default()
    };

    let mut backing_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(backing_rect)
            .id_salt((id_salt, "pin4_backing")),
    );
    backing_ui.visuals_mut().text_cursor.stroke = Stroke::NONE;
    backing_ui.visuals_mut().selection = egui::style::Selection::default();
    if any_box_clicked {
        sync_pin4_cursor_to_end(backing_ui.ctx(), field_id, buf);
    }
    let r = backing_ui.add(
        egui::TextEdit::singleline(buf)
            .id(field_id)
            .frame(false)
            .password(!*show_pin)
            .text_color(Color32::TRANSPARENT)
            .desired_width(2.0),
    );
    if request_focus || any_box_clicked {
        r.request_focus();
        sync_pin4_cursor_to_end(backing_ui.ctx(), field_id, buf);
        backing_ui.ctx().request_repaint();
    }
    if r.changed() {
        let cleaned = sanitize_pin4_input(buf);
        if cleaned != *buf {
            *buf = cleaned;
        }
        out.changed = true;
    }
    if r.lost_focus() && enter_in_frame {
        out.enter_pressed = true;
    }
    out.has_focus = r.has_focus() || request_focus || any_box_clicked;

    out
}

/// Render either the unified clickable 4-box PIN surface or standard custom passcode input with label & live status.
#[allow(clippy::too_many_arguments)]
fn render_labeled_passcode_input(
    ui: &mut egui::Ui,
    pal: &ThemePalette,
    id_salt: &str,
    label: &str,
    status: Option<&FieldStatus>,
    buf: &mut String,
    mode: PasscodeMode,
    hint: &str,
    show_passcode: &mut bool,
    request_focus: bool,
) -> FieldOutput {
    render_field_header(ui, pal, label, status);
    match mode {
        PasscodeMode::Pin4 => {
            render_pin4_entry(ui, pal, id_salt, buf, show_passcode, request_focus, false)
        }
        PasscodeMode::Custom => render_passcode_field(
            ui,
            pal,
            id_salt,
            buf,
            mode,
            hint,
            show_passcode,
            request_focus,
        ),
    }
}

/// Build a primary or danger submit button styled subtly muted until `ready_hint` is `None`.
fn render_submit_button(
    ui: &mut egui::Ui,
    pal: &ThemePalette,
    label: &str,
    danger: bool,
    readiness_hint: Option<&str>,
) -> bool {
    let ready = readiness_hint.is_none();
    let (fill, text_col, stroke) = if ready {
        (
            if danger { pal.danger } else { pal.primary },
            pal.primary_fg,
            Stroke::NONE,
        )
    } else {
        (
            if danger {
                pal.danger.linear_multiply(0.24)
            } else {
                pal.bg_hover
            },
            pal.text_secondary,
            Stroke::new(1.0_f32, pal.border),
        )
    };
    let btn = egui::Button::new(
        RichText::new(label)
            .color(text_col)
            .size(12.8)
            .strong(),
    )
    .fill(fill)
    .stroke(stroke)
    .rounding(Rounding::same(6.0));
    let mut resp = ui.add(btn).on_hover_cursor(CursorIcon::PointingHand);
    if let Some(reason) = readiness_hint {
        resp = resp.on_hover_text(reason);
    }
    resp.clicked()
}

/// Apply focus update and track which field was focused across `"Hiện"` / `"Ẩn"` clicks.
fn track_and_advance_focus(
    step_idx: u8,
    out: FieldOutput,
    focus_step: &mut Option<u8>,
    last_focused_step: &mut Option<u8>,
    next_step_on_enter: Option<u8>,
    auto_advance_to: Option<u8>,
) {
    if out.has_focus {
        *last_focused_step = Some(step_idx);
    }
    if out.toggle_clicked && focus_step.is_none() {
        *focus_step = Some(last_focused_step.unwrap_or(step_idx));
    }
    if let Some(next) = auto_advance_to {
        *focus_step = Some(next);
    } else if out.enter_pressed {
        if let Some(next) = next_step_on_enter {
            *focus_step = Some(next);
        }
    }
}

/// Persistent Vietnamese field labels and hint strings for Setup and Change flows.
fn passcode_field_copy(mode: PasscodeMode) -> (&'static str, &'static str, &'static str, &'static str) {
    match mode {
        PasscodeMode::Pin4 => (
            "Mã PIN mới",
            "Nhập 4 chữ số (0-9)",
            "Xác nhận mã PIN",
            "Nhập lại 4 chữ số PIN",
        ),
        PasscodeMode::Custom => (
            "Mật khẩu mới",
            "Nhập 4–32 ký tự",
            "Xác nhận mật khẩu",
            "Nhập lại mật khẩu mới",
        ),
    }
}

/// Render the paired New + Confirm passcode inputs with live status pills, Enter navigation, and Pin4 auto-advance.
#[allow(clippy::too_many_arguments)]
fn render_new_and_confirm_fields(
    ui: &mut egui::Ui,
    pal: &ThemePalette,
    new_id: &str,
    confirm_id: &str,
    mode: PasscodeMode,
    new_buf: &mut String,
    confirm_buf: &mut String,
    show_passcode: &mut bool,
    focus_step: &mut Option<u8>,
    last_focused_step: &mut Option<u8>,
    new_step_idx: u8,
    confirm_step_idx: u8,
) -> (bool, bool) {
    let prev_new_chars = new_buf.chars().count();
    let new_status = evaluate_new_field_status(mode, new_buf);
    let (new_label, new_hint, confirm_label, confirm_hint) = passcode_field_copy(mode);
    let req_new = focus_step.take_if(|s| *s == new_step_idx).is_some();
    let new_out = render_labeled_passcode_input(
        ui,
        pal,
        new_id,
        new_label,
        Some(&new_status),
        new_buf,
        mode,
        new_hint,
        show_passcode,
        req_new,
    );
    let auto_advance = if should_auto_advance_pin4(mode, prev_new_chars, new_buf) {
        Some(confirm_step_idx)
    } else {
        None
    };
    track_and_advance_focus(
        new_step_idx,
        new_out,
        focus_step,
        last_focused_step,
        Some(confirm_step_idx),
        auto_advance,
    );

    ui.add_space(9.0);
    let confirm_status = evaluate_confirm_field_status(mode, new_buf, confirm_buf);
    let req_confirm = focus_step.take_if(|s| *s == confirm_step_idx).is_some();
    let confirm_out = render_labeled_passcode_input(
        ui,
        pal,
        confirm_id,
        confirm_label,
        Some(&confirm_status),
        confirm_buf,
        mode,
        confirm_hint,
        show_passcode,
        req_confirm,
    );
    track_and_advance_focus(
        confirm_step_idx,
        confirm_out,
        focus_step,
        last_focused_step,
        None,
        None,
    );

    (new_out.changed || confirm_out.changed, confirm_out.enter_pressed)
}

/// Render collapsible quick auto-lock defaults inside the Quick-Setup Modal.
fn render_setup_quick_defaults(
    ui: &mut egui::Ui,
    pal: &ThemePalette,
    state: &mut AppLockSetupModalState,
) {
    let summary = quick_defaults_summary(
        state.lock_on_sleep,
        state.idle_timeout_secs,
        state.hide_notifications_when_locked,
    );
    egui::Frame::none()
        .fill(pal.bg_chat_canvas)
        .stroke(Stroke::new(1.0_f32, pal.border))
        .rounding(Rounding::same(8.0))
        .inner_margin(Margin::symmetric(10.0, 8.0))
        .show(ui, |ui| {
            let hdr_h = 22.0_f32;
            let (hdr_rect, hdr_resp) =
                ui.allocate_exact_size(Vec2::new(ui.available_width(), hdr_h), Sense::click());
            if hdr_resp.hovered() {
                ui.painter().rect_filled(
                    hdr_rect.expand2(Vec2::new(4.0, 2.0)),
                    Rounding::same(6.0),
                    pal.bg_hover.linear_multiply(0.55),
                );
            }
            ui.painter().text(
                Pos2::new(hdr_rect.left() + 2.0, hdr_rect.center().y),
                Align2::LEFT_CENTER,
                "Tuỳ chọn tự khoá nhanh",
                FontId::new(12.0, font_family_semibold()),
                pal.text_primary,
            );
            let chev = if state.auto_options_expanded {
                lucide::CHEVRON_DOWN
            } else {
                lucide::CHEVRON_RIGHT
            };
            draw_lucide_icon(
                ui.painter(),
                Pos2::new(hdr_rect.right() - 8.0, hdr_rect.center().y),
                chev,
                13.0,
                pal.text_secondary,
            );
            if hdr_resp
                .on_hover_cursor(CursorIcon::PointingHand)
                .clicked()
            {
                state.auto_options_expanded = !state.auto_options_expanded;
            }

            if !state.auto_options_expanded {
                ui.add_space(2.0);
                ui.label(RichText::new(summary).color(pal.text_muted).size(11.3));
            } else {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Tự khoá khi máy ngủ")
                            .color(pal.text_primary)
                            .size(12.4),
                    );
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        render_toggle_switch(ui, &mut state.lock_on_sleep, pal);
                    });
                });
                ui.add_space(8.0);
                ui.label(
                    RichText::new("Tự khoá khi không thao tác")
                        .color(pal.text_primary)
                        .size(12.4),
                );
                ui.add_space(4.0);
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(6.0, 6.0);
                    for &preset in &IDLE_PRESETS_SECS {
                        let active = state.idle_timeout_secs == preset;
                        let btn = egui::Button::new(
                            RichText::new(idle_timeout_label(preset))
                                .size(11.6)
                                .color(if active {
                                    pal.primary_fg
                                } else {
                                    pal.text_secondary
                                })
                                .strong(),
                        )
                        .fill(if active { pal.primary } else { pal.bg_input })
                        .stroke(Stroke::new(
                            1.0_f32,
                            if active { pal.primary } else { pal.border },
                        ))
                        .rounding(Rounding::same(6.0));
                        if ui
                            .add(btn)
                            .on_hover_cursor(CursorIcon::PointingHand)
                            .clicked()
                        {
                            state.idle_timeout_secs = preset;
                        }
                    }
                });
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Ẩn nội dung thông báo khi khoá")
                            .color(pal.text_primary)
                            .size(12.4),
                    );
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        render_toggle_switch(ui, &mut state.hide_notifications_when_locked, pal);
                    });
                });
            }
        });
}

impl ZaloGuiApp {
    /// Open the dedicated centered quick-setup modal for enabling App Lock.
    pub(super) fn open_app_lock_setup_modal(&mut self, ctx: &egui::Context) {
        self.app_lock_ui.reset_settings_drafts();
        if self.app_lock_config.enabled || self.app_locked {
            return;
        }
        let next = AppLockSetupModalState {
            open: true,
            mode: PasscodeMode::Pin4,
            new_passcode: String::new(),
            confirm_passcode: String::new(),
            show_passcode: false,
            lock_on_sleep: self.app_lock_config.lock_on_sleep,
            idle_timeout_secs: self.app_lock_config.idle_timeout_secs,
            hide_notifications_when_locked: self.app_lock_config.hide_notifications_when_locked,
            auto_options_expanded: false,
            error: None,
            focus_step: Some(0),
            last_focused_step: Some(0),
        };
        save_ctx_temp(ctx, APP_LOCK_SETUP_STATE_KEY, next);
        ctx.request_repaint();
    }

    /// Render the dedicated frosted-backdrop Quick-Setup Modal when open.
    pub(super) fn render_app_lock_setup_modal(
        &mut self,
        ctx: &egui::Context,
        pal: &ThemePalette,
    ) {
        let mut state: AppLockSetupModalState = load_ctx_temp(ctx, APP_LOCK_SETUP_STATE_KEY);
        if !state.open {
            return;
        }
        if self.app_locked || self.app_lock_config.enabled {
            scrub_transient_ctx_state(ctx);
            return;
        }
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, Key::Escape)) {
            ctx.data_mut(|d| {
                d.remove_temp::<AppLockSetupModalState>(Id::new(APP_LOCK_SETUP_STATE_KEY));
            });
            return;
        }

        let screen = ctx.screen_rect();
        let max_w = (screen.width() - 32.0).max(260.0);
        let card_w = max_w.min(392.0);
        let max_h = (screen.height() - 48.0).max(240.0);
        let mut close_modal = false;
        let mut enable_succeeded = false;

        egui::Area::new(Id::new("app_lock_setup_backdrop"))
            .order(egui::Order::Tooltip)
            .fixed_pos(screen.min)
            .show(ctx, |ui| {
                let (bg_rect, bg_resp) =
                    ui.allocate_exact_size(screen.size(), Sense::click_and_drag());
                ui.painter().rect_filled(
                    bg_rect,
                    Rounding::ZERO,
                    if pal.is_dark {
                        Color32::from_rgba_unmultiplied(6, 9, 16, 178)
                    } else {
                        Color32::from_rgba_unmultiplied(15, 23, 42, 132)
                    },
                );
                if bg_resp.clicked() {
                    close_modal = true;
                }
            });

        let card_resp = egui::Area::new(Id::new("app_lock_setup_card"))
            .order(egui::Order::Tooltip)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                egui::Frame::none()
                    .fill(pal.bg_elevated)
                    .stroke(Stroke::new(1.2_f32, pal.border))
                    .rounding(Rounding::same(14.0))
                    .shadow(egui::epaint::Shadow {
                        offset: Vec2::new(0.0, 12.0),
                        blur: 32.0,
                        spread: 0.0,
                        color: Color32::from_black_alpha(if pal.is_dark { 150 } else { 70 }),
                    })
                    .inner_margin(Margin::symmetric(20.0, 18.0))
                    .show(ui, |ui| {
                        let inner_w = (card_w - 40.0).max(220.0);
                        ui.set_width(inner_w);
                        egui::ScrollArea::vertical()
                            .max_height(max_h)
                            .auto_shrink([false, true])
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    let (badge_rect, _) =
                                        ui.allocate_exact_size(Vec2::splat(30.0), Sense::hover());
                                    ui.painter().circle_filled(
                                        badge_rect.center(),
                                        15.0,
                                        pal.primary.linear_multiply(0.14),
                                    );
                                    draw_lucide_icon(
                                        ui.painter(),
                                        badge_rect.center(),
                                        lucide::SHIELD_CHECK,
                                        15.5,
                                        pal.primary,
                                    );
                                    ui.add_space(4.0);
                                    ui.vertical(|ui| {
                                        ui.label(
                                            RichText::new("Thiết lập khoá bảo mật")
                                                .font(FontId::new(15.5, font_family_semibold()))
                                                .color(pal.text_primary),
                                        );
                                        ui.label(
                                            RichText::new(
                                                "Bảo vệ ứng dụng bằng PIN 4 số hoặc mật khẩu riêng.",
                                            )
                                            .color(pal.text_secondary)
                                            .size(11.8),
                                        );
                                    });
                                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                                        let (x_r, x_resp) = ui.allocate_exact_size(
                                            Vec2::splat(26.0),
                                            Sense::click(),
                                        );
                                        if x_resp.hovered() {
                                            ui.painter().rect_filled(
                                                x_r,
                                                Rounding::same(6.0),
                                                pal.bg_hover,
                                            );
                                        }
                                        draw_lucide_icon(
                                            ui.painter(),
                                            x_r.center(),
                                            lucide::X,
                                            14.0,
                                            if x_resp.hovered() {
                                                pal.text_primary
                                            } else {
                                                pal.text_secondary
                                            },
                                        );
                                        if x_resp
                                            .on_hover_cursor(CursorIcon::PointingHand)
                                            .on_hover_text("Đóng (Esc)")
                                            .clicked()
                                        {
                                            close_modal = true;
                                        }
                                    });
                                });

                                ui.add_space(12.0);
                                if render_segmented_mode_selector(
                                    ui,
                                    pal,
                                    "Loại mã khoá",
                                    &mut state.mode,
                                ) {
                                    state.new_passcode.clear();
                                    state.confirm_passcode.clear();
                                    state.error = None;
                                    state.focus_step = Some(0);
                                    state.last_focused_step = Some(0);
                                }

                                ui.add_space(12.0);
                                let (fields_changed, confirm_enter) = render_new_and_confirm_fields(
                                    ui,
                                    pal,
                                    APP_LOCK_SETUP_NEW_ID,
                                    APP_LOCK_SETUP_CONFIRM_ID,
                                    state.mode,
                                    &mut state.new_passcode,
                                    &mut state.confirm_passcode,
                                    &mut state.show_passcode,
                                    &mut state.focus_step,
                                    &mut state.last_focused_step,
                                    0,
                                    1,
                                );
                                if fields_changed {
                                    state.error = None;
                                }

                                ui.add_space(12.0);
                                render_setup_quick_defaults(ui, pal, &mut state);

                                if let Some(ref err) = state.error {
                                    ui.add_space(8.0);
                                    ui.label(
                                        RichText::new(err)
                                            .color(pal.danger_text)
                                            .size(12.2),
                                    );
                                }

                                ui.add_space(14.0);
                                let readiness = setup_readiness_hint(
                                    state.mode,
                                    &state.new_passcode,
                                    &state.confirm_passcode,
                                );
                                let mut submit_clicked = false;
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    submit_clicked = render_submit_button(
                                        ui,
                                        pal,
                                        "Lưu & Bật",
                                        false,
                                        readiness,
                                    );
                                    if ui
                                        .button("Huỷ")
                                        .on_hover_cursor(CursorIcon::PointingHand)
                                        .clicked()
                                    {
                                        close_modal = true;
                                    }
                                });

                                if submit_clicked || confirm_enter {
                                    match validate_setup_submit(
                                        state.mode,
                                        &state.new_passcode,
                                        &state.confirm_passcode,
                                    ) {
                                        Err((bad_step, msg)) => {
                                            state.error = Some(msg.to_string());
                                            state.focus_step = Some(bad_step);
                                        }
                                        Ok(()) => {
                                            match AppLockConfig::create_enabled(
                                                state.mode,
                                                &state.new_passcode,
                                            ) {
                                                Ok(mut cfg) => {
                                                    cfg.lock_on_sleep = state.lock_on_sleep;
                                                    cfg.idle_timeout_secs = state.idle_timeout_secs;
                                                    cfg.hide_notifications_when_locked =
                                                        state.hide_notifications_when_locked;
                                                    if let Err(err) = persist_app_lock_config(&cfg)
                                                    {
                                                        state.error = Some(format!(
                                                            "Không thể lưu app_lock.json: {err}"
                                                        ));
                                                    } else {
                                                        self.app_lock_config = cfg;
                                                        set_idle_lock_timeout_secs(
                                                            self.app_lock_config.idle_timeout_secs,
                                                        );
                                                        self.app_lock_ui.corrupted_config = false;
                                                        self.app_lock_ui.reset_settings_drafts();
                                                        self.show_toast(
                                                            "Đã bật khoá bảo mật ứng dụng",
                                                        );
                                                        enable_succeeded = true;
                                                    }
                                                }
                                                Err(err) => {
                                                    state.error = Some(format!(
                                                        "Không thể tạo mã khoá: {err}"
                                                    ));
                                                }
                                            }
                                        }
                                    }
                                }
                            });
                    });
            });

        ctx.move_to_top(card_resp.response.layer_id);

        if close_modal || enable_succeeded {
            ctx.data_mut(|d| {
                d.remove_temp::<AppLockSetupModalState>(Id::new(APP_LOCK_SETUP_STATE_KEY));
            });
            ctx.request_repaint();
        } else {
            if state.focus_step.is_some() {
                ctx.request_repaint();
            }
            save_ctx_temp(ctx, APP_LOCK_SETUP_STATE_KEY, state);
        }
    }

    /// Render the full-screen passcode lock gate blocking workspace and login panels.
    pub(super) fn render_lock_screen(&mut self, ctx: &egui::Context, pal: &ThemePalette) {
        scrub_transient_ctx_state_except_unlock(ctx);
        let cmd_label = if cfg!(target_os = "macos") {
            "Cmd+L"
        } else {
            "Ctrl+L"
        };
        let now = crate::login::unix_now();
        let lockout_rem = self.app_lock_config.remaining_lockout_secs(now);
        let mut submit_candidate: Option<String> = None;
        let mut show_unlock_passcode: bool = load_ctx_temp(ctx, APP_LOCK_UNLOCK_SHOW_KEY);

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(pal.bg_chat_canvas))
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        let avail = ui.available_size();
                        let card_w = (avail.x - 32.0).max(240.0).clamp(240.0, 380.0);
                        ui.vertical_centered(|ui| {
                            ui.add_space(((avail.y - 360.0) * 0.42).max(24.0));

                            egui::Frame::none()
                                .fill(pal.bg_elevated)
                                .stroke(Stroke::new(1.0_f32, pal.border))
                                .rounding(Rounding::same(14.0))
                                .inner_margin(Margin::symmetric(26.0, 24.0))
                                .show(ui, |ui| {
                                    ui.set_width(card_w);
                                    ui.vertical_centered(|ui| {
                                        let (badge_rect, _) = ui.allocate_exact_size(
                                            Vec2::splat(48.0),
                                            Sense::hover(),
                                        );
                                        ui.painter().circle_filled(
                                            badge_rect.center(),
                                            24.0,
                                            pal.primary.linear_multiply(0.14),
                                        );
                                        draw_lucide_icon(
                                            ui.painter(),
                                            badge_rect.center(),
                                            lucide::LOCK,
                                            22.0,
                                            pal.primary,
                                        );
                                        ui.add_space(10.0);

                                        ui.label(
                                            RichText::new("Ứng dụng đang khoá")
                                                .font(FontId::new(18.0, font_family_semibold()))
                                                .color(pal.text_primary),
                                        );
                                        ui.add_space(4.0);
                                        ui.label(
                                            RichText::new(format!(
                                                "Nhập mã khoá để tiếp tục ({cmd_label} để khoá nhanh)"
                                            ))
                                            .color(pal.text_secondary)
                                            .size(12.3),
                                        );
                                        ui.add_space(16.0);

                                        if self.app_lock_ui.corrupted_config {
                                            egui::Frame::none()
                                                .fill(pal.danger.linear_multiply(0.10))
                                                .stroke(Stroke::new(1.0_f32, pal.danger_text))
                                                .rounding(Rounding::same(8.0))
                                                .inner_margin(Margin::symmetric(12.0, 10.0))
                                                .show(ui, |ui| {
                                                    ui.label(
                                                        RichText::new(
                                                            "Tệp cấu hình app_lock.json không đọc được hoặc đã bị hỏng. Vui lòng chọn \"Quên mã khoá? Đăng xuất\" bên dưới để đặt lại phiên đăng nhập.",
                                                        )
                                                        .color(pal.danger_text)
                                                        .size(12.2),
                                                    );
                                                });
                                            ui.add_space(12.0);
                                        } else if lockout_rem > 0 {
                                            ctx.request_repaint_after(Duration::from_millis(500));
                                            egui::Frame::none()
                                                .fill(pal.danger.linear_multiply(0.10))
                                                .stroke(Stroke::new(1.0_f32, pal.danger_text))
                                                .rounding(Rounding::same(8.0))
                                                .inner_margin(Margin::symmetric(12.0, 10.0))
                                                .show(ui, |ui| {
                                                    ui.label(
                                                        RichText::new(format!(
                                                            "Quá nhiều lần nhập sai — thử lại sau {lockout_rem} giây"
                                                        ))
                                                        .color(pal.danger_text)
                                                        .size(12.8)
                                                        .strong(),
                                                    );
                                                });
                                            ui.add_space(12.0);
                                            if self.app_lock_config.mode == PasscodeMode::Pin4 {
                                                render_pin4_lockout_boxes(ui, pal);
                                            } else {
                                                ui.add_enabled_ui(false, |ui| {
                                                    let mut disabled_buf = String::new();
                                                    let mut disabled_show = false;
                                                    let _ = render_passcode_field(
                                                        ui,
                                                        pal,
                                                        "app_lock_unlock_disabled",
                                                        &mut disabled_buf,
                                                        self.app_lock_config.mode,
                                                        "Đang tạm khoá…",
                                                        &mut disabled_show,
                                                        false,
                                                    );
                                                });
                                            }
                                            ui.add_space(12.0);
                                        } else {
                                            let req_focus = self.app_lock_ui.focus_requested;
                                            self.app_lock_ui.focus_requested = false;
                                            match self.app_lock_config.mode {
                                                PasscodeMode::Pin4 => {
                                                    let out = render_pin4_entry(
                                                        ui,
                                                        pal,
                                                        APP_LOCK_UNLOCK_PIN4_ID,
                                                        &mut self.app_lock_ui.unlock_input,
                                                        &mut show_unlock_passcode,
                                                        req_focus,
                                                        true,
                                                    );
                                                    if out.toggle_clicked {
                                                        self.app_lock_ui.focus_requested = true;
                                                    }
                                                    if out.changed {
                                                        self.app_lock_ui.unlock_error = None;
                                                    }
                                                    match classify_pin4_submit(
                                                        &self.app_lock_ui.unlock_input,
                                                        out.enter_pressed,
                                                    ) {
                                                        Pin4SubmitAction::Submit => {
                                                            submit_candidate = Some(
                                                                self.app_lock_ui
                                                                    .unlock_input
                                                                    .clone(),
                                                            );
                                                        }
                                                        Pin4SubmitAction::Incomplete => {
                                                            self.app_lock_ui.unlock_error = Some(
                                                                "Vui lòng nhập đủ 4 chữ số PIN"
                                                                    .to_string(),
                                                            );
                                                            self.app_lock_ui.focus_requested = true;
                                                            ctx.request_repaint();
                                                        }
                                                        Pin4SubmitAction::None => {
                                                            if out.enter_pressed {
                                                                self.app_lock_ui.focus_requested =
                                                                    true;
                                                            }
                                                        }
                                                    }
                                                }
                                                PasscodeMode::Custom => {
                                                    ui.with_layout(
                                                        Layout::top_down(Align::Min),
                                                        |ui| {
                                                            let out = render_labeled_passcode_input(
                                                                ui,
                                                                pal,
                                                                APP_LOCK_UNLOCK_CUSTOM_ID,
                                                                "Mật khẩu mở khoá",
                                                                None,
                                                                &mut self.app_lock_ui.unlock_input,
                                                                PasscodeMode::Custom,
                                                                "Nhập mật khẩu mở khoá",
                                                                &mut show_unlock_passcode,
                                                                req_focus,
                                                            );
                                                            if out.toggle_clicked {
                                                                self.app_lock_ui.focus_requested =
                                                                    true;
                                                            }
                                                            if out.changed {
                                                                self.app_lock_ui.unlock_error = None;
                                                            }
                                                            ui.add_space(10.0);
                                                            let ready = !self
                                                                .app_lock_ui
                                                                .unlock_input
                                                                .is_empty();
                                                            let unlock_btn = egui::Button::new(
                                                                RichText::new("Mở khoá")
                                                                    .color(if ready {
                                                                        pal.primary_fg
                                                                    } else {
                                                                        pal.text_secondary
                                                                    })
                                                                    .size(13.5)
                                                                    .strong(),
                                                            )
                                                            .fill(if ready {
                                                                pal.primary
                                                            } else {
                                                                pal.bg_hover
                                                            })
                                                            .stroke(if ready {
                                                                Stroke::NONE
                                                            } else {
                                                                Stroke::new(1.0_f32, pal.border)
                                                            })
                                                            .rounding(Rounding::same(8.0))
                                                            .min_size(Vec2::new(
                                                                ui.available_width(),
                                                                36.0,
                                                            ));
                                                            let mut resp = ui
                                                                .add(unlock_btn)
                                                                .on_hover_cursor(
                                                                    CursorIcon::PointingHand,
                                                                );
                                                            if !ready {
                                                                resp = resp.on_hover_text(
                                                                    "Nhập mật khẩu để mở khoá",
                                                                );
                                                            }
                                                            let btn_clicked = resp.clicked();
                                                            if (out.enter_pressed || btn_clicked)
                                                                && ready
                                                            {
                                                                submit_candidate = Some(
                                                                    self.app_lock_ui
                                                                        .unlock_input
                                                                        .clone(),
                                                                );
                                                            } else if out.enter_pressed
                                                                || btn_clicked
                                                            {
                                                                if btn_clicked {
                                                                    self.app_lock_ui.unlock_error =
                                                                        Some(
                                                                            "Vui lòng nhập mật khẩu"
                                                                                .to_string(),
                                                                        );
                                                                }
                                                                self.app_lock_ui.focus_requested =
                                                                    true;
                                                            }
                                                        },
                                                    );
                                                }
                                            }
                                            if let Some(ref err) = self.app_lock_ui.unlock_error {
                                                ui.add_space(8.0);
                                                ui.label(
                                                    RichText::new(err)
                                                        .color(pal.danger_text)
                                                        .size(12.2),
                                                );
                                            }
                                            ui.add_space(12.0);
                                        }

                                        if let Some(is_ringing) = self
                                            .active_call
                                            .as_ref()
                                            .map(|c| c.is_incoming_ringing)
                                        {
                                            let (strip_txt, action_txt) = if is_ringing {
                                                (
                                                    "📲 Cuộc gọi đến — mở khoá để xem & trả lời",
                                                    "Từ chối",
                                                )
                                            } else {
                                                ("📞 Cuộc gọi đang diễn ra", "Kết thúc")
                                            };
                                            egui::Frame::none()
                                                .fill(pal.bg_chat_canvas)
                                                .stroke(Stroke::new(1.0_f32, pal.border))
                                                .rounding(Rounding::same(8.0))
                                                .inner_margin(Margin::symmetric(12.0, 9.0))
                                                .show(ui, |ui| {
                                                    ui.set_min_width(ui.available_width());
                                                    let mut call_action_clicked = false;
                                                    let make_call_btn = || {
                                                        egui::Button::new(
                                                            RichText::new(action_txt)
                                                                .color(pal.primary_fg)
                                                                .size(12.0)
                                                                .strong(),
                                                        )
                                                        .fill(pal.danger)
                                                        .rounding(Rounding::same(6.0))
                                                    };
                                                    if ui.available_width() < 320.0 {
                                                        ui.vertical(|ui| {
                                                            ui.add(
                                                                egui::Label::new(
                                                                    RichText::new(strip_txt)
                                                                        .color(pal.text_primary)
                                                                        .size(12.2),
                                                                )
                                                                .wrap(),
                                                            );
                                                            ui.add_space(6.0);
                                                            if ui
                                                                .add(make_call_btn())
                                                                .on_hover_cursor(
                                                                    CursorIcon::PointingHand,
                                                                )
                                                                .clicked()
                                                            {
                                                                call_action_clicked = true;
                                                            }
                                                        });
                                                    } else {
                                                        ui.with_layout(
                                                            Layout::right_to_left(Align::Center),
                                                            |ui| {
                                                                if ui
                                                                    .add(make_call_btn())
                                                                    .on_hover_cursor(
                                                                        CursorIcon::PointingHand,
                                                                    )
                                                                    .clicked()
                                                                {
                                                                    call_action_clicked = true;
                                                                }
                                                                ui.with_layout(
                                                                    Layout::left_to_right(
                                                                        Align::Center,
                                                                    ),
                                                                    |ui| {
                                                                        ui.add(
                                                                            egui::Label::new(
                                                                                RichText::new(
                                                                                    strip_txt,
                                                                                )
                                                                                .color(
                                                                                    pal.text_primary,
                                                                                )
                                                                                .size(12.2),
                                                                            )
                                                                            .wrap(),
                                                                        );
                                                                    },
                                                                );
                                                            },
                                                        );
                                                    }
                                                    if call_action_clicked {
                                                        if let Some(tx) = WORKER_TX.get() {
                                                            let cmd = if is_ringing {
                                                                WorkerCmd::AnswerCall {
                                                                    accept: false,
                                                                    cam_start_off: false,
                                                                }
                                                            } else {
                                                                WorkerCmd::EndCall
                                                            };
                                                            let _ = tx.send(cmd);
                                                        }
                                                        self.active_call = None;
                                                    }
                                                });
                                            ui.add_space(10.0);
                                        }

                                        let forgot_resp = ui
                                            .add(
                                                egui::Button::new(
                                                    RichText::new("Quên mã khoá? Đăng xuất")
                                                        .color(pal.link_text)
                                                        .size(12.2),
                                                )
                                                .frame(false),
                                            )
                                            .on_hover_cursor(CursorIcon::PointingHand);
                                        if forgot_resp.clicked() {
                                            self.app_lock_ui.confirm_forgot_open =
                                                !self.app_lock_ui.confirm_forgot_open;
                                        }

                                        if self.app_lock_ui.confirm_forgot_open {
                                            ui.add_space(8.0);
                                            egui::Frame::none()
                                                .fill(pal.bg_chat_canvas)
                                                .stroke(Stroke::new(1.0_f32, pal.border))
                                                .rounding(Rounding::same(8.0))
                                                .inner_margin(Margin::symmetric(12.0, 10.0))
                                                .show(ui, |ui| {
                                                    ui.label(
                                                        RichText::new(
                                                            "Thao tác này sẽ đăng xuất toàn bộ tài khoản đang lưu trên thiết bị để gỡ khoá. Lịch sử tin nhắn SQLite (accounts/*_history.db) vẫn được giữ nguyên.",
                                                        )
                                                        .color(pal.text_secondary)
                                                        .size(11.8),
                                                    );
                                                    ui.add_space(8.0);
                                                    ui.horizontal(|ui| {
                                                        let confirm_btn = egui::Button::new(
                                                            RichText::new("Xác nhận đăng xuất")
                                                                .color(pal.primary_fg)
                                                                .size(12.2)
                                                                .strong(),
                                                        )
                                                        .fill(pal.danger)
                                                        .rounding(Rounding::same(6.0));
                                                        if ui
                                                            .add(confirm_btn)
                                                            .on_hover_cursor(
                                                                CursorIcon::PointingHand,
                                                            )
                                                            .clicked()
                                                        {
                                                            if let Some(tx) = WORKER_TX.get() {
                                                                let _ = tx.send(
                                                                    WorkerCmd::ForgetLockAndSignOutAll,
                                                                );
                                                            }
                                                            self.app_lock_ui.confirm_forgot_open =
                                                                false;
                                                            self.status_msg =
                                                                "Đang đăng xuất toàn bộ tài khoản…"
                                                                    .into();
                                                        }
                                                        if ui
                                                            .button(
                                                                RichText::new("Huỷ")
                                                                    .color(pal.text_primary)
                                                                    .size(12.2),
                                                            )
                                                            .on_hover_cursor(
                                                                CursorIcon::PointingHand,
                                                            )
                                                            .clicked()
                                                        {
                                                            self.app_lock_ui.confirm_forgot_open =
                                                                false;
                                                        }
                                                    });
                                                });
                                        }
                                    });
                                });
                            ui.add_space(24.0);
                        });
                    });
            });

        if let Some(candidate) = submit_candidate {
            let outcome = self.app_lock_config.verify_attempt(&candidate, now);
            let _ = persist_app_lock_config(&self.app_lock_config);
            self.app_lock_ui.unlock_input.clear();
            match outcome {
                VerifyOutcome::Unlocked => {
                    self.app_locked = false;
                    self.app_lock_ui.unlock_error = None;
                    self.app_lock_ui.corrupted_config = false;
                    self.app_lock_ui.confirm_forgot_open = false;
                    self.app_lock_ui.focus_requested = false;
                    self.app_lock_ui.last_activity_ui_time = ctx.input(|i| i.time);
                    show_unlock_passcode = false;
                    ctx.request_repaint();
                }
                VerifyOutcome::Wrong { .. } | VerifyOutcome::LockedOut { .. } => {
                    self.app_lock_ui.unlock_error = Some(format_verify_error(outcome));
                    self.app_lock_ui.focus_requested = true;
                    ctx.request_repaint();
                }
            }
        }

        save_ctx_temp(ctx, APP_LOCK_UNLOCK_SHOW_KEY, show_unlock_passcode);
    }

    /// Render the Settings -> Privacy `Khoá bảo mật (App Lock)` card; returns `true` when `Khoá ngay` is clicked.
    pub(super) fn render_privacy_app_lock_section(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        pal: &ThemePalette,
    ) -> bool {
        let mut lock_now_requested = false;
        let quick_lock_shortcut = if cfg!(target_os = "macos") {
            "Cmd+L"
        } else {
            "Ctrl+L"
        };

        let mut form_ui: AppLockSettingsFormUi = load_ctx_temp(ctx, APP_LOCK_SETTINGS_UI_KEY);
        if self.app_lock_ui.settings_action != form_ui.last_action {
            form_ui = AppLockSettingsFormUi {
                last_action: self.app_lock_ui.settings_action,
                show_passcode: false,
                focus_step: if self.app_lock_ui.settings_action != AppLockSettingsAction::Idle {
                    Some(0)
                } else {
                    None
                },
                last_focused_step: if self.app_lock_ui.settings_action != AppLockSettingsAction::Idle
                {
                    Some(0)
                } else {
                    None
                },
            };
        }

        ui.add_space(4.0);
        ui.label(
            RichText::new("Khoá bảo mật (App Lock)")
                .color(pal.text_primary)
                .size(13.5)
                .strong(),
        );
        ui.add_space(6.0);

        egui::Frame::none()
            .fill(pal.bg_elevated)
            .stroke(Stroke::new(1.0_f32, pal.border))
            .rounding(Rounding::same(8.0))
            .inner_margin(Margin::symmetric(14.0, 12.0))
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());

                if !self.app_lock_config.enabled {
                    if self.app_lock_ui.settings_action != AppLockSettingsAction::Enabling {
                        ui.label(
                            RichText::new(
                                "Bảo vệ ứng dụng bằng mã PIN 4 số hoặc mật khẩu khi khởi động, máy ngủ, hoặc không thao tác.",
                            )
                            .color(pal.text_secondary)
                            .size(12.2),
                        );
                        ui.add_space(10.0);
                        let enable_btn = egui::Button::new(
                            RichText::new("Bật khoá bảo mật")
                                .color(pal.primary_fg)
                                .size(13.0)
                                .strong(),
                        )
                        .fill(pal.primary)
                        .rounding(Rounding::same(6.0));
                        if ui
                            .add(enable_btn)
                            .on_hover_cursor(CursorIcon::PointingHand)
                            .clicked()
                        {
                            self.open_app_lock_setup_modal(ctx);
                        }
                    } else {
                        if render_segmented_mode_selector(
                            ui,
                            pal,
                            "Loại mã khoá",
                            &mut self.app_lock_ui.draft_mode,
                        ) {
                            self.app_lock_ui.draft_new.clear();
                            self.app_lock_ui.draft_confirm.clear();
                            self.app_lock_ui.settings_error = None;
                            form_ui.focus_step = Some(0);
                            form_ui.last_focused_step = Some(0);
                        }
                        ui.add_space(10.0);
                        let (fields_changed, confirm_enter) = render_new_and_confirm_fields(
                            ui,
                            pal,
                            APP_LOCK_ENABLE_NEW_ID,
                            APP_LOCK_ENABLE_CONFIRM_ID,
                            self.app_lock_ui.draft_mode,
                            &mut self.app_lock_ui.draft_new,
                            &mut self.app_lock_ui.draft_confirm,
                            &mut form_ui.show_passcode,
                            &mut form_ui.focus_step,
                            &mut form_ui.last_focused_step,
                            0,
                            1,
                        );
                        if fields_changed {
                            self.app_lock_ui.settings_error = None;
                        }

                        render_settings_panel_error(
                            ui,
                            ctx,
                            pal,
                            &self.app_lock_config,
                            &mut self.app_lock_ui.settings_error,
                        );
                        ui.add_space(10.0);
                        let readiness = setup_readiness_hint(
                            self.app_lock_ui.draft_mode,
                            &self.app_lock_ui.draft_new,
                            &self.app_lock_ui.draft_confirm,
                        );
                        ui.horizontal(|ui| {
                            let clicked =
                                render_submit_button(ui, pal, "Lưu & Bật", false, readiness);
                            if clicked || confirm_enter {
                                match validate_setup_submit(
                                    self.app_lock_ui.draft_mode,
                                    &self.app_lock_ui.draft_new,
                                    &self.app_lock_ui.draft_confirm,
                                ) {
                                    Err((bad_step, msg)) => {
                                        self.app_lock_ui.settings_error = Some(msg.to_string());
                                        form_ui.focus_step = Some(bad_step);
                                    }
                                    Ok(()) => match AppLockConfig::create_enabled(
                                        self.app_lock_ui.draft_mode,
                                        &self.app_lock_ui.draft_new,
                                    ) {
                                        Ok(cfg) => {
                                            if let Err(err) = persist_app_lock_config(&cfg) {
                                                self.app_lock_ui.settings_error = Some(format!(
                                                    "Không thể lưu app_lock.json: {err}"
                                                ));
                                            } else {
                                                self.app_lock_config = cfg;
                                                set_idle_lock_timeout_secs(
                                                    self.app_lock_config.idle_timeout_secs,
                                                );
                                                self.app_lock_ui.corrupted_config = false;
                                                self.app_lock_ui.reset_settings_drafts();
                                                form_ui = AppLockSettingsFormUi::default();
                                                self.show_toast("Đã bật khoá bảo mật ứng dụng");
                                            }
                                        }
                                        Err(err) => {
                                            self.app_lock_ui.settings_error =
                                                Some(format!("Không thể tạo mã khoá: {err}"));
                                        }
                                    },
                                }
                            }
                            if ui
                                .button("Huỷ")
                                .on_hover_cursor(CursorIcon::PointingHand)
                                .clicked()
                            {
                                self.app_lock_ui.reset_settings_drafts();
                                form_ui = AppLockSettingsFormUi::default();
                            }
                        });
                    }
                } else {
                    let mode_label = match self.app_lock_config.mode {
                        PasscodeMode::Pin4 => "Đang bật · PIN 4 số",
                        PasscodeMode::Custom => "Đang bật · Mật khẩu tuỳ chỉnh",
                    };
                    ui.horizontal_wrapped(|ui| {
                        let (dot_r, _) =
                            ui.allocate_exact_size(Vec2::new(16.0, 16.0), Sense::hover());
                        draw_lucide_icon(
                            ui.painter(),
                            dot_r.center(),
                            lucide::SHIELD_CHECK,
                            14.0,
                            pal.accent_online,
                        );
                        ui.label(
                            RichText::new(mode_label)
                                .color(pal.accent_online)
                                .size(12.8)
                                .strong(),
                        );
                    });
                    ui.add_space(6.0);
                    ui.horizontal_wrapped(|ui| {
                        let lock_btn = egui::Button::new(
                            RichText::new(format!("Khoá ngay ({quick_lock_shortcut})"))
                                .color(pal.primary_fg)
                                .size(12.2)
                                .strong(),
                        )
                        .fill(pal.primary)
                        .rounding(Rounding::same(6.0));
                        if ui
                            .add(lock_btn)
                            .on_hover_cursor(CursorIcon::PointingHand)
                            .clicked()
                        {
                            lock_now_requested = true;
                        }

                        let changing_active =
                            self.app_lock_ui.settings_action == AppLockSettingsAction::Changing;
                        let change_btn = egui::Button::new(
                            RichText::new("Đổi mã khoá").size(12.2).color(if changing_active {
                                if pal.is_dark {
                                    pal.primary_hover
                                } else {
                                    pal.primary
                                }
                            } else {
                                pal.text_primary
                            }),
                        )
                        .fill(if changing_active {
                            pal.bg_selected
                        } else {
                            pal.bg_input_track
                        });
                        if ui
                            .add(change_btn)
                            .on_hover_cursor(CursorIcon::PointingHand)
                            .clicked()
                        {
                            self.app_lock_ui.reset_settings_drafts();
                            self.app_lock_ui.draft_mode = self.app_lock_config.mode;
                            self.app_lock_ui.settings_action = AppLockSettingsAction::Changing;
                            form_ui = AppLockSettingsFormUi {
                                last_action: AppLockSettingsAction::Changing,
                                show_passcode: false,
                                focus_step: Some(0),
                                last_focused_step: Some(0),
                            };
                        }

                        let disabling_active =
                            self.app_lock_ui.settings_action == AppLockSettingsAction::Disabling;
                        let disable_btn = egui::Button::new(
                            RichText::new("Tắt khoá bảo mật").size(12.2).color(
                                if disabling_active {
                                    pal.danger_text
                                } else {
                                    pal.text_primary
                                },
                            ),
                        )
                        .fill(if disabling_active {
                            pal.danger.linear_multiply(0.16)
                        } else {
                            pal.bg_input_track
                        });
                        if ui
                            .add(disable_btn)
                            .on_hover_cursor(CursorIcon::PointingHand)
                            .clicked()
                        {
                            self.app_lock_ui.reset_settings_drafts();
                            self.app_lock_ui.settings_action = AppLockSettingsAction::Disabling;
                            form_ui = AppLockSettingsFormUi {
                                last_action: AppLockSettingsAction::Disabling,
                                show_passcode: false,
                                focus_step: Some(0),
                                last_focused_step: Some(0),
                            };
                        }
                    });

                    if self.app_lock_ui.settings_action == AppLockSettingsAction::Changing {
                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(10.0);

                        let cur_status = evaluate_current_field_status(
                            self.app_lock_config.mode,
                            &self.app_lock_ui.draft_current,
                        );
                        let req_cur = form_ui.focus_step.take_if(|s| *s == 0).is_some();
                        let cur_hint = match self.app_lock_config.mode {
                            PasscodeMode::Pin4 => "Nhập 4 chữ số PIN hiện tại",
                            PasscodeMode::Custom => "Nhập mật khẩu hiện tại",
                        };
                        let cur_out = render_labeled_passcode_input(
                            ui,
                            pal,
                            APP_LOCK_CHANGE_CURRENT_ID,
                            "Mã khoá hiện tại",
                            Some(&cur_status),
                            &mut self.app_lock_ui.draft_current,
                            self.app_lock_config.mode,
                            cur_hint,
                            &mut form_ui.show_passcode,
                            req_cur,
                        );
                        track_and_advance_focus(
                            0,
                            cur_out,
                            &mut form_ui.focus_step,
                            &mut form_ui.last_focused_step,
                            Some(1),
                            None,
                        );
                        if cur_out.changed {
                            self.app_lock_ui.settings_error = None;
                        }

                        ui.add_space(10.0);
                        if render_segmented_mode_selector(
                            ui,
                            pal,
                            "Loại mã khoá mới",
                            &mut self.app_lock_ui.draft_mode,
                        ) {
                            self.app_lock_ui.draft_new.clear();
                            self.app_lock_ui.draft_confirm.clear();
                            self.app_lock_ui.settings_error = None;
                            form_ui.focus_step = Some(1);
                            form_ui.last_focused_step = Some(1);
                        }

                        ui.add_space(10.0);
                        let (fields_changed, confirm_enter) = render_new_and_confirm_fields(
                            ui,
                            pal,
                            APP_LOCK_CHANGE_NEW_ID,
                            APP_LOCK_CHANGE_CONFIRM_ID,
                            self.app_lock_ui.draft_mode,
                            &mut self.app_lock_ui.draft_new,
                            &mut self.app_lock_ui.draft_confirm,
                            &mut form_ui.show_passcode,
                            &mut form_ui.focus_step,
                            &mut form_ui.last_focused_step,
                            1,
                            2,
                        );
                        if fields_changed {
                            self.app_lock_ui.settings_error = None;
                        }

                        render_settings_panel_error(
                            ui,
                            ctx,
                            pal,
                            &self.app_lock_config,
                            &mut self.app_lock_ui.settings_error,
                        );
                        ui.add_space(10.0);
                        let readiness = change_readiness_hint(
                            self.app_lock_config.mode,
                            &self.app_lock_ui.draft_current,
                            self.app_lock_ui.draft_mode,
                            &self.app_lock_ui.draft_new,
                            &self.app_lock_ui.draft_confirm,
                        );
                        ui.horizontal(|ui| {
                            let clicked =
                                render_submit_button(ui, pal, "Xác nhận đổi mã", false, readiness);
                            if clicked || confirm_enter {
                                match validate_change_submit(
                                    self.app_lock_config.mode,
                                    &self.app_lock_ui.draft_current,
                                    self.app_lock_ui.draft_mode,
                                    &self.app_lock_ui.draft_new,
                                    &self.app_lock_ui.draft_confirm,
                                ) {
                                    Err((bad_step, msg)) => {
                                        self.app_lock_ui.settings_error = Some(msg.to_string());
                                        form_ui.focus_step = Some(bad_step);
                                    }
                                    Ok(()) => {
                                        let now = crate::login::unix_now();
                                        let mut next_cfg = self.app_lock_config.clone();
                                        let res = next_cfg.change_passcode(
                                            &self.app_lock_ui.draft_current,
                                            self.app_lock_ui.draft_mode,
                                            &self.app_lock_ui.draft_new,
                                            now,
                                        );
                                        match res {
                                            Ok(ChangePasscodeOutcome::Changed) => {
                                                if let Err(err) = persist_app_lock_config(&next_cfg)
                                                {
                                                    self.app_lock_ui.settings_error =
                                                        Some(format!(
                                                            "Không thể lưu app_lock.json: {err}"
                                                        ));
                                                } else {
                                                    self.app_lock_config = next_cfg;
                                                    self.app_lock_ui.reset_settings_drafts();
                                                    form_ui = AppLockSettingsFormUi::default();
                                                    self.show_toast("Đã đổi mã khoá ứng dụng");
                                                }
                                            }
                                            Ok(ChangePasscodeOutcome::InvalidNew(msg)) => {
                                                self.app_lock_ui.settings_error =
                                                    Some(msg.to_string());
                                                form_ui.focus_step = Some(1);
                                            }
                                            Ok(ChangePasscodeOutcome::VerifyFailed(outcome)) => {
                                                self.app_lock_config = next_cfg;
                                                let _ =
                                                    persist_app_lock_config(&self.app_lock_config);
                                                self.app_lock_ui.draft_current.clear();
                                                self.app_lock_ui.settings_error =
                                                    Some(format_verify_error(outcome));
                                                form_ui.focus_step = Some(0);
                                                if self.app_lock_config.remaining_lockout_secs(now)
                                                    > 0
                                                {
                                                    ctx.request_repaint_after(
                                                        Duration::from_millis(500),
                                                    );
                                                }
                                                ctx.request_repaint();
                                            }
                                            Err(err) => {
                                                self.app_lock_ui.settings_error = Some(format!(
                                                    "Không thể đổi mã khoá: {err}"
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                            if ui
                                .button("Huỷ")
                                .on_hover_cursor(CursorIcon::PointingHand)
                                .clicked()
                            {
                                self.app_lock_ui.reset_settings_drafts();
                                form_ui = AppLockSettingsFormUi::default();
                            }
                        });
                    } else if self.app_lock_ui.settings_action == AppLockSettingsAction::Disabling
                    {
                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(10.0);
                        ui.label(
                            RichText::new("Nhập mã khoá hiện tại để tắt Khoá bảo mật.")
                                .color(pal.text_secondary)
                                .size(12.2),
                        );
                        ui.add_space(8.0);
                        let cur_status = evaluate_current_field_status(
                            self.app_lock_config.mode,
                            &self.app_lock_ui.draft_current,
                        );
                        let req_cur = form_ui.focus_step.take_if(|s| *s == 0).is_some();
                        let cur_hint = match self.app_lock_config.mode {
                            PasscodeMode::Pin4 => "Nhập 4 chữ số PIN hiện tại",
                            PasscodeMode::Custom => "Nhập mật khẩu hiện tại",
                        };
                        let cur_out = render_labeled_passcode_input(
                            ui,
                            pal,
                            APP_LOCK_DISABLE_CURRENT_ID,
                            "Mã khoá hiện tại",
                            Some(&cur_status),
                            &mut self.app_lock_ui.draft_current,
                            self.app_lock_config.mode,
                            cur_hint,
                            &mut form_ui.show_passcode,
                            req_cur,
                        );
                        track_and_advance_focus(
                            0,
                            cur_out,
                            &mut form_ui.focus_step,
                            &mut form_ui.last_focused_step,
                            None,
                            None,
                        );
                        if cur_out.changed {
                            self.app_lock_ui.settings_error = None;
                        }

                        render_settings_panel_error(
                            ui,
                            ctx,
                            pal,
                            &self.app_lock_config,
                            &mut self.app_lock_ui.settings_error,
                        );
                        ui.add_space(10.0);
                        let readiness = disable_readiness_hint(
                            self.app_lock_config.mode,
                            &self.app_lock_ui.draft_current,
                        );
                        ui.horizontal(|ui| {
                            let clicked =
                                render_submit_button(ui, pal, "Xác nhận tắt", true, readiness);
                            if clicked || cur_out.enter_pressed {
                                if let Err(msg) = validate_disable_submit(
                                    self.app_lock_config.mode,
                                    &self.app_lock_ui.draft_current,
                                ) {
                                    self.app_lock_ui.settings_error = Some(msg.to_string());
                                    form_ui.focus_step = Some(0);
                                } else {
                                    let now = crate::login::unix_now();
                                    let mut next_cfg = self.app_lock_config.clone();
                                    let outcome = next_cfg
                                        .verify_attempt(&self.app_lock_ui.draft_current, now);
                                    self.app_lock_ui.draft_current.clear();
                                    match outcome {
                                        VerifyOutcome::Unlocked => {
                                            if let Err(err) = AppLockConfig::remove_default() {
                                                crate::logger::log_error(
                                                    "app_lock",
                                                    &err.to_string(),
                                                );
                                                self.app_lock_ui.settings_error = Some(format!(
                                                    "Không thể xoá app_lock.json: {err}"
                                                ));
                                            } else {
                                                self.app_lock_config = AppLockConfig::default();
                                                set_idle_lock_timeout_secs(0);
                                                self.app_lock_ui.corrupted_config = false;
                                                self.app_lock_ui.reset_settings_drafts();
                                                form_ui = AppLockSettingsFormUi::default();
                                                self.show_toast("Đã tắt khoá bảo mật");
                                            }
                                        }
                                        VerifyOutcome::Wrong { .. }
                                        | VerifyOutcome::LockedOut { .. } => {
                                            self.app_lock_config = next_cfg;
                                            let _ = persist_app_lock_config(&self.app_lock_config);
                                            self.app_lock_ui.settings_error =
                                                Some(format_verify_error(outcome));
                                            form_ui.focus_step = Some(0);
                                            if self.app_lock_config.remaining_lockout_secs(now) > 0
                                            {
                                                ctx.request_repaint_after(Duration::from_millis(
                                                    500,
                                                ));
                                            }
                                            ctx.request_repaint();
                                        }
                                    }
                                }
                            }
                            if ui
                                .button("Huỷ")
                                .on_hover_cursor(CursorIcon::PointingHand)
                                .clicked()
                            {
                                self.app_lock_ui.reset_settings_drafts();
                                form_ui = AppLockSettingsFormUi::default();
                            }
                        });
                    }

                    if self.app_lock_config.enabled {
                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(8.0);
                        let mut prefs_changed = false;

                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new("Tự khoá khi máy ngủ (sleep)")
                                    .color(pal.text_primary)
                                    .size(13.0),
                            );
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                if render_toggle_switch(
                                    ui,
                                    &mut self.app_lock_config.lock_on_sleep,
                                    pal,
                                )
                                .changed()
                                {
                                    prefs_changed = true;
                                }
                            });
                        });
                        ui.add_space(8.0);

                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new("Tự khoá khi không thao tác")
                                    .color(pal.text_primary)
                                    .size(13.0),
                            );
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                let prev_idle = self.app_lock_config.idle_timeout_secs;
                                egui::ComboBox::from_id_salt("app_lock_idle_timeout_combo")
                                    .selected_text(idle_timeout_label(prev_idle))
                                    .width(130.0)
                                    .show_ui(ui, |ui| {
                                        for &preset in &IDLE_PRESETS_SECS {
                                            ui.selectable_value(
                                                &mut self.app_lock_config.idle_timeout_secs,
                                                preset,
                                                idle_timeout_label(preset),
                                            );
                                        }
                                    });
                                if self.app_lock_config.idle_timeout_secs != prev_idle {
                                    prefs_changed = true;
                                }
                            });
                        });
                        ui.add_space(8.0);

                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new("Ẩn nội dung thông báo khi đang khoá")
                                    .color(pal.text_primary)
                                    .size(13.0),
                            );
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                if render_toggle_switch(
                                    ui,
                                    &mut self.app_lock_config.hide_notifications_when_locked,
                                    pal,
                                )
                                .changed()
                                {
                                    prefs_changed = true;
                                }
                            });
                        });

                        if prefs_changed {
                            let _ = persist_app_lock_config(&self.app_lock_config);
                            set_idle_lock_timeout_secs(self.app_lock_config.idle_timeout_secs);
                        }
                    }
                }
            });

        if self.app_lock_ui.settings_action == AppLockSettingsAction::Idle {
            ctx.data_mut(|d| {
                d.remove_temp::<AppLockSettingsFormUi>(Id::new(APP_LOCK_SETTINGS_UI_KEY));
            });
        } else {
            if form_ui.focus_step.is_some() {
                ctx.request_repaint();
            }
            save_ctx_temp(ctx, APP_LOCK_SETTINGS_UI_KEY, form_ui);
        }

        ui.add_space(14.0);
        lock_now_requested
    }
}

/// Scrub quick-setup and settings drafts when the lock screen renders without disturbing unlock reveal toggle.
fn scrub_transient_ctx_state_except_unlock(ctx: &egui::Context) {
    ctx.data_mut(|d| {
        d.remove_temp::<AppLockSetupModalState>(Id::new(APP_LOCK_SETUP_STATE_KEY));
        d.remove_temp::<AppLockSettingsFormUi>(Id::new(APP_LOCK_SETTINGS_UI_KEY));
    });
}
