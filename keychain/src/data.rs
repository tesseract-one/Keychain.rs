use crate::mnemonic::Language;
use serde_json;
use std::collections::HashMap;

use crate::network::Network;

pub type Error = serde_json::error::Error;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
enum Version {
  V1 = 1,
  V2 = 2
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WalletDataV1 {
  #[serde(serialize_with = "serialize::se_key_map", deserialize_with = "serialize::de_key_map")]
  pub keys: HashMap<Network, Vec<u8>>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WalletDataV2 {
  #[serde(serialize_with = "serialize::se_opt_vec", deserialize_with = "serialize::de_opt_vec")]
  pub seed: Option<Vec<u8>>,
  pub mnemonic: Option<String>,
  pub dictionary: Option<Language>,
  #[serde(serialize_with = "serialize::se_key_map", deserialize_with = "serialize::de_key_map")]
  pub keys: HashMap<Network, Vec<u8>>
}

impl WalletDataV2 {
  fn from_v1(v1: &WalletDataV1) -> Result<Self, Error> {
    Ok(Self { seed: None, mnemonic: None, dictionary: None, keys: v1.keys.clone() })
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionedData {
  version: Version,
  #[serde(with = "serialize::Base64")]
  data: Vec<u8>
}

impl VersionedData {
  pub fn new(v2: &WalletDataV2) -> Result<Self, Error> {
    serde_json::to_vec(v2).map(|data| Self { version: Version::V2, data })
  }

  pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
    serde_json::from_slice(bytes)
  }

  pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
    serde_json::to_vec(self)
  }

  pub fn get_data(&self) -> Result<WalletDataV2, Error> {
    match self.version {
      Version::V1 => {
        let v1: WalletDataV1 = serde_json::from_slice(self.data.as_slice())?;
        WalletDataV2::from_v1(&v1)
      }
      Version::V2 => serde_json::from_slice(self.data.as_slice())
    }
  }
}

// Custom data serializaion/deserialization methods
mod serialize {
  use crate::network::Network;
  use serde::{Deserialize, Deserializer, Serialize, Serializer};
  use std::collections::HashMap;

  base64_serde_type!(pub Base64, base64::STANDARD);

  #[derive(Serialize)]
  struct SeVecWrapper<'a>(#[serde(with = "Base64")] &'a Vec<u8>);

  #[derive(Deserialize)]
  struct DeVecWrapper(#[serde(with = "Base64")] Vec<u8>);

  pub fn se_opt_vec<S: Serializer>(
    option: &Option<Vec<u8>>, serializer: S
  ) -> Result<S::Ok, S::Error> {
    option.as_ref().map(|vec| SeVecWrapper(vec.as_ref())).serialize(serializer)
  }

  pub fn de_opt_vec<'de, D: Deserializer<'de>>(
    deserializer: D
  ) -> Result<Option<Vec<u8>>, D::Error> {
    Option::<DeVecWrapper>::deserialize(deserializer).map(|wrapped| wrapped.map(|w| w.0))
  }

  pub fn se_key_map<S: Serializer>(
    keys: &HashMap<Network, Vec<u8>>, serializer: S
  ) -> Result<S::Ok, S::Error> {
    let map = keys.iter().map(|(k, v)| (k, SeVecWrapper(v)));
    serializer.collect_seq(map)
  }

  pub fn de_key_map<'de, D: Deserializer<'de>>(
    deserializer: D
  ) -> Result<HashMap<Network, Vec<u8>>, D::Error> {
    let mut map: HashMap<Network, Vec<u8>> = HashMap::new();
    for (net, DeVecWrapper(key)) in Vec::<(Network, DeVecWrapper)>::deserialize(deserializer)? {
      map.insert(net, key);
    }
    Ok(map)
  }
}
