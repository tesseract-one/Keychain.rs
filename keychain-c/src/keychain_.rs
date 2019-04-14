use keychain::{ Keychain as RKeychain };
use network::{ Network, NetworksPtr };
use key_path::KeyPath;
use std::mem;
use result::{ DataPtr, CResult, ErrorPtr, Ptr };
use std::ffi::{ c_void };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KeychainPtr(*mut c_void);

impl Ptr<RKeychain> for KeychainPtr {
  unsafe fn as_ref(&self) -> &RKeychain {
    (self.0 as *mut RKeychain).as_ref().unwrap()
  }

  unsafe fn free(&mut self) {
    if self.0.is_null() { return; }
    let _: Box<RKeychain> = Box::from_raw(self.0 as *mut RKeychain);
    self.0 = std::ptr::null_mut();
  }
}

impl KeychainPtr {
  pub fn new(keychain: RKeychain) -> Self {
    let ptr = Box::into_raw(Box::new(keychain));
    mem::forget(ptr);
    Self(ptr as *mut c_void)
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NewKeychainData {
  data: DataPtr,
  keychain: KeychainPtr
}

impl NewKeychainData {
  pub fn new(keychain: RKeychain, data: &[u8]) -> Self {
    Self {
      data: DataPtr::from(data),
      keychain: KeychainPtr::new(keychain)
    }
  }
}

#[no_mangle]
pub unsafe extern "C" fn keychain_networks(keychain: &KeychainPtr) -> NetworksPtr {
  keychain.as_ref()
    .networks()
    .into()
}

#[no_mangle]
pub unsafe extern "C" fn keychain_pub_key(
  keychain: &KeychainPtr, network: Network, path: KeyPath, key: &mut DataPtr, error: &mut ErrorPtr
) -> bool {
  keychain.as_ref()
    .pub_key(&network.into(), &path)
    .map(|data| DataPtr::from(data))
    .response(key, error)
}

#[no_mangle]
pub unsafe extern "C" fn keychain_sign(
  keychain: &KeychainPtr, network: Network, data: *const u8, data_len: usize, path: KeyPath,
  signature: &mut DataPtr, error: &mut ErrorPtr
) -> bool {
  let data_slice = std::slice::from_raw_parts(data, data_len);
  keychain.as_ref()
    .sign(&network.into(), data_slice, &path)
    .map(|data| DataPtr::from(data))
    .response(signature, error)
}

#[no_mangle]
pub unsafe extern "C" fn keychain_verify(
  keychain: &KeychainPtr, network: Network,
  data: *const u8, data_len: usize,
  signature: *const u8, signature_len: usize,
  path: KeyPath, result: &mut bool, error: &mut ErrorPtr
) -> bool {
  let data_slice = std::slice::from_raw_parts(data, data_len);
  let signature_slice = std::slice::from_raw_parts(signature, signature_len);

  keychain.as_ref()
    .verify(&network.into(), data_slice, signature_slice, &path)
    .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn delete_keychain(keychain: &mut KeychainPtr) {
  keychain.free();
}