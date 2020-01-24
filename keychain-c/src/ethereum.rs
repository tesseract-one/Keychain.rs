use crate::error::ErrorPtr;
use crate::key_path::KeyPath;
use keychain::networks::ethereum::KeyPath as RKeyPath;
use keychain::KeyPath as IKeyPath;
use keychain::Network as RNetwork;
use crate::network::Network;
use crate::utils::panic::handle_exception_result;
use crate::utils::result::CResult;

static ETHEREUM: Network = Network(RNetwork::ETHEREUM.0);

#[no_mangle]
pub extern "C" fn NETWORK_ETHEREUM() -> Network {
  ETHEREUM
}

#[no_mangle]
pub unsafe extern "C" fn keypath_ethereum_new(
  account: u32, path: &mut KeyPath, error: &mut ErrorPtr
) -> bool {
  handle_exception_result(|| {
    RKeyPath::new(account).map_err(|err| err.into()).map(|kp| (&kp as &dyn IKeyPath).into())
  })
  .response(path, error)
}

#[no_mangle]
pub unsafe extern "C" fn keypath_ethereum_new_metamask(
  account: u32, path: &mut KeyPath, error: &mut ErrorPtr
) -> bool {
  handle_exception_result(|| {
    RKeyPath::new_metamask(account)
      .map_err(|err| err.into())
      .map(|kp| (&kp as &dyn IKeyPath).into())
  })
  .response(path, error)
}
