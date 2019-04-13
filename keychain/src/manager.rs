use std::collections::HashMap;

use error::Error;
use network::Network;
use key::Key;
use keychain::Keychain;
use key_factory::KeyFactory;
use networks::all_networks;
use entropy::{ Entropy, OsEntropy };
use mnemonic::{ generate as generate_mnemonic, Language, seed_from_mnemonic };
use crypt;
use data::{ VersionedData, WalletDataV1 };

pub struct KeychainManager {
  factories: HashMap<Network, Box<KeyFactory>>,
  random: Box<Entropy>,
  seed_size: usize
}

impl KeychainManager {
  pub fn new() -> Result<Self, Error> {
    Self::with_factory_objs(all_networks())
  }

  pub fn with_networks(networks: &[Network]) -> Result<Self, Error> {
    let filtered: Vec<Box<KeyFactory>> = all_networks()
      .into_iter()
      .filter(|network| {
        let ntype = network.network();
        networks.iter().position(|nt| nt == &ntype).is_some()
      })
      .collect();
    Self::with_factory_objs(filtered)
  }

  #[cfg(feature = "custom-networks")]
  pub fn with_factories(
    factories: Vec<Box<KeyFactory>>
  ) -> Result<Self, Error> {
    Self::with_factory_objs(factories)
  }

  pub fn has_network(&self, nt: &Network) -> bool {
    self.factories.contains_key(nt)
  }

  pub fn get_key_factory<'a>(&'a self, nt: &Network) -> Option<&'a KeyFactory> {
    self.factories.get(nt).map(|n| n.as_ref())
  }

  pub fn generate_mnemonic(&self, language: Option<Language>) -> Result<String, Error> {
    generate_mnemonic(self.seed_size, language.unwrap_or_default(), self.random.as_ref())
      .map_err(|err| { err.into() })
  }

  pub fn keychain_from_seed(&self, seed: &[u8], password: &str) -> Result<(Keychain, Vec<u8>), Error> {
    if seed.len() != 64 {
      return Err(Error::InvalidSeedSize(seed.len()))
    }
    self.factories.values()
      .fold(Ok(Vec::new()), |res, fact| {
        res
          .and_then(|vec| {
            fact.key_data_from_seed(&seed)
              .map_err(|err| { Error::from_key_error(&fact.network(), err) })
              .map(|data| { (vec, data) }) }
          )
          .and_then(|(mut vec, data)| {
            fact.key_from_data(&data)
              .map_err(|err| { Error::from_key_error(&fact.network(), err) })
              .map(|key| {
                vec.push((fact.network(), key, data));
                vec
              })
          })
      })
      .and_then(|pkeys| {
        let mut keys: Vec<Box<Key>> = Vec::new();
        let mut data: HashMap<Network, Vec<u8>> = HashMap::new();
        for (network, key, key_data) in pkeys {
          keys.push(key);
          data.insert(network, key_data);
        }
        let keychain = Keychain::new(keys);
        VersionedData::new(&WalletDataV1 { keys: data })
          .and_then(|data| { data.to_bytes() })
          .map_err(|err| { err.into() })
          .map(|data| { (keychain, crypt::encrypt(&data, password, self.random.as_ref())) })
      })
  }

  pub fn keychain_from_mnemonic(&self, mnemonic: &str, password: &str, language: Option<Language>) -> Result<(Keychain, Vec<u8>), Error> {
    seed_from_mnemonic(mnemonic, "", self.seed_size, language.unwrap_or_default())
      .map_err(|err| { err.into() })
      .and_then(|seed| { self.keychain_from_seed(&seed, password) })
  }

  pub fn keychain_from_data(&self, data: &[u8], password: &str) -> Result<Keychain, Error> {
    crypt::decrypt(data, password)
      .map_err(|err| { err.into() })
      .and_then(|data| {
        VersionedData::from_bytes(&data)
          .and_then(|data| { data.get_data() })
          .map_err(|err| { err.into() })
      })
      .and_then(|data| {
        data.keys.into_iter()
          .fold(Ok(Vec::new()), |res, (network, key)| {
            res.and_then(|mut vec| {
              match self.factories.get(&network) {
                None => Ok(vec),
                Some(factory) =>
                  factory.key_from_data(&key)
                    .map_err(|err| Error::from_key_error(&network, err) )
                    .map(|key| {
                      vec.push(key);
                      vec
                    })
              }
            })
          })
      })
      .map(|keys| Keychain::new(keys))
  }

  pub fn change_password(&self, encrypted: &[u8], old_password: &str, new_password: &str) -> Result<Vec<u8>, Error> {
    crypt::decrypt(encrypted, old_password)
      .map_err(|err| { err.into() })
      .map(|data| { crypt::encrypt(&data, new_password, self.random.as_ref()) })
  }

  #[cfg(feature = "backup")]
  pub fn get_keys_data(&self, encrypted: &[u8], password: &str) -> Result<Vec<(Network, Vec<u8>)>, Error> {
    crypt::decrypt(encrypted, password)
      .map_err(|err| { err.into() })
      .and_then(|data| {
        VersionedData::from_bytes(&data)
          .and_then(|data| { data.get_data() })
          .map_err(|err| { err.into() })
      })
      .map(|data| { data.keys.into_iter().collect() })
  }
}

// Private methods
impl KeychainManager {
  fn with_factory_objs(factories: Vec<Box<KeyFactory>>) -> Result<Self, Error> {
    Self::calculate_seed_size(&factories)
      .and_then(|seed| OsEntropy::new().map(|rnd| (seed, rnd)).map_err(|err| Error::EntropyGeneratorError(err)))
      .map(|(seed_size, random)| {
        let map: HashMap<Network, Box<KeyFactory>> =
          factories.into_iter().map(|ft| { (ft.network(), ft) }).collect();
        Self { 
          seed_size,
          random: Box::new(random),
          factories: map
        }
      })
  }

  fn calculate_seed_size(factories: &[Box<KeyFactory>]) -> Result<usize, Error> {
    let mut min = 0;
    let mut max = std::usize::MAX;
    for factory in factories.into_iter() {
      let ssize = factory.seed_size();
      min = min.max(ssize.min);
      max = max.min(ssize.max);
    }
    if min == 0 {
      return Err(Error::CantCalculateSeedSize(min, max));
    }
    if max >= min { Ok(min) } else { Err(Error::CantCalculateSeedSize(min, max)) }
  }
}