use clap::Args;

use super::Execute;

#[derive(Args)]
pub struct Config {}

impl Execute for Config {
    fn execute(&self) -> anyhow::Result<()> {
        todo!()
    }
}
