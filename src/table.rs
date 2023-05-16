use std::collections::HashMap;

use enum_dispatch::enum_dispatch;
use crate::file::FileType;
use crate::file::Csv;
use crate::file::Excel;

pub struct Table {
    map: HashMap<String, Vec<String>>,
}

impl Table {
    pub fn new(map: HashMap<String, Vec<String>>) -> Self {
        Table { map }
    }
}

#[enum_dispatch]
pub trait Tabular {
    fn into_table(&self) -> anyhow::Result<Table>;
}
