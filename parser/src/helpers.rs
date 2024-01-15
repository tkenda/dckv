use std::io::SeekFrom;

use tokio::io::{AsyncReadExt, AsyncSeekExt};

use crate::{DataEncoding, ParserError, Result};

pub(crate) struct ParserCore<'r, S: AsyncReadExt + AsyncSeekExt + Unpin + Send> {
    stream: &'r mut S,
    encoding: DataEncoding,
}

impl<'r, S: AsyncReadExt + AsyncSeekExt + Unpin + Send> ParserCore<'r, S> {
    pub(crate) fn new(stream: &'r mut S) -> Self {
        Self {
            stream,
            encoding: DataEncoding::ExplicitVRLittleEndian,
        }
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

    /// Read Group / Element from a DICOM tag.
    pub(crate) async fn read_group_element(&mut self) -> Result<(u16, u16)> {
        match self.encoding {
            DataEncoding::ImplicitVRLittleEndian | DataEncoding::ExplicitVRLittleEndian => Ok((
                self.stream.read_u16_le().await?,
                self.stream.read_u16_le().await?,
            )),
            DataEncoding::ExplicitVRBigEndian => {
                Ok((self.stream.read_u16().await?, self.stream.read_u16().await?))
            }
        }
    }

    pub(crate) async fn read_vr(&mut self) -> Result<[u8; 2]> {
        let mut buf = [0u8; 2];
        self.stream.read_exact(&mut buf).await?;
        Ok(buf)
    }
}
