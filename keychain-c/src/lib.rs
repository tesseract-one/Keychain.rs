extern crate keychain;
extern crate num_traits;

mod key_path;
mod keychain_;
mod manager;
mod network;
mod panic;
mod result;

pub use key_path::*;
pub use keychain_::*;
pub use manager::*;
pub use network::*;
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
