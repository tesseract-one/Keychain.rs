
#[cfg(feature = "cardano")]
pub mod cardano;

#[cfg(feature = "bitcoin")]
pub mod bitcoin;

#[cfg(feature = "ethereum")]
pub mod ethereum;

use super::key_factory::KeyFactory;

pub fn all_networks<'a>() -> Vec<Box<KeyFactory>> {
  let mut networks: Vec<Box<KeyFactory>> = Vec::new();
  #[cfg(feature = "cardano")]
  {
    networks.push(cardano::KeyFactory::boxed());
  }
  #[cfg(feature = "ethereum")]
  {
    networks.push(ethereum::KeyFactory::boxed());
  }
  #[cfg(feature = "bitcoin")]
  {
    networks.push(bitcoin::KeyFactory::boxed());
  }
  networks
}