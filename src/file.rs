use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context};

use std::fmt::Debug;

#[derive(strum_macros::EnumString)]
pub enum FileExtension {
    #[strum(serialize = "xlsx")]
    Xlsx,
    #[strum(serialize = "xls")]
    Xls,
    #[strum(serialize = "csv")]
    Csv,
}

//impl FromStr for FileExtension {
//    type Err = anyhow::Error;
//
//    fn from_str(s: &str) -> Result<Self, Self::Err> {
//        let extension = match s {
//            "xlsx" => Self::Xlsx,
//            "xls" => Self::Xls,
//            "csv" => Self::Csv,
//            _ => bail!("Unknown extension type: {s}. Please provide one of: [xlsx, xls, csv]"),
//        };
//
//        Ok(extension)
//    }
//}

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
