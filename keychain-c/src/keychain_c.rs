use crate::error::ErrorPtr;
use crate::key_path::KeyPath;
use crate::network::{Network, NetworksPtr};
use crate::utils::data::DataPtr;
use crate::utils::panic::{handle_exception, handle_exception_result};
use crate::utils::ptr::Ptr;
use crate::utils::result::CResult;
use keychain::Keychain as RKeychain;
use std::ffi::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KeychainPtr(*mut c_void);

impl Ptr<RKeychain> for KeychainPtr {
  unsafe fn rust_ref(&self) -> &RKeychain {
    (self.0 as *mut RKeychain).as_ref().unwrap()
  }
  unsafe fn free(&mut self) {
    if self.0.is_null() {
      return;
    }
    let _: Box<RKeychain> = Box::from_raw(self.0 as *mut RKeychain);
    self.0 = std::ptr::null_mut();
  }
}

impl KeychainPtr {
  pub fn new(keychain: RKeychain) -> Self {
    Self(Box::into_raw(Box::new(keychain)) as *mut c_void)
  }
}

#[no_mangle]
pub unsafe extern "C" fn keychain_networks(
  keychain: &KeychainPtr, networks: &mut NetworksPtr, error: &mut ErrorPtr
) -> bool {
  handle_exception(|| keychain.rust_ref().networks().into()).response(networks, error)
}

#[no_mangle]
pub unsafe extern "C" fn keychain_pub_key(
  keychain: &KeychainPtr, network: Network, path: KeyPath, key: &mut DataPtr, error: &mut ErrorPtr
) -> bool {
  handle_exception_result(|| {
    keychain.rust_ref().pub_key(&network.into(), &path).map(|data| DataPtr::from(data))
  })
  .response(key, error)
}

#[no_mangle]
pub unsafe extern "C" fn keychain_sign(
  keychain: &KeychainPtr, network: Network, data: *const u8, data_len: usize, path: KeyPath,
  signature: &mut DataPtr, error: &mut ErrorPtr
) -> bool {
  handle_exception_result(|| {
    let data_slice = std::slice::from_raw_parts(data, data_len);
    keychain.rust_ref().sign(&network.into(), data_slice, &path).map(|data| DataPtr::from(data))
  })
  .response(signature, error)
}

#[no_mangle]
pub unsafe extern "C" fn keychain_verify(
  keychain: &KeychainPtr, network: Network, data: *const u8, data_len: usize, signature: *const u8,
  signature_len: usize, path: KeyPath, result: &mut bool, error: &mut ErrorPtr
) -> bool {
  handle_exception_result(|| {
    let data_slice = std::slice::from_raw_parts(data, data_len);
    let signature_slice = std::slice::from_raw_parts(signature, signature_len);

    keychain.rust_ref().verify(&network.into(), data_slice, signature_slice, &path)
  })
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn delete_keychain(keychain: &mut KeychainPtr) {
  keychain.free();
}
