use futures::prelude::*;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum StorageLoadError<'a> {
  NoKey(&'a str),
  StorageError(&'a str, &'a Error)
}

impl<'a> fmt::Display for StorageLoadError<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      StorageLoadError::NoKey(key) => write!(f, "Key {} doesn't exist", key),
      StorageLoadError::StorageError(key, err) => write!(f, "Key {} storage error: {}", key, err)
    }
  }
}

impl<'a> Error for StorageLoadError<'a> {}

pub trait Storage {
  fn has_bytes(&self, key: &str) -> Box<Future<Item = bool, Error = Box<Error>>>;

  fn load_bytes<'a>(&self, key: &'a str) -> Box<Future<Item = Vec<u8>, Error = StorageLoadError<'a>> + 'a>;

  fn save_bytes(&self, key: &str, bytes: &[u8]) -> Box<Future<Item = (), Error = Box<Error>>>;
}