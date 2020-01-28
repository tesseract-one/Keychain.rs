use crate::error::ErrorPtr;
use crate::keychain_c::KeychainPtr;
use crate::network::Network;
use crate::utils::data::DataPtr;
use crate::utils::panic::{handle_exception, handle_exception_result};
use crate::utils::ptr::{IntoArrayPtr, SizedPtr, UnsizedPtr};
use crate::utils::result::CResult;
use crate::utils::string::{CharPtr, ToCString};
use keychain::{KeychainManager as RKeychainManager, Language as RLanguage};
use num_traits::FromPrimitive;
use std::ffi::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KeychainManagerPtr(*mut c_void);

impl SizedPtr for KeychainManagerPtr {
  type Type = RKeychainManager;

  fn from_ptr(ptr: *mut c_void) -> KeychainManagerPtr {
    Self(ptr)
  }

  fn get_ptr(&self) -> *mut c_void {
    self.0
  }

  fn set_ptr(&mut self, ptr: *mut c_void) {
    self.0 = ptr;
  }
}

impl KeychainManagerPtr {
  fn new(manager: RKeychainManager) -> Self {
    Self(Box::into_raw(Box::new(manager)) as *mut c_void)
  }
}

#[repr(C)]
#[derive(Primitive, Copy, Clone)]
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

impl From<RLanguage> for Language {
  fn from(lang: RLanguage) -> Self {
    Self::from_i64(lang as i64).unwrap()
  }
}

#[no_mangle]
pub unsafe extern "C" fn keychain_manager_new(
  manager: &mut KeychainManagerPtr, error: &mut ErrorPtr
) -> bool {
  handle_exception_result(|| {
    RKeychainManager::new().map(|manager| KeychainManagerPtr::new(manager))
  })
  .response(manager, error)
}

#[no_mangle]
pub unsafe extern "C" fn keychain_manager_has_network(
  manager: &KeychainManagerPtr, network: Network, has: &mut bool, error: &mut ErrorPtr
) -> bool {
  handle_exception(|| manager.get_ref().has_network(&network.into())).response(has, error)
}

#[no_mangle]
pub unsafe extern "C" fn keychain_manager_generate_mnemonic(
  manager: &KeychainManagerPtr, lang: Language, mnemonic: &mut CharPtr, error: &mut ErrorPtr
) -> bool {
  handle_exception_result(|| {
    manager.get_ref().generate_mnemonic(lang.rust()).map(|mnemonic| mnemonic.to_cstr())
  })
  .response(mnemonic, error)
}

#[no_mangle]
pub unsafe extern "C" fn keychain_manager_keychain_data_from_seed(
  manager: &KeychainManagerPtr, seed: *const u8, seed_len: usize, password: CharPtr,
  data: &mut DataPtr, error: &mut ErrorPtr
) -> bool {
  handle_exception_result(|| {
    let seed_slice = std::slice::from_raw_parts(seed, seed_len);
    manager
      .get_ref()
      .keychain_data_from_seed(seed_slice, password.get_ref())
      .map(|data| data.into_array_ptr())
  })
  .response(data, error)
}

#[no_mangle]
pub unsafe extern "C" fn keychain_manager_keychain_data_from_mnemonic(
  manager: &KeychainManagerPtr, mnemonic: CharPtr, password: CharPtr, lang: Language,
  data: &mut DataPtr, error: &mut ErrorPtr
) -> bool {
  handle_exception_result(|| {
    manager
      .get_ref()
      .keychain_data_from_mnemonic(mnemonic.get_ref(), password.get_ref(), lang.rust())
      .map(|data| data.into_array_ptr())
  })
  .response(data, error)
}

#[no_mangle]
pub unsafe extern "C" fn keychain_manager_keychain_from_data(
  manager: &KeychainManagerPtr, data: *const u8, data_len: usize, password: CharPtr,
  keychain: &mut KeychainPtr, error: &mut ErrorPtr
) -> bool {
  handle_exception_result(|| {
    let data_slice = std::slice::from_raw_parts(data, data_len);
    manager
      .get_ref()
      .keychain_from_data(data_slice, password.get_ref())
      .map(|keychain| KeychainPtr::new(keychain))
  })
  .response(keychain, error)
}

#[no_mangle]
pub unsafe extern "C" fn keychain_manager_add_network(
  manager: &KeychainManagerPtr, data: *const u8, data_len: usize, password: CharPtr,
  network: Network, response: &mut DataPtr, error: &mut ErrorPtr
) -> bool {
  handle_exception_result(|| {
    let data_slice = std::slice::from_raw_parts(data, data_len);
    manager
      .get_ref()
      .add_network(data_slice, password.get_ref(), network.into())
      .map(|data| data.into_array_ptr())
  })
  .response(response, error)
}

#[no_mangle]
pub unsafe extern "C" fn keychain_manager_change_password(
  manager: &KeychainManagerPtr, data: *const u8, data_len: usize, old_password: CharPtr,
  new_password: CharPtr, response: &mut DataPtr, error: &mut ErrorPtr
) -> bool {
  handle_exception_result(|| {
    let data_slice = std::slice::from_raw_parts(data, data_len);
    manager
      .get_ref()
      .change_password(data_slice, old_password.get_ref(), new_password.get_ref())
      .map(|data| data.into_array_ptr())
  })
  .response(response, error)
}

#[no_mangle]
pub unsafe extern "C" fn delete_keychain_manager(manager: &mut KeychainManagerPtr) {
  manager.free();
}
