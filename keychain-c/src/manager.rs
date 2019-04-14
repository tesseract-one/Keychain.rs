use keychain::{ KeychainManager as RKeychainManager, Language as RLanguage };
use keychain_::{ NewKeychainData, KeychainPtr };
use std::mem;
use std::ffi::{ c_void, CStr };
use std::os::raw::c_char;
use result::{ CResult, PChar, ToCString, DataPtr, ErrorPtr, Ptr };
use network::Network;
use num_traits::FromPrimitive;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KeychainManagerPtr(*mut c_void);

impl Ptr<RKeychainManager> for KeychainManagerPtr {
  unsafe fn as_ref(&self) -> &RKeychainManager {
    (self.0 as *mut RKeychainManager).as_ref().unwrap()
  }

  unsafe fn free(&mut self) {
    if self.0.is_null() { return; }
    let _: Box<RKeychainManager> = Box::from_raw(self.0 as *mut RKeychainManager);
    self.0 = std::ptr::null_mut();
  }
}

impl KeychainManagerPtr {
  fn new(manager: RKeychainManager) -> Self {
    let ptr = Box::into_raw(Box::new(manager));
    mem::forget(ptr);
    Self(ptr as *mut c_void)
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Language {
  English = 0,
  French = 1,
  Japanese = 2,
  Korean = 3,
  ChineseSimplified = 4,
  ChineseTraditional = 5,
  Italian = 6,
  Spanish = 7
}

impl Language {
  fn rust(&self) -> Option<RLanguage> {
    RLanguage::from_i64(*self as i64)
  }
}

#[no_mangle]
pub unsafe extern "C" fn keychain_manager_new(manager: &mut KeychainManagerPtr, error: &mut ErrorPtr) -> bool {
  RKeychainManager::new()
    .map(|manager| KeychainManagerPtr::new(manager))
    .response(manager, error)
}

#[no_mangle]
pub unsafe extern "C" fn keychain_manager_has_network(manager: &KeychainManagerPtr, network: Network) -> bool {
  manager.as_ref().has_network(&network.into())
}

#[no_mangle]
pub unsafe extern "C" fn keychain_manager_generate_mnemonic(
  manager: &KeychainManagerPtr, lang: Language, mnemonic: &mut PChar, error: &mut ErrorPtr
) -> bool {
  manager.as_ref().generate_mnemonic(lang.rust())
    .map(|mnemonic| mnemonic.to_cstr())
    .response(mnemonic, error)
}

#[no_mangle]
pub unsafe extern "C" fn keychain_manager_keychain_from_seed(
  manager: &KeychainManagerPtr, seed: *const u8, seed_len: usize, password: PChar,
  data: &mut NewKeychainData, error: &mut ErrorPtr
) -> bool {
  let seed_slice = std::slice::from_raw_parts(seed, seed_len);
  let pwd = CStr::from_ptr(password as *const c_char).to_str().unwrap();
  manager.as_ref()
    .keychain_from_seed(seed_slice, pwd)
    .map(|(keychain, data)| NewKeychainData::new(keychain, &data))
    .response(data, error)
}

#[no_mangle]
pub unsafe extern "C" fn keychain_manager_keychain_from_mnemonic(
  manager: &KeychainManagerPtr, mnemonic: PChar, password: PChar, lang: Language,
  data: &mut NewKeychainData, error: &mut ErrorPtr
) -> bool {
  let mnemonic = CStr::from_ptr(mnemonic as *const c_char).to_str().unwrap();
  let pwd = CStr::from_ptr(password as *const c_char).to_str().unwrap();
  manager.as_ref()
    .keychain_from_mnemonic(mnemonic, pwd, lang.rust())
    .map(|(keychain, data)| NewKeychainData::new(keychain, &data))
    .response(data, error)
}

#[no_mangle]
pub unsafe extern "C" fn keychain_manager_keychain_from_data(
  manager: &KeychainManagerPtr, data: *const u8, data_len: usize, password: PChar,
  keychain: &mut KeychainPtr, error: &mut ErrorPtr
) -> bool {
  let data_slice = std::slice::from_raw_parts(data, data_len);
  let pwd = CStr::from_ptr(password as *const c_char).to_str().unwrap();
  manager.as_ref()
    .keychain_from_data(data_slice, pwd)
    .map(|keychain| KeychainPtr::new(keychain))
    .response(keychain, error)
}

#[no_mangle]
pub unsafe extern "C" fn keychain_manager_change_password(
  manager: &KeychainManagerPtr, data: *const u8, data_len: usize, old_password: PChar, new_password: PChar,
  response: &mut DataPtr, error: &mut ErrorPtr 
) -> bool {
  let data_slice = std::slice::from_raw_parts(data, data_len);
  let old_pwd = CStr::from_ptr(old_password as *const c_char).to_str().unwrap();
  let new_pwd = CStr::from_ptr(new_password as *const c_char).to_str().unwrap();
  manager.as_ref()
    .change_password(data_slice, old_pwd, new_pwd)
    .map(|data| DataPtr::from(data))
    .response(response, error)
}

#[no_mangle]
pub unsafe extern "C" fn delete_keychain_manager(manager: &mut KeychainManagerPtr) {
  manager.free();
}