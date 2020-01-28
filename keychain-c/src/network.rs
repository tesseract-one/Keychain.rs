use crate::utils::ptr::{ArrayPtr, IntoArrayPtr};
use keychain::Network as RNetwork;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Network(pub u32);

impl From<Network> for RNetwork {
  fn from(net: Network) -> Self {
    RNetwork(net.0)
  }
}

impl From<RNetwork> for Network {
  fn from(net: RNetwork) -> Self {
    Network(net.0)
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NetworksPtr {
  ptr: *const Network,
  count: usize
}

impl ArrayPtr for NetworksPtr {
  type Element = Network;

  fn from_ptr(ptr: *const Network, count: usize) -> NetworksPtr {
    Self { ptr, count }
  }

  fn get_ptr(&self) -> *const Network {
    self.ptr
  }

  fn get_count(&self) -> usize {
    self.count
  }

  fn set_ptr(&mut self, ptr: *const Network) {
    self.ptr = ptr;
  }
}

impl From<Vec<RNetwork>> for NetworksPtr {
  fn from(data: Vec<RNetwork>) -> Self {
    data.into_iter().map(|net| net.into()).collect::<Vec<Network>>().into_array_ptr()
  }
}

#[no_mangle]
pub unsafe extern "C" fn delete_networks(networks: &mut NetworksPtr) {
  networks.free();
}
