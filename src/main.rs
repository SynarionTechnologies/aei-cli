mod commands;

use clap::Parser;

/// Command-line interface for the Autonomous & Evolutive Intelligence Framework.
#[derive(Parser)]
#[command(
    name = "aei-cli",
    version,
    about = "Command-line interface for the Autonomous & Evolutive Intelligence Framework (AEIF)"
)]
struct Cli {
    /// Optional subcommand to execute.
    #[command(subcommand)]
    command: Option<commands::Command>,
}

/// Entry point for the AEI CLI.
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(cmd) => commands::execute(cmd),
        None => println!("AEIF CLI initialized"),
    }
}
