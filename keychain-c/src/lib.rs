extern crate keychain;

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

mod error;
mod key_path;
mod keychain_c;
mod manager;
mod network;
mod utils;

pub use error::*;
pub use key_path::*;
pub use keychain_c::*;
pub use manager::*;
pub use network::*;

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

#[no_mangle]
pub unsafe extern "C" fn keychain_init_library() {
  utils::panic::hide_exceptions();
}
