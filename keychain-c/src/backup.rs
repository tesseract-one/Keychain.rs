use keychain::Network as RNetwork;
use manager::KeychainManagerPtr;
use network::Network;
use panic::handle_exception_result;
use result::{ArrayPtr, CResult, CharPtr, DataPtr, ErrorPtr, Ptr};
use std::ffi::CStr;
use std::os::raw::c_char;

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

impl ArrayPtr<KeyBackupElem> for KeyBackupPtr {
  unsafe fn as_ref(&self) -> &[KeyBackupElem] {
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

#[no_mangle]
pub unsafe extern "C" fn keychain_manager_get_keys_data(
  manager: &KeychainManagerPtr, encrypted: *const u8, encrypted_len: usize, password: CharPtr,
  data: &mut KeyBackupPtr, error: &mut ErrorPtr
) -> bool {
  handle_exception_result(|| {
    let data_slice = std::slice::from_raw_parts(encrypted, encrypted_len);
    let pwd = CStr::from_ptr(password as *const c_char).to_str().unwrap();
    manager.as_ref().get_keys_data(data_slice, pwd).map(|backup| KeyBackupPtr::from(backup))
  })
  .response(data, error)
}

#[no_mangle]
pub unsafe extern "C" fn delete_key_backup(backup: &mut KeyBackupPtr) {
  backup.free();
}
