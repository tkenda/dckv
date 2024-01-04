use thiserror::Error;

use crate::Category;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("RocksDB error: {0}")]
    RocksDBError(#[from] rocksdb::Error),

    #[error("Can't create handle for category: {0}.")]
    CategoryHandleError(Category),
}
