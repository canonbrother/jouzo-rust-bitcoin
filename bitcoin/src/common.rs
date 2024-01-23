
use crate::{
  consensus::{Encodable, Decodable, encode},
  io,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Maybe<T>(pub Option<T>);
impl<T: Encodable + std::fmt::Debug> Encodable for Maybe<T> {
    fn consensus_encode<W: io::Write + ?Sized>(
        &self,
        w: &mut W,
    ) -> Result<usize, io::Error> {
        match &self.0 {
            Some(v) => v.consensus_encode(w),
            None => Ok(0),
        }
    }
}

impl<T: Decodable + std::fmt::Debug> Decodable for Maybe<T> {
    fn consensus_decode<R: io::Read + ?Sized>(
        r: &mut R,
    ) -> Result<Self, encode::Error> {
        match T::consensus_decode(r) {
            Ok(v) => Ok(Self(Some(v))),
            Err(encode::Error::Io(e))
                if e.kind() == io::ErrorKind::UnexpectedEof =>
            {
                Ok(Self(None))
            }
            Err(e) => Err(e),
        }
    }
}

impl<T> From<Option<T>> for Maybe<T> {
    fn from(v: Option<T>) -> Self {
        Self(v)
    }
}
