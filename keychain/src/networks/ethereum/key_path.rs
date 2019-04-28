use key_path::{Error, KeyPath as IKeyPath, BIP44_PURPOSE, BIP44_SOFT_UPPER_BOUND};

/// the BIP44 coin type for Ethereum
pub const BIP44_COIN_TYPE: u32 = 0x8000003c;

#[derive(Debug, Copy, Clone)]
pub struct KeyPath {
  account: u32,
  address: u32
}

impl KeyPath {
  pub fn new(account: u32) -> Result<Self, Error> {
    if account >= BIP44_SOFT_UPPER_BOUND {
      return Err(Error::InvalidAccount(account));
    }
    Ok(KeyPath { account: account + BIP44_SOFT_UPPER_BOUND, address: 0 })
  }

  pub fn new_metamask(account: u32) -> Result<Self, Error> {
    if account >= BIP44_SOFT_UPPER_BOUND {
      return Err(Error::InvalidAccount(account));
    }
    Ok(KeyPath { account: BIP44_SOFT_UPPER_BOUND, address: account })
  }
}

impl IKeyPath for KeyPath {
  fn purpose(&self) -> u32 {
    BIP44_PURPOSE
  }

  fn coin(&self) -> u32 {
    BIP44_COIN_TYPE
  }

  fn account(&self) -> u32 {
    self.account
  }

  fn change(&self) -> u32 {
    0
  }

  fn address(&self) -> u32 {
    self.address
  }
}
