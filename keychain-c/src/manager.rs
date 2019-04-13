use keychain::{ KeychainManager as RKeychainManager, Language as RLanguage };
use keychain_::{ KeychainData, Keychain };
use std::mem;
use std::ffi::{ c_void, CStr };
use std::os::raw::c_char;
use result::{ CResult, PChar, ToCString, Data, Error };
use network::Network;
use num_traits::FromPrimitive;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KeychainManager(*mut c_void);

impl KeychainManager {
  fn new(manager: RKeychainManager) -> Self {
    let ptr = Box::into_raw(Box::new(manager));
    mem::forget(ptr);
    Self(ptr as *mut c_void)
  }

  pub unsafe fn rust(&self) -> &RKeychainManager {
    (self.0 as *mut RKeychainManager).as_ref().unwrap()
  }

  unsafe fn drop(&mut self) {
    let _: Box<RKeychainManager> = Box::from_raw(self.0 as *mut RKeychainManager);
    self.0 = std::ptr::null_mut();
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
pub unsafe extern "C" fn keychain_manager_new(manager: &mut KeychainManager, error: &mut Error) -> bool {
  RKeychainManager::new()
    .map(|manager| KeychainManager::new(manager))
    .response(manager, error)
}

#[no_mangle]
pub unsafe extern "C" fn keychain_manager_has_network(manager: &KeychainManager, network: Network) -> bool {
  manager.rust().has_network(&network.into())
}

#[no_mangle]
pub unsafe extern "C" fn keychain_manager_generate_mnemonic(
  manager: &KeychainManager, lang: Language, mnemonic: &mut PChar, error: &mut Error
) -> bool {
  manager.rust().generate_mnemonic(lang.rust())
    .map(|mnemonic| mnemonic.to_cstr())
    .response(mnemonic, error)
}

#[no_mangle]
pub unsafe extern "C" fn keychain_manager_keychain_from_seed(
  manager: &KeychainManager, seed: *const u8, seed_len: usize, password: PChar,
  data: &mut KeychainData, error: &mut Error
) -> bool {
  let seed_slice = std::slice::from_raw_parts(seed, seed_len);
  let pwd = CStr::from_ptr(password as *const c_char).to_str().unwrap();
  manager.rust()
    .keychain_from_seed(seed_slice, pwd)
    .map(|(keychain, data)| KeychainData::new(keychain, &data))
    .response(data, error)
}

#[no_mangle]
pub unsafe extern "C" fn keychain_manager_keychain_from_mnemonic(
  manager: &KeychainManager, mnemonic: PChar, password: PChar, lang: Language,
  data: &mut KeychainData, error: &mut Error
) -> bool {
  let mnemonic = CStr::from_ptr(mnemonic as *const c_char).to_str().unwrap();
  let pwd = CStr::from_ptr(password as *const c_char).to_str().unwrap();
  manager.rust()
    .keychain_from_mnemonic(mnemonic, pwd, lang.rust())
    .map(|(keychain, data)| KeychainData::new(keychain, &data))
    .response(data, error)
}

#[no_mangle]
pub unsafe extern "C" fn keychain_manager_keychain_from_data(
  manager: &KeychainManager, data: *const u8, data_len: usize, password: PChar,
  keychain: &mut Keychain, error: &mut Error
) -> bool {
  let data_slice = std::slice::from_raw_parts(data, data_len);
  let pwd = CStr::from_ptr(password as *const c_char).to_str().unwrap();
  manager.rust()
    .keychain_from_data(data_slice, pwd)
    .map(|keychain| Keychain::new(keychain))
    .response(keychain, error)
}

#[no_mangle]
pub unsafe extern "C" fn keychain_manager_change_password(
  manager: &KeychainManager, data: *const u8, data_len: usize, old_password: PChar, new_password: PChar,
  response: &mut Data, error: &mut Error 
) -> bool {
  let data_slice = std::slice::from_raw_parts(data, data_len);
  let old_pwd = CStr::from_ptr(old_password as *const c_char).to_str().unwrap();
  let new_pwd = CStr::from_ptr(new_password as *const c_char).to_str().unwrap();
  manager.rust()
    .change_password(data_slice, old_pwd, new_pwd)
    .map(|data| Data::from(data))
    .response(response, error)
}

#[no_mangle]
pub unsafe extern "C" fn delete_keychain_manager(manager: &mut KeychainManager) {
  manager.drop();
}