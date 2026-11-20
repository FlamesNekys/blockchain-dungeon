use std::fmt;

use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hash([u8; 32]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[error("контрольная сумма занимает 32 байта, а получено {0}")]
pub struct WrongHashLength(pub usize);

impl Hash {
    pub const ZERO: Hash = Hash([0; 32]);

    #[must_use]
    pub fn of(bytes: &[u8]) -> Hash {
        Hash(Sha256::digest(bytes).into())
    }

    #[must_use]
    pub fn from_bytes(bytes: [u8; 32]) -> Hash {
        Hash(bytes)
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hash({self})")
    }
}

impl TryFrom<&[u8]> for Hash {
    type Error = WrongHashLength;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        <[u8; 32]>::try_from(bytes)
            .map_err(|_| WrongHashLength(bytes.len()))
            .map(Hash)
    }
}
