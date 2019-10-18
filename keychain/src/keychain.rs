use error::Error;
use key::Key;
use key_path::KeyPath;
use network::Network;
use std::collections::HashMap;

pub struct Keychain {
  keys: HashMap<Network, Box<dyn Key>>
}

impl Keychain {
  pub fn new(keys: Vec<Box<dyn Key>>) -> Self {
    let converted: HashMap<Network, Box<dyn Key>> =
      keys.into_iter().map(|key| (key.network(), key)).collect();
    Keychain { keys: converted }
  }

  pub fn has_network(&self, net: &Network) -> bool {
    self.keys.get(net).is_some()
  }

  pub fn networks(&self) -> Vec<Network> {
    self.keys.keys().cloned().collect()
  }

  pub fn pub_key(&self, network: &Network, path: &dyn KeyPath) -> Result<Vec<u8>, Error> {
    self._pk(network)?.pub_key(path).map_err(|err| Error::from_key_error(network, err))
  }

  pub fn sign(&self, network: &Network, data: &[u8], path: &dyn KeyPath) -> Result<Vec<u8>, Error> {
    self._pk(network)?.sign(data, path).map_err(|err| Error::from_key_error(network, err))
  }

  pub fn verify(
    &self, network: &Network, data: &[u8], signature: &[u8], path: &dyn KeyPath
  ) -> Result<bool, Error> {
    self
      ._pk(network)?
      .verify(data, signature, path)
      .map_err(|err| Error::from_key_error(network, err))
  }
}

impl Keychain {
  fn _pk<'a>(&'a self, network: &Network) -> Result<&'a dyn Key, Error> {
    self.keys.get(network)
      .map(|key| key.as_ref())
      .ok_or_else(|| Error::KeyDoesNotExist(network.clone()))
  }
}
