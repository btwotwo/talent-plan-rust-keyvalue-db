//! Simple key-value store for education purposes
// #![deny(missing_docs)]
pub mod err;
pub use err::{KvStoreErrorKind, Result};
use std::io::Seek;
use std::path::PathBuf;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::path;
use std::{ops::DerefMut, sync::RwLock};

use bson;
use bson::{Bson};
use io::SeekFrom;
use serde::{Deserialize, Serialize};
use KvStoreErrorKind::KeyDoesNotExist;

type OffsetMap = HashMap<String, u64>;

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
    map: OffsetMap,
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
        let mut path = path.into();
        path.push("foo.db");

        fn create_file(path: &PathBuf) -> Result<File> {
            Ok(OpenOptions::new()
                .create(true)
                .read(true)
                .append(true)
                .open(&path)?)
        }

        let file = create_file(&path)?;

        let map = KvStore::replay(file)?;

        let file = create_file(&path)?;

        let locked_file = RwLock::new(file);

        Ok(KvStore {
            db_file: locked_file,
            map,
        })
    }

    fn replay(file: File) -> Result<OffsetMap> {
        let mut file = BufReader::new(file);
        let mut result = HashMap::<String, u64>::new();
        let max_length = file.seek(SeekFrom::End(0))?;

        if max_length == 0 {
            return Ok(result);
        };

        file.seek(SeekFrom::Start(0))?;

        loop {
            let pos = file.seek(SeekFrom::Current(0))?;
            let doc = bson::decode_document(&mut file)?;
            
            match bson::from_bson(bson::Bson::Document(doc))? {
                Command::Set { key, value } => {
                    result.insert(key, pos);
                }
                Command::Rm { key } => {
                    result.remove(&key);
                }
            };
            
            let new_pos = file.seek(SeekFrom::Current(0))?;
            if new_pos >= max_length {
                break;
            }
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
        let command = Command::Set { key, value };
        serialize_command(command, &mut file)?;
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
        let mut file = self.db_file.write()?;
        let command_pos = match self.map.get(&key).map(|x| x.to_owned()) {
            None => return Ok(None),
            Some(val) => val
        };
        file.seek(SeekFrom::Start(command_pos))?;

        let doc = bson::decode_document(&mut file.deref_mut())?;

        match bson::from_bson(bson::Bson::Document(doc))? {
            Command::Set { key: _, value } => {
                Ok(Some(value))
            }
            Command::Rm { key: _ } => {
                Err(KvStoreErrorKind::CorruptedDatabaseEntry.into())
            }
        }
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
        if self.map.contains_key(&key) {
            let mut file = self.db_file.write()?;
            let command = Command::Rm { key };
            serialize_command(command, &mut file)?;
            Ok(())
        } else {
            Err(KeyDoesNotExist.into())
        }
    }
}

fn serialize_command(command: Command, mut file: &mut File) -> Result<()> {
    let command = match bson::to_bson(&command)? {
        Bson::Document(doc) => doc,
        _ => unreachable!(),
    };
    bson::encode_document(file.deref_mut(), &command)?;
    file.flush()?;
    Ok(())
}
