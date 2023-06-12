use crate::file::ExcelTable;
use clap::Args;
use std::path::PathBuf;

use super::Execute;
use crate::file::{Csv, FileType};
use crate::settings;
use crate::table::Table;
use anyhow::{anyhow, Context};

#[derive(Args)]
pub struct ConvertArgs {
    /// Input file that should be converted
    #[arg(long = "file")]
    file_path: PathBuf,

    #[arg(long = "config")]
    config_path: PathBuf,

    /// Name of the Spreadsheet to be parsed. Only required, if relevant for the file type, e.g., '.xlsx'.
    #[arg(long)]
    sheet_name: Option<String>,
}

impl Execute for ConvertArgs {
    fn execute(&self) -> anyhow::Result<()> {
        eprintln!("Processing: {:?}", &self.file_path);

        let file_type = FileType::try_from(self.file_path.clone())?;
        eprintln!("Format: {:?}", file_type);

        self.validate_args(&file_type)
            .context("argument validation failed")?;

        let table: Table = match file_type {
            FileType::Excel => {
                let sheet = self.sheet_name.clone().expect("sheet name to be present");

                ExcelTable::new(self.file_path.clone(), sheet).try_into()?
            }
            FileType::Csv => Csv::new(self.file_path.clone()).try_into()?,
        };

        eprintln!("Table: {:?}", table);

        let cfg = settings::Settings::new(&self.config_path);

        eprintln!("{:#?}", cfg);

        Ok(())
    }
}

impl ConvertArgs {
    fn validate_args(&self, file_type: &FileType) -> anyhow::Result<()> {
        match file_type {
            FileType::Excel => self
                .sheet_name
                .as_ref()
                .map(|_name| ())
                .ok_or_else(|| anyhow!("File type requires a sheet name: {:?}", file_type)),
            FileType::Csv => Ok(()),
        }
    }
}
