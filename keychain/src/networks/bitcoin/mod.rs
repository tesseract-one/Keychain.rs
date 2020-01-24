mod factory;
mod key;
mod key_path;

use crate::network::Network;

impl Network {
  pub const BITCOIN: Network = Network(key_path::COIN_TYPE);
}

pub use self::factory::KeyFactory;
pub use self::key_path::KeyPath;
