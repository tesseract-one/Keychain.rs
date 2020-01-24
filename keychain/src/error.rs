use crate::crypt::DecryptError as CryptError;
use crate::data::Error as DataError;
use crate::key::Error as KeyError;
use crate::key_path::Error as KeyPathError;
use crate::mnemonic::Error as MnemonicError;
use crate::network::Network;
use std::error::Error as AnyError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
  WrongPassword,
  NotEnoughData,
  SeedIsNotSaved,
  CantCalculateSeedSize(usize, usize),
  InvalidSeedSize(usize),
  KeyDoesNotExist(Network),
  KeyAlreadyExist(Network),
  NetworkIsNotSupported(Network),
  DataError(DataError),
  KeyError(Network, KeyError),
  MnemonicError(MnemonicError),
  KeyPathError(KeyPathError)
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &Error::WrongPassword => write!(f, "Wrong password"),
      &Error::NotEnoughData => write!(f, "Not enough data to load keychain"),
      &Error::SeedIsNotSaved => write!(f, "Seed is not saved"),
      &Error::CantCalculateSeedSize(min, max) => {
        write!(f, "Can't calculate seed size for networks: min({}), max({})", min, max)
      }
      &Error::InvalidSeedSize(size) => write!(f, "Invalid seed size {}", size),
      &Error::KeyDoesNotExist(nt) => write!(f, "Key for {} doesn't exist", nt),
      &Error::KeyAlreadyExist(nt) => write!(f, "Key for {} already exist in keychain", nt),
      &Error::NetworkIsNotSupported(nt) => write!(f, "Network {} is not supported", nt),
      &Error::DataError(ref err) => write!(f, "Data parsing error {}", err),
      &Error::KeyError(ref nt, ref err) => write!(f, "Key error {} for network {}", err, nt),
      &Error::MnemonicError(ref err) => write!(f, "Mnemonic error {}", err),
      &Error::KeyPathError(ref err) => write!(f, "Key path error {}", err)
    }
  }
}

impl AnyError for Error {}

impl From<MnemonicError> for Error {
  fn from(err: MnemonicError) -> Self {
    Error::MnemonicError(err)
  }
}

impl From<DataError> for Error {
  fn from(err: DataError) -> Self {
    Error::DataError(err)
  }
}

impl From<CryptError> for Error {
  fn from(err: CryptError) -> Self {
    match err {
      CryptError::NotEnoughData => Error::NotEnoughData,
      CryptError::DecryptionFailed => Error::WrongPassword
    }
  }
}

impl From<KeyPathError> for Error {
  fn from(err: KeyPathError) -> Self {
    Error::KeyPathError(err)
  }
}

impl Error {
  pub fn from_key_error(net: &Network, err: KeyError) -> Self {
    Error::KeyError(net.clone(), err)
  }
}
