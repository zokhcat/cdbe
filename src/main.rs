use storage::{column::{Column, ColumnStore}, table::TableSchema};

pub mod storage;

fn main() {
    println!("Columnar Database Engine!");

    let store = ColumnStore::new("data");
    
    let table = TableSchema {
        table_name: "employees".to_string(),
        columns: vec![Column {
            name: "age".to_string(),
            data_type: "int".to_string(),
        }],
    };

    store.insert_row(&table, vec!["25"]);
    store.insert_row(&table, vec!["30"]);
    store.insert_row(&table, vec!["40"]);
    store.insert_row(&table, vec!["50"]);

    let results = store.filter_column(&table, "age", "30");
    println!("Filtered Results: {:?}", results); 
}
