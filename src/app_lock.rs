//! Application passcode lock configuration, PBKDF2 verification, and lock-policy helpers.

use anyhow::{bail, Context, Result};
use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::Path;

/// Default relative filename for persisted application lock settings.
pub const APP_LOCK_FILE: &str = "app_lock.json";
/// Current schema version for [`AppLockConfig`].
pub const CURRENT_VERSION: u32 = 1;
/// Default PBKDF2-HMAC-SHA256 iteration count for newly created passcodes.
pub const DEFAULT_ITERATIONS: u32 = 100_000;
/// Minimum accepted PBKDF2-HMAC-SHA256 iteration count when loading persisted config.
pub const MIN_ITERATIONS: u32 = 10_000;
/// CSPRNG salt length in bytes.
pub const SALT_LEN: usize = 16;
/// Number of consecutive failed verification attempts before cooldown begins.
pub const MAX_FAILED_ATTEMPTS: u32 = 5;
/// Brute-force cooldown duration in seconds.
pub const LOCKOUT_SECS: i64 = 30;
/// Wall-clock forward jump threshold in seconds indicating OS sleep/hibernate.
pub const SLEEP_JUMP_THRESHOLD_SECS: i64 = 45;
/// UI auto-lock idle presets in seconds (`0` disables idle auto-lock).
pub const IDLE_PRESETS_SECS: [u64; 4] = [0, 300, 900, 1800];

/// Selected passcode format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PasscodeMode {
    /// Four ASCII digits (`0`-`9`).
    #[default]
    Pin4,
    /// Arbitrary UTF-8 passcode of 4..=32 Unicode scalar values.
    Custom,
}

/// Result of attempting to verify a passcode against an [`AppLockConfig`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyOutcome {
    /// Candidate matched the stored hash; counters have been reset.
    Unlocked,
    /// Candidate did not match; cooldown is not yet active.
    Wrong {
        /// Remaining attempts before lockout triggers.
        remaining_attempts: u32,
    },
    /// Verification is blocked by brute-force cooldown.
    LockedOut {
        /// Remaining seconds until cooldown expires (`1..=LOCKOUT_SECS`).
        remaining_secs: i64,
    },
}

/// Result of attempting to change an existing passcode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangePasscodeOutcome {
    /// Current passcode verified and new passcode persisted in memory.
    Changed,
    /// Current passcode verified, but the replacement failed validation.
    InvalidNew(&'static str),
    /// Current passcode failed verification or was blocked by lockout.
    VerifyFailed(VerifyOutcome),
}

fn default_version() -> u32 {
    CURRENT_VERSION
}

fn default_iterations() -> u32 {
    DEFAULT_ITERATIONS
}

fn default_true() -> bool {
    true
}

/// Persisted application lock state and preferences (`app_lock.json`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppLockConfig {
    /// Schema version (must equal [`CURRENT_VERSION`]).
    #[serde(default = "default_version")]
    pub version: u32,
    /// Whether the lock screen is enabled.
    #[serde(default)]
    pub enabled: bool,
    /// Active passcode validation mode.
    #[serde(default)]
    pub mode: PasscodeMode,
    /// Standard Base64-encoded CSPRNG salt.
    #[serde(default)]
    pub salt_b64: String,
    /// Standard Base64-encoded 32-byte PBKDF2-HMAC-SHA256 digest.
    #[serde(default)]
    pub hash_b64: String,
    /// PBKDF2-HMAC-SHA256 iteration count.
    #[serde(default = "default_iterations")]
    pub iterations: u32,
    /// Whether OS sleep/hibernate jumps trigger immediate lock.
    #[serde(default = "default_true")]
    pub lock_on_sleep: bool,
    /// Idle auto-lock timeout in seconds (`0` disables idle locking).
    #[serde(default)]
    pub idle_timeout_secs: u64,
    /// Whether notifications and toast previews are masked while locked.
    #[serde(default = "default_true")]
    pub hide_notifications_when_locked: bool,
    /// Consecutive wrong passcode attempts since the last success/change/disable.
    #[serde(default)]
    pub failed_attempts: u32,
    /// Unix timestamp (seconds) until which verification is locked out (`0` if inactive).
    #[serde(default)]
    pub lockout_until_unix: i64,
}

