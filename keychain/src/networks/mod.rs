#[cfg(feature = "cardano")]
pub mod cardano;

#[cfg(feature = "bitcoin")]
pub mod bitcoin;

#[cfg(feature = "ethereum")]
pub mod ethereum;

use super::key_factory::KeyFactory;

pub fn all_networks<'a>() -> Vec<Box<dyn KeyFactory>> {
  let mut networks: Vec<Box<dyn KeyFactory>> = Vec::new();
  #[cfg(feature = "cardano")]
  {
    networks.push(cardano::KeyFactory::new().boxed());
  }
  #[cfg(feature = "ethereum")]
  {
    networks.push(ethereum::KeyFactory::new().boxed());
  }
  #[cfg(feature = "bitcoin")]
  {
    networks.push(bitcoin::KeyFactory::new().boxed());
  }
  networks
}
