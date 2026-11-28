//! Native Windows notification-area icon and message callbacks on the GUI thread.

use std::{cell::Cell, io, mem::size_of, ptr::null_mut, rc::Rc};

use eframe::egui;
use windows_sys::{
    core::w,
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM},
        System::LibraryLoader::GetModuleHandleW,
        UI::{Shell::*, WindowsAndMessaging::*},
    },
};

const TRAY_MESSAGE: u32 = WM_APP + 1;
const KEY_SELECT: u32 = NIN_SELECT | 1;
const OPEN_COMMAND: u32 = 1;
const QUIT_COMMAND: u32 = 2;
const CLASS: windows_sys::core::PCWSTR = w!("ctdozlite.NotificationArea");

#[derive(Clone, Copy, PartialEq, Eq)]
/// A user action or lifecycle event from the Windows shell.
pub(super) enum Action {
    Open,
    Quit,
    ExplorerRestarted,
}

struct Callbacks {
    ctx: egui::Context,
    taskbar_created: u32,
    action: Cell<Option<Action>>,
}

/// Owns the icon and the hidden window that receives its callbacks.
pub(super) struct Tray {
    hwnd: HWND,
    icon: HICON,
    callbacks: Rc<Callbacks>,
    parked_window: ParkedWindow,
    parked_since_ms: i64,
    message_alerts: super::TrayMessageAlerts,
}

/// The main window retains its normal placement while parked as a tool window.
struct ParkedWindow {
    hwnd: HWND,
    style: isize,
    placement: WINDOWPLACEMENT,
}

impl ParkedWindow {
    fn capture(hwnd: HWND) -> io::Result<Self> {
        let mut placement = WINDOWPLACEMENT {
            length: size_of::<WINDOWPLACEMENT>() as u32,
            ..Default::default()
        };
        // SAFETY: The app owns hwnd, and placement is writable for the duration of the call.
        unsafe {
            if GetWindowPlacement(hwnd, &mut placement) == 0 {
                return Err(io::Error::last_os_error());
            }
            Ok(Self {
                hwnd,
                style: GetWindowLongPtrW(hwnd, GWL_EXSTYLE),
                placement,
            })
        }
    }

    fn park(&self) -> io::Result<()> {
        let tray_style = (self.style | WS_EX_TOOLWINDOW as isize) & !(WS_EX_APPWINDOW as isize);
        // SAFETY: This runs on the main window's owning thread.
        unsafe {
            ShowWindow(self.hwnd, SW_HIDE);
            SetWindowLongPtrW(self.hwnd, GWL_EXSTYLE, tray_style);
            if GetWindowLongPtrW(self.hwnd, GWL_EXSTYLE) != tray_style {
                self.restore();
                return Err(io::Error::other(
                    "Windows did not apply the tray window style",
                ));
            }
            ShowWindow(self.hwnd, SW_SHOWMINNOACTIVE);
            if IsIconic(self.hwnd) == 0 || IsWindowVisible(self.hwnd) == 0 {
                self.restore();
                return Err(io::Error::other("Windows did not minimize the tray window"));
            }
        }
        Ok(())
    }

    fn restore(&self) {
        let mut placement = self.placement;
        if placement.showCmd == SW_SHOWMINIMIZED as u32 {
            placement.showCmd = if placement.flags & WPF_RESTORETOMAXIMIZED != 0 {
                SW_SHOWMAXIMIZED as u32
            } else {
                SW_SHOWNORMAL as u32
            };
        }
        // SAFETY: The app still owns hwnd and placement was read from this window.
        unsafe {
            ShowWindow(self.hwnd, SW_HIDE);
            SetWindowLongPtrW(self.hwnd, GWL_EXSTYLE, self.style);
            if SetWindowPlacement(self.hwnd, &placement) == 0 {
                ShowWindow(self.hwnd, SW_RESTORE);
            }
        }
    }
}