impl Default for AppLockConfig {
    fn default() -> Self {
        Self {
            version: CURRENT_VERSION,
            enabled: false,
            mode: PasscodeMode::Pin4,
            salt_b64: String::new(),
            hash_b64: String::new(),
            iterations: DEFAULT_ITERATIONS,
            lock_on_sleep: true,
            idle_timeout_secs: 0,
            hide_notifications_when_locked: true,
            failed_attempts: 0,
            lockout_until_unix: 0,
        }
    }
}

struct HmacSha256State {
    inner_base: Sha256,
    outer_base: Sha256,
}

impl HmacSha256State {
    fn new(key: &[u8]) -> Self {
        let mut key_block = [0u8; 64];
        if key.len() > 64 {
            let digest = Sha256::digest(key);
            key_block[..32].copy_from_slice(&digest);
        } else {
            key_block[..key.len()].copy_from_slice(key);
        }
        let mut ipad = [0x36u8; 64];
        let mut opad = [0x5cu8; 64];
        for i in 0..64 {
            ipad[i] ^= key_block[i];
            opad[i] ^= key_block[i];
        }
        Self {
            inner_base: Sha256::new_with_prefix(ipad),
            outer_base: Sha256::new_with_prefix(opad),
        }
    }

    fn mac_slices(&self, slices: &[&[u8]]) -> [u8; 32] {
        let mut inner = self.inner_base.clone();
        for s in slices {
            inner.update(s);
        }
        let inner_hash = inner.finalize();
        let mut outer = self.outer_base.clone();
        outer.update(inner_hash);
        outer.finalize().into()
    }
}

/// Derive a 32-byte key using PBKDF2-HMAC-SHA256.
pub fn pbkdf2_hmac_sha256(password: &[u8], salt: &[u8], iterations: u32) -> [u8; 32] {
    if iterations == 0 {
        return [0u8; 32];
    }
    let hmac = HmacSha256State::new(password);
    let block_index = 1u32.to_be_bytes();
    let mut u = hmac.mac_slices(&[salt, &block_index]);
    let mut acc = u;
    for _ in 1..iterations {
        u = hmac.mac_slices(&[&u]);
        for (a, b) in acc.iter_mut().zip(u.iter()) {
            *a ^= *b;
        }
    }
    acc
}

/// Compare two byte slices in constant time without short-circuiting on content.
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (&x, &y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

/// Validate a passcode candidate for the given [`PasscodeMode`].
pub fn validate_passcode(
    mode: PasscodeMode,
    candidate: &str,
) -> std::result::Result<(), &'static str> {
    match mode {
        PasscodeMode::Pin4 => {
            if candidate.chars().count() == 4 && candidate.chars().all(|c| c.is_ascii_digit()) {
                Ok(())
            } else {
                Err("Mã PIN phải gồm đúng 4 chữ số (0-9)")
            }
        }
        PasscodeMode::Custom => {
            let len = candidate.chars().count();
            if (4..=32).contains(&len) {
                Ok(())
            } else {
                Err("Mật khẩu phải có từ 4 đến 32 ký tự")
            }
        }
    }
}

fn generate_salt_and_hash(passcode: &str) -> Result<(String, String)> {
    let mut salt = [0u8; SALT_LEN];
    getrandom::fill(&mut salt).map_err(|e| anyhow::anyhow!("CSPRNG failed: {e}"))?;
    let hash = pbkdf2_hmac_sha256(passcode.as_bytes(), &salt, DEFAULT_ITERATIONS);
    Ok((STANDARD.encode(salt), STANDARD.encode(hash)))
}

