use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::bail;
use clap::Args;

use crate::printer;

use super::Execute;

#[derive(Args)]
pub struct Convert {
    /// Input file that should be converted
    #[arg(long)]
    file: PathBuf,
}

impl Execute for Convert {
    fn execute(&self) -> anyhow::Result<()> {
        // read file
        // check file type
        if !self.file.is_file() {
            bail!("{:?} is not a file", self.file.file_name())
        }
        printer::eprintln_debug("Processing", &self.file);

        let format = FileFormat::from(&self.file)?;
        
        printer::eprintln_debug("Format", &format);
        
        Ok(())
    }
}

#[derive(Debug)]
enum FileFormat {
    Excel,
    Csv,
}

impl FileFormat {
    fn from<P: AsRef<Path>>(value: P) -> anyhow::Result<Self> {
        let extension = value.as_ref()
            .extension()
            .ok_or_else(|| anyhow!("file has no extension: {:?}", value.as_ref()))?
            .to_str()
            .ok_or_else(|| {
                anyhow!(
                    "file extenstion is not a valid UTF-8 String: {:?}",
                    value.as_ref()
                )
            })?;
        
        Ok(extension.parse()?)
    }
}

impl FromStr for FileFormat {
    type Err = anyhow::Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let format = match s {
            "xlsx" => Self::Excel,
            "csv" => Self::Csv,
            _ => bail!("Unknown extension type: {s}. Please provide one of: [xlsx, csv]"),
        };

        Ok(format)   
    }
}
