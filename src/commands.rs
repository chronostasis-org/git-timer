// src/commands.rs
use crate::storage;
use crate::timer::TimerData;
use anyhow::{Context, Result};
use chrono::Local;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;

pub fn start_timer(timer_data: &mut TimerData, data_path: &Path) -> Result<()> {
    log::info!("Starting git-timer");

    // Exit if timer is already running
    if timer_data.is_running() {
        let msg = format!(
            "Timer is already running. Use 'git-timer status' to check or 'git-timer commit' to end it.",
        );
        log::warn!("{}", msg);
        eprintln!("Error: {}", msg);
        std::process::exit(1); // Exit with error code
    }

    let now = Local::now();
    timer_data.start = Some(now);
    timer_data.end = None;

    timer_data.save(data_path)?;
    println!("git-timer: Started timer at {}", now.format("%H:%M:%S"));
    Ok(())
}

pub fn show_status(timer_data: &TimerData) -> Result<()> {
    if timer_data.start.is_some() {
        if timer_data.is_running() {
            if let Some((mins, secs)) = timer_data.calculate_duration() {
                println!(
                    "Timer has been running for {} minutes and {} seconds",
                    mins, secs
                );
            }
        } else if timer_data.end.is_some() {
            if let Some((mins, secs)) = timer_data.calculate_duration() {
                println!(
                    "Timer has completed after {} minutes and {} seconds",
                    mins, secs
                );
            }
        }
    } else {
        println!("No timer running.");
    }
    Ok(())
}

pub fn commit_with_timer(
    message: Option<&str>,
    timer_data: &mut TimerData,
    data_path: &Path,
) -> Result<()> {
    log::info!("Committing with timer data");

    // If timer is not running but has end time,
    // it means a previous commit attempt failed but timer was stopped
    if !timer_data.is_running() {
        if timer_data.start.is_some() && timer_data.end.is_some() {
            log::info!("Found stopped timer data from previous commit attempt");
            println!("Reusing timer data from previous commit attempt");
        } else {
            // No usable timer data at all
            log::warn!("No timer is currently running");
            eprintln!("No timer is currently running");
            std::process::exit(1);
        }
    } else {
        // Timer is running, stop it now
        let now = Local::now();
        timer_data.end = Some(now);

        // Save timer state before committing
        timer_data.save(data_path)?;
    }

    // Calculate timer duration - will work for both running and previously stopped timers
    let timer_footer = if let Some((mins, secs)) = timer_data.calculate_duration() {
        format!("\n\n[Timer: {} minutes {} seconds]", mins, secs)
    } else {
        "".to_string()
    };

    // Execute git commit command based on whether -m flag was used
    match message {
        Some(msg) => {
            // -m flag was used, append timer data to message
            let full_message = format!("{}{}", msg, timer_footer);
            let output = Command::new("git")
                .args(["commit", "-m", &full_message])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .with_context(|| "Failed to execute git commit command")?;

            // Display git output
            print!("{}", String::from_utf8_lossy(&output.stdout));
            eprint!("{}", String::from_utf8_lossy(&output.stderr));

            if !output.status.success() {
                return Err(anyhow::anyhow!("Git commit failed. Timer data preserved."));
            }
        }
        None => {
            // No -m flag, open editor with template containing timer footer
            let mut temp_file = NamedTempFile::new()
                .with_context(|| "Failed to create temporary commit template file")?;

            // Write timer footer to template
            writeln!(
                temp_file,
                "

# Timer: {} minutes {} seconds
#",
                timer_footer
                    .trim_start_matches("\n\n[Timer: ")
                    .trim_end_matches("]")
                    .split_once(' ')
                    .unwrap_or(("0", "0 seconds"))
                    .0,
                timer_footer
                    .trim_start_matches("\n\n[Timer: ")
                    .trim_end_matches("]")
                    .split_once(' ')
                    .unwrap_or(("0", "0 seconds"))
                    .1
            )?;

            // Set GIT_COMMIT_TEMPLATE environment variable
            let template_path = temp_file.path().to_string_lossy().to_string();

            // Call git commit with the template
            let status = Command::new("git")
                .args(["commit", "--template", &template_path])
                .status()
                .with_context(|| "Failed to execute git commit command")?;

            if !status.success() {
                return Err(anyhow::anyhow!("Git commit failed. Timer data preserved."));
            }
        }
    }

    // Clean up by removing the file only after successful commit
    storage::remove_timer_file(data_path)?;

    Ok(())
}
