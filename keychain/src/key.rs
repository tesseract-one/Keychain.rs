use std::fmt;
use key_path::{ KeyPath, Error as KeyPathError };
use network::Network;
use mnemonic::{ Error as MnemonicError };

#[cfg(any(feature = "ethereum", feature = "bitcoin"))]
use secp256k1_bip32::{ KeyError as SecpKeyError };

#[derive(Debug)]
pub enum Error {
  InvalidKeyPath(KeyPathError),
  InvalidMnemonic(MnemonicError),
  InvalidKeySize(usize, usize),
  InvalidKeyData(Box<std::error::Error>),
  InvalidSignatureSize(usize, usize),
  SignError(Box<std::error::Error>)
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &Error::InvalidKeyPath(ref err) => write!(f, "Key Path error: {}", err),
      &Error::InvalidMnemonic(ref err) => write!(f, "Mnemonic error: {}", err),
      &Error::InvalidKeySize(size, good) => write!(f, "Invalid key size {}, accepts {}", size, good),
      &Error::InvalidKeyData(ref err) => write!(f, "Invalid key data: {}", err),
      &Error::InvalidSignatureSize(size, good) => write!(f, "Invalid signature size {}, accepts {}", size, good),
      &Error::SignError(ref err) => write!(f, "Sign error: {}", err)
    }
  }
}

#[cfg(any(feature = "ethereum", feature = "bitcoin"))]
impl Error {
  pub fn from_secp_sign_error(err: SecpKeyError) -> Self {
    Error::SignError(Box::new(err))
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

#[cfg(any(feature = "ethereum", feature = "bitcoin"))]
impl From<SecpKeyError> for Error {
  fn from(err: SecpKeyError) -> Self {
    match err {
      SecpKeyError::InvalidSignature(bad, good) => Error::InvalidSignatureSize(bad, good),
      _ => Error::InvalidKeyData(Box::new(err))
    }
  }
}

impl std::error::Error for Error {}

pub trait Key {
  fn network(&self) -> Network;

  fn pub_key(&self, path: &KeyPath) -> Result<Vec<u8>, Error>;

  fn sign(&self, data: &[u8], path: &KeyPath) -> Result<Vec<u8>, Error>;

  fn verify(&self, data: &[u8], signature: &[u8], path: &KeyPath) -> Result<bool, Error>;

  fn boxed(self) -> Box<Key> where Self: Sized + 'static {
    Box::new(self)
  }
}