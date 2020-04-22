use failure;
use failure::{Backtrace, Context, Fail};
use std::fmt;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::sync::{PoisonError, RwLockWriteGuard};

#[derive(Debug)]
pub struct KvStoreError {
    inner: Context<KvStoreErrorKind>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum KvStoreErrorKind {
    #[fail(display = "General error")]
    General,

    #[fail(display = "Error opening or writing to database")]
    DatabaseFileError,

    #[fail(display = "Error serializing the command")]
    SerializationError,

    #[fail(display = "Error deserializing from database file")]
    DeserializationError,

    #[fail(display = "Error writing to the file. The file lock is poisoned")]
    PoisonedLockError,

    #[fail(display = "Key not found")]
    KeyDoesNotExist,
}

impl Fail for KvStoreError {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for KvStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl KvStoreError {
    pub fn kind(&self) -> KvStoreErrorKind {
        *self.inner.get_context()
    }
}

impl From<KvStoreErrorKind> for KvStoreError {
    fn from(kind: KvStoreErrorKind) -> KvStoreError {
        KvStoreError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<KvStoreErrorKind>> for KvStoreError {
    fn from(inner: Context<KvStoreErrorKind>) -> KvStoreError {
        KvStoreError { inner }
    }
}

impl From<io::Error> for KvStoreError {
    fn from(err: io::Error) -> KvStoreError {
        dbg!(err);
        KvStoreErrorKind::DatabaseFileError.into()
    }
}

impl From<bson::EncoderError> for KvStoreError {
    fn from(err: bson::EncoderError) -> KvStoreError {
        dbg!(err);
        KvStoreErrorKind::SerializationError.into()
    }
}

impl From<bson::DecoderError> for KvStoreError {
    fn from(_: bson::DecoderError) -> Self {
        KvStoreErrorKind::DeserializationError.into()
    }
}

impl From<PoisonError<RwLockWriteGuard<'_, File>>> for KvStoreError {
    fn from(err: PoisonError<RwLockWriteGuard<'_, File>>) -> KvStoreError {
        dbg!(err);
        KvStoreErrorKind::PoisonedLockError.into()
    }
}
pub type Result<TRes> = std::result::Result<TRes, KvStoreError>;
