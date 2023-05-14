use clap::Subcommand;
use enum_dispatch::enum_dispatch;
use crate::cli::cmd::config::Config;

use self::convert::Convert;

mod convert;
mod config;

#[enum_dispatch(Execute)]
#[derive(Subcommand)]
pub enum Command {
    /// Configuration options
    Config(Config),
    Convert(Convert)
}

#[enum_dispatch]
pub trait Execute {
    fn execute(&self) -> anyhow::Result<()>;
}