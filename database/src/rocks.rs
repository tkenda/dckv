use rocksdb::{DBPinnableSlice, Options, DB};
use std::{path::Path, sync::Arc};
use strum::VariantNames;

use crate::DatabaseError;

use super::{Category, DBActions, Result};

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

impl RocksDB {
    fn category_handle(&self, category: Category) -> Result<&rocksdb::ColumnFamily> {
        self.0
            .cf_handle(category.as_ref())
            .ok_or(DatabaseError::CategoryHandleError(category))
    }
}

impl<'r> DBActions<'r, RocksDB, RocksDBConfig, DBPinnableSlice<'r>> for RocksDB {
    fn open(config: &RocksDBConfig) -> Result<Self> {
        let mut cfs_raw = Vec::new();

        // Check if database path already exists and list all column families.
        if Path::new(&config.path).exists() {
            cfs_raw = DB::list_cf(&Options::default(), &config.path)?;
        }

        // Vec<String> to Vec<&str>.
        let mut cfs: Vec<&str> = cfs_raw.iter().map(AsRef::as_ref).collect();

        // Add categories.
        for category in Category::VARIANTS {
            if !cfs.contains(category) {
                cfs.push(category);
            }
        }

        let mut db_opts = Options::default();
        // The column family will be created if it is missing.
        db_opts.create_missing_column_families(true);
        // The database will be created if it is missing.
        db_opts.create_if_missing(true);

        Ok(Self(Arc::new(DB::open_cf(&db_opts, &config.path, cfs)?)))
    }

    fn get(&self, category: Category, key: &[u8]) -> Result<Option<Vec<u8>>> {
        let cf = self.category_handle(category)?;

        Ok(self.0.get_cf(cf, key)?)
    }

    fn get_ref(&'r self, category: Category, key: &[u8]) -> Result<Option<DBPinnableSlice<'r>>> {
        let cf = self.category_handle(category)?;
        Ok(self.0.get_pinned_cf(cf, key)?)
    }

    fn put(&self, category: Category, key: &[u8], value: &[u8]) -> Result<()> {
        let cf = self.category_handle(category)?;
        self.0.put_cf(cf, key, value)?;
        Ok(())
    }

    fn delete(&self, category: Category, key: &[u8]) -> Result<()> {
        let cf = self.category_handle(category)?;
        self.0.delete_cf(cf, key)?;
        Ok(())
    }
}