impl AppLockConfig {
    /// Construct a new enabled [`AppLockConfig`] with a fresh CSPRNG salt and PBKDF2 digest.
    pub fn create_enabled(mode: PasscodeMode, passcode: &str) -> Result<Self> {
        validate_passcode(mode, passcode).map_err(|msg| anyhow::anyhow!(msg))?;
        let (salt_b64, hash_b64) = generate_salt_and_hash(passcode)?;
        Ok(Self {
            version: CURRENT_VERSION,
            enabled: true,
            mode,
            salt_b64,
            hash_b64,
            iterations: DEFAULT_ITERATIONS,
            ..Self::default()
        })
    }

    /// Remaining brute-force cooldown seconds clamped to `0..=LOCKOUT_SECS`.
    pub fn remaining_lockout_secs(&self, now_unix: i64) -> i64 {
        if self.lockout_until_unix <= 0 {
            return 0;
        }
        self.lockout_until_unix
            .saturating_sub(now_unix)
            .clamp(0, LOCKOUT_SECS)
    }

    /// Verify a candidate passcode, updating persisted lockout counters in place.
    pub fn verify_attempt(&mut self, candidate: &str, now_unix: i64) -> VerifyOutcome {
        let rem = self.remaining_lockout_secs(now_unix);
        if rem > 0 {
            return VerifyOutcome::LockedOut {
                remaining_secs: rem,
            };
        }
        if self.enabled && self.iterations >= MIN_ITERATIONS {
            if let (Ok(salt), Ok(expected)) = (
                STANDARD.decode(&self.salt_b64),
                STANDARD.decode(&self.hash_b64),
            ) {
                if !salt.is_empty() && expected.len() == 32 {
                    let derived = pbkdf2_hmac_sha256(candidate.as_bytes(), &salt, self.iterations);
                    if constant_time_eq(&derived, &expected) {
                        self.failed_attempts = 0;
                        self.lockout_until_unix = 0;
                        return VerifyOutcome::Unlocked;
                    }
                }
            }
        }
        self.failed_attempts = self.failed_attempts.saturating_add(1);
        if self.failed_attempts >= MAX_FAILED_ATTEMPTS {
            self.lockout_until_unix = now_unix.saturating_add(LOCKOUT_SECS);
            return VerifyOutcome::LockedOut {
                remaining_secs: LOCKOUT_SECS,
            };
        }
        self.lockout_until_unix = 0;
        VerifyOutcome::Wrong {
            remaining_attempts: MAX_FAILED_ATTEMPTS - self.failed_attempts,
        }
    }

    /// Verify `current` passcode, validate `new_passcode`, and rotate salt/hash while preserving preferences.
    pub fn change_passcode(
        &mut self,
        current: &str,
        new_mode: PasscodeMode,
        new_passcode: &str,
        now_unix: i64,
    ) -> Result<ChangePasscodeOutcome> {
        let outcome = self.verify_attempt(current, now_unix);
        if outcome != VerifyOutcome::Unlocked {
            return Ok(ChangePasscodeOutcome::VerifyFailed(outcome));
        }
        if let Err(msg) = validate_passcode(new_mode, new_passcode) {
            return Ok(ChangePasscodeOutcome::InvalidNew(msg));
        }
        let (salt_b64, hash_b64) = generate_salt_and_hash(new_passcode)?;
        self.version = CURRENT_VERSION;
        self.enabled = true;
        self.mode = new_mode;
        self.salt_b64 = salt_b64;
        self.hash_b64 = hash_b64;
        self.iterations = DEFAULT_ITERATIONS;
        self.failed_attempts = 0;
        self.lockout_until_unix = 0;
        Ok(ChangePasscodeOutcome::Changed)
    }

