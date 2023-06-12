use crate::cli::cmd::config::ConfigArgs;
use clap::Subcommand;
use enum_dispatch::enum_dispatch;

use self::convert::ConvertArgs;

mod config;
mod convert;

#[enum_dispatch(Execute)]
#[derive(Subcommand)]
pub enum Command {
    /// Configuration options
    Config(ConfigArgs),
    Convert(ConvertArgs),
}

#[enum_dispatch]
pub trait Execute {
    fn execute(&self) -> anyhow::Result<()>;
}
