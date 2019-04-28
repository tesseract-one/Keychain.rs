use error::Error;
use key::Key;
use key_path::KeyPath;
use network::Network;
use std::collections::HashMap;

pub struct Keychain {
  keys: HashMap<Network, Box<Key>>
}

impl Keychain {
  pub fn new(keys: Vec<Box<Key>>) -> Self {
    let converted: HashMap<Network, Box<Key>> =
      keys.into_iter().map(|key| (key.network(), key)).collect();
    Keychain { keys: converted }
  }

  pub fn networks(&self) -> Vec<Network> {
    self.keys.keys().cloned().collect()
  }

  pub fn pub_key(&self, network: &Network, path: &KeyPath) -> Result<Vec<u8>, Error> {
    self._pk(network)?.pub_key(path).map_err(|err| Error::from_key_error(network, err))
  }

  pub fn sign(&self, network: &Network, data: &[u8], path: &KeyPath) -> Result<Vec<u8>, Error> {
    self._pk(network)?.sign(data, path).map_err(|err| Error::from_key_error(network, err))
  }

  pub fn verify(
    &self, network: &Network, data: &[u8], signature: &[u8], path: &KeyPath
  ) -> Result<bool, Error> {
    self
      ._pk(network)?
      .verify(data, signature, path)
      .map_err(|err| Error::from_key_error(network, err))
  }
}

impl Keychain {
  fn _pk<'a>(&'a self, network: &Network) -> Result<(&'a Key), Error> {
    match self.keys.get(network) {
      None => Err(Error::KeyDoesNotExist(network.clone())),
      Some(key) => Ok(key.as_ref())
    }
  }
}
