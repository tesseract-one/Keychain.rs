use super::wallet::Wallet;
use std::any::Any;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum NetworkType {
  Cardano,
  Ethereum,
  EOS
}

impl fmt::Display for NetworkType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      NetworkType::Cardano => write!(f, "Cardano"),
      NetworkType::Ethereum => write!(f, "Ethereum"),
      NetworkType::EOS => write!(f, "EOS")
    }
  }
}

pub trait Network {
  fn get_network_type(&self) -> NetworkType;

  fn get_seed_words_amount(&self) -> u8;

  fn create_wallet(&self, mnemonic: &[&str]) -> Box<Wallet>;

  fn send_transaction(&self, transaction: &Any);
}

pub trait TypedNetwork : Network {
  type Wallet: Wallet;

  fn create_wallet(&self, mnemonic: &[&str]) -> Self::Wallet;
}
