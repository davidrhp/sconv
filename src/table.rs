use std::collections::HashMap;

use crate::config::Config;

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Not all columns are present")]
    NotAllColumnsPresent,
}

pub trait TableState {}

#[derive(Debug)]
pub struct UnvalidatedTable {}

#[derive(Debug)]
pub struct ValidTable {
    config: Config,
}

impl TableState for UnvalidatedTable {}
impl TableState for ValidTable {}

#[derive(Debug)]
pub struct Table<S: TableState = UnvalidatedTable> {
    content: Box<TableContent>,
    state: S,
}

#[derive(Debug)]
struct TableContent {
    column_to_row_index: HashMap<String, usize>,
    rows: Vec<Vec<Type>>,
}

impl Table {
    pub fn new(column_to_row_index: HashMap<String, usize>, rows: Vec<Vec<Type>>) -> Self {
        Table {
            content: Box::new(TableContent {
                column_to_row_index,
                rows,
            }),
            state: UnvalidatedTable { },
        }
    }
}

impl Table<UnvalidatedTable> {


    pub fn validate(self, config: Config) -> Result<Table<ValidTable>, ValidationError> {
        config
            .translations
            .iter()
            .flat_map(|translation| &translation.fields)
            .map(|column| &column.name)
            .all(|name| self.content.column_to_row_index.contains_key(name))
            .then(|| Table {
                content: self.content,
                state: ValidTable { config },
            })
            .ok_or(ValidationError::NotAllColumnsPresent)
    }
}

#[derive(Debug)]
pub enum Type {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Empty,
}
