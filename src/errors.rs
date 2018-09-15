use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum WalletLoadError<'a> {
  NoWallet(&'a str),
  WrongPassword(&'a str),
  BadData(&'a str),
  UnknownError(&'a str, &'a Error)
}

impl<'a> fmt::Display for WalletLoadError<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      WalletLoadError::NoWallet(id) => write!(f, "Wallet with id {} doesn't exist", id),
      WalletLoadError::WrongPassword(id) => write!(f, "Wrong password for wallet with id {}", id),
      WalletLoadError::UnknownError(id, err) => write!(f, "Unknown error for id {}: {}", id, err),
      WalletLoadError::BadData(id) => write!(f, "Bad data for wallet with id {}", id)
    }
  }
}

impl<'a> Error for WalletLoadError<'a> {}