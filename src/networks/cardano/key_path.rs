use key_path::{ Bip44_KeyPath, BIP44_SOFT_UPPER_BOUND };

/// the BIP44 coin type is set, by default, to cardano ada.
pub const BIP44_COIN_TYPE : u32 = 0x80000717;

#[derive(Debug, Copy, Clone)]
pub struct KeyPath {
  account: u32,
  change: u32,
  address: u32
}

impl KeyPath {
  pub fn new(account: u32, change: u32, address: u32) -> Self {
    KeyPath { account: account + BIP44_SOFT_UPPER_BOUND, change, address }
  }
}

impl Bip44_KeyPath for KeyPath {
  fn coin(&self) -> u32 {
    BIP44_COIN_TYPE
  }

  fn account(&self) -> u32 {
    self.account
  }
  
  fn change(&self) -> u32 {
    self.change
  }

  fn address(&self) -> u32 {
    self.address
  }
}