## CDBE - Columar Database engine

- An attempt to achieve faster querying through column wise data storage than row-wise storage.
- Data is stored in three formats
    - `.data` for storage data of models
    - `.meta` for metadata of schema
    - `.idx` for indexing, min/max indexing(yet to research how to implement)

## Implementation
- [ ] Implement Basic Columnar Storage
    - [x] Define Table Schema & Metadata
    - [x] Implement Column-Wise Metadata storage
    - [ ] Implement Metadata Loading & Table Initialization
- [ ] Implement Indexing for faster queries
    - [ ] Add Min-Max for fast filtering
    - [ ] Implementing Offset-Based Index for faster reads
- [ ] Query Execution
    - [ ] Implement query execution in your columnar DB