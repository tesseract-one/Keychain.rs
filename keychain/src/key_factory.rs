use key::{Error as KeyError, Key};
use network::Network;

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

pub trait KeyFactory {
  fn new() -> Self
  where
    Self: Sized;

  fn network(&self) -> Network;

  fn seed_size(&self) -> SeedSize;

  fn key_from_data(&self, data: &[u8]) -> Result<Box<Key>, KeyError>;

  fn key_data_from_seed(&self, seed: &[u8]) -> Result<Vec<u8>, KeyError>;

  fn boxed() -> Box<Self>
  where
    Self: Sized
  {
    Box::new(Self::new())
  }
}
