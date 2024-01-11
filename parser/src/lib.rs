use strum::{AsRefStr, Display, EnumVariantNames};
use tokio::io::AsyncReadExt;

mod error;

pub use error::ParserError;

pub type Result<T> = std::result::Result<T, ParserError>;

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

pub type DBResult<T, DBError> = std::result::Result<T, DBError>;

pub trait DBActions<
    'r,
    DB: DBActions<'r, DB, DBConfig, RV, DBError>,
    DBConfig,
    RV: AsRef<[u8]>,
    DBError,
>
{
    fn put(&self, category: &Category, key: &[u8], value: &[u8]) -> DBResult<(), DBError>;
    fn get(&self, category: &Category, key: &[u8]) -> DBResult<Option<Vec<u8>>, DBError>;
    fn get_ref(&'r self, category: &Category, key: &[u8]) -> DBResult<Option<RV>, DBError>;
    fn delete(&self, category: &Category, key: &[u8]) -> DBResult<(), DBError>;
    // range
    // prefix
}

pub trait Parser<'r, DB, DBConfig, RV, DBError>
where
    RV: AsRef<[u8]>,
    DB: DBActions<'r, DB, DBConfig, RV, DBError>,
    DBError: Into<ParserError> + std::fmt::Debug,
    Self: DBActions<'r, DB, DBConfig, RV, DBError>,
{
    async fn store<S: AsyncReadExt + Unpin>(&self, mut stream: S) -> Result<()> {
        let value = stream.read_u8().await.unwrap();
        println!("value: {}", value);

        let key = "key".to_string().as_bytes().to_vec();

        self.put(&Category::Dataset, &key, &[value]).unwrap();

        Ok(())
    }
}
