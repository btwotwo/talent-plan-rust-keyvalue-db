//! Simple key-value store for education purposes
#![deny(missing_docs)]
use std::collections::HashMap;
#[derive(Default)]
/// Simple key-value store
pub struct KvStore {
    data: HashMap<String, String>,
}

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
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
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
    pub fn get(&self, key: String) -> Option<String> {
        self.data.get(&key).cloned()
    }

    /// Removes value with the given `key`
    /// # Example
    /// ```rust
    /// # use kvs::KvStore;
    /// let mut kvstore = KvStore::new();
    /// kvstore.set("foo".to_string(), "bar".to_string());
    /// kvstore.remove("foo".to_string());
    /// assert_eq!(kvstore.get("foo".to_string()), None);
    pub fn remove(&mut self, key: String) {
        self.data.remove(&key);
    }
}
