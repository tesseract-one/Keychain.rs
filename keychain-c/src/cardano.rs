use keychain::{ Network as RNetwork };
use keychain::{ KeyPath as IKeyPath };
use keychain::networks::cardano::{ KeyPath as RKeyPath };
use result::{ CResult, ErrorPtr };
use network::Network;
use key_path::KeyPath;

static CARDANO: Network = Network(RNetwork::CARDANO.0);

#[no_mangle]
pub extern "C" fn NETWORK_CARDANO() -> Network { CARDANO }

#[no_mangle]
pub unsafe extern "C" fn keypath_cardano_new(
  account: u32, change: u32, address: u32,
  path: &mut KeyPath, error: &mut ErrorPtr
) -> bool {
  RKeyPath::new(account, change, address)
    .map_err(|err| err.into())
    .map(|kp| (&kp as &IKeyPath).into())
    .response(path, error)
}
