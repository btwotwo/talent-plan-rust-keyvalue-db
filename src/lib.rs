//! Simple key-value store for education purposes
// #![deny(missing_docs)]
use failure;
use failure::{Backtrace, Context, Fail};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::path;

#[derive(Default)]
/// Simple key-value store
pub struct KvStore {
    data: HashMap<String, String>,
}

#[derive(Debug)]
pub struct KvStoreError {
    inner: Context<KvStoreErrorKind>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum KvStoreErrorKind {
    #[fail(display = "General error")]
    General,
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

pub type Result<TRes> = std::result::Result<TRes, KvStoreError>;

impl KvStore {
    /// Create a new instance of KvStore
    /// # Example
    /// ```rust
    /// use kvs::KvStore;
    /// let kvstore = KvStore::new();
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the `value` with the given `key`
    /// # Example
    /// ```rust
    /// # use kvs::KvStore;
    /// let mut kvstore = KvStore::new();
    /// kvstore.set("foo".to_string(), "bar".to_string())
    /// ```
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.data.insert(key, value);
        Ok(())
    }

    /// Gets value for the given `key`
    /// # Example
    /// ```rust
    /// # use kvs::KvStore;
    /// let mut kvstore = KvStore::new();
    /// kvstore.set("foo".to_string(), "bar".to_string());
    /// assert_eq!(kvstore.get("foo".to_string()), Some("bar".to_owned()));
    /// assert_eq!(kvstore.get("bar".to_string()), None);
    /// ```
    /// # Note
    /// Returns cloned value.
    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.data.get(&key).cloned())
    }

    /// Removes value with the given `key`
    /// # Example
    /// ```rust
    /// # use kvs::KvStore;
    /// let mut kvstore = KvStore::new();
    /// kvstore.set("foo".to_string(), "bar".to_string());
    /// kvstore.remove("foo".to_string());
    /// assert_eq!(kvstore.get("foo".to_string()), None);
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.data.remove(&key);
        Ok(())
    }

    pub fn open(path: impl Into<path::PathBuf>) -> Result<KvStore> {
        unimplemented!()
    }
}
