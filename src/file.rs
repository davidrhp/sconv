use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, bail};
use enum_dispatch::enum_dispatch;

use std::fmt::Debug;



mod excel;
mod csv;

pub use excel::Excel;
pub use csv::Csv;

#[enum_dispatch(Tabular)]
#[derive(Debug, strum_macros::Display)]
pub enum FileType {
    Excel(Excel),
    Csv(Csv),
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
    fn from_extension(path: PathBuf, extension: FileExtension) -> anyhow::Result<Self> {
        let format = match extension {
            FileExtension::Xlsx | FileExtension::Xls => Self::Excel(Excel::new(path)),
            FileExtension::Csv => Self::Csv(Csv::new(path)),
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
