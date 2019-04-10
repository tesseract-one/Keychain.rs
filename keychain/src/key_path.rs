use std::fmt;

// special number for BIP44 purpose
pub const BIP44_PURPOSE: u32 = 0x8000002C;

// the soft derivation is upper bounded
pub const BIP44_SOFT_UPPER_BOUND: u32 = 0x80000000;

// length of bip44
pub const KEY_PATH_PARTS_COUNT: usize = 6;

#[derive(Debug, Clone)]
pub enum Error {
  InvalidPartsCount(usize),
  InvalidPurpose(u32, u32),
  InvalidPathMarker(String),
  InvalidCoin(u32, u32),
  InvalidAccount(u32),
  InvalidChange(u32),
  InvalidAddress(u32),
  EmptyValueAtIndex(usize),
  ParseErrorAtIndex(usize, std::num::ParseIntError)
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &Error::InvalidPartsCount(count) => write!(f, "Invalid parts count {}, expected: {}", count, KEY_PATH_PARTS_COUNT),
      &Error::InvalidPurpose(purpose, exptected) => write!(f, "Invalid purpose {}, expected: {}", purpose, exptected),
      &Error::InvalidPathMarker(ref marker) => write!(f, "Invalid path marker '{}', expected: 'm'", marker),
      &Error::InvalidCoin(coin, accepts) => write!(f, "Invalid coin {}, expected: {}", coin, accepts),
      &Error::InvalidAccount(account) => write!(f, "Invalid account {}", account),
      &Error::InvalidChange(change) => write!(f, "Invalid change {}", change),
      &Error::InvalidAddress(addr) => write!(f, "Invalid address {}", addr),
      &Error::EmptyValueAtIndex(index) => write!(f, "Found empty value at index: {}", index),
      &Error::ParseErrorAtIndex(index, ref err) => write!(f, "Can't parse number at index {}, error: {}", index, err),
    }
  }
}

impl std::error::Error for Error {}

pub trait KeyPath {
  fn purpose(&self) -> u32;
  fn coin(&self) -> u32;
  fn account(&self) -> u32;
  fn change(&self) -> u32;
  fn address(&self) -> u32;
}

pub struct GenericKeyPath {
  purpose: u32,
  coin: u32,
  account: u32,
  change: u32,
  address: u32
}

impl GenericKeyPath {
  fn hard_int(index: usize, s: &str) -> Result<u32, Error> {
    if s.len() == 0 {
      return Err(Error::EmptyValueAtIndex(index))
    }
    Self::soft_int(index, &s[..s.len() - 1]).map(|val| { val + BIP44_SOFT_UPPER_BOUND })
  }

  fn soft_int(index: usize, s: &str) -> Result<u32, Error> {
    if s.len() == 0 {
      return Err(Error::EmptyValueAtIndex(index));
    }
    str::parse::<u32>(s)
      .map_err(|err| { Error::ParseErrorAtIndex(index, err) })
  }

  fn parse_int(index: usize, s: &str) -> Result<u32, Error> {
    if s.len() == 0 {
      return Err(Error::EmptyValueAtIndex(index));
    }
    if &s[s.len()-1..] == "'" {
      Self::hard_int(index, s)
    } else {
      Self::soft_int(index, s)
    }
  }

  fn print_int(val: u32) -> String {
    if val >= BIP44_SOFT_UPPER_BOUND {
      format!("{}'", (val - BIP44_SOFT_UPPER_BOUND))
    } else {
      val.to_string()
    }
  }

  pub fn from(path: &str) -> Result<Self, Error> {
    let parts: Vec<&str> = path.split("/").map(|s| { s.trim() }).collect();
    if parts.len() != KEY_PATH_PARTS_COUNT {
      return Err(Error::InvalidPartsCount(parts.len()));
    }
    if parts[0] != "m" {
      return Err(Error::InvalidPathMarker(parts[0].to_owned()));
    }

    Self::parse_int(1, parts[1])
      .and_then(|purpose| Self::parse_int(2, parts[2]).map(|coin| (purpose, coin)))
      .and_then(|(purpose, coin)| Self::parse_int(3, parts[3]).map(|account| (purpose, coin, account)))
      .and_then(|(purpose, coin, account)| {
        Self::parse_int(4, parts[4]).map(|change| (purpose, coin, account, change)) 
      })
      .and_then(|(purpose, coin, account, change)| {
        Self::parse_int(5, parts[5]).map(|address| (purpose, coin, account, change, address))
      })
      .map(|(purpose, coin, account, change, address)| GenericKeyPath { purpose, coin, account, change, address })
  }

  pub fn to_string(&self) -> String {
    self.into()
  }
}

impl KeyPath for GenericKeyPath {
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

impl<'a> From<&'a GenericKeyPath> for String {
  fn from(path: &'a GenericKeyPath) -> Self {
    format!(
      "m/{}/{}/{}/{}/{}",
      GenericKeyPath::print_int(path.purpose()),
      GenericKeyPath::print_int(path.coin()),
      GenericKeyPath::print_int(path.account()),
      GenericKeyPath::print_int(path.change()),
      GenericKeyPath::print_int(path.address())
    )
  }
}