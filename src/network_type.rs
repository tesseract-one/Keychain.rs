#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum NetworkType {
  #[cfg(feature = "cardano")]
  Cardano,
  #[cfg(feature = "ethereum")]
  Ethereum,
  #[cfg(feature = "custom-networks")]
  Custom(u32)
}

impl NetworkType {
  pub fn all() -> Vec<NetworkType> {
    let mut types: Vec<NetworkType> = Vec::new();
    #[cfg(feature = "cardano")]
    {
      types.push(NetworkType::Cardano);
    }
    #[cfg(feature = "ethereum")]
    {
      types.push(NetworkType::Ethereum);
    }
    types
  }
}