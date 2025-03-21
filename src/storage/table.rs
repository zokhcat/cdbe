use std::{collections::HashMap, fs::{self}};

use serde::{Deserialize, Serialize};

use super::column::Column;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TableSchema {
    pub table_name: String,
    pub columns: Vec<Column>,
}

impl TableSchema {
    pub fn new(table_name: String, columns: Vec<String>) -> Self {
        let parsed_columns = columns
            .iter()
            .map(|col| {
                let parts: Vec<&str> = col.split(':').collect();
                if parts.len() != 2 {
                    panic!("Invalid column format. Use 'name:type'. Example: 'age:int'");
                }
                Column {
                    name: parts[0].to_string(),
                    data_type: parts[1].to_string(),
                }
            })
            .collect();

        Self {
            table_name,
            columns: parsed_columns,
        }
    }

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

    pub fn load_metadata(base_path: &str) -> HashMap<String, TableSchema> {
        let mut tables = HashMap::new();

        let base_paths = fs::read_dir(base_path).unwrap();
        for entry in base_paths {
            let path = entry.unwrap().path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("meta") {
                if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                    let table_schema = TableSchema::load(base_path, file_stem);
                    tables.insert(file_stem.to_string(), table_schema);
                }
            }
        }

        if tables.is_empty() {
            println!("No tables found in the database.");
        } else {
            println!("Tables present in the database:");
            for (table_name, schema) in &tables {
                let columns: Vec<String> = schema.columns.iter().map(|col| format!("{} ({})", col.name, col.data_type)).collect();
                println!("- {} [{}]", table_name, columns.join(", "));
            }
        }

        tables
    }
}