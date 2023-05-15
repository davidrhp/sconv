use crate::cli::cmd::Command;
use clap::Parser;

mod cmd;

pub use cmd::Execute;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}
