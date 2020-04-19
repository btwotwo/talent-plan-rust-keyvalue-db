//! Simple key-value store for education purposes
// #![deny(missing_docs)]
pub mod err;
pub use err::Result;

use std::fs::File;
use std::path;
use std::sync::RwLock;
use std::io::{BufReader, BufWriter, Read, Write};

use serde::{Deserialize, Serialize};
use bson;
use bson::Bson;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
enum Command {
    #[serde(rename = "s")]
    Set {
        #[serde(rename = "k")]
        key: String, 

        #[serde(rename = "v")]
        value: String
    },

    #[serde(rename = "r")]
    Rm {
        #[serde(rename = "k")]
        key: String
    }
}

/// Simple key-value store
pub struct KvStore {
    /// File which store stores logs into
    db_file: RwLock<File>,
}

impl KvStore {
    /// Create a new instance of KvStore with log file in the specified location
    /// # Example
    /// ```rust
    /// use kvs::KvStore;
    /// let kvstore = KvStore::new("foo.db");
    /// ```
    pub fn open(path: impl Into<path::PathBuf>) -> Result<KvStore> {
        use std::fs::OpenOptions;

        let file = OpenOptions::new()
            .read(true)
            .append(true)
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
    /// let mut kvstore = KvStore::new("foo.db");
    /// kvstore.set("foo".to_string(), "bar".to_string())
    /// ```
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        use std::ops::DerefMut;
        let mut file = self.db_file.write()?;
        let command = match bson::to_bson(&Command::Set{key, value})? {
            Bson::Document(doc) => doc,
            _ => panic!("this shouldn't happen")
        };

        bson::encode_document(&mut file.deref_mut(), &command)?;

        file.flush()?;

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
        unimplemented!()
    }

    /// Removes value with the given `key`
    /// # Example
    /// ```rust
    /// # use kvs::KvStore;
    /// let mut kvstore = KvStore::new("foo.db`");
    /// kvstore.set("foo".to_string(), "bar".to_string());
    /// kvstore.remove("foo".to_string());
    /// assert_eq!(kvstore.get("foo".to_string()), None);
    pub fn remove(&mut self, key: String) -> Result<()> {
        todo!()
        // use std::ops::DerefMut;
        // let mut file = self.db_file.write()?;
        // let command = match bson::to_bson(&Command::Set{key, value})? {
        //     Bson::Document(doc) => doc,
        //     _ => panic!("this shouldn't happen")
        // };

        // bson::encode_document(&mut file.deref_mut(), &command)?;

        // file.flush()?;

        // Ok(())
    }
}
