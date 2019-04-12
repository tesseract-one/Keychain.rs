mod factory;
mod key;
mod key_path;

use network::Network;

impl Network {
  pub const BITCOIN: Network = Network { code: key_path::COIN_TYPE };
}

pub use self::factory::KeyFactory;
pub use self::key_path::KeyPath;