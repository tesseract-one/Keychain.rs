use futures::prelude::*;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum StorageLoadError {
  NoKey(String),
  StorageError(String, Box<Error>)
}

impl fmt::Display for StorageLoadError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      StorageLoadError::NoKey(key) => write!(f, "Key {} doesn't exist", key),
      StorageLoadError::StorageError(key, err) => write!(f, "Key {} storage error: {}", key, err)
    }
  }
}

impl Error for StorageLoadError {}

pub trait Storage {
  fn has_bytes(&self, key: &str) -> Box<Future<Item = bool, Error = StorageLoadError>>;

  fn load_bytes(&self, key: &str) -> Box<Future<Item = Vec<u8>, Error = StorageLoadError>>;

  fn save_bytes(&self, key: &str, bytes: &[u8]) -> Box<Future<Item = (), Error = StorageLoadError>>;

  fn remove_bytes(&self, key: &str) -> Box<Future<Item = (), Error = StorageLoadError>>;
}