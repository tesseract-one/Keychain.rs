use error::ErrorPtr;
use key_path::KeyPath;
use keychain::networks::cardano::KeyPath as RKeyPath;
use keychain::KeyPath as IKeyPath;
use keychain::Network as RNetwork;
use network::Network;
use utils::panic::handle_exception_result;
use utils::result::CResult;

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
