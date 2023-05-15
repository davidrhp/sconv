use std::path::PathBuf;

use anyhow::bail;
use clap::Args;

use crate::file::FileExtension;

use super::Execute;

#[derive(Args)]
pub struct Convert {
    /// Input file that should be converted
    #[arg(long)]
    file: PathBuf,
}

impl Execute for Convert {
    fn execute(&self) -> anyhow::Result<()> {
        eprintln!("Processing: {:?}", &self.file);

        let format = FileFormat::try_from(self.file.clone())?;
        eprintln!("Format: {}", format);

        Ok(())
    }
}

#[derive(Debug, strum_macros::Display)]
enum FileFormat {
    Excel(PathBuf),
    Csv(PathBuf),
}

impl TryFrom<PathBuf> for FileFormat {
    type Error = anyhow::Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        if !value.is_file() {
            bail!("{:?} is not a file", value)
        }

        let extension = FileExtension::try_from(&value)?;

        FileFormat::from_extension(value, extension)
    }
}

impl FileFormat {
    fn from_extension(path: PathBuf, extension: FileExtension) -> anyhow::Result<Self> {
        let format = match extension {
            FileExtension::Xlsx | FileExtension::Xls => Self::Excel(path),
            FileExtension::Csv => Self::Csv(path),
        };

        Ok(format)
    }
}
