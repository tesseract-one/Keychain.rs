use network::{ NetworkType, Network };
use key_storage::KeyStorage;
use network::PrivateKey;
use key_path::Bip44_KeyPath;
use std::collections::HashMap;
use provider::Networks;

pub struct HDWallet {
  name: String,
  private_keys: HashMap<NetworkType, Box<PrivateKey>>,
}

impl HDWallet {
  pub fn new(name: &str, key_storage: KeyStorage, networks: Networks) -> Self {
    let private_keys = key_storage
      .keys()
      .into_iter()
      .filter(|nt| { networks.contains_key(&nt.0) })
      .map(|nt| { 
        (nt.0, networks.get(&nt.0).unwrap().key_from_data(&nt.1).unwrap())
      }).collect();
    Self { name: String::from(name), private_keys }
  }

  pub fn supported_networks(&self) -> Vec<NetworkType> {
    self.private_keys.keys().cloned().collect()
  }

  pub fn pub_key(&self, network: &NetworkType, path: &Bip44_KeyPath) -> Option<Vec<u8>> {
    self.private_keys.get(network).map(|pk| { pk.pub_key(path).unwrap() })
  }

  pub fn sign(&self, network: &NetworkType, data: &[u8], path: &Bip44_KeyPath) -> Option<Vec<u8>> {
    self.private_keys.get(network).map(|pk| { pk.sign(data, path).unwrap() })
  }
}
