use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "git-timer")]
#[command(about = "A coding time tracker", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Starts timer
    #[command()]
    Start {
        /// Timer's name
        timer_name: String,
    },
    /// TODO
    #[command()]
    Status,
    /// Commit and add timer data
    #[command(arg_required_else_help = true)]
    Commit {
        /// Commit message
        message: String,
    }
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Start { timer_name } => {
            println!("Started timer {timer_name}");
        },
        Commands::Status => {
            println!("Status info");
        },
        Commands::Commit { message } => {
            println!("Ran command: git commit -m \"{message}\"");
        },
    }
}