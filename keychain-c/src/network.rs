use keychain::{ Network as RNetwork }; 

#[repr(C)]
pub struct Network(pub u32);

#[no_mangle]
pub static NETWORK_ETHEREUM: Network = Network(RNetwork::ETHEREUM.0);

#[no_mangle]
pub static NETWORK_CARDANO: Network = Network(RNetwork::CARDANO.0);

#[no_mangle]
pub static NETWORK_BITCOIN: Network = Network(RNetwork::BITCOIN.0);

impl From<Network> for RNetwork {
  fn from(net: Network) -> Self {
    RNetwork(net.0)
  }
}