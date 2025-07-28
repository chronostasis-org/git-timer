use crate::storage;
use crate::timer::TimerData;
use anyhow::Result;
use chrono::Local;
use std::path::Path;

pub fn start_timer(timer_name: &str, timer_data: &mut TimerData, data_path: &Path) -> Result<()> {
    log::info!("Starting timer '{}'", timer_name);

    // Check if a timer is already running
    if timer_data.is_running() {
        // TODO: output warning and exit, do not overwrite running timer
        log::warn!("Overwriting existing running timer");
    }

    let now = Local::now();
    timer_data.name = Some(timer_name.to_owned());
    timer_data.start = Some(now);
    timer_data.end = None;

    timer_data.save(data_path)?;
    // TODO: better output - coloring, etc
    println!(
        "Started timer '{}' at {}",
        timer_name,
        now.format("%H:%M:%S")
    );
    Ok(())
}

pub fn show_status(timer_data: &TimerData) -> Result<()> {
    if let Some(name) = &timer_data.name {
        if timer_data.is_running() {
            if let Some((mins, secs)) = timer_data.calculate_duration() {
                println!(
                    "Timer '{}' has been running for {} minutes and {} seconds",
                    name, mins, secs
                );
            }
        } else if timer_data.end.is_some() {
            if let Some((mins, secs)) = timer_data.calculate_duration() {
                println!(
                    "Timer '{}' has completed after {} minutes and {} seconds",
                    name, mins, secs
                );
            }
        }
    } else {
        println!("No timer running.");
    }
    Ok(())
}

pub fn commit_with_timer(
    message: &str,
    timer_data: &mut TimerData,
    data_path: &Path,
) -> Result<()> {
    log::info!("Committing with timer data");

    // Check if a timer is actually running
    if !timer_data.is_running() {
        log::warn!("No timer was running, creating implicit timer");
    }

    let now = Local::now();
    timer_data.end = Some(now);

    // Execute git command (this would be implemented to actually run git)
    log::debug!("Running git command with message: {}", message);
    println!("Running: git commit -m \"{}\"", message);

    // Show timer results
    if let Some((mins, secs)) = timer_data.calculate_duration() {
        if let Some(name) = &timer_data.name {
            println!(
                "Timer '{}' stopped after {} minutes and {} seconds",
                name, mins, secs
            );
        }
    }

    // Save final state
    timer_data.save(data_path)?;

    // Clean up by removing the file
    storage::remove_timer_file(data_path)?;

    Ok(())
}
