use serde_json;
use std::collections::HashMap;

use network::Network;

pub type Error = serde_json::error::Error;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
enum Version {
  V1 = 1
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WalletDataV1 {
  pub keys: HashMap<Network, Vec<u8>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionedData {
  version: Version,
  data: Vec<u8>
}

impl VersionedData {
  pub fn new(v1: &WalletDataV1) -> Result<Self, Error> {
    serde_json::to_vec(v1).map(|data| Self { version: Version::V1, data })
  }

  pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
    serde_json::from_slice(bytes)
  }

  pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
    serde_json::to_vec(self)
  }

  pub fn get_data(&self) -> Result<WalletDataV1, Error> {
    match self.version {
      Version::V1 => serde_json::from_slice(self.data.as_slice())
    }
  }
}

