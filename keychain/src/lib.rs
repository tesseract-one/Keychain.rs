
// External crates
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate cryptoxide;
extern crate rand;
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
//pub use wallet::HDWallet;
pub use network::Network;
pub use manager::KeychainManager;
pub use key_path::GenericKeyPath;

#[cfg(feature = "custom-networks")]
pub use key_path::KeyPath;
pub use entropy::*;