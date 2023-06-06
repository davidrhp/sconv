use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context};

use std::fmt::Debug;

mod csv;
mod excel;

pub use csv::Csv;
pub use excel::ExcelTable;

#[derive(Debug, strum_macros::Display)]
pub enum FileType {
    Excel,
    Csv,
}

impl TryFrom<PathBuf> for FileType {
    type Error = anyhow::Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        if !value.is_file() {
            bail!("{:?} is not a file", value)
        }

        let extension = FileExtension::try_from(&value)?;

        FileType::from_extension(value, extension)
    }
}

impl FileType {
    fn from_extension(_path: PathBuf, extension: FileExtension) -> anyhow::Result<Self> {
        let format = match extension {
            FileExtension::Xlsx | FileExtension::Xls => Self::Excel,
            FileExtension::Csv => Self::Csv,
        };

        Ok(format)
    }
}

#[derive(strum_macros::EnumString)]
enum FileExtension {
    #[strum(serialize = "xlsx")]
    Xlsx,
    #[strum(serialize = "xls")]
    Xls,
    #[strum(serialize = "csv")]
    Csv,
}

impl TryFrom<&PathBuf> for FileExtension {
    type Error = anyhow::Error;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        let extension = extension_string(value)?;
        extension.parse().with_context(|| {
            format!(
                "failed to parse file extension from extension string: {}",
                extension
            )
        })
    }
}

fn extension_string<P>(path: P) -> anyhow::Result<String>
where
    P: AsRef<Path> + Debug,
{
    let extension = path
        .as_ref()
        .extension()
        .ok_or_else(|| anyhow!("file has no extension: {:?}", path.as_ref()))?
        .to_str()
        .ok_or_else(|| {
            anyhow!(
                "file extenstion is not a valid UTF-8 String: {:?}",
                path.as_ref()
            )
        })?;

    Ok(extension.into())
}
