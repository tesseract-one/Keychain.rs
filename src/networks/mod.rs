
#[cfg(feature = "cardano")]
pub mod cardano;

use super::Network;

pub fn all_networks<'a>() -> Vec<Box<Network>> {
  let mut networks: Vec<Box<Network>> = Vec::new();
  #[cfg(feature = "cardano")]
  {
    networks.push(cardano::Network::boxed());
  }
  networks
}