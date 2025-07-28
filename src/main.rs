// src/main.rs
mod cli;
mod commands;
mod logging;
mod storage;
mod timer;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    // Initialize logging
    logging::init_logger();
    log::info!("git-timer starting");

    // Check for git repository
    if !storage::is_in_git_repo() {
        eprintln!("Error: Not in a git repository. Please run git-timer inside a git repository.");
        std::process::exit(1);
    }

    // Parse CLI arguments
    let args = cli::parse_args();

    // Get data path with repo-specific unique identifier
    let data_path =
        storage::get_timer_path().with_context(|| "Failed to determine timer data path")?;

    // Ensure parent directory exists
    if let Some(parent) = data_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory {:?}", parent))?;
        }
    }

    // Load timer data
    let mut timer_data = timer::TimerData::load(&data_path)?;

    // Process command
    match args.command {
        cli::Command::Start => {
            commands::start_timer(&mut timer_data, &data_path)?;
        }
        cli::Command::Status => {
            commands::show_status(&timer_data)?;
        }
        cli::Command::Commit { message } => {
            commands::commit_with_timer(message.as_deref(), &mut timer_data, &data_path)?;
        }
    }

    Ok(())
}
