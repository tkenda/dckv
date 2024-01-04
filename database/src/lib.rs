use strum::{AsRefStr, Display, EnumVariantNames};

mod error;
mod rocks;

pub use error::DatabaseError;
pub use rocks::{RocksDB, RocksDBConfig};

pub type Result<T> = std::result::Result<T, DatabaseError>;

pub trait DBActions<DB: DBActions<DB, DBConfig>, DBConfig> {
    fn open(config: &DBConfig) -> Result<DB>;
    fn get(&self, category: Category, key: &[u8]) -> Result<Option<Vec<u8>>>;
    fn put(&self, category: Category, key: &[u8], value: &[u8]) -> Result<()>;
    fn delete(&self, category: Category, key: &[u8]) -> Result<()>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsRefStr, Display, EnumVariantNames)]
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
