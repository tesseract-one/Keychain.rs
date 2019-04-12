use std::fmt;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Network {
  pub code: u32
}

impl Network {
  pub fn all() -> Vec<Network> {
    let mut types: Vec<Network> = Vec::new();
    #[cfg(feature = "cardano")]
    {
      types.push(Network::CARDANO);
    }
    #[cfg(feature = "ethereum")]
    {
      types.push(Network::ETHEREUM);
    }
    types
  }
}

impl fmt::Display for Network {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return write!(f, "Network({})", self.code);
  }
}