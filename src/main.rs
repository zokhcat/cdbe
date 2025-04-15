#![feature(portable_simd)]

pub mod storage {
    pub mod column;
    pub mod table;
}
pub mod utils {
    pub mod simd;
}

use std::{collections::HashMap, fs, path::Path};

use storage::{column:: ColumnStore, table::TableSchema};
use clap::{command, Parser, Subcommand};
use utils::simd::{LogicalOp, SimdOp};


#[derive(Parser)]
#[command(name = "cdbe")]
#[command(about = "A simple columnar database implementation", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CreateTable{
        table_name: String,
        values: Vec<String>,
    },
    
    Insert {
        table_name: String,
        values: Vec<String>,
    },

    Scan {
        table_name: String,
        column_name: String
    },

    FilterSimdEq {
        table_name: String,
        column_name: String,
        threshold_value: i32
    },

    FilterSimdNotEq {
        table_name: String,
        column_name: String,
        threshold_value: i32
    },

    FilterSimdGt {
        table_name: String,
        column_name: String,
        threshold_value: i32
    },

    FilterSimdLt {
        table_name: String,
        column_name: String,
        threshold_value: i32
    },

    FilterSimdGtEq {
        table_name: String,
        column_name: String,
        threshold_value: i32
    },

    FilterSimdLtEq {
        table_name: String,
        column_name: String,
        threshold_value: i32
    },

    FilterSimdLogical {
        table_name: String,
        column1: String,
        op1: SimdOp,
        value1: i32,
        column2: String,
        op2: SimdOp,
        value2: i32,
        logic: LogicalOp,
    },

    FilterSimdEqAvx {
        table_name: String,
        column_name: String,
        threshold_value: i32
    },

    FilterSimdNotEqAvx {
        table_name: String,
        column_name: String,
        threshold_value: i32
    },

    FilterSimdGtAvx {
        table_name: String,
        column_name: String,
        threshold_value: i32
    },

    FilterSimdLtAvx {
        table_name: String,
        column_name: String,
        threshold_value: i32
    },

    FilterSimdGtEqAvx {
        table_name: String,
        column_name: String,
        threshold_value: i32
    },

    FilterSimdLtEqAvx {
        table_name: String,
        column_name: String,
        threshold_value: i32
    },

    ListTables,
}

