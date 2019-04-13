mod factory;
mod key;
mod key_path;

use network::Network;

impl Network {
  pub const CARDANO: Network = Network(key_path::BIP44_COIN_TYPE);
}

pub use self::key::{ KeyError as PrivateKeyError };
pub use self::factory::KeyFactory;
pub use self::key_path::KeyPath;