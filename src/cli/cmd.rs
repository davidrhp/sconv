use clap::Subcommand;
use crate::cli::cmd::config::Config;

mod config;

#[derive(Subcommand)]
pub enum Commands {
    /// Configuration options
    Config(Config),
}
