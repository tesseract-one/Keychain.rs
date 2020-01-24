use keychain::Network as RNetwork;
use crate::utils::ptr::Ptr;

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

impl Ptr<[Network]> for NetworksPtr {
  unsafe fn rust_ref(&self) -> &[Network] {
    std::slice::from_raw_parts(self.ptr, self.count)
  }

  unsafe fn free(&mut self) {
    if self.ptr.is_null() {
      return;
    }
    let _ = Vec::from_raw_parts(self.ptr as *mut &[Network], self.count, self.count);
    self.ptr = std::ptr::null();
  }
}

impl From<Vec<Network>> for NetworksPtr {
  fn from(data: Vec<Network>) -> Self {
    let len = data.len();
    let mut slice = data.into_boxed_slice();
    let out = slice.as_mut_ptr();
    std::mem::forget(slice);
    Self { ptr: out, count: len }
  }
}

impl From<Vec<RNetwork>> for NetworksPtr {
  fn from(data: Vec<RNetwork>) -> Self {
    data.into_iter().map(|net| net.into()).collect::<Vec<Network>>().into()
  }
}

#[no_mangle]
pub unsafe extern "C" fn delete_networks(networks: &mut NetworksPtr) {
  networks.free();
}
