//! Bounded macOS notification authorization bridge, without notification delivery.
use std::{
    path::{Path, PathBuf},
    process::Stdio,
    time::Duration,
};
use tokio::io::AsyncReadExt;

/// Authorization reported by the operating system helper.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthorizationStatus {
    /// The user has not chosen yet.
    NotDetermined,
    /// The user denied authorization.
    Denied,
    /// Alerts are authorized.
    Authorized,
    /// Quiet alerts are authorized.
    Provisional,
    /// Temporary authorization is available.
    Ephemeral,
    /// The operating system cannot report authorization.
    Unavailable,
}
impl AuthorizationStatus {
    /// Vietnamese label for the Settings panel.
    pub fn label(self) -> &'static str {
        match self {
            Self::NotDetermined => "Chưa yêu cầu quyền",
            Self::Denied => "Đã từ chối. Bạn có thể đổi trong Cài đặt hệ thống macOS.",
            Self::Authorized => "Đã cấp quyền",
            Self::Provisional => "Được phép thông báo im lặng",
            Self::Ephemeral => "Được cấp quyền tạm thời",
            Self::Unavailable => "Trạng thái quyền chưa khả dụng",
        }
    }
}
/// Bounded helper failures; raw helper output never reaches the UI.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotificationError {
    /// Permission requests require an explicit user gesture.
    ExplicitUserActionRequired,
    /// This platform or installation has no executable helper.
    HelperUnavailable,
    /// The helper exceeded its deadline.
    Timeout,
    /// The helper exited unsuccessfully or returned invalid/oversized output.
    InvalidResponse,
    /// The child pipe could not be read.
    Io,
}
impl std::fmt::Display for NotificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::ExplicitUserActionRequired => "Hãy bấm nút yêu cầu quyền để tiếp tục.",
            Self::HelperUnavailable => "Chưa tìm thấy trợ giúp quyền thông báo macOS.",
            Self::Timeout => "Đọc quyền thông báo quá thời gian. Hãy thử lại.",
            Self::InvalidResponse => "Trợ giúp quyền thông báo trả kết quả không hợp lệ.",
            Self::Io => "Lỗi đọc kết quả quyền thông báo. Hãy thử lại.",
        })
    }
}
impl std::error::Error for NotificationError {}

fn packaged_helper(executable: &Path) -> Option<PathBuf> {
    Some(
        executable
            .parent()?
            .parent()?
            .join("Helpers/DailyNotifier.app/Contents/MacOS/DailyNotifier"),
    )
}
fn helper_path() -> PathBuf {
    let packaged = std::env::current_exe()
        .ok()
        .and_then(|path| packaged_helper(&path));
    packaged.filter(|path| path.is_file()).unwrap_or_else(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("native/macos/build/DailyNotifier.app/Contents/MacOS/DailyNotifier")
    })
}
/// Spawn the helper, retrying briefly on `ETXTBSY`: a write handle to a freshly written executable
/// can still be held by a concurrently forked child that has not reached `exec` yet.
async fn spawn_helper(
    path: &Path,
    command: &str,
) -> Result<tokio::process::Child, NotificationError> {
    let mut attempt = 0;
    loop {
        match tokio::process::Command::new(path)
            .arg(command)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .kill_on_drop(true)
            .spawn()
        {
            Ok(child) => return Ok(child),
            Err(e) if e.kind() == std::io::ErrorKind::ExecutableFileBusy && attempt < 5 => {
                attempt += 1;
                tokio::time::sleep(Duration::from_millis(10 * attempt)).await;
            }
            Err(_) => return Err(NotificationError::HelperUnavailable),
        }
    }
}
async fn invoke(
    path: &Path,
    command: &str,
    deadline: Duration,
) -> Result<AuthorizationStatus, NotificationError> {
    let mut child = spawn_helper(path, command).await?;
    let operation = async {
        let mut output = Vec::new();
        child
            .stdout
            .take()
            .ok_or(NotificationError::Io)?
            .take(4097)
            .read_to_end(&mut output)
            .await
            .map_err(|_| NotificationError::Io)?;
        if output.len() > 4096 {
            return Err(NotificationError::InvalidResponse);
        }
        if !child
            .wait()
            .await
            .map_err(|_| NotificationError::Io)?
            .success()
        {
            return Err(NotificationError::InvalidResponse);
        }
        parse_status(std::str::from_utf8(&output).map_err(|_| NotificationError::InvalidResponse)?)
    };
    tokio::time::timeout(deadline, operation)
        .await
        .map_err(|_| NotificationError::Timeout)?
}
fn parse_status(output: &str) -> Result<AuthorizationStatus, NotificationError> {
    match output.trim() {
        "notDetermined" => Ok(AuthorizationStatus::NotDetermined),
        "denied" => Ok(AuthorizationStatus::Denied),
        "authorized" => Ok(AuthorizationStatus::Authorized),
        "provisional" => Ok(AuthorizationStatus::Provisional),
        "ephemeral" => Ok(AuthorizationStatus::Ephemeral),
        "unavailable" => Ok(AuthorizationStatus::Unavailable),
        _ => Err(NotificationError::InvalidResponse),
    }
}
async fn platform_invoke(
    command: &str,
    deadline: Duration,
) -> Result<AuthorizationStatus, NotificationError> {
    if !cfg!(target_os = "macos") {
        return Err(NotificationError::HelperUnavailable);
    }
    invoke(&helper_path(), command, deadline).await
}
/// Read authorization without asking for permission or posting a notification.
///
/// # Errors
/// Returns a typed error for unsupported platforms, missing helpers, deadlines or invalid output.
pub async fn status() -> Result<AuthorizationStatus, NotificationError> {
    platform_invoke("--status", Duration::from_secs(10)).await
}
/// Request authorization only after the user explicitly clicks the permission button.
///
/// # Errors
/// Returns `ExplicitUserActionRequired` before spawning when the gesture is absent,
/// or a typed helper error otherwise. This function never posts notifications.
pub async fn request_if_user_action(
    explicit_user_action: bool,
) -> Result<AuthorizationStatus, NotificationError> {
    if !explicit_user_action {
        return Err(NotificationError::ExplicitUserActionRequired);
    }
    platform_invoke("--request", Duration::from_secs(120)).await
}

/// UI-owned sequence gate: at most one request is pending; stale completions are ignored.
#[derive(Default)]
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
pub(crate) struct AuthorizationState {
    next: u64,
    pub(crate) pending: Option<u64>,
    pub(crate) result: Option<Result<AuthorizationStatus, NotificationError>>,
}
impl AuthorizationState {
    #[cfg_attr(not(target_os = "macos"), allow(dead_code))]
    pub(crate) fn begin(&mut self) -> Option<u64> {
        if self.pending.is_some() {
            return None;
        }
        self.next = self.next.checked_add(1)?;
        self.pending = Some(self.next);
        Some(self.next)
    }
    pub(crate) fn finish(
        &mut self,
        id: u64,
        result: Result<AuthorizationStatus, NotificationError>,
    ) {
        if self.pending == Some(id) {
            self.pending = None;
            self.result = Some(result);
        }
    }
}
