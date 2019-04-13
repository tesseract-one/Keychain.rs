use keychain::{ Keychain as RKeychain };
use network::{ Network, Networks };
use key_path::KeyPath;
use std::mem;
use result::{ Data, CResult, Error };
use std::ffi::{ c_void };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Keychain(*mut c_void);

impl Keychain {
  pub fn new(keychain: RKeychain) -> Self {
    let ptr = Box::into_raw(Box::new(keychain));
    mem::forget(ptr);
    Self(ptr as *mut c_void)
  }

  unsafe fn rust(&self) -> &RKeychain {
    (self.0 as *mut RKeychain).as_ref().unwrap()
  }

  unsafe fn drop(&mut self) {
    let _: Box<RKeychain> = Box::from_raw(self.0 as *mut RKeychain);
    self.0 = std::ptr::null_mut();
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KeychainData {
  data: Data,
  keychain: Keychain
}

impl KeychainData {
  pub fn new(keychain: RKeychain, data: &[u8]) -> Self {
    Self {
      data: Data::from(data),
      keychain: Keychain::new(keychain)
    }
  }
}

#[no_mangle]
pub unsafe extern "C" fn keychain_networks(keychain: &Keychain) -> Networks {
  keychain.rust()
    .networks()
    .into()
}

#[no_mangle]
pub unsafe extern "C" fn keychain_pub_key(
  keychain: &Keychain, network: Network, path: KeyPath, key: &mut Data, error: &mut Error
) -> bool {
  keychain.rust()
    .pub_key(&network.into(), &path)
    .map(|data| Data::from(data))
    .response(key, error)
}

#[no_mangle]
pub unsafe extern "C" fn keychain_sign(
  keychain: &Keychain, network: Network, data: *const u8, data_len: usize, path: KeyPath,
  signature: &mut Data, error: &mut Error
) -> bool {
  let data_slice = std::slice::from_raw_parts(data, data_len);
  keychain.rust()
    .sign(&network.into(), data_slice, &path)
    .map(|data| Data::from(data))
    .response(signature, error)
}

#[no_mangle]
pub unsafe extern "C" fn keychain_verify(
  keychain: &Keychain, network: Network,
  data: *const u8, data_len: usize,
  signature: *const u8, signature_len: usize,
  path: KeyPath, result: &mut bool, error: &mut Error
) -> bool {
  let data_slice = std::slice::from_raw_parts(data, data_len);
  let signature_slice = std::slice::from_raw_parts(signature, signature_len);

  keychain.rust()
    .verify(&network.into(), data_slice, signature_slice, &path)
    .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn delete_keychain(keychain: &mut Keychain) {
  keychain.drop();
}