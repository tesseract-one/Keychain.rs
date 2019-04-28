use std::collections::HashMap;

use crypt;
use data::{VersionedData, WalletDataV1};
use entropy::{Entropy, OsEntropy};
use error::Error;
use key::Key;
use key_factory::KeyFactory;
use keychain::Keychain;
use mnemonic::{generate as generate_mnemonic, seed_from_mnemonic, Language, SEED_SIZE};
use network::Network;
use networks::all_networks;

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
  pub fn with_factories(factories: Vec<Box<KeyFactory>>) -> Result<Self, Error> {
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
      .map_err(|err| err.into())
  }

  pub fn keychain_from_seed(
    &self, seed: &[u8], password: &str
  ) -> Result<(Keychain, Vec<u8>), Error> {
    if seed.len() != SEED_SIZE {
      return Err(Error::InvalidSeedSize(seed.len()));
    }
    let pkeys = self.factories.values().fold(
      Ok(Vec::new()),
      |res: Result<Vec<(Network, Box<Key>, Vec<u8>)>, Error>, fact| {
        let mut vec = res?;
        let data = fact
          .key_data_from_seed(&seed)
          .map_err(|err| Error::from_key_error(&fact.network(), err))?;
        let key =
          fact.key_from_data(&data).map_err(|err| Error::from_key_error(&fact.network(), err))?;
        vec.push((fact.network(), key, data));
        Ok(vec)
      }
    )?;

    let mut keys: Vec<Box<Key>> = Vec::new();
    let mut data: HashMap<Network, Vec<u8>> = HashMap::new();
    for (network, key, key_data) in pkeys {
      keys.push(key);
      data.insert(network, key_data);
    }
    let keychain = Keychain::new(keys);

    let bytes = VersionedData::new(&WalletDataV1 { keys: data })
      .and_then(|data| data.to_bytes())
      .map_err(|err| Error::from(err))?;

    Ok((keychain, crypt::encrypt(&bytes, password, self.random.as_ref())))
  }

  pub fn keychain_from_mnemonic(
    &self, mnemonic: &str, password: &str, language: Option<Language>
  ) -> Result<(Keychain, Vec<u8>), Error> {
    seed_from_mnemonic(mnemonic, "", self.seed_size, language.unwrap_or_default())
      .map_err(|err| err.into())
      .and_then(|seed| self.keychain_from_seed(&seed, password))
  }

  pub fn keychain_from_data(&self, data: &[u8], password: &str) -> Result<Keychain, Error> {
    let decrypted = crypt::decrypt(data, password).map_err(|err| Error::from(err))?;
    let v1 = VersionedData::from_bytes(&decrypted)
      .and_then(|data| data.get_data())
      .map_err(|err| Error::from(err))?;
    let keys = v1.keys.into_iter().fold(
      Ok(Vec::new()),
      |res: Result<Vec<Box<Key>>, Error>, (network, key)| {
        let mut vec = res?;
        match self.factories.get(&network) {
          None => Ok(vec),
          Some(factory) => {
            let pk =
              factory.key_from_data(&key).map_err(|err| Error::from_key_error(&network, err))?;
            vec.push(pk);
            Ok(vec)
          }
        }
      }
    )?;
    Ok(Keychain::new(keys))
  }

  pub fn change_password(
    &self, encrypted: &[u8], old_password: &str, new_password: &str
  ) -> Result<Vec<u8>, Error> {
    let decrypted = crypt::decrypt(encrypted, old_password).map_err(|err| Error::from(err))?;
    Ok(crypt::encrypt(&decrypted, new_password, self.random.as_ref()))
  }

  #[cfg(feature = "backup")]
  pub fn get_keys_data(
    &self, encrypted: &[u8], password: &str
  ) -> Result<Vec<(Network, Vec<u8>)>, Error> {
    let decrypted = crypt::decrypt(encrypted, password).map_err(|err| Error::from(err))?;
    let v1 = VersionedData::from_bytes(&decrypted)
      .and_then(|data| data.get_data())
      .map_err(|err| Error::from(err))?;
    Ok(v1.keys.into_iter().collect())
  }
}

// Private methods
impl KeychainManager {
  fn with_factory_objs(factories: Vec<Box<KeyFactory>>) -> Result<Self, Error> {
    let seed_size = Self::calculate_seed_size(&factories)?;
    let random = OsEntropy::new().map_err(|err| Error::from(err))?;
    let map: HashMap<Network, Box<KeyFactory>> =
      factories.into_iter().map(|ft| (ft.network(), ft)).collect();
    Ok(Self { seed_size, random: Box::new(random), factories: map })
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
    if max >= min {
      Ok(min)
    } else {
      Err(Error::CantCalculateSeedSize(min, max))
    }
  }
}
