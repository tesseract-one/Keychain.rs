
#[cfg(feature = "cardano-network")]
pub mod cardano;

use super::Network;

pub fn all_networks<'a>() -> &'a [Box<Network>] {
  let mut networks: Vec<Box<Network>> = Vec::new();
  #[cfg(feature = "cardano-network")]
  {
    networks.push(cardano::Network::boxed());
  }
  &networks
}