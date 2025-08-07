use clap::Subcommand;

pub mod version;

/// CLI subcommands for the AEI framework.
#[derive(Subcommand)]
pub enum Command {
    /// Display the AEI framework version.
    Version,
}

/// Execute the provided CLI command.
pub fn execute(command: Command) {
    match command {
        Command::Version => version::run(),
    }
}
