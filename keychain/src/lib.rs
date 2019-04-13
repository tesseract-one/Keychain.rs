
// External crates
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate cryptoxide;
extern crate rand;
#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

pub extern crate bip39;

#[cfg(feature = "cardano")]
extern crate ed25519_bip32;

#[cfg(any(feature = "ethereum", feature = "bitcoin"))]
extern crate secp256k1_bip32;

// Internal modules
mod data;
mod key_factory;
mod key;
mod keychain;
mod manager;
mod key_path;
mod mnemonic;
mod network;
mod entropy;
mod error;

// Public Modules
pub mod networks;
pub mod crypt;

//Exports
pub use error::Error;
pub use network::Network;
pub use keychain::Keychain;
pub use manager::KeychainManager;
pub use key_path::GenericKeyPath;
pub use key_path::KeyPath;
pub use mnemonic::Language;

#[cfg(feature = "custom-networks")]
pub use key_factory::*;
#[cfg(feature = "custom-networks")]
pub use key::{ Key, Error as KeyError };
#[cfg(feature = "custom-networks")]
pub use entropy::*;