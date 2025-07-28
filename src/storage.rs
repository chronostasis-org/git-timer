// src/storage.rs
use anyhow::{Context, Result};
use std::collections::hash_map::DefaultHasher;
use std::env;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

pub fn is_in_git_repo() -> bool {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim() == "true"
        }
        _ => false,
    }
}

pub fn get_repo_path() -> Result<PathBuf> {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .with_context(|| "Failed to execute git command")?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Failed to get git repository path"));
    }

    let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(PathBuf::from(path_str))
}

pub fn get_repo_unique_id() -> Result<String> {
    // Get absolute path of the git repo
    let repo_path = get_repo_path()?;

    // Hash the path to create a unique ID
    let mut hasher = DefaultHasher::new();
    repo_path.to_string_lossy().hash(&mut hasher);
    let hash = hasher.finish();

    // Return a shortened hash as hex
    Ok(format!("{:x}", hash))
}

pub fn get_timer_path() -> Result<PathBuf> {
    // Get unique ID for current repo
    let repo_id =
        get_repo_unique_id().with_context(|| "Failed to generate repository unique ID")?;

    // Construct filename with repo ID
    let timer_filename = format!("repo-{}.json", repo_id);

    // Use XDG_RUNTIME_DIR if available
    match env::var("XDG_RUNTIME_DIR") {
        Ok(runtime_dir) if !runtime_dir.is_empty() => {
            let timer_dir = PathBuf::from(runtime_dir).join("git-timer");
            let path = timer_dir.join(timer_filename);
            log::debug!("Using XDG_RUNTIME_DIR for timer data: {:?}", path);
            Ok(path)
        }
        Ok(_) => {
            log::warn!("XDG_RUNTIME_DIR is set, but empty, using local file");
            // Use a hidden directory in repo root
            let repo_path = get_repo_path()?;
            let path = repo_path.join(".git-timer").join(timer_filename);
            log::debug!("Fallback path: {:?}", path);
            Ok(path)
        }
        Err(e) => {
            log::warn!("XDG_RUNTIME_DIR not available ({}), using local file", e);
            // Use a hidden directory in repo root
            let repo_path = get_repo_path()?;
            let path = repo_path.join(".git-timer").join(timer_filename);
            log::debug!("Fallback path: {:?}", path);
            Ok(path)
        }
    }
}

pub fn remove_timer_file(path: &Path) -> Result<()> {
    if path.exists() {
        log::debug!("Removing timer file: {:?}", path);
        std::fs::remove_file(path)
            .with_context(|| format!("Failed to remove timer file at {:?}", path))?;
        log::debug!("Timer file removed successfully");
    }
    Ok(())
}
