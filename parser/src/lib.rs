use async_trait::async_trait;
use strum::{AsRefStr, Display, EnumVariantNames};

mod error;
mod helpers;

pub use error::ParserError;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

use crate::helpers::ParserStream;

pub type Result<T> = std::result::Result<T, ParserError>;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum DataEncoding {
    #[default]
    ImplicitVRLittleEndian,
    ExplicitVRLittleEndian,
    ExplicitVRBigEndian,
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

#[async_trait]
pub trait Parser<'r, DB, DBConfig, RV, DBError>
where
    RV: AsRef<[u8]>,
    DB: DBActions<'r, DB, DBConfig, RV, DBError>,
    DBError: Into<ParserError> + std::fmt::Debug,
    Self: DBActions<'r, DB, DBConfig, RV, DBError>,
{
    async fn store<S: AsyncReadExt + AsyncSeekExt + Unpin + Send>(&self, mut src: S) -> Result<()> {
        let mut stream = ParserStream::new(&mut src);

        /* Preamble */

        stream.skip_unused_preamble().await?;

        stream.validate_dicm().await?;

        /* Group 0x02 */

        let (group, element) = stream
            .read_group_element(DataEncoding::ExplicitVRLittleEndian)
            .await?;

        println!("{:?} {:?}", group, element);

        //self.put(&Category::Dataset, &key, &[value]).unwrap();

        Ok(())
    }
}
