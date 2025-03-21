use std::collections::HashMap;

use storage::{column:: ColumnStore, table::TableSchema};
use clap::{command, Parser, Subcommand};

pub mod storage;

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
        Commands::Scan { table_name } => {
            if let Some(schema) = tables.get(table_name) {
                let store = ColumnStore::new(base_path);
                store.scan_column(schema, &schema.columns[0].name);
            } else {
                println!("Table '{}' not found.", table_name);
            }
        }
        Commands::ListTables => {
            TableSchema::load_metadata(base_path);
        }
    }
}