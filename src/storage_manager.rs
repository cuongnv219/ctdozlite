//! Zalo-style 3-tier local data storage manager:
//! 1. Media (`<storage_root>/media/<uin>/{photos,videos,files,voices}`)
//! 2. Cache (`<storage_root>/cache/<uin>` + shared cache)
//! 3. System (`accounts/*_history.db`, session & settings JSONs)

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, RwLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaCategory {
    Photos,
    Videos,
    Files,
    Voices,
}

impl MediaCategory {
    pub fn folder_name(self) -> &'static str {
        match self {
            Self::Photos => "photos",
            Self::Videos => "videos",
            Self::Files => "files",
            Self::Voices => "voices",
        }
    }

    pub fn label_vi(self) -> &'static str {
        match self {
            Self::Photos => "Ảnh",
            Self::Videos => "Video",
            Self::Files => "File đính kèm",
            Self::Voices => "Tin nhắn thoại",
        }
    }

    pub fn from_filename(name: &str) -> Self {
        let ext = name
            .rsplit_once('.')
            .map(|(_, e)| e.trim().to_ascii_lowercase())
            .unwrap_or_default();
        match ext.as_str() {
            "jpg" | "jpeg" | "png" | "webp" | "gif" | "bmp" | "heic" | "jxl" | "avif" | "ico" => {
                Self::Photos
            }
            "mp4" | "mov" | "webm" | "mkv" | "m4v" | "avi" => Self::Videos,
            "aac" | "amr" | "m4a" | "mp3" | "ogg" | "opus" | "wav" | "flac" => Self::Voices,
            _ => Self::Files,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct StorageSnapshot {
    pub storage_root: PathBuf,
    pub drive_label: String,
    pub disk_total_bytes: u64,
    pub disk_free_bytes: u64,
    pub media_photos_bytes: u64,
    pub media_videos_bytes: u64,
    pub media_files_bytes: u64,
    pub media_voices_bytes: u64,
    pub media_other_bytes: u64,
    pub media_total_bytes: u64,
    pub cache_bytes: u64,
    pub system_bytes: u64,
    pub zalo_total_bytes: u64,
    pub updated_at_label: String,
}

static CUSTOM_STORAGE_ROOT: RwLock<Option<PathBuf>> = RwLock::new(None);
static SCAN_IN_PROGRESS: AtomicBool = AtomicBool::new(false);
static LATEST_SNAPSHOT: Mutex<Option<StorageSnapshot>> = Mutex::new(None);

/// Configure the custom storage root from `AppSettings`. Empty string = current data directory.
pub fn set_storage_root(custom: &str) {
    let trimmed = custom.trim();
    let next = if trimmed.is_empty() {
        None
    } else {
        Some(PathBuf::from(trimmed))
    };
    if let Ok(mut guard) = CUSTOM_STORAGE_ROOT.write() {
        *guard = next;
    }
}

/// Active storage root where `media/` and `cache/` live.
pub fn storage_root() -> PathBuf {
    if let Ok(guard) = CUSTOM_STORAGE_ROOT.read() {
        if let Some(ref custom) = *guard {
            return custom.clone();
        }
    }
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

/// Root folder holding all per-account media (`<storage_root>/media`).
pub fn media_root() -> PathBuf {
    storage_root().join("media")
}

/// Per-account media folder (`<storage_root>/media/<uin>` or `<storage_root>/media/shared`).
pub fn media_account_dir(uin: &str) -> PathBuf {
    let bucket = if crate::account_manager::is_safe_uin(uin) {
        uin
    } else {
        "shared"
    };
    media_root().join(bucket)
}

/// Ensure all 4 media category subdirectories exist for `uin` and return the requested category folder.
pub fn ensure_media_category_dir(uin: &str, category: MediaCategory) -> PathBuf {
    let base = media_account_dir(uin);
    for cat in [
        MediaCategory::Photos,
        MediaCategory::Videos,
        MediaCategory::Files,
        MediaCategory::Voices,
    ] {
        let _ = std::fs::create_dir_all(base.join(cat.folder_name()));
    }
    base.join(category.folder_name())
}

/// Ensure all 4 media category subdirectories exist for `uin` and return `<storage_root>/media/<uin>`.
pub fn ensure_media_account_dir(uin: &str) -> PathBuf {
    let _ = ensure_media_category_dir(uin, MediaCategory::Photos);
    media_account_dir(uin)
}

/// Root folder holding temporary caches (`<storage_root>/cache`).
pub fn cache_root() -> PathBuf {
    storage_root().join("cache")
}

/// Per-account cache directory (`<storage_root>/cache/<uin>` or `<storage_root>/cache/shared`).
pub fn cache_dir_for(uin: &str) -> PathBuf {
    let bucket = if crate::account_manager::is_safe_uin(uin) {
        uin
    } else {
        "shared"
    };
    cache_root().join(bucket)
}

/// Move `media/` and `cache/` from the previous storage root into `new_root`, then activate `new_root`.
pub fn relocate_storage_root(new_root: &Path) -> Result<(), String> {
    std::fs::create_dir_all(new_root)
        .map_err(|e| format!("Không thể tạo thư mục lưu trữ mới: {e}"))?;
    let probe = new_root.join(".zalo_storage_probe");
    std::fs::write(&probe, b"ok")
        .map_err(|e| format!("Thư mục mới không có quyền ghi: {e}"))?;
    let _ = std::fs::remove_file(&probe);

    let old_root = storage_root();
    let old_canon = old_root.canonicalize().unwrap_or_else(|_| old_root.clone());
    let new_canon = new_root.canonicalize().unwrap_or_else(|_| new_root.to_path_buf());

    if old_canon != new_canon {
        for sub in ["media", "cache"] {
            let src = old_root.join(sub);
            let dst = new_root.join(sub);
            if src.is_dir() {
                let _ = copy_dir_recursive(&src, &dst);
                let _ = std::fs::remove_dir_all(&src);
            } else {
                let _ = std::fs::create_dir_all(&dst);
            }
        }
    } else {
        let _ = std::fs::create_dir_all(new_root.join("media"));
        let _ = std::fs::create_dir_all(new_root.join("cache"));
    }

    set_storage_root(&new_root.display().to_string());
    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dest_path = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_recursive(&entry.path(), &dest_path)?;
        } else if ty.is_file() && !dest_path.exists() {
            let _ = std::fs::copy(entry.path(), dest_path);
        }
    }
    Ok(())
}

/// Delete all temporary cache files (`cache/`, legacy `stickers_cache/`, and `accounts/*/media_cache/`).
/// Returns total bytes freed.
pub fn clear_all_caches() -> u64 {
    let mut freed = 0u64;
    let c_root = cache_root();
    freed += clear_dir_contents(&c_root);
    let _ = std::fs::create_dir_all(&c_root);

    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let legacy_shared = cwd.join("stickers_cache");
    if legacy_shared.is_dir() {
        freed += clear_dir_contents(&legacy_shared);
        let _ = std::fs::remove_dir_all(&legacy_shared);
    }
    let accounts = cwd.join("accounts");
    if let Ok(entries) = std::fs::read_dir(&accounts) {
        for entry in entries.flatten() {
            let legacy_mc = entry.path().join("media_cache");
            if legacy_mc.is_dir() {
                freed += clear_dir_contents(&legacy_mc);
                let _ = std::fs::remove_dir_all(&legacy_mc);
            }
        }
    }
    freed
}

fn clear_dir_contents(dir: &Path) -> u64 {
    let mut freed = 0u64;
    let Ok(entries) = std::fs::read_dir(dir) else {
        return 0;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if let Ok(meta) = path.symlink_metadata() {
            if meta.is_dir() {
                freed += clear_dir_contents(&path);
                let _ = std::fs::remove_dir(&path);
            } else if meta.is_file() {
                let len = meta.len();
                if std::fs::remove_file(&path).is_ok() {
                    freed += len;
                }
            }
        }
    }
    freed
}

pub fn is_scanning() -> bool {
    SCAN_IN_PROGRESS.load(Ordering::Acquire)
}

pub fn latest_snapshot() -> Option<StorageSnapshot> {
    LATEST_SNAPSHOT.lock().ok().and_then(|g| g.clone())
}

/// Trigger a background disk scan if none is currently running.
pub fn trigger_scan() {
    if SCAN_IN_PROGRESS
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .is_err()
    {
        return;
    }
    std::thread::spawn(|| {
        let snap = compute_snapshot();
        if let Ok(mut guard) = LATEST_SNAPSHOT.lock() {
            *guard = Some(snap);
        }
        SCAN_IN_PROGRESS.store(false, Ordering::Release);
    });
}

fn compute_snapshot() -> StorageSnapshot {
    let root = storage_root();
    let _ = std::fs::create_dir_all(root.join("media"));
    let _ = std::fs::create_dir_all(root.join("cache"));

    let active = crate::account_manager::active_uin();
    if !active.is_empty() {
        let _ = ensure_media_account_dir(&active);
    }

    let drive_label = extract_drive_label(&root);
    let (disk_total_bytes, disk_free_bytes) = disk_space_for_path(&root).unwrap_or((0, 0));

    // 1. Media breakdown across all accounts under `<storage_root>/media`
    let mut media_photos_bytes = 0u64;
    let mut media_videos_bytes = 0u64;
    let mut media_files_bytes = 0u64;
    let mut media_voices_bytes = 0u64;
    let mut media_other_bytes = 0u64;

    let m_root = root.join("media");
    if let Ok(acc_entries) = std::fs::read_dir(&m_root) {
        for acc in acc_entries.flatten() {
            let acc_path = acc.path();
            if acc_path.is_dir() {
                if let Ok(sub_entries) = std::fs::read_dir(&acc_path) {
                    for sub in sub_entries.flatten() {
                        let sp = sub.path();
                        let name = sub.file_name().to_string_lossy().to_ascii_lowercase();
                        if sp.is_dir() {
                            let sz = dir_size(&sp);
                            match name.as_str() {
                                "photos" => media_photos_bytes += sz,
                                "videos" => media_videos_bytes += sz,
                                "files" => media_files_bytes += sz,
                                "voices" => media_voices_bytes += sz,
                                _ => media_other_bytes += sz,
                            }
                        } else if let Ok(meta) = sp.symlink_metadata() {
                            if meta.is_file() {
                                media_other_bytes += meta.len();
                            }
                        }
                    }
                }
            } else if let Ok(meta) = acc_path.symlink_metadata() {
                if meta.is_file() {
                    media_other_bytes += meta.len();
                }
            }
        }
    }

    let media_total_bytes = media_photos_bytes
        + media_videos_bytes
        + media_files_bytes
        + media_voices_bytes
        + media_other_bytes;

    let mut cache_bytes = dir_size(&root.join("cache"));
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    cache_bytes += dir_size(&cwd.join("stickers_cache"));
    if let Ok(entries) = std::fs::read_dir(cwd.join("accounts")) {
        for entry in entries.flatten() {
            let legacy_mc = entry.path().join("media_cache");
            if legacy_mc.is_dir() {
                cache_bytes += dir_size(&legacy_mc);
            }
        }
    }

    // 3. System data size (SQLite DBs, WAL/SHM, sessions, configs, sync keys)
    let mut system_bytes = 0u64;
    if let Ok(entries) = std::fs::read_dir(&cwd) {
        for entry in entries.flatten() {
            let p = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if let Ok(meta) = p.symlink_metadata() {
                if meta.is_file()
                    && (name.ends_with(".db")
                        || name.ends_with(".db-wal")
                        || name.ends_with(".db-shm")
                        || name.ends_with(".sqlite")
                        || name.ends_with(".json"))
                {
                    system_bytes += meta.len();
                }
            }
        }
    }
    if let Ok(entries) = std::fs::read_dir(cwd.join("accounts")) {
        for entry in entries.flatten() {
            let p = entry.path();
            if let Ok(meta) = p.symlink_metadata() {
                if meta.is_file() {
                    system_bytes += meta.len();
                } else if meta.is_dir() {
                    // Count account dir excluding legacy `media_cache` (already counted in cache)
                    if let Ok(sub) = std::fs::read_dir(&p) {
                        for item in sub.flatten() {
                            if item.file_name() == "media_cache" {
                                continue;
                            }
                            let ip = item.path();
                            if ip.is_dir() {
                                system_bytes += dir_size(&ip);
                            } else if let Ok(im) = ip.symlink_metadata() {
                                if im.is_file() {
                                    system_bytes += im.len();
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let zalo_total_bytes = media_total_bytes + cache_bytes + system_bytes;
    let updated_at_label = chrono::Local::now().format("%H:%M - %d/%m/%Y").to_string();

    StorageSnapshot {
        storage_root: root,
        drive_label,
        disk_total_bytes,
        disk_free_bytes,
        media_photos_bytes,
        media_videos_bytes,
        media_files_bytes,
        media_voices_bytes,
        media_other_bytes,
        media_total_bytes,
        cache_bytes,
        system_bytes,
        zalo_total_bytes,
        updated_at_label,
    }
}

fn dir_size(dir: &Path) -> u64 {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return 0;
    };
    let mut total = 0u64;
    for entry in entries.flatten() {
        let p = entry.path();
        if let Ok(meta) = p.symlink_metadata() {
            if meta.is_dir() {
                total += dir_size(&p);
            } else if meta.is_file() {
                total += meta.len();
            }
        }
    }
    total
}

fn extract_drive_label(path: &Path) -> String {
    let resolved = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let s = resolved.to_string_lossy();
    let clean = s.strip_prefix(r"\\?\").unwrap_or(&s);
    if clean.len() >= 2 && clean.as_bytes()[1] == b':' {
        return format!("({}:)", (clean.as_bytes()[0] as char).to_ascii_uppercase());
    }
    format!("({})", path.display())
}

fn disk_space_for_path(path: &Path) -> Option<(u64, u64)> {
    let target = if path.exists() {
        path.to_path_buf()
    } else {
        path.parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."))
    };
    #[cfg(windows)]
    {
        #[link(name = "kernel32")]
        extern "system" {
            fn GetDiskFreeSpaceExW(
                lp_directory_name: *const u16,
                lp_free_bytes_available_to_caller: *mut u64,
                lp_total_number_of_bytes: *mut u64,
                lp_total_number_of_free_bytes: *mut u64,
            ) -> i32;
        }
        use std::os::windows::ffi::OsStrExt;
        let mut wide: Vec<u16> = target.as_os_str().encode_wide().collect();
        wide.push(0);
        let mut free_avail = 0u64;
        let mut total = 0u64;
        let mut total_free = 0u64;
        let ok = unsafe {
            GetDiskFreeSpaceExW(
                wide.as_ptr(),
                &mut free_avail,
                &mut total,
                &mut total_free,
            )
        };
        if ok != 0 && total > 0 {
            return Some((total, free_avail));
        }
        None
    }
    #[cfg(unix)]
    {
        use std::ffi::CString;
        use std::os::unix::ffi::OsStrExt;
        let c_path = CString::new(target.as_os_str().as_bytes()).ok()?;
        let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };
        if unsafe { libc::statvfs(c_path.as_ptr(), &mut stat) } == 0 {
            let frsize = if stat.f_frsize > 0 {
                stat.f_frsize as u64
            } else {
                stat.f_bsize as u64
            };
            let total = (stat.f_blocks as u64).saturating_mul(frsize);
            let free = (stat.f_bavail as u64).saturating_mul(frsize);
            if total > 0 {
                return Some((total, free));
            }
        }
        None
    }
    #[cfg(not(any(windows, unix)))]
    {
        let _ = target;
        None
    }
}

pub fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    const GB: f64 = 1024.0 * 1024.0 * 1024.0;
    let b = bytes as f64;
    if bytes >= 1024 * 1024 * 1024 {
        format!("{:.1} GB", b / GB)
    } else if bytes >= 10 * 1024 * 1024 {
        format!("{:.0} MB", b / MB)
    } else if bytes >= 1024 * 1024 {
        format!("{:.1} MB", b / MB)
    } else if bytes >= 1024 {
        format!("{:.1} KB", b / KB)
    } else {
        format!("{bytes} B")
    }
}

pub fn open_in_file_manager(path: &Path) {
    let _ = std::fs::create_dir_all(path);
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("explorer").arg(path).spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(path).spawn();
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        let _ = std::process::Command::new("xdg-open").arg(path).spawn();
    }
}
