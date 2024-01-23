use async_trait::async_trait;
use dckv_parser::{DBGetHandler, Parser};
use rocksdb::{DBPinnableSlice, DBWithThreadMode, Options, SingleThreaded, DB};
use std::{collections::HashMap, sync::Arc};
use strum::{AsRefStr, Display, EnumVariantNames, VariantNames};
use tokio::sync::Mutex;

use super::{DBHandlerActions, Result};
use crate::DatabaseError;

#[derive(Debug, Copy, Clone, PartialEq, Eq, AsRefStr, Display, EnumVariantNames)]
pub enum Category {
    #[strum(serialize = "fmi")]
    FileMetaInfo,
    #[strum(serialize = "pri")]
    Private,
    #[strum(serialize = "stu")]
    Study,
    #[strum(serialize = "ser")]
    Series,
    #[strum(serialize = "can")]
    Canvas,
    #[strum(serialize = "nat")]
    Native,
    #[strum(serialize = "natref")]
    NativeByRef,
    #[strum(serialize = "cod")]
    Codec,
    #[strum(serialize = "fra")]
    Fragment,
    #[strum(serialize = "fraref")]
    FragmentByRef,
    #[strum(serialize = "res")]
    Resource,
    #[strum(serialize = "resref")]
    ResourceByRef,
    #[strum(serialize = "dat")]
    Dataset,
}

#[derive(Clone, Debug)]
pub struct RocksDBOpts {
    path: String,
}

impl RocksDBOpts {
    pub fn builder() -> RocksDBOptsBuilder {
        RocksDBOptsBuilder::default()
    }
}

#[derive(Clone, Debug, Default)]
pub struct RocksDBOptsBuilder {
    path: String,
}

impl RocksDBOptsBuilder {
    pub fn path<S: Into<String>>(mut self, path: S) -> Self {
        self.path = path.into();
        self
    }

    pub fn build(self) -> RocksDBOpts {
        RocksDBOpts { path: self.path }
    }
}

#[derive(Clone, Debug)]
pub struct RocksDBHandler(Arc<DBWithThreadMode<SingleThreaded>>);

impl RocksDBHandler {
    fn category_handle(&self, _key: &[u8]) -> Result<&rocksdb::ColumnFamily> {
        let category = Category::Canvas;

        self.0
            .cf_handle(category.as_ref())
            .ok_or(DatabaseError::CategoryHandleError(category))
    }
}

#[derive(Debug)]
struct RocksDBHandlerWrapper {
    handler: RocksDBHandler,
    counter: usize,
}

#[derive(Clone, Debug)]
pub struct RocksDB {
    db: Arc<Mutex<HashMap<[u8; 64], RocksDBHandlerWrapper>>>,
    opts: RocksDBOpts,
}

impl RocksDB {
    pub fn new(opts: RocksDBOpts) -> Self {
        Self {
            db: Arc::new(Mutex::new(HashMap::new())),
            opts,
        }
    }
}

#[async_trait]
impl<'r> DBGetHandler<'r, RocksDBHandler, DBPinnableSlice<'r>, DatabaseError> for RocksDB {
    async fn open(&self, series_iuid: &[u8; 64]) -> Result<RocksDBHandler> {
        let mut db = self.db.lock().await;

        match db.get_mut(series_iuid) {
            Some(t) => {
                t.counter += 1;
                Ok(t.handler.clone())
            }
            None => {
                let mut db_opts = Options::default();
                // The column family will be created if it is missing.
                db_opts.create_missing_column_families(true);
                // The database will be created if it is missing.
                db_opts.create_if_missing(true);

                let db_cf = DB::open_cf(&db_opts, &self.opts.path, Category::VARIANTS)?;
                let handler = RocksDBHandler(Arc::new(db_cf));

                let wrapper = RocksDBHandlerWrapper {
                    handler: handler.clone(),
                    counter: 1,
                };

                db.insert(*series_iuid, wrapper);

                Ok(handler)
            }
        }
    }

    async fn close(&self, series_iuid: &[u8; 64]) {
        let mut db = self.db.lock().await;

        if let Some(t) = db.get_mut(series_iuid) {
            t.counter -= 1;

            if t.counter == 0 {
                db.remove(series_iuid);
            }
        }
    }
}

impl<'r> DBHandlerActions<'r, DBPinnableSlice<'r>, DatabaseError> for RocksDBHandler {
    fn put(&self, key: &[u8], value: &[u8]) -> Result<()> {
        let cf = self.category_handle(key)?;
        self.0.put_cf(cf, key, value)?;
        Ok(())
    }

    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        let cf = self.category_handle(key)?;
        Ok(self.0.get_cf(cf, key)?)
    }

    fn get_ref(&'r self, key: &[u8]) -> Result<Option<DBPinnableSlice<'r>>> {
        let cf = self.category_handle(key)?;
        Ok(self.0.get_pinned_cf(cf, key)?)
    }

    fn delete(&self, key: &[u8]) -> Result<()> {
        let cf = self.category_handle(key)?;
        self.0.delete_cf(cf, key)?;
        Ok(())
    }
}

impl<'r> Parser<'r, RocksDB, RocksDBHandler, DBPinnableSlice<'r>, DatabaseError> for RocksDB {}
