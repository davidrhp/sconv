use std::{fs::File, io::BufReader, path::Path};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub translations: Vec<Translation>,
}

#[derive(Deserialize, Debug)]
pub struct Translation {
    pub name: String,
    pub fields: Vec<Column>,
}

impl Settings {
    pub fn new<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        Ok(serde_yaml::from_reader(reader)?)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypeDescription {
    Int,
    Float,
    String,
    Bool,
}

#[derive(Debug, Deserialize)]
pub struct Column {
    name: String,
    #[serde(rename = "type")]
    type_description: TypeDescription,
    #[serde(flatten)]
    unit: Unit,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "unit")]
pub enum Unit {
    Single,
    Multiple(Multiple),
}

#[derive(Debug, Deserialize)]
pub struct Multiple {
    pub separator: String,
}
