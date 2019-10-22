use error::ErrorPtr;
use keychain::Network as RNetwork;
use manager::{KeychainManagerPtr, Language};
use network::Network;
use utils::data::DataPtr;
use utils::panic::handle_exception_result;
use utils::ptr::Ptr;
use utils::result::CResult;
use utils::string::{CharPtr, ToCString};

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

impl Ptr<[KeyBackupElem]> for KeyBackupPtr {
  unsafe fn rust_ref(&self) -> &[KeyBackupElem] {
    std::slice::from_raw_parts(self.ptr, self.count)
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

impl KeyBackupPtr {
  fn from(data: Vec<(RNetwork, Vec<u8>)>) -> Self {
    let mapped: Vec<KeyBackupElem> = data
      .into_iter()
      .map(|(net, data)| KeyBackupElem { network: net.into(), data: DataPtr::from(data) })
      .collect();

    let len = mapped.len();
    let mut slice = mapped.into_boxed_slice();
    let out = slice.as_mut_ptr();
    std::mem::forget(slice);
    Self { ptr: out, count: len }
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

impl Ptr<str> for MnemonicInfoPtr {
  unsafe fn rust_ref(&self) -> &str {
    (&self.mnemonic as &dyn Ptr<str>).rust_ref()
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
    manager
      .rust_ref()
      .get_keys_data(data_slice, password.rust_ref())
      .map(|backup| KeyBackupPtr::from(backup))
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
      .rust_ref()
      .retrieve_mnemonic(data_slice, password.rust_ref())
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
