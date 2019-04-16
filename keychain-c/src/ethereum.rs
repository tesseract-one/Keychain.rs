use keychain::{ Network as RNetwork };
use keychain::{ KeyPath as IKeyPath };
use keychain::networks::ethereum::{ KeyPath as RKeyPath };
use result::{ CResult, ErrorPtr };
use network::Network;
use key_path::KeyPath;
use panic::handle_exception_result;

static ETHEREUM: Network = Network(RNetwork::ETHEREUM.0);

#[no_mangle]
pub extern "C" fn NETWORK_ETHEREUM() -> Network { ETHEREUM }

#[no_mangle]
pub unsafe extern "C" fn keypath_ethereum_new(
  account: u32, path: &mut KeyPath, error: &mut ErrorPtr
) -> bool {
  handle_exception_result(|| {
    RKeyPath::new(account)
      .map_err(|err| err.into())
      .map(|kp| (&kp as &IKeyPath).into())
  }).response(path, error)
}

#[no_mangle]
pub unsafe extern "C" fn keypath_ethereum_new_metamask(
  account: u32, path: &mut KeyPath, error: &mut ErrorPtr
) -> bool {
  handle_exception_result(|| {
    RKeyPath::new_metamask(account)
      .map_err(|err| err.into())
      .map(|kp| (&kp as &IKeyPath).into())
  }).response(path, error)
}