#![feature(portable_simd)]

pub mod storage {
    pub mod column;
    pub mod table;
}
pub mod utils {
    pub mod simd;
}

use std::collections::HashMap;

use storage::{column:: ColumnStore, table::TableSchema};
use clap::{command, Parser, Subcommand};
use utils::simd::SimdOp;


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

    ListTables,
}

fn main() {
    let cli = Cli::parse();
    let base_path = "data";
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
        Commands::ListTables => {
            TableSchema::load_metadata(base_path);
        }
    }
}