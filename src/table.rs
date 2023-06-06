use std::collections::HashMap;

#[derive(Debug)]
pub struct Table {
    column_to_row_index: HashMap<String, usize>,
    rows: Vec<Vec<Type>>,
}

impl Table {
    pub fn new(column_to_row_index: HashMap<String, usize>, rows: Vec<Vec<Type>>) -> Self {
        Table {
            column_to_row_index,
            rows,
        }
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
