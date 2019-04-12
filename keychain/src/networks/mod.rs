
// #[cfg(feature = "cardano")]
// pub mod cardano;

// #[cfg(feature = "bitcoin")]
// pub mod bitcoin;

// #[cfg(feature = "ethereum")]
// pub mod ethereum;

use super::key_factory::KeychainKeyFactory;

pub fn all_networks<'a>() -> Vec<Box<KeychainKeyFactory>> {
  let mut networks: Vec<Box<KeychainKeyFactory>> = Vec::new();
  // #[cfg(feature = "cardano")]
  // {
  //   networks.push(cardano::Network::boxed());
  // }
  networks
}