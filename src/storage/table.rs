use std::fs;

use serde::{Deserialize, Serialize};

use super::column::Column;

#[derive(Serialize, Deserialize, Debug)]
pub struct TableSchema {
    pub table_name: String,
    pub columns: Vec<Column>,
}

impl TableSchema {
    pub fn save(&self, base_path: &str) {
        let path = format!("{}/{}.meta", base_path, self.table_name);
        let json = serde_json::to_string_pretty(self).unwrap();
        fs::write(path, json).unwrap();
    }

    pub fn load(base_path: &str, table_name: &str) -> Self  {
        let path = format!("{}/{}.meta", base_path, table_name);
        let json = fs::read_to_string(path).unwrap();
        serde_json::from_str(&json).unwrap()
    }
}