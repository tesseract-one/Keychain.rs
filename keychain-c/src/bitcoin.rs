use crate::error::ErrorPtr;
use crate::key_path::KeyPath;
use crate::network::Network;
use crate::utils::panic::handle_exception_result;
use crate::utils::result::CResult;
use keychain::networks::bitcoin::KeyPath as RKeyPath;
use keychain::KeyPath as IKeyPath;
use keychain::Network as RNetwork;

static BITCOIN: Network = Network(RNetwork::BITCOIN.0);

#[no_mangle]
pub extern "C" fn NETWORK_BITCOIN() -> Network {
  BITCOIN
}

#[no_mangle]
pub unsafe extern "C" fn keypath_bitcoin_new_bip44(
  testnet: bool, account: u32, change: u32, address: u32, path: &mut KeyPath, error: &mut ErrorPtr
) -> bool {
  handle_exception_result(|| {
    RKeyPath::bip44(testnet, account, change, address)
      .map_err(|err| err.into())
      .map(|kp| (&kp as &dyn IKeyPath).into())
  })
  .response(path, error)
}

#[no_mangle]
pub unsafe extern "C" fn keypath_bitcoin_new_bip49(
  testnet: bool, account: u32, change: u32, address: u32, path: &mut KeyPath, error: &mut ErrorPtr
) -> bool {
  handle_exception_result(|| {
    RKeyPath::bip49(testnet, account, change, address)
      .map_err(|err| err.into())
      .map(|kp| (&kp as &dyn IKeyPath).into())
  })
  .response(path, error)
}

#[no_mangle]
pub unsafe extern "C" fn keypath_bitcoin_new_bip84(
  testnet: bool, account: u32, change: u32, address: u32, path: &mut KeyPath, error: &mut ErrorPtr
) -> bool {
  handle_exception_result(|| {
    RKeyPath::bip84(testnet, account, change, address)
      .map_err(|err| err.into())
      .map(|kp| (&kp as &dyn IKeyPath).into())
  })
  .response(path, error)
}
