use calamine::{Reader, open_workbook, Xlsx, DataType};


use std::path::PathBuf;

use crate::table::Tabular;

#[derive(Debug)]
pub struct Excel {
    file: PathBuf
}

impl Excel{
    pub fn new(file: PathBuf) -> Self {
        Excel { file }
    }
}

impl Tabular for Excel {
    fn into_table(&self) -> anyhow::Result<crate::table::Table> {
        let mut wb: Xlsx<_>  = open_workbook(&self.file)?;
        
//        wb.sheet_names()
        
        
        todo!()
    }
}