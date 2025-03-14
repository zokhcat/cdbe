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
- [ ] Query Execution
    - [ ] Implement query execution in your columnar DB