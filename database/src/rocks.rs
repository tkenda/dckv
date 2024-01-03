use rocksdb::{ColumnFamilyDescriptor, Options, DB};
use std::sync::Arc;

use super::{DBActions, Result};

pub struct RocksDBConfig {
    path: String,
}

impl RocksDBConfig {
    pub fn builder() -> RocksDBConfigBuilder {
        RocksDBConfigBuilder::default()
    }
}

#[derive(Clone, Debug, Default)]
pub struct RocksDBConfigBuilder {
    path: String,
}

impl RocksDBConfigBuilder {
    pub fn path<S: Into<String>>(mut self, path: S) -> Self {
        self.path = path.into();
        self
    }

    pub fn build(self) -> RocksDBConfig {
        RocksDBConfig { path: self.path }
    }
}

#[derive(Clone, Debug)]
pub struct RocksDB(Arc<DB>);

impl DBActions<RocksDB, RocksDBConfig> for RocksDB {
    fn open(config: &RocksDBConfig, group: &str) -> Result<Self> {
        let mut db_opts = Options::default();
        // The column family will be created if it is missing.
        db_opts.create_missing_column_families(true);
        // The database will be created if it is missing.
        db_opts.create_if_missing(true);

        Ok(Self(Arc::new(DB::open_cf(
            &db_opts,
            &config.path,
            vec![group],
        )?)))
    }

    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        Ok(self.0.get(key)?)
    }

    fn put(&self, key: &[u8], value: &[u8]) -> Result<()> {
        self.0.put(key, value)?;
        Ok(())
    }

    fn delete(&self, key: &[u8]) -> Result<()> {
        self.0.delete(key)?;
        Ok(())
    }
}
