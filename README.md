## CDBE - Columnar Database engine

- An attempt to achieve faster querying through column wise data storage than row-wise storage.
- Data is stored in three formats
    - `.data` for storage data of models
    - `.meta` for metadata of schema
    - `.idx` for indexing, min/max indexing(yet to research how to implement)

## Implementation
- [x] Implement Basic Columnar Storage
    - [x] Define Table Schema & Metadata
    - [x] Implement Column-Wise Metadata storage
    - [x] Implement Metadata Loading & Table Initialization
- [x] Implement Indexing for faster queries
    - [x] Add Min-Max for fast filtering
    - [x] Implementing Offset-Based Index for faster reads
- [x] Query Execution
    - [x] Implement query execution in your columnar DB

<hr>

<i>NOTE: Rustup Nightly version is required to run</i>

# CLI Reference

✅ Create a table

```sh
cargo run -- create-table users id:int name:string age:int
```
🟢 Output:

```sh
Table 'users' created!
```
✅ Insert a row

```sh
cargo run -- insert users 1 "Alice" 25
```
🟢 Output:

```sh
Inserted into 'users': ["1", "Alice", "25"]
```
✅ Scan table

```sh
cargo run -- scan users age
```
🟢 Output:

```sh
Read value: 25
Read value: 32
Read value: 43
Read value: 54
Read value: 65
Read value: 35
Read value: 54
```

✅ Filter using x86 SIMD instructions

<i>NOTE: This only supports int data types</i>

- for values equal to a threshold value
```sh
cargo run -- filter-simd-eq users age 54
```
🟢 Output:

```sh
Matched value at index 3: 54
Matched value at index 6: 54
```

- for values not equal to a threshold value
```sh
cargo run -- filter-simd-not-eq users age 25
```
🟢 Output:

```sh
Matched value at index 1: 32
Matched value at index 2: 43
Matched value at index 3: 54
Matched value at index 4: 65
Matched value at index 5: 35
Matched value at index 6: 54
```

- for values greater than a threshold value

```sh
cargo run -- filter-simd-gt users age 30
```
🟢 Output:

```sh
Matched value at index 1: 32
Matched value at index 2: 43
Matched value at index 3: 54
Matched value at index 4: 65
Matched value at index 5: 35
Matched value at index 6: 54
```

- for values lesser than a threshold value

```sh
cargo run -- filter-simd-lt users age 30
```
🟢 Output:

```sh
Matched value at index 0: 25
```
- for values lesser than equal to a threshold value

```sh
cargo run -- filter-simd-lt-eq users age 30
```
🟢 Output:

```sh
Matched value at index 0: 25
```

- for values greater than equal to a threshold value

```sh
cargo run -- filter-simd-gt-eq users age 30
```
🟢 Output:

```sh
Matched value at index 1: 32
Matched value at index 2: 43
Matched value at index 3: 54
Matched value at index 4: 65
Matched value at index 5: 35
Matched value at index 6: 54
```

- for using logical operator

```sh
cargo run -- filter-simd-logical users age gt 25 age lt 54 or
```
🟢 Output:

```sh
Matched row at index 5: age = 35, age = 35
Matched row at index 4: age = 65, age = 65
Matched row at index 1: age = 32, age = 32
Matched row at index 2: age = 43, age = 43
Matched row at index 3: age = 54, age = 54
Matched row at index 6: age = 54, age = 54
Matched row at index 0: age = 25, age = 25
```


✅ List tables

```sh
cargo run -- list-tables
```


🟢 Output:

```sh
Tables present in the database:
- users [id (int), name (string), age (int)]
```

# API Reference

## Modules

### `column.rs`
Handles column storage and operations on individual columns.

#### Structs

##### `Column`
Represents a column in a table.
```rust
pub struct Column {
    pub name: String,
    pub data_type: String,
}
```

##### `ColumnStore`
Manages column-based storage operations.
```rust
pub struct ColumnStore {
    pub base_path: String,
}
```

##### `MinMaxIndex`
Stores min-max index metadata for filtering.
```rust
pub struct MinMaxIndex {
    pub chunk_offset: u64,
    pub min_value: String,
    pub max_value: String,
}
```

#### Methods

##### `ColumnStore::new(base_path: &str) -> Self`
Creates a new column store and initializes the base directory.

##### `ColumnStore::insert_row(&self, table: &TableSchema, values: Vec<&str>)`
Inserts a row into the column store, updating min-max indexes.

##### `ColumnStore::scan_column(&self, table: &TableSchema, column_name: &str)`
Reads all values from a specified column and prints them.

##### `ColumnStore::filter_column(&self, table: &TableSchema, column_name: &str, predicate: &str) -> Vec<String>`
Filters a column based on a predicate using min-max indexes and returns matching values.

---

### `table.rs`
Handles table schema management and metadata storage.

#### Structs

##### `TableSchema`
Represents a table schema.
```rust
pub struct TableSchema {
    pub table_name: String,
    pub columns: Vec<Column>,
}
```

#### Methods

#### `TableSchema::new(table_name: String, columns: Vec<String>) -> Self`
Creates a table with the name and columns inside it.

##### `TableSchema::save(&self, base_path: &str)`
Saves the table schema metadata as a JSON file.

##### `TableSchema::load(base_path: &str, table_name: &str) -> Self`
Loads the table schema from metadata storage.

##### `TableSchema::load_metadata(base_path: &str) -> HashMap<String, TableSchema>`
Loads all table metadata in the base directory and prints the available tables.

---

### `mod.rs`
Module declarations for `column.rs` and `table.rs`.

```rust
pub mod column;
pub mod table;
```

---

### `main.rs`
Entrypoint for the program.

```rust
use storage::{column::{Column, ColumnStore}, table::TableSchema};

pub mod storage;
```

---

## Usage Example

```rust
fn main() {
    let store = ColumnStore::new("./data");
    let schema = TableSchema {
        table_name: "users".to_string(),
        columns: vec![
            Column { name: "id".to_string(), data_type: "int".to_string() },
            Column { name: "name".to_string(), data_type: "string".to_string() },
        ],
    };
    
    schema.save("./data");
    store.insert_row(&schema, vec!["1", "Alice"]);
    store.scan_column(&schema, "name");
    let results = store.filter_column(&schema, "name", "Alice");
    println!("Filtered results: {:?}", results);
}
```

This reference provides a clear overview of the API structure, usage, and example implementation.

## Future todos:
- [x] Implement command line using clap
- [ ] test the benchmark with postgresql database.