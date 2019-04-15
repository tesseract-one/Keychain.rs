use serde_json;
use std::collections::HashMap;

use network::Network;

pub type Error = serde_json::error::Error;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
enum Version {
  V1 = 1
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WalletDataV1 {
  #[serde(serialize_with = "serialize::se_key_map", deserialize_with = "serialize::de_key_map")]
  pub keys: HashMap<Network, Vec<u8>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionedData {
  version: Version,
  #[serde(with = "serialize::Base64")]
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

// Custom data serializaion/deserialization methods
mod serialize {
  use serde::{ Serializer, Deserialize, Deserializer };
  use std::collections::HashMap;
  use network::Network;

  base64_serde_type!(pub Base64, base64::STANDARD);
  
  pub fn se_key_map<S: Serializer>(keys: &HashMap<Network, Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error> {
    #[derive(Serialize)]
    struct Wrapper<'a>(#[serde(with = "Base64")] &'a Vec<u8>);

    let map = keys.iter().map(|(k, v)| (k, Wrapper(v)));
    serializer.collect_seq(map)
  }
  
  pub fn de_key_map<'de, D: Deserializer<'de>>(deserializer: D) -> Result<HashMap<Network, Vec<u8>>, D::Error> {
    #[derive(Deserialize)]
    struct Wrapper(#[serde(with = "Base64")] Vec<u8>);

    let mut map: HashMap<Network, Vec<u8>> = HashMap::new();
    for (net, Wrapper(key)) in Vec::<(Network, Wrapper)>::deserialize(deserializer)? {
      map.insert(net, key);
    }
    Ok(map)
  }
}
