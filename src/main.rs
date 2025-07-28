mod cli;
mod commands;
mod logging;
mod storage;
mod timer;

use anyhow::Result;

fn main() -> Result<()> {
    // Initialize logging
    logging::init_logger();
    log::info!("git-timer starting");

    // Parse CLI arguments
    let args = cli::parse_args();

    // Get data path
    let data_path = storage::get_timer_path();

    // Load timer data
    let mut timer_data = timer::TimerData::load(&data_path)?;

    // Process command
    match args.command {
        cli::Command::Start { timer_name } => {
            commands::start_timer(&timer_name, &mut timer_data, &data_path)?;
        }
        cli::Command::Status => {
            commands::show_status(&timer_data)?;
        }
        cli::Command::Commit { message } => {
            commands::commit_with_timer(&message, &mut timer_data, &data_path)?;
        }
    }

    Ok(())
}
