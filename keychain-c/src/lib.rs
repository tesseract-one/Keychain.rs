extern crate keychain;
extern crate num_traits;

mod network;
mod manager;
mod result;
mod keychain_;
mod key_path;
mod panic;

pub use network::*;
pub use manager::*;
pub use keychain_::*;
pub use key_path::*;
pub use result::*;

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
  panic::hide_exceptions();
}