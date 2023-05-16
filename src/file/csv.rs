use std::path::PathBuf;

use crate::table::Tabular;

#[derive(Debug)]
pub struct Csv {
    file: PathBuf
}

impl Csv{
    pub fn new(file: PathBuf) -> Self {
        Csv { file }
    }
}

impl Tabular for Csv {
    fn into_table(&self) -> anyhow::Result<crate::table::Table> {
        todo!()
    }
}