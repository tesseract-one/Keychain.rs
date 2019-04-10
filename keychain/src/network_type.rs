use std::fmt;

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

impl fmt::Display for NetworkType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    #[cfg(feature = "cardano")]
    {
      if self == &NetworkType::Cardano {
        return write!(f, "Cardano");
      }
    }
    #[cfg(feature = "ethereum")]
    {
      if self == &NetworkType::Ethereum {
        return write!(f, "Ethereum");
      }
    }
    #[cfg(feature = "custom-networks")]
    {
      if let &NetworkType::Custom(id) = self {
        return write!(f, "Custom({})", id);
      }
    }
    Err(fmt::Error {})
  }
}