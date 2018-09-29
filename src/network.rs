use network_type::NetworkType;
use private_key::{ PrivateKey, Error as PrivateKeyError };

#[derive(Debug, Copy, Clone)]
pub struct SeedSize {
  pub min: usize,
  pub max: usize
}

impl SeedSize {
  pub fn min_words(&self) -> usize {
    self.min / 32 * 3
  }

  pub fn max_words(&self) -> usize {
    self.max / 32 * 3
  }
}

pub trait Network {
  fn new() -> Self where Self: Sized;

  fn get_type(&self) -> NetworkType;

  fn get_seed_size(&self) -> SeedSize;

  fn key_from_data(&self, data: &[u8]) -> Result<Box<PrivateKey>, PrivateKeyError>;

  fn key_data_from_mnemonic(&self, mnemonic: &str) -> Result<Vec<u8>, PrivateKeyError>;

  fn boxed() -> Box<Self> where Self: Sized {
    Box::new(Self::new())
  }
}
