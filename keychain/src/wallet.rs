use std::fmt;
use network_type::NetworkType;
use network::Network;
use key_storage::KeyStorage;
use private_key::{ Error as PrivateKeyError, PrivateKey };
use key_path::KeyPath;
use std::collections::HashMap;
use provider::Networks;

#[derive(Debug)]
pub enum Error {
  KeyNotFound(NetworkType),
  KeyError(NetworkType, PrivateKeyError)
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &Error::KeyNotFound(nt) => write!(f, "Don't have key for network: {}", nt),
      &Error::KeyError(nt, ref err) => write!(f, "Key error: {} for network: {}", err, nt)
    }
  }
}

impl std::error::Error for Error {}

pub struct HDWallet {
  name: String,
  private_keys: HashMap<NetworkType, Box<PrivateKey>>,
}

impl HDWallet {
  pub fn new(name: &str, key_storage: KeyStorage, networks: Networks) -> Result<Self, Error> {
    let start: Result<Vec<(NetworkType, Box<PrivateKey>)>, Error> = Ok(Vec::with_capacity(networks.len()));
    key_storage.keys()
      .into_iter()
      .filter(|(nt, _)| { networks.contains_key(nt) })
      .fold(start, |res, (nt, data)| {
        match res {
          Err(err) => Err(err),
          Ok(mut vec) => networks.get(&nt).unwrap().key_from_data(&data)
            .map(|data| {
              vec.push((nt, data));
              vec
            })
            .map_err(|err| Error::KeyError(nt, err))
        }
      })
      .map(|keys| Self { name: name.to_owned(), private_keys: keys.into_iter().collect() } )
  }

  pub fn key_storage_for_mnemonic(mnemonic: &str, networks: &HashMap<NetworkType, Box<Network>>) -> Result<KeyStorage, Error> {
    let start: Result<Vec<(NetworkType, Vec<u8>)>, Error> = Ok(Vec::with_capacity(networks.len()));
    networks.into_iter()
      .fold(start, |res, (nt, network)| {
        match res {
          Err(err) => Err(err),
          Ok(mut vec) => network.key_data_from_mnemonic(mnemonic)
            .map(|data| {
              vec.push((*nt, data));
              vec
            })
            .map_err(|err| Error::KeyError(*nt, err))
        }
      })
      .map(|keys| KeyStorage::new(&keys))
  }

  pub fn name(&self) -> String {
    self.name.clone()
  }

  pub fn supported_networks(&self) -> Vec<NetworkType> {
    self.private_keys.keys().cloned().collect()
  }

  pub fn pub_key(&self, network: &NetworkType, path: &KeyPath) -> Result<Vec<u8>, Error> {
    match self.private_keys.get(network) {
      None => Err(Error::KeyNotFound(*network)),
      Some(pk) => pk.pub_key(path).map_err(|err| Error::KeyError(*network, err))
    }
  }

  pub fn verify(&self, network: &NetworkType, data: &[u8], signature: &[u8], path: &KeyPath) -> Result<bool, Error> {
    match self.private_keys.get(network) {
      None => Err(Error::KeyNotFound(*network)),
      Some(pk) => pk.verify(data, signature, path).map_err(|err| Error::KeyError(*network, err))
    }
  }

  pub fn sign(&self, network: &NetworkType, data: &[u8], path: &KeyPath) -> Result<Vec<u8>, Error> {
    match self.private_keys.get(network) {
      None => Err(Error::KeyNotFound(*network)),
      Some(pk) => pk.sign(data, path).map_err(|err| Error::KeyError(*network, err))
    }
  }
}
