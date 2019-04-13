use manager::KeychainManager;
use network::Network;
use result::{ Data, CResult, PChar, Error };
use keychain::{ Network as RNetwork };
use std::os::raw::c_char;
use std::ffi::CStr;
use libc::{ malloc, free, c_void };
use std::mem;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KeyBackupElem {
  pub network: Network,
  pub data: Data
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KeyBackup {
  pub ptr: *const KeyBackupElem,
  pub count: usize
}

impl KeyBackup {
  fn from(data: Vec<(RNetwork, Vec<u8>)>) -> Self {
    let mapped: Vec<KeyBackupElem> = data.into_iter()
      .map(|(net, data)|
        KeyBackupElem { network: net.into(), data: Data::from(data) }
      ).collect();

    let dataptr = unsafe { malloc(mapped.len() * mem::size_of::<KeyBackupElem>()) as *mut KeyBackupElem };
    let slice = unsafe { std::slice::from_raw_parts_mut(dataptr, mapped.len()) };
    slice.copy_from_slice(mapped.as_ref());
    Self { ptr: dataptr, count: mapped.len() }
  }
}

#[no_mangle]
pub unsafe extern "C" fn keychain_manager_get_keys_data(
  manager: &KeychainManager, encrypted: *const u8, encrypted_len: usize, password: PChar,
  data: &mut KeyBackup, error: &mut Error
) -> bool {
  let data_slice = std::slice::from_raw_parts(encrypted, encrypted_len);
  let pwd = CStr::from_ptr(password as *const c_char).to_str().unwrap();
  manager.rust()
    .get_keys_data(data_slice, pwd)
    .map(|backup| KeyBackup::from(backup))
    .response(data, error)
}

#[no_mangle]
pub unsafe extern "C" fn delete_key_backup(backup: &mut KeyBackup) {
  if backup.ptr.is_null() { return; }
  let kbslice = std::slice::from_raw_parts_mut(backup.ptr as *mut KeyBackupElem, backup.count);
  for elem in kbslice.into_iter() {
    elem.data.free();
  }
  free(backup.ptr as *mut c_void);
  backup.ptr = std::ptr::null_mut();
}