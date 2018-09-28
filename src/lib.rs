
// External crates
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_cbor;
extern crate futures;
extern crate cryptoxide;

// Internal modules
mod data;
mod network;
mod key_storage;
mod wallet;
mod provider;
mod key_path;
mod mnemonic;
mod network_type;

// Public Modules
pub mod networks;
pub mod bip39;
pub mod util;
pub mod external;

//Exports
pub use wallet::HDWallet;
pub use network::Network;
pub use network_type::NetworkType;
pub use provider::{ HDWalletProvider, Error as ProviderError };
pub use key_path::Bip44;

#[cfg(feature = "custom-networks")]
pub use key_path::Bip44KeyPath;