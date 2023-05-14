use std::process::ExitCode;

use clap::Parser;
use sconv::cli::{Cli, Execute};

fn main() -> ExitCode {
    let cli = Cli::parse();
    
    if let Err(e) = cli.command.execute() {
        eprintln!("{e}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