impl Tray {
    /// Install the icon before the caller hides its main window.
    ///
    /// # Errors
    /// Returns an error if the callback window or shell icon cannot be created.
    pub(super) fn new(ctx: &egui::Context, hwnd: usize) -> io::Result<Self> {
        let parked_window = ParkedWindow::capture(hwnd as HWND)?;
        // SAFETY: The GUI thread owns the class and keeps callbacks alive until DestroyWindow.
        unsafe {
            let instance = GetModuleHandleW(std::ptr::null());
            let icon = LoadIconW(instance, 1_usize as windows_sys::core::PCWSTR);
            let taskbar_created = RegisterWindowMessageW(w!("TaskbarCreated"));
            if icon.is_null() || taskbar_created == 0 {
                return Err(io::Error::last_os_error());
            }
            let class = WNDCLASSW {
                lpfnWndProc: Some(window_proc),
                hInstance: instance,
                lpszClassName: CLASS,
                ..Default::default()
            };
            if RegisterClassW(&class) == 0 {
                return Err(io::Error::last_os_error());
            }
            let callbacks = Rc::new(Callbacks {
                ctx: ctx.clone(),
                taskbar_created,
                action: Cell::new(None),
            });
            let hwnd = CreateWindowExW(
                WS_EX_TOOLWINDOW,
                CLASS,
                w!("ctdozlite"),
                WS_POPUP,
                0,
                0,
                0,
                0,
                null_mut(),
                null_mut(),
                instance,
                Rc::as_ptr(&callbacks).cast(),
            );
            if hwnd.is_null() {
                let error = io::Error::last_os_error();
                UnregisterClassW(CLASS, instance);
                return Err(error);
            }
            let tray = Self {
                hwnd,
                icon,
                callbacks,
                parked_window,
                parked_since_ms: crate::clock::normalize_ts_ms(crate::login::unix_now()),
                message_alerts: Default::default(),
            };
            if tray.register() {
                tray.parked_window.park()?;
                Ok(tray)
            } else {
                Err(io::Error::other("Windows did not register the tray icon"))
            }
        }
    }

    /// Return the main window to its normal taskbar style and placement.
    pub(super) fn restore_window(self) {
        self.parked_window.restore();
    }

    /// Record eligible live delivery separately from the unread and message stores.
    pub(super) fn record_message_alerts(
        &mut self,
        account: &str,
        thread: &str,
        messages: &[super::ChatMessage],
        seen: &str,
    ) -> bool {
        self.message_alerts
            .record(account, thread, messages, seen, self.parked_since_ms)
    }

    /// Reinstall the icon after Explorer has discarded its notification area.
    pub(super) fn register(&self) -> bool {
        let mut data = icon_data(self.hwnd);
        data.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP | NIF_SHOWTIP;
        data.hIcon = self.icon;
        data.uCallbackMessage = TRAY_MESSAGE;
        data.szTip = wide_text("ctdozlite");
        data.Anonymous.uVersion = NOTIFYICON_VERSION_4;
        // SAFETY: The callback window is live, and data is initialized for both calls.
        unsafe {
            Shell_NotifyIconW(NIM_ADD, &data) != 0 && Shell_NotifyIconW(NIM_SETVERSION, &data) != 0
        }
    }

    /// Take the action queued by the shell callback.
    pub(super) fn take_action(&self) -> Option<Action> {
        self.callbacks.action.take()
    }

    /// Ask Windows to show a private message alert under the tray icon.
    pub(super) fn notify(&self, body: &str) -> bool {
        let mut data = icon_data(self.hwnd);
        data.uFlags = NIF_INFO;
        data.szInfoTitle = wide_text("ctdozlite");
        data.szInfo = wide_text(body);
        data.dwInfoFlags = NIIF_INFO | NIIF_RESPECT_QUIET_TIME | NIIF_NOSOUND;
        // SAFETY: The window still owns this icon and data lives through the call.
        unsafe { Shell_NotifyIconW(NIM_MODIFY, &data) != 0 }
    }
}