fn main() {
    let cli = Cli::parse();
    let base_path = "data";

    let data_dir = Path::new("./data");

    if !data_dir.exists() {
        match fs::create_dir_all(data_dir) {
            Ok(_) => println!("Created data directory at ./data"),
            Err(e) => eprintln!("Failed to create data directory: {}", e),
        }
    }

    if std::is_x86_feature_detected!("avx2") {
        println!("AVX2 is supported!");
    } else {
        println!("AVX2 is NOT supported.");
    }
    
    let tables: HashMap<String, TableSchema> = TableSchema::load_metadata(base_path);

    match &cli.command {
        Commands::CreateTable { table_name, values } => {
            let schema = TableSchema::new( table_name.clone(), values.clone());
            schema.save(base_path);
            println!("Table '{}' created!", table_name);
        }
        Commands::Insert { table_name, values } => {
            if let Some(schema) = tables.get(table_name) {
                let store: ColumnStore = ColumnStore::new(base_path);
                store.insert_row(schema, values.iter().map(String::as_str).collect());
                println!("Inserted into '{}': {:?}", table_name, values);
            } else {
                println!("Table '{}' not found.", table_name);
            }
        }
        Commands::Scan { table_name, column_name } => {
            if let Some(schema) = tables.get(table_name) {
                let store = ColumnStore::new(base_path);
                store.scan_column(schema, column_name);
            } else {
                println!("Table '{}' not found.", table_name);
            }
        }
        Commands::FilterSimdEq { table_name, column_name, threshold_value } => {
            if let Some(schema) = tables.get(table_name) {
                let store = ColumnStore::new(base_path);
                store.filter_column_simd(schema, column_name, *threshold_value, SimdOp::Eq);
            } else {
                println!("Table '{}' not found.", table_name);
            }
        }
        Commands::FilterSimdNotEq { table_name, column_name, threshold_value } => {
            if let Some(schema) = tables.get(table_name) {
                let store = ColumnStore::new(base_path);
                store.filter_column_simd(schema, column_name, *threshold_value, SimdOp::Ne);
            } else {
                println!("Table '{}' not found.", table_name);
            }
        }
        Commands::FilterSimdLt { table_name, column_name, threshold_value } => {
            if let Some(schema) = tables.get(table_name) {
                let store = ColumnStore::new(base_path);
                store.filter_column_simd(schema, column_name, *threshold_value, SimdOp::Lt);
            } else {
                println!("Table '{}' not found.", table_name);
            }
        }
        Commands::FilterSimdGt { table_name, column_name, threshold_value } => {
            if let Some(schema) = tables.get(table_name) {
                let store = ColumnStore::new(base_path);
                store.filter_column_simd(schema, column_name, *threshold_value, SimdOp::Gt);
            } else {
                println!("Table '{}' not found.", table_name);
            }
        }
        Commands::FilterSimdLtEq { table_name, column_name, threshold_value } => {
            if let Some(schema) = tables.get(table_name) {
                let store = ColumnStore::new(base_path);
                store.filter_column_simd(schema, column_name, *threshold_value, SimdOp::Le);
            } else {
                println!("Table '{}' not found.", table_name);
            }
        }
        Commands::FilterSimdGtEq { table_name, column_name, threshold_value } => {
            if let Some(schema) = tables.get(table_name) {
                let store = ColumnStore::new(base_path);
                store.filter_column_simd(schema, column_name, *threshold_value, SimdOp::Ge);
            } else {
                println!("Table '{}' not found.", table_name);
            }
        }
        Commands::FilterSimdLogical { table_name, column1, op1, value1, column2, op2, value2, logic } => {
            if let Some(schema) = tables.get(table_name) {
                let store = ColumnStore::new(base_path);
                store.filter_columns_logical_simd(
                    schema,
                    &column1,
                    *op1,
                    *value1,
                    &column2,
                    *op2,
                    *value2,
                    *logic,
                );
            } else {
                println!("Table '{}' not found.", table_name);
            }
        }
        Commands::FilterSimdEqAvx { table_name, column_name, threshold_value } => {
            if let Some(schema) = tables.get(table_name) {
                let store = ColumnStore::new(base_path);
                store.filter_column_simd_avx(schema, column_name, *threshold_value, SimdOp::Eq);
            } else {
                println!("Table '{}' not found.", table_name);
            }
        }
        Commands::FilterSimdNotEqAvx { table_name, column_name, threshold_value } => {
            if let Some(schema) = tables.get(table_name) {
                let store = ColumnStore::new(base_path);
                store.filter_column_simd_avx(schema, column_name, *threshold_value, SimdOp::Ne);
            } else {
                println!("Table '{}' not found.", table_name);
            }
        }
        Commands::FilterSimdGtAvx { table_name, column_name, threshold_value } => {
            if let Some(schema) = tables.get(table_name) {
                let store = ColumnStore::new(base_path);
                store.filter_column_simd_avx(schema, column_name, *threshold_value, SimdOp::Gt);
            } else {
                println!("Table '{}' not found.", table_name);
            }
        }
        Commands::FilterSimdLtAvx { table_name, column_name, threshold_value } => {
            if let Some(schema) = tables.get(table_name) {
                let store = ColumnStore::new(base_path);
                store.filter_column_simd_avx(schema, column_name, *threshold_value, SimdOp::Lt);
            } else {
                println!("Table '{}' not found.", table_name);
            }
        }
        Commands::FilterSimdLtEqAvx { table_name, column_name, threshold_value } => {
            if let Some(schema) = tables.get(table_name) {
                let store = ColumnStore::new(base_path);
                store.filter_column_simd_avx(schema, column_name, *threshold_value, SimdOp::Le);
            } else {
                println!("Table '{}' not found.", table_name);
            }
        }
        Commands::FilterSimdGtEqAvx { table_name, column_name, threshold_value } => {
            if let Some(schema) = tables.get(table_name) {
                let store = ColumnStore::new(base_path);
                store.filter_column_simd_avx(schema, column_name, *threshold_value, SimdOp::Ge);
            } else {
                println!("Table '{}' not found.", table_name);
            }
        }
        Commands::ListTables => {
            TableSchema::load_metadata(base_path);
        }
    }
}