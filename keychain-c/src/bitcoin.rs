use keychain::{ Network as RNetwork };
use keychain::{ KeyPath as IKeyPath };
use keychain::networks::bitcoin::{ KeyPath as RKeyPath };
use result::{ CResult, Error };
use network::Network;
use key_path::KeyPath;

static BITCOIN: Network = Network(RNetwork::BITCOIN.0);

#[no_mangle]
pub extern "C" fn NETWORK_BITCOIN() -> Network { BITCOIN }

#[no_mangle]
pub unsafe extern "C" fn keypath_bitcoin_new_bip44(
  testnet: bool, account: u32, change: u32, address: u32,
  path: &mut KeyPath, error: &mut Error
) -> bool {
  RKeyPath::bip44(testnet, account, change, address)
    .map_err(|err| err.into())
    .map(|kp| (&kp as &IKeyPath).into())
    .response(path, error)
}

#[no_mangle]
pub unsafe extern "C" fn keypath_bitcoin_new_bip49(
  testnet: bool, account: u32, change: u32, address: u32,
  path: &mut KeyPath, error: &mut Error
) -> bool {
  RKeyPath::bip49(testnet, account, change, address)
    .map_err(|err| err.into())
    .map(|kp| (&kp as &IKeyPath).into())
    .response(path, error)
}

#[no_mangle]
pub unsafe extern "C" fn keypath_bitcoin_new_bip84(
  testnet: bool, account: u32, change: u32, address: u32,
  path: &mut KeyPath, error: &mut Error
) -> bool {
  RKeyPath::bip84(testnet, account, change, address)
    .map_err(|err| err.into())
    .map(|kp| (&kp as &IKeyPath).into())
    .response(path, error)
}