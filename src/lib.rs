//! Simple key-value store for education purposes
// #![deny(missing_docs)]
pub mod err;
pub use err::Result;

use std::fs::File;
use std::path;
use std::sync::RwLock;
use std::io::{BufReader, BufWriter};

use serde::{Deserialize, Serialize};
use rmp_serde::{Serializer, Deserializer};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct SetCommand {
    key: String,
    value: String,
}

/// Simple key-value store
pub struct KvStore {
    /// File which store stores logs into
    db_file: RwLock<File>,
}

impl KvStore {
    /// Create a new instance of KvStore
    /// # Example
    /// ```rust
    /// use kvs::KvStore;
    /// let kvstore = KvStore::new();
    /// ```
    pub fn open(path: impl Into<path::PathBuf>) -> Result<KvStore> {
        use std::fs::OpenOptions;

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path.into())?;
        let locked_file = RwLock::new(file);

        Ok(KvStore {
            db_file: locked_file,
        })
    }

    /// Sets the `value` with the given `key`
    /// # Example
    /// ```rust
    /// # use kvs::KvStore;
    /// let mut kvstore = KvStore::new();
    /// kvstore.set("foo".to_string(), "bar".to_string())
    /// ```
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let mut file = self.db_file.write();
        
        let command = SetCommand {key, value};
        let command = command.serialize()
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
        unimplemented!()
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
        unimplemented!()
    }
}
