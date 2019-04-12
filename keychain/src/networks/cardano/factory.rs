use key_factory::{ KeyFactory as IKeyFactory, SeedSize };
use key::{ Key as IKey, Error as KeyError };
use bip39::Seed;
use super::key::Key;
use network::Network;

pub struct KeyFactory;

impl IKeyFactory for KeyFactory {
  fn new() -> Self {
    Self {}
  }

  fn network(&self) -> Network {
    Network::CARDANO
  }

  fn seed_size(&self) -> SeedSize {
    SeedSize { min: 96, max: 256 }
  }

  fn key_from_data(&self, data: &[u8]) -> Result<Box<IKey>, KeyError> {
    Key::from_data(data).map(|pk| pk.boxed())
  }

  fn key_data_from_seed(&self, seed: &[u8]) -> Result<Vec<u8>, KeyError> {
    Seed::from_slice(seed)
      .map_err(|err| KeyError::InvalidMnemonic(err.into()) )
      .and_then(|seed| Key::data_from_seed(&seed))
  }
}