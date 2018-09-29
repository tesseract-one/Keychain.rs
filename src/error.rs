use entropy::OsEntropyError;
use network_type::NetworkType;
use data::{ Error as DataError };
use wallet::{ Error as WalletError };
use storage::{ Error as StorageError };
use private_key::{ Error as PrivateKeyError };
use util::crypt::{ DecryptError as CryptError };
use std::error::{ Error as AnyError };
use std::fmt;

#[derive(Debug)]
pub enum Error {
  WalletDoesNotExist(String),
  StorageError(String, Box<AnyError>),
  WrongPassword(String),
  NotEnoughData(String),
  CantCalculateSeedSize(usize, usize),
  InvalidSeedSize(String, usize),
  PrivateKeyDoesNotExist(String, NetworkType),
  DataParseError(String, DataError),
  PrivateKeyError(String, NetworkType, PrivateKeyError),
  EntropyGeneratorError(OsEntropyError),
  WalletError(String, WalletError)
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &Error::WalletDoesNotExist(ref name) => write!(f, "Wallet with name {} doesn't exist", name),
      &Error::StorageError(ref name, ref err) => write!(f, "Storage error {} for wallet {}", name, err),
      &Error::WrongPassword(ref name) => write!(f, "Wrong password for wallet {}", name),
      &Error::NotEnoughData(ref name) => write!(f, "Not enough data to load wallet {}", name),
      &Error::CantCalculateSeedSize(min, max) => write!(f, "Can't calculate seed size for networks: min({}), max({})", min, max),
      &Error::InvalidSeedSize(ref name, size) => write!(f, "Invalid seed size {} for wallet {}", size, name),
      &Error::PrivateKeyDoesNotExist(ref name, nt) => write!(f, "Private key for {} doesn't exist in wallet {}", nt, name),
      &Error::DataParseError(ref name, ref err) => write!(f, "Data parsing error {} for wallet {}", err, name),
      &Error::PrivateKeyError(ref name, ref nt, ref err) => write!(f, "Private Key error {} for network {} in wallet {}", err, nt, name),
      &Error::EntropyGeneratorError(ref err) => write!(f, "Entropy generator error {}", err),
      &Error::WalletError(ref name, ref err) => write!(f, "Error {} in wallet {}", name, err)
    }
  }
}

impl AnyError for Error {}

impl From<StorageError> for Error {
  fn from(err: StorageError) -> Self {
    match err {
      StorageError::KeyDoesNotExist(name) => Error::WalletDoesNotExist(name),
      StorageError::InternalError(name, err) => Error::StorageError(name, err),
    }
  }
}

impl From<OsEntropyError> for Error {
  fn from(err: OsEntropyError) -> Self {
    Error::EntropyGeneratorError(err)
  }
}

impl Error {
  pub fn from_decrypt_error(name: &str, err: CryptError) -> Self {
    match err {
      CryptError::NotEnoughData => Error::NotEnoughData(name.to_owned()),
      CryptError::DecryptionFailed => Error::WrongPassword(name.to_owned()),
    }
  }

  pub fn from_data_error(name: &str, err: DataError) -> Self {
    Error::DataParseError(name.to_owned(), err)
  }

  pub fn from_wallet_error(name: String, err: WalletError) -> Self {
    Error::WalletError(name, err)
  }

  pub fn from_key_error(name: String, nt: NetworkType, err: PrivateKeyError) -> Self {
    Error::PrivateKeyError(name, nt, err)
  }
}