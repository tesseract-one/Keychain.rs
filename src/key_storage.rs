use std::collections::HashMap;
use network_type::NetworkType;

#[derive(Serialize, Deserialize, Clone)]
pub struct KeyStorage {
  keys: HashMap<NetworkType, Vec<u8>>
}

impl KeyStorage {
  pub fn new(keys: &[(NetworkType, Vec<u8>)]) -> Self {
    Self { keys: keys.into_iter().cloned().collect() }
  }

  pub fn get_saved_networks(&self) -> Vec<NetworkType> {
    self.keys.keys().cloned().collect()
  }

  pub fn key(&self, network: &NetworkType) -> Option<Vec<u8>> {
    self.keys.get(network).cloned()
  }

  pub fn keys(&self) -> Vec<(NetworkType, Vec<u8>)> {
    (&self.keys).into_iter().map(|(nt, network)| (*nt, network.clone())).collect()
  }
}