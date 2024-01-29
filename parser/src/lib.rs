use async_trait::async_trait;
use std::fmt::Debug;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

mod error;
mod helpers;

pub use error::ParserError;

use crate::helpers::ParserCore;

pub type Result<T> = std::result::Result<T, ParserError>;
pub type DBResult<T, DBError> = std::result::Result<T, DBError>;

#[async_trait]
pub trait DBGetHandler<'r, DBHandler, RV, DBError>
where
    RV: AsRef<[u8]>,
    DBError: Into<ParserError> + Debug,
{
    async fn open(&self, series_iuid: &[u8; 64]) -> DBResult<DBHandler, DBError>;
    async fn close(&self, series_iuid: &[u8; 64]);
}

pub trait DBHandlerActions<'r, RV, DBError>
where
    RV: AsRef<[u8]>,
    DBError: Into<ParserError> + Debug,
{
    fn put(&self, key: &[u8], value: &[u8]) -> DBResult<(), DBError>;
    fn get(&self, key: &[u8]) -> DBResult<Option<Vec<u8>>, DBError>;
    fn get_ref(&'r self, key: &[u8]) -> DBResult<Option<RV>, DBError>;
    fn delete(&self, key: &[u8]) -> DBResult<(), DBError>;
    // range
    // prefix
}

#[async_trait]
pub trait Parser<'r, DB, DBHandler, RV, DBError>
where
    RV: AsRef<[u8]>,
    DBError: Into<ParserError> + Debug,
    DBHandler: DBHandlerActions<'r, RV, DBError> + Send,
    ParserError: From<DBError>,
    Self: DBGetHandler<'r, DBHandler, RV, DBError>,
{
    async fn inner_store<S: AsyncReadExt + AsyncSeekExt + Unpin + Send>(
        &'r self,
        series_iuid: &[u8; 64],
        parser: &mut ParserCore<'_, S>,
    ) -> Result<()> {
        let handler = self.open(&[0; 64]).await?;

        handler.put(&[0], &[0])?;

        Ok(())
    }

    async fn store<S: AsyncReadExt + AsyncSeekExt + Unpin + Send>(
        &'r self,
        mut stream: S,
    ) -> Result<()> {
        let mut parser = ParserCore::new(&mut stream);

        /* DICOM PARSE */

        parser.skip_unused_preamble().await?;

        parser.validate_dicm().await?;

        let series_iuid = [0; 64];

        /* SAVE DB */

        let ret = self.inner_store(&series_iuid, &mut parser).await;

        self.close(&series_iuid).await;

        ret
    }

    async fn read<S: AsyncWriteExt + AsyncSeekExt + Unpin + Send>(
        &self,
        index: usize,
        mut stream: S,
    ) -> Result<()> {
        todo!()
    }
}
