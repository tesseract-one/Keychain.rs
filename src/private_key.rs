use std::fmt;
use key_path::{ Bip44KeyPath, Error as KeyPathError };
use mnemonic::{ Error as MnemonicError };

#[derive(Debug)]
pub enum Error {
  InvalidKeyPath(KeyPathError),
  InvalidMnemonic(MnemonicError),
  InvalidKeySize(usize, usize),
  InvalidKeyData(Box<std::error::Error>),
  SignError(Box<std::error::Error>)
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &Error::InvalidKeyPath(ref err) => write!(f, "Key Path error: {}", err),
      &Error::InvalidMnemonic(ref err) => write!(f, "Mnemonic error: {}", err),
      &Error::InvalidKeySize(size, good) => write!(f, "Invalid key size {}, accepts {}", size, good),
      &Error::InvalidKeyData(ref err) => write!(f, "Invalid key data: {}", err),
      &Error::SignError(ref err) => write!(f, "Sign error", err)
    }
  }
}

impl From<MnemonicError> for Error {
  fn from(err: MnemonicError) -> Self {
    Error::InvalidMnemonic(err)
  }
}

impl From<KeyPathError> for Error {
  fn from(err: KeyPathError) -> Self {
    Error::InvalidKeyPath(err)
  }
}

impl std::error::Error for Error {}

pub trait PrivateKey {
  fn from_data(data: &[u8]) -> Result<Self, Error> where Self: Sized;

  fn pub_key(&self, path: &Bip44KeyPath) -> Result<Vec<u8>, Error>;

  fn sign(&self, data: &[u8], path: &Bip44KeyPath) -> Result<Vec<u8>, Error>;

  fn boxed(self) -> Box<PrivateKey> where Self: Sized + 'static {
    Box::new(self)
  }
}