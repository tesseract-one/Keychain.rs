use std::collections::HashMap;

use crypt;
use data::{VersionedData, WalletDataV2};
use entropy::{Entropy, OsEntropy};
use error::Error;
use key::Key;
use key_factory::KeyFactory;
use keychain::Keychain;
use mnemonic::{generate_entropy, mnemonic_from_entropy, seed_from_mnemonic, Language, SEED_SIZE};
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
    generate_entropy(self.seed_size, self.random.as_ref())
      .and_then(|ent| mnemonic_from_entropy(&ent, language.unwrap_or_default()))
      .map_err(|err| err.into())
  }

  pub fn keychain_data_from_seed(&self, seed: &[u8], password: &str) -> Result<Vec<u8>, Error> {
    self.new_keychain_data(Some(seed), password, None, None)
  }

  pub fn keychain_data_from_mnemonic(
    &self, mnemonic: &str, password: &str, language: Option<Language>
  ) -> Result<Vec<u8>, Error> {
    let lang = language.unwrap_or_default();
    self.new_keychain_data(None, password, Some(mnemonic), Some(lang))
  }

  pub fn keychain_from_data(&self, data: &[u8], password: &str) -> Result<Keychain, Error> {
    let v2 = Self::keychain_data_from_bytes(data, password)?;
    let keys = v2.keys.into_iter().fold(
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

  pub fn add_network(
    &self, encrypted: &[u8], password: &str, network: Network
  ) -> Result<Vec<u8>, Error> {
    let factory = self.factories.get(&network).ok_or(Error::NetworkIsNotSupported(network))?;
    let mut data = Self::keychain_data_from_bytes(encrypted, password)?;
    if data.keys.get(&network).is_some() {
      return Err(Error::KeyAlreadyExist(network));
    }

    let seed = self.seed_from_data(
      data.seed.as_ref().map(|s| s.as_ref()),
      data.mnemonic.as_ref().map(|m| m.as_ref()),
      data.dictionary
    )?;

    let key_data =
      factory.key_data_from_seed(&seed).map_err(|err| Error::from_key_error(&network, err))?;

    data.keys.insert(network, key_data);

    let bytes =
      VersionedData::new(&data).and_then(|data| data.to_bytes()).map_err(|err| Error::from(err))?;

    Ok(crypt::encrypt(&bytes, password, self.random.as_ref()))
  }

  #[cfg(feature = "backup")]
  pub fn retrieve_mnemonic(
    &self, encrypted: &[u8], password: &str
  ) -> Result<(String, Language), Error> {
    let data = Self::keychain_data_from_bytes(encrypted, password)?;
    let mnemonic = data.mnemonic.ok_or(Error::SeedIsNotSaved)?;
    let lang = data.dictionary.ok_or(Error::SeedIsNotSaved)?;
    Ok((mnemonic, lang))
  }

  #[cfg(feature = "backup")]
  pub fn get_keys_data(
    &self, encrypted: &[u8], password: &str
  ) -> Result<Vec<(Network, Vec<u8>)>, Error> {
    let v2 = Self::keychain_data_from_bytes(encrypted, password)?;
    Ok(v2.keys.into_iter().collect())
  }
}

// Private methods
impl KeychainManager {
  fn with_factory_objs(factories: Vec<Box<KeyFactory>>) -> Result<Self, Error> {
    let seed_size = Self::calculate_seed_size(&factories)?;
    let random = OsEntropy::new();
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

  fn seed_from_data(
    &self, seed: Option<&[u8]>, mnemonic: Option<&str>, lang: Option<Language>
  ) -> Result<Vec<u8>, Error> {
    match seed {
      Some(seed) => {
        if seed.len() == SEED_SIZE {
          Ok(Vec::from(seed))
        } else {
          Err(Error::InvalidSeedSize(seed.len()))
        }
      }
      None => {
        let mnem = mnemonic.ok_or(Error::SeedIsNotSaved)?;
        let lang = lang.ok_or(Error::SeedIsNotSaved)?;
        seed_from_mnemonic(mnem, "", self.seed_size, lang).map_err(|err| Error::from(err))
      }
    }
  }

  fn new_keychain_data(
    &self, seed: Option<&[u8]>, password: &str, mnemonic: Option<&str>, lang: Option<Language>
  ) -> Result<Vec<u8>, Error> {
    let calculated_seed = self.seed_from_data(seed, mnemonic, lang)?;

    let pkeys = self.factories.values().fold(
      Ok(Vec::new()),
      |res: Result<Vec<(Network, Vec<u8>)>, Error>, fact| {
        let mut vec = res?;
        let data = fact
          .key_data_from_seed(&calculated_seed)
          .map_err(|err| Error::from_key_error(&fact.network(), err))?;
        vec.push((fact.network(), data));
        Ok(vec)
      }
    )?;

    let data = WalletDataV2 {
      seed: seed.map(|s| Vec::from(s)),
      mnemonic: mnemonic.map(|m| m.to_owned()),
      dictionary: lang,
      keys: pkeys.into_iter().collect()
    };

    let bytes =
      VersionedData::new(&data).and_then(|data| data.to_bytes()).map_err(|err| Error::from(err))?;

    Ok(crypt::encrypt(&bytes, password, self.random.as_ref()))
  }

  fn keychain_data_from_bytes(bytes: &[u8], password: &str) -> Result<WalletDataV2, Error> {
    let decrypted = crypt::decrypt(bytes, password).map_err(|err| Error::from(err))?;
    VersionedData::from_bytes(&decrypted)
      .and_then(|data| data.get_data())
      .map_err(|err| Error::from(err))
  }
}
