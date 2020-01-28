use super::ptr::UnsizedPtr;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub type CharPtr = *const c_char;

impl UnsizedPtr for CharPtr {
  type Type = str;

  unsafe fn get_ref(&self) -> &str {
    CStr::from_ptr(*self).to_str().unwrap()
  }

  unsafe fn free(&mut self) {
    let _ = CString::from_raw(*self as *mut c_char);
    *self = std::ptr::null();
  }
}

#[no_mangle]
pub unsafe extern "C" fn delete_string(mut ptr: CharPtr) {
  (&mut ptr).free();
}

pub trait ToCString {
  fn to_cstr(&self) -> CharPtr;
}

impl ToCString for &str {
  fn to_cstr(&self) -> CharPtr {
    CString::new(self.as_bytes()).unwrap().into_raw()
  }
}

impl ToCString for String {
  fn to_cstr(&self) -> CharPtr {
    CString::new(self.as_bytes()).unwrap().into_raw()
  }
}
