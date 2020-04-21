//! Simple key-value store for education purposes
// #![deny(missing_docs)]
pub mod err;
use std::convert::TryInto;
pub use err::Result;
use std::io::Seek;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::path;
use std::{ops::DerefMut, sync::RwLock};

use bson;
use bson::Bson;
use io::SeekFrom;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
enum Command {
    #[serde(rename = "s")]
    Set {
        #[serde(rename = "k")]
        key: String,

        #[serde(rename = "v")]
        value: String,
    },

    #[serde(rename = "r")]
    Rm {
        #[serde(rename = "k")]
        key: String,
    },
}

/// Simple key-value store
pub struct KvStore {
    /// File which store stores logs into
    db_file: RwLock<File>,
    map: HashMap<String, String>
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
        let path = path.into();

        let file = OpenOptions::new()
            .read(true)
            .open(&path)?;

        let map = KvStore::replay(file)?;

        let file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&path)?;

        let locked_file = RwLock::new(file);

        Ok(KvStore {
            db_file: locked_file,
            map
        })
    }

    fn replay(mut file: File) -> Result<HashMap<String, String>> {
        // let mut file = BufReader::new(file);
        let mut result = HashMap::<String, String>::new();
        
        let max_length: usize = file.seek(SeekFrom::End(0))?.try_into().unwrap();
        file.seek(SeekFrom::Start(0))?;
        
        loop {
            let mut buf = vec![0; 1];
            file.read_exact(&mut buf)?;
            let doc_length: usize = buf[0].into();
            let current_pos = file.seek(SeekFrom::Current(-1))?;

            let doc = bson::decode_document(&mut file)?;

            match bson::from_bson(bson::Bson::Document(doc))? {
                Command::Set { key, value } => {
                    result.insert(key, value);
                }
                Command::Rm { key } => {
                    result.remove(&key);
                }
            };

            let new_pos = doc_length + (current_pos as usize);

            if new_pos >= max_length {
                break;
            }
            file.seek(SeekFrom::Start(new_pos.try_into().unwrap()))?;
        }

        Ok(result)
    }

    /// Sets the `value` with the given `key`
    /// # Example
    /// ```rust
    /// # use kvs::KvStore;
    /// let mut kvstore = KvStore::new("foo.db");
    /// kvstore.set("foo".to_string(), "bar".to_string())
    /// ```
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let mut file = self.db_file.write()?;
        let command = match bson::to_bson(&Command::Set { key, value })? {
            Bson::Document(doc) => doc,
            _ => unreachable!(),
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
        let mut file = self.db_file.write()?;
        let command = match bson::to_bson(&Command::Rm { key })? {
            Bson::Document(doc) => doc,
            _ => unreachable!(),
        };
        bson::encode_document(&mut file.deref_mut(), &command)?;
        file.flush()?;
        todo!("check for key existence");
        Ok(())
    }
}
