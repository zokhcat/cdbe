use serde::{Deserialize, Serialize};
use std::{fs::{self, File, OpenOptions}, io::{BufRead, BufReader, Read, Seek, SeekFrom, Write}, vec};

use super::table::TableSchema;
use crate::utils::simd::filter_simd_gt_32;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Column {
    pub name: String,
    pub data_type: String
}

pub struct ColumnStore {
    pub base_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MinMaxIndex {
    pub chunk_offset: u64,
    pub min_value: String,
    pub max_value: String,
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
            let data_path = format!("{}/{}_{}.data", self.base_path, table.table_name, column.name);
            let index_path = format!("{}/{}_{}.idx", self.base_path, table.table_name, column.name);

            let mut data_file = OpenOptions::new().create(true).append(true).open(data_path).unwrap();
            let mut index_file = OpenOptions::new().create(true).append(true).open(&index_path).unwrap();

            let offset = data_file.seek(SeekFrom::End(0)).unwrap();

            let (mut min_value, mut max_value) = match column.data_type.as_str() {
                "int" => {
                    let val:i32 = values[i].parse().unwrap();
                    data_file.write_all(&val.to_le_bytes()).unwrap();
                    (val.to_string(), val.to_string())
                }
                "string" => {
                    let val = values[i];
                    let len = val.len() as u32;
                    data_file.write_all(&len.to_le_bytes()).unwrap();
                    data_file.write_all(val.as_bytes()).unwrap();
                    (val.to_string(), val.to_string())
                }
                _ => panic!("Unsupported data type"),
            };

            let final_min = min_value.clone();
            let final_max = max_value.clone();

            if let Ok(existing_index) = File::open(&index_path) {
                let reader = BufReader::new(existing_index);
                for line in reader.lines() {
                    let entry: MinMaxIndex = serde_json::from_str(&line.unwrap()).unwrap();
                    min_value = min_value.min(entry.min_value);
                    max_value = max_value.max(entry.max_value);
                }
            }

            let index_entry = MinMaxIndex {
                chunk_offset: offset,
                min_value: final_min,
                max_value: final_max,
            };

            let serialized_index = serde_json::to_string(&index_entry).unwrap() + "\n";
            index_file.write_all(serialized_index.as_bytes()).unwrap();
        }
    }

    pub fn scan_column(&self, table: &TableSchema, column_name: &str) {
        let path = format!("{}/{}_{}.data", self.base_path, table.table_name, column_name);
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);

        match table.columns.iter().find(|c| c.name == column_name).unwrap().data_type.as_str() {
            "int" => {
                let mut buffer = [0u8; 4];
                loop {
                    match reader.read_exact(&mut buffer) {
                    Ok(_) => {
                        let val = i32::from_le_bytes(buffer);
                        println!("Read value: {}", val);
                    }
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::UnexpectedEof {
                            break;
                        } else {
                        panic!("Failed to read int column: {:?}", e);
                        }
                    }
                }
            }
        }
        "string" => {
            loop {
                let mut len_buf = [0u8; 4];
                match reader.read_exact(&mut len_buf) {
                    Ok(_) => {
                        let len = u32::from_le_bytes(len_buf) as usize;
                        let mut buffer = vec![0u8; len];
                        reader.read_exact(&mut buffer).unwrap();
                        let val = String::from_utf8(buffer).unwrap();
                        println!("Read value: {}", val);
                    }
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::UnexpectedEof {
                            break;
                        } else {
                            panic!("Failed to read string column: {:?}", e);
                        }
                        }
                    }
                }
            }
            _ => panic!("Unsupported data type"),
        }
    }

    pub fn filter_column(&self, table: &TableSchema, column_name: &str, predicate: &str) -> Vec<String> {
        let index_path = format!("{}/{}_{}.idx", self.base_path, table.table_name, column_name);
        let data_path = format!("{}/{}_{}.data", self.base_path, table.table_name, column_name);

        let index_file = File::open(&index_path).unwrap();
        let mut data_file = File::open(&data_path).unwrap();

        let reader = BufReader::new(index_file);
        let mut results = Vec::new();

        for line in reader.lines() {
            let index_entry: MinMaxIndex = serde_json::from_str(&line.unwrap()).unwrap();

            if predicate < &index_entry.min_value || predicate > &index_entry.max_value {
                continue;
            }

            data_file.seek(SeekFrom::Start(index_entry.chunk_offset)).unwrap();
            
            match table.columns.iter().find(|c| c.name == column_name).unwrap().data_type.as_str() {
                "int" => {
                    let mut buffer = [0u8; 4];
                    while data_file.read_exact(&mut buffer).is_ok() {
                        let val = i32::from_le_bytes(buffer);
                        if val.to_string() == predicate {
                            results.push(val.to_string());
                        }
                    }
                }
                "string" => {
                    let mut len_buf = [0u8; 4];
                    while data_file.read_exact(&mut len_buf).is_ok() {
                        let len = u32::from_le_bytes(len_buf) as usize;
                        let mut buffer = vec![0u8; len];
                        data_file.read_exact(&mut buffer).unwrap();
                        let val = String::from_utf8(buffer).unwrap();
                        if val == predicate {
                            results.push(val);
                        }
                    }
                }
                _ => panic!("Unsupported data type"),
            }
        }

        results
    }

    pub fn filter_column_simd(&self, table: &TableSchema, column_name: &str, threshold_value: i32) {
        let path = format!("{}/{}_{}.data", self.base_path, table.table_name, column_name);
        let mut file = File::open(path).unwrap();
        let mut buffer = Vec::new();
        let mut reader = BufReader::new(&mut file);
        let mut val_buf = [0u8; 4];
    
        while reader.read_exact(&mut val_buf).is_ok() {
            let val = i32::from_le_bytes(val_buf);
            buffer.push(val);
        }
    
        let matching_indices = filter_simd_gt_32(&buffer, threshold_value);
    
        for idx in matching_indices {
            println!("Matched value at index {}: {}", idx, buffer[idx]);
        }
    }
    
}