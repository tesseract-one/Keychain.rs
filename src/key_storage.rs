use std::collections::HashMap;
use super::network::NetworkType;

#[derive(Serialize, Deserialize, Clone)]
pub struct KeyStorage {
  keys: HashMap<NetworkType, Vec<u8>>
}

impl KeyStorage {
  pub fn get_saved_networks(&self) -> Vec<NetworkType> {
    self.keys.keys().cloned().collect()
  }

  pub fn key(&self, network: &NetworkType) -> Option<Vec<u8>> {
    self.keys.get(network).cloned()
  }

  pub fn keys(&self) -> Vec<(NetworkType, Vec<u8>)> {
    self.keys.into_iter().collect()
  }
}