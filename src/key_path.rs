// special number for BIP44 purpose
pub const BIP44_PURPOSE: u32 = 0x8000002C;

// the soft derivation is upper bounded
pub const BIP44_SOFT_UPPER_BOUND: u32 = 0x80000000;

// length of bip44
pub const BIP44_PARTS_COUNT: usize = 6;

#[derive(Debug, Clone)]
pub enum Error {
  WrongPartsCount,
  WrongPurpose,
  WrongMarker,
  WrongCoin,
  WrongChange,
  EmptyValueAtIndex(usize),
  ParseErrorAtIndex(usize, std::num::ParseIntError),
  NonHardenedValueAtIndex(usize),
  HardenedValueAtIndex(usize)
}

pub trait Bip44_KeyPath {
  fn purpose(&self) -> u32 {
    BIP44_PURPOSE
  }

  fn coin(&self) -> u32;
  fn account(&self) -> u32;
  fn change(&self) -> u32;
  fn address(&self) -> u32;
}

pub struct Bip_44 {
  coin: u32,
  account: u32,
  change: u32,
  address: u32
}

impl Bip_44 {
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
    str::parse::<u32>(s)
      .map_err(|err| { Error::ParseErrorAtIndex(index, err) })
      .and_then(|val| {
        if val >= BIP44_SOFT_UPPER_BOUND { Err(Error::HardenedValueAtIndex(index)) } else { Ok(val) }
      })
  }

  pub fn from(path: &str) -> Result<Self, Error> {
    let parts: Vec<&str> = path.split("/").map(|s| { s.trim() }).collect();
    if parts.len() != BIP44_PARTS_COUNT {
      return Err(Error::WrongPurpose);
    }
    if parts[0] != "m" {
      return Err(Error::WrongMarker);
    }
    let purpose = Self::hard_int(1, parts[1]);
    if let Err(err) = purpose {
      return Err(err);
    }
    if purpose.unwrap() != BIP44_PURPOSE {
      return Err(Error::WrongPurpose);
    }
    Self::hard_int(2, parts[2])
      .and_then(|coin| Self::hard_int(3, parts[3]).map(|account| (coin, account)))
      .and_then(|(coin, account)| {
        Self::soft_int(4, parts[4]).map(|change| (coin, account, change)) 
      })
      .and_then(|(coin, account, change)| {
        Self::soft_int(5, parts[5]).map(|address| (coin, account, change, address))
      })
      .map(|(coin, account, change, address)| Bip_44 { coin, account, change, address })
  }
}

impl Bip44_KeyPath for Bip_44 {
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

impl<'a> From<&'a Bip44_KeyPath> for String {
  fn from(path: &'a Bip44_KeyPath) -> Self {
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