    /// Load and strictly validate an [`AppLockConfig`] from `path` (`Ok(None)` when absent).
    pub fn load_from_path(path: &Path) -> Result<Option<Self>> {
        let raw = match std::fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => {
                return Err(e)
                    .with_context(|| format!("Cannot read app lock config {}", path.display()));
            }
        };
        let cfg: Self = serde_json::from_str(&raw)
            .with_context(|| format!("Invalid app lock config JSON {}", path.display()))?;
        if cfg.version != CURRENT_VERSION {
            bail!("Unsupported app lock config version: {}", cfg.version);
        }
        if cfg.iterations < MIN_ITERATIONS {
            bail!(
                "App lock iterations {} below minimum {MIN_ITERATIONS}",
                cfg.iterations
            );
        }
        let salt = STANDARD
            .decode(&cfg.salt_b64)
            .context("Invalid salt_b64 in app lock config")?;
        let hash = STANDARD
            .decode(&cfg.hash_b64)
            .context("Invalid hash_b64 in app lock config")?;
        if cfg.enabled && (salt.is_empty() || hash.len() != 32) {
            bail!("Enabled app lock config is missing valid salt or 32-byte hash");
        }
        Ok(Some(cfg))
    }

    /// Persist this configuration privately and atomically to `path`.
    pub fn save_to_path(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self).context("Serialize app lock config")?;
        crate::data_dir::write_private_atomic(path, json.as_bytes())
    }

    /// Load the default `app_lock.json` file from the active data directory.
    pub fn load_default() -> Result<Option<Self>> {
        Self::load_from_path(Path::new(APP_LOCK_FILE))
    }

    /// Persist this configuration to `app_lock.json` in the active data directory.
    pub fn save_default(&self) -> Result<()> {
        self.save_to_path(Path::new(APP_LOCK_FILE))
    }

    /// Delete `app_lock.json` from the active data directory if present.
    pub fn remove_default() -> Result<()> {
        match std::fs::remove_file(Path::new(APP_LOCK_FILE)) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(e).with_context(|| format!("Cannot remove {APP_LOCK_FILE}")),
        }
    }
}

/// Decide `(should_lock, config_corrupted)` on application startup.
pub fn should_lock_on_startup(
    config_result: &Result<Option<AppLockConfig>>,
    has_saved_credentials: bool,
) -> (bool, bool) {
    match config_result {
        Ok(Some(cfg)) => (has_saved_credentials && cfg.enabled, false),
        Ok(None) => (false, false),
        Err(_) => (has_saved_credentials, true),
    }
}

/// Detect forward wall-clock jumps >= [`SLEEP_JUMP_THRESHOLD_SECS`] when sleep lock is enabled.
pub fn classify_sleep_jump(
    prev_wall_unix: i64,
    now_wall_unix: i64,
    lock_on_sleep: bool,
) -> Option<i64> {
    if !lock_on_sleep {
        return None;
    }
    let delta = now_wall_unix.saturating_sub(prev_wall_unix);
    (delta >= SLEEP_JUMP_THRESHOLD_SECS).then_some(delta)
}

/// Determine whether the UI idle timer has expired, suppressing expiry during active calls.
pub fn is_idle_expired(
    last_activity_ui_time: f64,
    now_ui_time: f64,
    idle_timeout_secs: u64,
    in_active_call: bool,
) -> bool {
    if idle_timeout_secs == 0 || in_active_call {
        return false;
    }
    if !last_activity_ui_time.is_finite()
        || !now_ui_time.is_finite()
        || now_ui_time < last_activity_ui_time
    {
        return false;
    }
    (now_ui_time - last_activity_ui_time) >= idle_timeout_secs as f64
}

/// Mask notification title and body when locked and content hiding is active.
pub fn mask_notification_content(
    app_locked: bool,
    hide_content: bool,
    title: &str,
    body: &str,
    unread_count: usize,
) -> (String, String) {
    if app_locked && hide_content {
        let masked_body = match unread_count {
            0 => "Bạn có tin nhắn mới".to_string(),
            1 => "Bạn có 1 tin nhắn mới".to_string(),
            count => format!("Bạn có {count} tin nhắn mới"),
        };
        ("ctdozlite".to_string(), masked_body)
    } else {
        (title.to_string(), body.to_string())
    }
}

/// Mask transient toast text when locked and content hiding is active.
pub fn mask_toast_text(
    app_locked: bool,
    hide_content: bool,
    fallback: &str,
    text: String,
) -> String {
    if app_locked && hide_content {
        fallback.to_string()
    } else {
        text
    }
}

/// Return whether notification popups permit interactive actions (`!app_locked`).
pub fn popup_allows_interaction(app_locked: bool) -> bool {
    !app_locked
}
