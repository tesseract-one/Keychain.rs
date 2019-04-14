use keychain::{ Network as RNetwork };
use libc::{ malloc, free };
 use std::ffi::c_void;
use std::mem;

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
pub struct Networks {
  ptr: *const Network,
  count: usize  
}

impl From<Vec<Network>> for Networks {
  fn from(data: Vec<Network>) -> Self {
    let dataptr = unsafe { malloc(data.len() * mem::size_of::<Network>()) as *mut Network };
    let slice = unsafe { std::slice::from_raw_parts_mut(dataptr, data.len()) };
    slice.copy_from_slice(data.as_ref());
    Self { ptr: dataptr, count: data.len() }
  }
}

impl From<Vec<RNetwork>> for Networks {
  fn from(data: Vec<RNetwork>) -> Self {
    data.into_iter().map(|net| net.into()).collect::<Vec<Network>>().into()
  }
}

#[no_mangle]
pub unsafe extern "C" fn delete_networks(networks: &mut Networks) {
  if !networks.ptr.is_null() {
    free(networks.ptr as *mut c_void);
  }
  networks.ptr = std::ptr::null();
}