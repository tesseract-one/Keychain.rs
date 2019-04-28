use key_path::{Error, KeyPath as IKeyPath, BIP44_PURPOSE, BIP44_SOFT_UPPER_BOUND};

/// the coin type for mainnet.
pub const COIN_TYPE: u32 = 0x80000000;

/// the coin type for testnet.
pub const COIN_TYPE_TESTNET: u32 = 0x80000001;

/// BIP49 purpose
pub const BIP49_PURPOSE: u32 = 0x80000031;

/// BIP84 purpose
pub const BIP84_PURPOSE: u32 = 0x80000054;

#[derive(Debug, Copy, Clone)]
pub struct KeyPath {
  purpose: u32,
  coin: u32,
  account: u32,
  change: u32,
  address: u32
}

impl KeyPath {
  #[inline]
  fn coin(testnet: bool) -> u32 {
    if testnet {
      COIN_TYPE_TESTNET
    } else {
      COIN_TYPE
    }
  }

  #[inline]
  fn is_valid(account: u32, change: u32, address: u32) -> Result<(), Error> {
    if account >= BIP44_SOFT_UPPER_BOUND {
      return Err(Error::InvalidAccount(account));
    }
    if change != 0 && change != 1 {
      return Err(Error::InvalidChange(change));
    }
    if address >= BIP44_SOFT_UPPER_BOUND {
      return Err(Error::InvalidAddress(change));
    }
    Ok(())
  }

  pub fn bip44(testnet: bool, account: u32, change: u32, address: u32) -> Result<Self, Error> {
    Self::is_valid(account, change, address)?;
    Ok(KeyPath {
      purpose: BIP44_PURPOSE,
      coin: Self::coin(testnet),
      account: account + BIP44_SOFT_UPPER_BOUND,
      change,
      address
    })
  }

  pub fn bip49(testnet: bool, account: u32, change: u32, address: u32) -> Result<Self, Error> {
    Self::is_valid(account, change, address)?;
    Ok(KeyPath {
      purpose: BIP49_PURPOSE,
      coin: Self::coin(testnet),
      account: account + BIP44_SOFT_UPPER_BOUND,
      change,
      address
    })
  }

  pub fn bip84(testnet: bool, account: u32, change: u32, address: u32) -> Result<Self, Error> {
    Self::is_valid(account, change, address)?;
    Ok(KeyPath {
      purpose: BIP84_PURPOSE,
      coin: Self::coin(testnet),
      account: account + BIP44_SOFT_UPPER_BOUND,
      change,
      address
    })
  }
}

impl IKeyPath for KeyPath {
  fn purpose(&self) -> u32 {
    self.purpose
  }

  fn coin(&self) -> u32 {
    self.coin
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