fn icon_data(hwnd: HWND) -> NOTIFYICONDATAW {
    NOTIFYICONDATAW {
        cbSize: size_of::<NOTIFYICONDATAW>() as u32,
        hWnd: hwnd,
        uID: 1,
        ..Default::default()
    }
}

impl Drop for Tray {
    fn drop(&mut self) {
        // SAFETY: The GUI thread still owns the window, class, and callbacks.
        unsafe {
            Shell_NotifyIconW(NIM_DELETE, &icon_data(self.hwnd));
            DestroyWindow(self.hwnd);
            UnregisterClassW(CLASS, GetModuleHandleW(std::ptr::null()));
        }
    }
}

fn wide_text<const N: usize>(value: &str) -> [u16; N] {
    let mut text = [0; N];
    for (out, unit) in text.iter_mut().take(N - 1).zip(value.encode_utf16()) {
        *out = unit;
    }
    text
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // SAFETY: WM_NCCREATE provides the Rc pointer, which is cleared before its owner drops.
    unsafe {
        if message == WM_NCCREATE {
            let create = &*(lparam as *const CREATESTRUCTW);
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, create.lpCreateParams as isize);
        }
        if message == WM_NCDESTROY {
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
        }
        let callbacks = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const Callbacks;
        if let Some(callbacks) = callbacks.as_ref() {
            let action = if message == callbacks.taskbar_created {
                Some(Action::ExplorerRestarted)
            } else if message == TRAY_MESSAGE {
                let event = (lparam as u32) & 0xffff;
                match event {
                    NIN_SELECT | KEY_SELECT | NIN_BALLOONUSERCLICK => Some(Action::Open),
                    WM_CONTEXTMENU => {
                        let ptr = callbacks as *const Callbacks;
                        Rc::increment_strong_count(ptr);
                        let callbacks = Rc::from_raw(ptr);
                        if let Some(action) = tray_menu(hwnd, wparam) {
                            queue_action(&callbacks, action);
                        }
                        return 0;
                    }
                    _ => None,
                }
            } else {
                return DefWindowProcW(hwnd, message, wparam, lparam);
            };
            if let Some(action) = action {
                queue_action(callbacks, action);
            }
            return 0;
        }
        DefWindowProcW(hwnd, message, wparam, lparam)
    }
}

fn queue_action(callbacks: &Callbacks, action: Action) {
    let queued = callbacks.action.get();
    if queued != Some(Action::Quit) && (action != Action::ExplorerRestarted || queued.is_none()) {
        callbacks.action.set(Some(action));
        callbacks.ctx.request_repaint();
    }
}

fn tray_menu(hwnd: HWND, anchor: WPARAM) -> Option<Action> {
    let x = (anchor as u16 as i16) as i32;
    let y = ((anchor >> 16) as u16 as i16) as i32;
    // SAFETY: hwnd is the live callback window and the popup is local to this call.
    unsafe {
        let menu = CreatePopupMenu();
        if menu.is_null() {
            return None;
        }
        let result = if AppendMenuW(menu, MF_STRING, OPEN_COMMAND as usize, w!("Mở ctdozlite")) != 0
            && AppendMenuW(
                menu,
                MF_STRING,
                QUIT_COMMAND as usize,
                w!("Thoát hoàn toàn"),
            ) != 0
        {
            SetForegroundWindow(hwnd);
            let selected = TrackPopupMenu(
                menu,
                TPM_RETURNCMD | TPM_RIGHTBUTTON | TPM_NONOTIFY,
                x,
                y,
                0,
                hwnd,
                std::ptr::null(),
            );
            PostMessageW(hwnd, WM_NULL, 0, 0);
            match selected as u32 {
                OPEN_COMMAND => Some(Action::Open),
                QUIT_COMMAND => Some(Action::Quit),
                _ => {
                    Shell_NotifyIconW(NIM_SETFOCUS, &icon_data(hwnd));
                    None
                }
            }
        } else {
            None
        };
        DestroyMenu(menu);
        result
    }
}
