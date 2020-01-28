use crate::error::ErrorPtr;
use crate::key_path::KeyPath;
use crate::network::{Network, NetworksPtr};
use crate::utils::data::DataPtr;
use crate::utils::panic::{handle_exception, handle_exception_result};
use crate::utils::ptr::{IntoArrayPtr, SizedPtr};
use crate::utils::result::CResult;
use keychain::Keychain as RKeychain;
use std::ffi::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KeychainPtr(*mut c_void);

impl SizedPtr for KeychainPtr {
  type Type = RKeychain;

  fn from_ptr(ptr: *mut c_void) -> KeychainPtr {
    Self(ptr)
  }

  fn get_ptr(&self) -> *mut c_void {
    self.0
  }

  fn set_ptr(&mut self, ptr: *mut c_void) {
    self.0 = ptr;
  }
}

#[no_mangle]
pub unsafe extern "C" fn keychain_networks(
  keychain: &KeychainPtr, networks: &mut NetworksPtr, error: &mut ErrorPtr
) -> bool {
  handle_exception(|| keychain.get_ref().networks().into()).response(networks, error)
}

#[no_mangle]
pub unsafe extern "C" fn keychain_pub_key(
  keychain: &KeychainPtr, network: Network, path: KeyPath, key: &mut DataPtr, error: &mut ErrorPtr
) -> bool {
  handle_exception_result(|| {
    keychain.get_ref().pub_key(&network.into(), &path).map(|data| data.into_array_ptr())
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
    keychain.get_ref().sign(&network.into(), data_slice, &path).map(|data| data.into_array_ptr())
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

    keychain.get_ref().verify(&network.into(), data_slice, signature_slice, &path)
  })
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn delete_keychain(keychain: &mut KeychainPtr) {
  keychain.free();
}
