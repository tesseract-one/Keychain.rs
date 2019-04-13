use keychain::{ Network as RNetwork };
use keychain::{ KeyPath as IKeyPath };
use keychain::networks::ethereum::{ KeyPath as RKeyPath };
use result::{ CResult, Error };
use network::Network;
use key_path::KeyPath;

static ETHEREUM: Network = Network(RNetwork::ETHEREUM.0);

#[no_mangle]
pub extern "C" fn NETWORK_ETHEREUM() -> Network { ETHEREUM }

#[no_mangle]
pub unsafe extern "C" fn keypath_ethereum_new(
  account: u32, path: &mut KeyPath, error: &mut Error
) -> bool {
  RKeyPath::new(account)
    .map_err(|err| err.into())
    .map(|kp| (&kp as &IKeyPath).into())
    .response(path, error)
}

#[no_mangle]
pub unsafe extern "C" fn keypath_ethereum_new_metamask(
  account: u32, path: &mut KeyPath, error: &mut Error
) -> bool {
  RKeyPath::new_metamask(account)
    .map_err(|err| err.into())
    .map(|kp| (&kp as &IKeyPath).into())
    .response(path, error)
}