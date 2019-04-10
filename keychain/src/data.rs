use serde_cbor;
use super::key_storage::KeyStorage;

pub type Error = serde_cbor::error::Error;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
enum Version {
  V1
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WalletDataV1 {
  pub private_keys: KeyStorage
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionedData {
  version: Version,
  data: Vec<u8>
}

impl VersionedData {
  pub fn new(v1: &WalletDataV1) -> Result<Self, Error> {
    serde_cbor::to_vec(v1).map(|data| Self { version: Version::V1, data })
  }

  pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
    serde_cbor::from_slice(bytes)
  }

  pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
    serde_cbor::to_vec(self)
  }

  pub fn get_data(&self) -> Result<WalletDataV1, Error> {
    match self.version {
      Version::V1 => serde_cbor::from_slice(self.data.as_slice())
    }
  }
}

