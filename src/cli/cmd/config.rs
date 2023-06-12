use clap::Args;

use super::Execute;

#[derive(Args)]
pub struct ConfigArgs {}

impl Execute for ConfigArgs {
    fn execute(&self) -> anyhow::Result<()> {
        todo!()
    }
}
