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
  fn network(&self) -> Network;

  fn seed_size(&self) -> SeedSize;

  fn key_from_data(&self, data: &[u8]) -> Result<Box<dyn Key>, KeyError>;

  fn key_data_from_seed(&self, seed: &[u8]) -> Result<Vec<u8>, KeyError>;

  fn boxed(self) -> Box<Self>
  where
    Self: Sized
  {
    Box::new(self)
  }
}
