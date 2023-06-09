use std::path::PathBuf;

use crate::table::{Table};

#[derive(Debug)]
pub struct Csv {
    _file: PathBuf,
}

impl Csv {
    pub fn new(_file: PathBuf) -> Self {
        Csv { _file }
    }
}

impl TryFrom<Csv> for Table {
    type Error = anyhow::Error;

    fn try_from(_value: Csv) -> Result<Self, Self::Error> {
        todo!()
    }
}
