use keychain::Error as RError;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub trait Ptr<T: ?Sized> {
  unsafe fn as_ref(&self) -> &T;
  unsafe fn free(&mut self);
}

pub trait ArrayPtr<T> {
  unsafe fn as_ref(&self) -> &[T];
  unsafe fn free(&mut self);
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ErrorType {
  Panic = -1,
  WrongPassword = 0,
  NotEnoughData = 1,
  SeedIsNotSaved = 2,
  CantCalculateSeedSize = 3,
  DataError = 4,
  InvalidSeedSize = 5,
  KeyDoesNotExist = 6,
  KeyAlreadyExist = 7,
  NetworkIsNotSupported = 8,
  KeyError = 9,
  KeyPathError = 10,
  MnemonicError = 11
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ErrorPtr {
  error_type: ErrorType,
  message: CharPtr
}

impl Ptr<str> for ErrorPtr {
  unsafe fn as_ref(&self) -> &str {
    (&self.message as &Ptr<str>).as_ref()
  }

  unsafe fn free(&mut self) {
    if self.message.is_null() {
      return;
    }
    self.message.free();
  }
}

impl ErrorPtr {
  fn error_type(err: &RError) -> ErrorType {
    match err {
      &RError::WrongPassword => ErrorType::WrongPassword,
      &RError::NotEnoughData => ErrorType::NotEnoughData,
      &RError::SeedIsNotSaved => ErrorType::SeedIsNotSaved,
      &RError::CantCalculateSeedSize(_, _) => ErrorType::CantCalculateSeedSize,
      &RError::DataError(_) => ErrorType::DataError,
      &RError::InvalidSeedSize(_) => ErrorType::InvalidSeedSize,
      &RError::KeyDoesNotExist(_) => ErrorType::KeyDoesNotExist,
      &RError::KeyAlreadyExist(_) => ErrorType::KeyAlreadyExist,
      &RError::NetworkIsNotSupported(_) => ErrorType::NetworkIsNotSupported,
      &RError::KeyError(_, _) => ErrorType::KeyError,
      &RError::KeyPathError(_) => ErrorType::KeyPathError,
      &RError::MnemonicError(_) => ErrorType::MnemonicError
    }
  }

  pub fn new(err: &RError) -> Self {
    Self { error_type: Self::error_type(err), message: format!("{}", err).to_cstr() }
  }

  pub fn panic(msg: &str) -> Self {
    Self { error_type: ErrorType::Panic, message: msg.to_cstr() }
  }
}

#[no_mangle]
pub unsafe extern "C" fn delete_error(error: &mut ErrorPtr) {
  error.free();
}

pub trait CResult<T> {
  fn response(&self, val: &mut T, error: &mut ErrorPtr) -> bool;
}

impl<T: Copy> CResult<T> for Result<T, ErrorPtr> {
  fn response(&self, val: &mut T, error: &mut ErrorPtr) -> bool {
    match self {
      Err(err) => {
        *error = *err;
        false
      }
      Ok(value) => {
        *val = *value;
        true
      }
    }
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DataPtr {
  ptr: *const u8,
  len: usize
}

impl ArrayPtr<u8> for DataPtr {
  unsafe fn as_ref(&self) -> &[u8] {
    std::slice::from_raw_parts(self.ptr, self.len)
  }

  unsafe fn free(&mut self) {
    if self.ptr.is_null() {
      return;
    }
    let _ = Vec::from_raw_parts(self.ptr as *mut u8, self.len, self.len);
    self.ptr = std::ptr::null();
  }
}

impl From<&[u8]> for DataPtr {
  fn from(data: &[u8]) -> Self {
    Vec::from(data).into()
  }
}

impl From<Vec<u8>> for DataPtr {
  fn from(data: Vec<u8>) -> Self {
    let len = data.len();
    let mut slice = data.into_boxed_slice();
    let out = slice.as_mut_ptr();
    std::mem::forget(slice);
    Self { ptr: out, len: len }
  }
}

#[no_mangle]
pub unsafe extern "C" fn delete_data(data: &mut DataPtr) {
  data.free();
}

pub type CharPtr = *const c_char;

impl Ptr<str> for CharPtr {
  unsafe fn as_ref(&self) -> &str {
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
