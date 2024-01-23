use std::io::SeekFrom;

use tokio::io::{AsyncReadExt, AsyncSeekExt};

use crate::{ParserError, Result};

pub(crate) struct ParserCore<'r, S: AsyncReadExt + AsyncSeekExt + Unpin + Send> {
    stream: &'r mut S,
}

impl<'r, S: AsyncReadExt + AsyncSeekExt + Unpin + Send> ParserCore<'r, S> {
    pub(crate) fn new(stream: &'r mut S) -> Self {
        Self { stream }
    }

    /// Skip 128 unused bytes at the beginning of the DICOM file.
    pub(crate) async fn skip_unused_preamble(&mut self) -> Result<()> {
        self.stream.seek(SeekFrom::Start(128)).await?;
        Ok(())
    }

    /// Validate DICM preamble.
    pub(crate) async fn validate_dicm(&mut self) -> Result<()> {
        if self.stream.read_u32().await? == 0x4449434D {
            Ok(())
        } else {
            Err(ParserError::InvalidPreamble)
        }
    }
}
