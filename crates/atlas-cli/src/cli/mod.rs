use clap::{command, Command, Subcommand};

pub mod auth;

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Manage the CLI's authentication state.
    #[command(subcommand)]
    Auth(auth::Auth),
}

pub fn new() -> Command {
    let root_command = command!();
    let root_command = Commands::augment_subcommands(root_command);

    root_command
}
