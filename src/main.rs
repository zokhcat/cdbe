use storage::{column::{Column, ColumnStore}, table::TableSchema};

pub mod storage;

fn main() {
    println!("Columnar Database Engine!");

    // Code to test out the implementation
    let schema = TableSchema {
        table_name: "employees".to_string(),
        columns: vec![
            Column { name: "id".to_string(), data_type: "int".to_string() },
            Column { name: "name".to_string(), data_type: "string".to_string() },
            Column { name: "age".to_string(), data_type: "int".to_string()},
        ],
    };

    schema.save("data/");

    let db = ColumnStore::new("data/");
    db.insert_row(&schema, vec!["1", "Alice", "25"]);
    db.insert_row(&schema, vec!["2", "Bob", "30"]);
}
