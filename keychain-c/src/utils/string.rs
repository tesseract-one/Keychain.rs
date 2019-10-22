use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use super::ptr::Ptr;

pub type CharPtr = *const c_char;

impl Ptr<str> for CharPtr {
  unsafe fn rust_ref(&self) -> &str {
    CStr::from_ptr(*self).to_str().unwrap()
  }

  unsafe fn free(&mut self) {
    let _ = CString::from_raw(*self as *mut c_char);
    *self = std::ptr::null();
  }
}

#[no_mangle]
pub unsafe extern "C" fn delete_string(ptr: CharPtr) {
  let mut mptr = ptr;
  mptr.free();
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
