use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("RocksDB error: {0}")]
    RocksDBError(#[from] rocksdb::Error),
}
