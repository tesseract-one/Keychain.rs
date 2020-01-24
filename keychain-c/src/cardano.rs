use crate::error::ErrorPtr;
use crate::key_path::KeyPath;
use crate::network::Network;
use crate::utils::panic::handle_exception_result;
use crate::utils::result::CResult;
use keychain::networks::cardano::KeyPath as RKeyPath;
use keychain::KeyPath as IKeyPath;
use keychain::Network as RNetwork;

static CARDANO: Network = Network(RNetwork::CARDANO.0);

#[no_mangle]
pub extern "C" fn NETWORK_CARDANO() -> Network {
  CARDANO
}

#[no_mangle]
pub unsafe extern "C" fn keypath_cardano_new(
  account: u32, change: u32, address: u32, path: &mut KeyPath, error: &mut ErrorPtr
) -> bool {
  handle_exception_result(|| {
    RKeyPath::new(account, change, address)
      .map_err(|err| err.into())
      .map(|kp| (&kp as &dyn IKeyPath).into())
  })
  .response(path, error)
}
