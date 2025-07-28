use anyhow::Result;
use std::env;
use std::path::{Path, PathBuf};

pub fn get_timer_path() -> PathBuf {
    match env::var("XDG_RUNTIME_DIR") {
        Ok(runtime_dir) if !runtime_dir.is_empty() => {
            let path = PathBuf::from(runtime_dir).join("git-timer.json");
            log::debug!("Using XDG_RUNTIME_DIR for timer data: {:?}", path);
            path
        }
        Ok(_) => {
            log::warn!("XDG_RUNTIME_DIR is set, but empty, using local file");
            let path = PathBuf::from("./.git-timer.json");
            log::debug!("Fallback path: {:?}", path);
            path
        }
        Err(e) => {
            log::warn!("XDG_RUNTIME_DIR not available ({}), using local file", e);
            let path = PathBuf::from("./.git-timer.json");
            log::debug!("Fallback path: {:?}", path);
            path
        }
    }
}

pub fn remove_timer_file(path: &Path) -> Result<()> {
    if path.exists() {
        log::debug!("Removing timer file: {:?}", path);
        std::fs::remove_file(path)?;
        log::debug!("Timer file removed successfully");
    }
    Ok(())
}
