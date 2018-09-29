use std::fmt;

// special number for BIP44 purpose
pub const BIP44_PURPOSE: u32 = 0x8000002C;

// the soft derivation is upper bounded
pub const BIP44_SOFT_UPPER_BOUND: u32 = 0x80000000;

// length of bip44
pub const BIP44_PARTS_COUNT: usize = 6;

#[derive(Debug, Clone)]
pub enum Error {
  InvalidPartsCount(usize),
  InvalidPurpose(u32),
  InvalidPathMarker(String),
  InvalidCoin(u32, u32),
  InvalidChange(u32),
  InvalidAddress(u32),
  EmptyValueAtIndex(usize),
  ParseErrorAtIndex(usize, std::num::ParseIntError),
  NonHardenedValueAtIndex(usize),
  HardenedValueAtIndex(usize)
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &Error::InvalidPartsCount(count) => write!(f, "Invalid BIP44 parts count {}, expected: {}", count, BIP44_PARTS_COUNT),
      &Error::InvalidPurpose(purpose) => write!(f, "Invalid BIP44 purpose {}, expected: {}", purpose, BIP44_PURPOSE),
      &Error::InvalidPathMarker(ref marker) => write!(f, "Invalid path marker '{}', expected: 'm'", marker),
      &Error::InvalidCoin(coin, accepts) => write!(f, "Invalid coin {}, expected: {}", coin, accepts),
      &Error::InvalidChange(change) => write!(f, "Invalid change {}", change),
      &Error::InvalidAddress(addr) => write!(f, "Invalid address {}", addr),
      &Error::EmptyValueAtIndex(index) => write!(f, "Found empty value at index: {}", index),
      &Error::ParseErrorAtIndex(index, ref err) => write!(f, "Can't parse number at index {}, error: {}", index, err),
      &Error::NonHardenedValueAtIndex(index) => write!(f, "Value at index {} should be hardened", index),
      &Error::HardenedValueAtIndex(index) => write!(f, "Value at index {} should be non-hardened", index)
    }
  }
}

impl std::error::Error for Error {}

pub trait Bip44KeyPath {
  fn purpose(&self) -> u32 {
    BIP44_PURPOSE
  }

  fn coin(&self) -> u32;
  fn account(&self) -> u32;
  fn change(&self) -> u32;
  fn address(&self) -> u32;
}

pub struct Bip44 {
  coin: u32,
  account: u32,
  change: u32,
  address: u32
}

impl Bip44 {
  fn hard_int(index: usize, s: &str) -> Result<u32, Error> {
    if s.len() == 0 {
      return Err(Error::EmptyValueAtIndex(index))
    }
    if &s[s.len()-1..] != "'" {
      return Err(Error::NonHardenedValueAtIndex(index))
    }
    Self::soft_int(index, &s[..s.len() - 1]).map(|val| { val + BIP44_SOFT_UPPER_BOUND })
  }

  fn soft_int(index: usize, s: &str) -> Result<u32, Error> {
    if s.len() == 0 {
      return Err(Error::EmptyValueAtIndex(index));
    }
    if &s[s.len()-1..] == "'" {
      return Err(Error::HardenedValueAtIndex(index))
    }
    str::parse::<u32>(s)
      .map_err(|err| { Error::ParseErrorAtIndex(index, err) })
      .and_then(|val| {
        if val >= BIP44_SOFT_UPPER_BOUND { Err(Error::HardenedValueAtIndex(index)) } else { Ok(val) }
      })
  }

  pub fn from(path: &str) -> Result<Self, Error> {
    let parts: Vec<&str> = path.split("/").map(|s| { s.trim() }).collect();
    if parts.len() != BIP44_PARTS_COUNT {
      return Err(Error::InvalidPartsCount(parts.len()));
    }
    if parts[0] != "m" {
      return Err(Error::InvalidPathMarker(parts[0].to_owned()));
    }

    Self::hard_int(1, parts[1])
      .and_then(|purpose| 
        if purpose != BIP44_PURPOSE {
          Err(Error::InvalidPurpose(purpose))
        } else {
          Self::hard_int(2, parts[2])
        }
      )
      .and_then(|coin| Self::hard_int(3, parts[3]).map(|account| (coin, account)))
      .and_then(|(coin, account)| {
        Self::soft_int(4, parts[4]).map(|change| (coin, account, change)) 
      })
      .and_then(|(coin, account, change)| {
        Self::soft_int(5, parts[5]).map(|address| (coin, account, change, address))
      })
      .map(|(coin, account, change, address)| Bip44 { coin, account, change, address })
  }
}

impl Bip44KeyPath for Bip44 {
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

impl<'a> From<&'a Bip44KeyPath> for String {
  fn from(path: &'a Bip44KeyPath) -> Self {
    format!(
      "m/{}'/{}'/{}'/{}/{}",
      path.purpose() - BIP44_SOFT_UPPER_BOUND,
      path.coin() - BIP44_SOFT_UPPER_BOUND,
      path.account() - BIP44_SOFT_UPPER_BOUND,
      path.change(),
      path.address()
    )
  }
}