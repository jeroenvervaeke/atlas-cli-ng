use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Auth {
    /// Configure a profile to store access settings for your MongoDB deployment.
    Login(Login),
}

#[derive(Debug, Args)]
pub struct Login {
    /// Don't try to open a browser session.
    #[arg(long, default_value_t = false)]
    no_browser: bool,
}
