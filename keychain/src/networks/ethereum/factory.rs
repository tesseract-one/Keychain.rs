use super::key::Key;
use bip39::Seed;
use key::{Error as KeyError, Key as IKey};
use key_factory::{KeyFactory as IKeyFactory, SeedSize};
use network::Network;

pub struct KeyFactory;

impl KeyFactory {
  pub fn new() -> Self {
    Self {}
  }
}

impl IKeyFactory for KeyFactory {
  fn network(&self) -> Network {
    Network::ETHEREUM
  }

  fn seed_size(&self) -> SeedSize {
    SeedSize { min: 128, max: 256 }
  }

  fn key_from_data(&self, data: &[u8]) -> Result<Box<dyn IKey>, KeyError> {
    Key::from_data(data).map(|pk| -> Box<dyn IKey> { pk.boxed() })
  }

  fn key_data_from_seed(&self, seed: &[u8]) -> Result<Vec<u8>, KeyError> {
    let seed = Seed::from_slice(seed).map_err(|err| KeyError::InvalidMnemonic(err.into()))?;
    Key::data_from_seed(&seed)
  }
}
