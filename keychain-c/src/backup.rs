use crate::error::ErrorPtr;
use crate::manager::{KeychainManagerPtr, Language};
use crate::network::Network;
use crate::utils::data::DataPtr;
use crate::utils::panic::handle_exception_result;
use crate::utils::ptr::{ArrayPtr, IntoArrayPtr, SizedPtr, UnsizedPtr};
use crate::utils::result::CResult;
use crate::utils::string::{CharPtr, ToCString};
use keychain::Network as RNetwork;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KeyBackupElem {
  pub network: Network,
  pub data: DataPtr
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KeyBackupPtr {
  pub ptr: *const KeyBackupElem,
  pub count: usize
}

impl From<Vec<(RNetwork, Vec<u8>)>> for KeyBackupPtr {
  fn from(data: Vec<(RNetwork, Vec<u8>)>) -> Self {
    data
      .into_iter()
      .map(|(net, data)| KeyBackupElem { network: net.into(), data: data.into_array_ptr() })
      .collect::<Vec<KeyBackupElem>>()
      .into_array_ptr()
  }
}

impl ArrayPtr for KeyBackupPtr {
  type Element = KeyBackupElem;

  fn from_ptr(ptr: *const KeyBackupElem, count: usize) -> KeyBackupPtr {
    Self { ptr, count }
  }

  fn get_ptr(&self) -> *const KeyBackupElem {
    self.ptr
  }

  fn get_count(&self) -> usize {
    self.count
  }

  fn set_ptr(&mut self, ptr: *const KeyBackupElem) {
    self.ptr = ptr;
  }

  unsafe fn free(&mut self) {
    if self.ptr.is_null() {
      return;
    }
    let vec = Vec::from_raw_parts(self.ptr as *mut KeyBackupElem, self.count, self.count);
    for mut elem in vec.into_iter() {
      elem.data.free();
    }
    self.ptr = std::ptr::null();
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MnemonicInfoPtr {
  mnemonic: CharPtr,
  language: Language
}

impl MnemonicInfoPtr {
  fn new(mnemonic: String, language: Language) -> Self {
    Self { mnemonic: mnemonic.to_cstr(), language }
  }
}

impl UnsizedPtr for MnemonicInfoPtr {
  type Type = str;

  unsafe fn get_ref(&self) -> &str {
    (&self.mnemonic as &dyn UnsizedPtr<Type = str>).get_ref()
  }

  unsafe fn free(&mut self) {
    if self.mnemonic.is_null() {
      return;
    }
    self.mnemonic.free();
  }
}

#[no_mangle]
pub unsafe extern "C" fn keychain_manager_get_keys_data(
  manager: &KeychainManagerPtr, encrypted: *const u8, encrypted_len: usize, password: CharPtr,
  data: &mut KeyBackupPtr, error: &mut ErrorPtr
) -> bool {
  handle_exception_result(|| {
    let data_slice = std::slice::from_raw_parts(encrypted, encrypted_len);
    manager.get_ref().get_keys_data(data_slice, password.get_ref()).map(|backup| backup.into())
  })
  .response(data, error)
}

#[no_mangle]
pub unsafe extern "C" fn keychain_manager_retrieve_mnemonic(
  manager: &KeychainManagerPtr, data: *const u8, data_len: usize, password: CharPtr,
  mnemonic: &mut MnemonicInfoPtr, error: &mut ErrorPtr
) -> bool {
  handle_exception_result(|| {
    let data_slice = std::slice::from_raw_parts(data, data_len);
    manager
      .get_ref()
      .retrieve_mnemonic(data_slice, password.get_ref())
      .map(|(mnemonic, lang)| MnemonicInfoPtr::new(mnemonic, lang.into()))
  })
  .response(mnemonic, error)
}

#[no_mangle]
pub unsafe extern "C" fn delete_key_backup(backup: &mut KeyBackupPtr) {
  backup.free();
}

#[no_mangle]
pub unsafe extern "C" fn delete_mnemonic_info(info: &mut MnemonicInfoPtr) {
  info.free();
}
