use crate::table::Tabular;
use clap::Args;
use std::path::PathBuf;

use super::Execute;
use crate::file::FileType;
use anyhow::{anyhow, Context};

#[derive(Args)]
pub struct Convert {
    /// Input file that should be converted
    #[arg(long)]
    file: PathBuf,

    /// Name of the Spreadsheet to be parsed. Only required, if relevant for the file type, e.g., '.xlsx'.
    #[arg(long)]
    sheet_name: Option<String>,
}

impl Execute for Convert {
    fn execute(&self) -> anyhow::Result<()> {
        eprintln!("Processing: {:?}", &self.file);

        let file_type = FileType::try_from(self.file.clone())?;
        eprintln!("Format: {}", file_type);

        self.validate_args(&file_type)
            .context("argument validation failed")?;

        file_type.into_table()?;

        Ok(())
    }
}

impl Convert {
    fn validate_args(&self, file_type: &FileType) -> anyhow::Result<()> {
        match file_type {
            FileType::Excel(_) => self
                .sheet_name
                .as_ref()
                .map(|_name| ())
                .ok_or_else(|| anyhow!("File type requires a sheet name: {}", file_type)),
            FileType::Csv(_) => Ok(()),
        }
    }
}
