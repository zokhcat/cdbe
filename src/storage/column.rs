use serde::{Deserialize, Serialize};
use std::{fs::{self, OpenOptions}, io::Write};

use super::table::TableSchema;

#[derive(Serialize, Deserialize, Debug)]
pub struct Column {
    pub name: String,
    pub data_type: String
}

pub struct ColumnStore {
    pub base_path: String,
}

impl ColumnStore  {
    pub fn new(base_path: &str) -> Self {
        fs::create_dir_all(base_path).unwrap();
        Self {
            base_path: base_path.to_string()
        }
    }

    pub fn insert_row(&self, table: &TableSchema, values: Vec<&str>) {
        for(i, column) in table.columns.iter().enumerate() {
            let path = format!("{}/{}_{}.data", self.base_path, table.table_name, column.name);

            let mut file = OpenOptions::new().create(true).append(true).open(path).unwrap();

            match column.data_type.as_str() {
                "int" => {
                    let val:i32 = values[i].parse().unwrap();
                    file.write_all(&val.to_le_bytes()).unwrap();
                }
                "string" => {
                    let val = values[i].as_bytes();
                    let len = val.len() as u32;
                    file.write_all(&len.to_le_bytes()).unwrap();
                    file.write_all(val).unwrap();
                }
                _ => panic!("Unsupported data type"),
            }
        }
    }
}