use network_type::NetworkType;
use key_storage::KeyStorage;
use network::{ PrivateKey, Error as NetworkError };
use key_path::Bip44KeyPath;
use std::collections::HashMap;
use provider::Networks;

#[derive(Debug, Clone)]
pub enum Error {
  InvalidKeyPath,
  InvalidKeySize,
  InvalidKeyData,
  MnemonicError
}

impl From<NetworkError> for Error {
  fn from(err: NetworkError) -> Self {
    match err {
      NetworkError::BadKeyData => Error::InvalidKeyData,
      NetworkError::WrongKeyPath => Error::InvalidKeyPath,
    }
  }
}

pub struct HDWallet {
  name: String,
  private_keys: HashMap<NetworkType, Box<PrivateKey>>,
}

impl HDWallet {
  pub fn new(name: &str, key_storage: KeyStorage, networks: Networks) -> Result<Self, Error> {
    let start: Result<Vec<(NetworkType, Box<PrivateKey>)>, NetworkError> = Ok(Vec::with_capacity(networks.len()));
    key_storage.keys()
      .into_iter()
      .filter(|(nt, _)| { networks.contains_key(nt) })
      .fold(start, |res, (nt, data)| {
        match res {
          Err(err) => Err(err),
          Ok(mut vec) => networks.get(&nt).unwrap().key_from_data(&data).map(|data| {
            vec.push((nt, data));
            vec
          })
        }
      })
      .map(|keys| Self { name: name.to_owned(), private_keys: keys.into_iter().collect() } )
      .map_err(|err| err.into())
  }

  pub fn name(&self) -> String {
    self.name.clone()
  }

  pub fn supported_networks(&self) -> Vec<NetworkType> {
    self.private_keys.keys().cloned().collect()
  }

  pub fn pub_key(&self, network: &NetworkType, path: &Bip44KeyPath) -> Option<Vec<u8>> {
    self.private_keys.get(network).map(|pk| { pk.pub_key(path).unwrap() })
  }

  pub fn sign(&self, network: &NetworkType, data: &[u8], path: &Bip44KeyPath) -> Option<Vec<u8>> {
    self.private_keys.get(network).map(|pk| { pk.sign(data, path).unwrap() })
  }
}
