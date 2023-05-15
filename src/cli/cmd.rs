use crate::cli::cmd::config::Config;
use clap::Subcommand;
use enum_dispatch::enum_dispatch;

use self::convert::Convert;

mod config;
mod convert;

#[enum_dispatch(Execute)]
#[derive(Subcommand)]
pub enum Command {
    /// Configuration options
    Config(Config),
    Convert(Convert),
}

#[enum_dispatch]
pub trait Execute {
    fn execute(&self) -> anyhow::Result<()>;
}
