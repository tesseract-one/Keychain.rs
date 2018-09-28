use futures::prelude::*;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
  KeyDoesNotExist(String),
  InternalError(String, Box<error::Error>)
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &Error::KeyDoesNotExist(ref key) => write!(f, "Key {} doesn't exist", key),
      &Error::InternalError(ref key, ref err) => write!(f, "Key {} storage error: {}", key, err)
    }
  }
}

impl error::Error for Error {}

pub trait Storage {
  fn has_bytes(&self, key: &str) -> Box<Future<Item = bool, Error = Error>>;

  fn load_bytes(&self, key: &str) -> Box<Future<Item = Vec<u8>, Error = Error>>;

  fn save_bytes(&self, key: &str, bytes: &[u8]) -> Box<Future<Item = (), Error = Error>>;

  fn remove_bytes(&self, key: &str) -> Box<Future<Item = (), Error = Error>>;
}