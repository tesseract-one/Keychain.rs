use manager::KeychainManagerPtr;
use network::Network;
use result::{ DataPtr, CResult, PChar, ErrorPtr, ArrayPtr, Ptr };
use keychain::{ Network as RNetwork };
use std::os::raw::c_char;
use std::ffi::CStr;
use libc::{ malloc, free, c_void };
use std::mem;

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
    if self.ptr.is_null() { return; }
    let kbslice = std::slice::from_raw_parts_mut(self.ptr as *mut KeyBackupElem, self.count);
    for elem in kbslice.into_iter() {
      elem.data.free();
    }
    free(self.ptr as *mut c_void);
    self.ptr = std::ptr::null_mut();
  }
}

impl KeyBackupPtr {
  fn from(data: Vec<(RNetwork, Vec<u8>)>) -> Self {
    let mapped: Vec<KeyBackupElem> = data.into_iter()
      .map(|(net, data)|
        KeyBackupElem { network: net.into(), data: DataPtr::from(data) }
      ).collect();

    let dataptr = unsafe { malloc(mapped.len() * mem::size_of::<KeyBackupElem>()) as *mut KeyBackupElem };
    let slice = unsafe { std::slice::from_raw_parts_mut(dataptr, mapped.len()) };
    slice.copy_from_slice(mapped.as_ref());
    Self { ptr: dataptr, count: mapped.len() }
  }
}

#[no_mangle]
pub unsafe extern "C" fn keychain_manager_get_keys_data(
  manager: &KeychainManagerPtr, encrypted: *const u8, encrypted_len: usize, password: PChar,
  data: &mut KeyBackupPtr, error: &mut ErrorPtr
) -> bool {
  let data_slice = std::slice::from_raw_parts(encrypted, encrypted_len);
  let pwd = CStr::from_ptr(password as *const c_char).to_str().unwrap();
  manager.as_ref()
    .get_keys_data(data_slice, pwd)
    .map(|backup| KeyBackupPtr::from(backup))
    .response(data, error)
}

#[no_mangle]
pub unsafe extern "C" fn delete_key_backup(backup: &mut KeyBackupPtr) {
  backup.free();
}