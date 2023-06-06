use std::{collections::HashMap, path::PathBuf};

use anyhow::{anyhow, bail, Context};
use calamine::{open_workbook, DataType, Range, Reader, Xlsx};

use crate::table::{Table, Type};

const HEADER_ROW: usize = 1;

#[derive(Debug)]
pub struct ExcelTable {
    file: PathBuf,
    sheet: String,
}

impl ExcelTable {
    pub fn new(file: PathBuf, sheet: String) -> Self {
        ExcelTable { file, sheet }
    }

    fn extract_headers(worksheet: &Range<DataType>) -> HashMap<String, usize> {
        worksheet
            .rows()
            .take(HEADER_ROW)
            .flat_map(|row| row.iter())
            .enumerate()
            .map(|(index, value)| (index, Convertable::to_string(value)))
            .filter_map(|(index, value)| value.map(|str| (str, index)))
            .collect()
    }

    fn extract_rows(worksheet: Range<DataType>) -> anyhow::Result<Vec<Vec<Type>>> {
        worksheet
            .rows()
            .skip(HEADER_ROW)
            .map(|row| row.to_vec())
            .map(|row| {
                row.into_iter()
                    .map(|value| value.try_into())
                    .collect::<anyhow::Result<Vec<Type>>>()
            })
            .collect()
    }
}

impl TryFrom<ExcelTable> for Table {
    type Error = anyhow::Error;

    fn try_from(value: ExcelTable) -> Result<Self, Self::Error> {
        let mut wb: Xlsx<_> = open_workbook(&value.file)?;

        let ws = wb
            .worksheet_range(&value.sheet)
            .with_context(|| format!("No sheet named: {}", value.sheet))?
            .map_err(|e| anyhow!("Error while reading the worksheet {}: {e}", value.sheet))?;

        eprintln!("Processing sheet: {}", value.sheet);

        // process headers
        let headers = ExcelTable::extract_headers(&ws);

        // process values
        let rows = ExcelTable::extract_rows(ws)?;

        Ok(Table::new(headers, rows))
    }
}

impl Convertable for DataType {
    fn to_string(&self) -> Option<String> {
        match self {
            DataType::Int(v) => Some(v.to_string()),
            DataType::Float(v) => Some(v.to_string()),
            DataType::String(v) => Some(v.to_owned()),
            DataType::Bool(v) => Some(v.to_string()),
            DataType::DateTime(v) => Some(v.to_string()),
            DataType::DateTimeIso(v) => Some(v.to_string()),
            DataType::DurationIso(v) => Some(v.to_string()),
            DataType::Error(_) => None,
            DataType::Empty => None,
        }
    }
}

impl TryFrom<DataType> for Type {
    type Error = anyhow::Error;

    fn try_from(value: DataType) -> Result<Self, Self::Error> {
        let result = match value {
            DataType::Int(v) => Type::Int(v),
            DataType::Float(v) => Type::Float(v),
            DataType::String(v) => Type::String(v),
            DataType::Bool(v) => Type::Bool(v),
            DataType::Empty => Type::Empty,
            DataType::DateTime(_) | DataType::DateTimeIso(_) | DataType::DurationIso(_) => {
                let date = value
                    .as_datetime()
                    .map(|dt| dt.to_string())
                    .ok_or_else(|| anyhow!("couldn't extract date value from: {}", value))?;

                Type::String(date)
            }
            DataType::Error(err) => bail!("Invalid type: {}", err),
        };

        Ok(result)
    }
}

trait Convertable {
    fn to_string(&self) -> Option<String>;
}
