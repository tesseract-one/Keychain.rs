// External crates
// data serialization / deserialization
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_repr;
#[macro_use]
extern crate base64_serde;
extern crate base64;

// Crypt
extern crate cryptoxide;
extern crate rand;

// Enum conversion to and from number
#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

// our bip39 lib
pub extern crate bip39;

#[cfg(feature = "cardano")]
extern crate ed25519_bip32;

#[cfg(any(feature = "ethereum", feature = "bitcoin"))]
extern crate secp256k1_bip32;

// Internal modules
mod data;
mod entropy;
mod error;
mod key;
mod key_factory;
mod key_path;
mod keychain;
mod manager;
mod mnemonic;
mod network;

// Public Modules
pub mod crypt;
pub mod networks;

//Exports
pub use error::Error;
pub use key_path::GenericKeyPath;
pub use key_path::KeyPath;
pub use keychain::Keychain;
pub use manager::KeychainManager;
pub use mnemonic::Language;
pub use network::Network;

#[cfg(feature = "custom-networks")]
pub use entropy::*;
#[cfg(feature = "custom-networks")]
pub use key::{Error as KeyError, Key};
#[cfg(feature = "custom-networks")]
pub use key_factory::*;
