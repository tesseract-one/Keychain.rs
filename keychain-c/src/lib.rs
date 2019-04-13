extern crate keychain;
extern crate libc;

extern crate num_traits;

mod network;
mod manager;
mod result;
mod keychain_;
mod key_path;

pub use network::{Network, Networks};
pub use manager::*;
pub use keychain_::*;
pub use key_path::*;

#[cfg(feature = "ethereum")]
mod ethereum;
#[cfg(feature = "ethereum")]
pub use ethereum::*;

#[cfg(feature = "cardano")]
mod cardano;
#[cfg(feature = "cardano")]
pub use cardano::*;

#[cfg(feature = "bitcoin")]
mod bitcoin;
#[cfg(feature = "bitcoin")]
pub use bitcoin::*;

#[cfg(feature = "backup")]
mod backup;
#[cfg(feature = "backup")]
pub use backup::*;