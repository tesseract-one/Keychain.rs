use keychain::{ Error as RError };
use std::os::raw::c_char;
use std::error::{ Error as IError };
use libc::{ malloc, free, c_void };

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ErrorType {
  WrongPassword = 0,
  NotEnoughData = 1,
  CantCalculateSeedSize = 2,
  DataError = 3,
  EntropyGeneratorError = 4,
  InvalidSeedSize = 5,
  KeyDoesNotExist = 6,
  KeyError = 7,
  KeyPathError = 8,
  MnemonicError = 9
}

#[repr(C)]
pub struct Error {
  error_type: ErrorType,
  message: PChar
}

impl Error {
  fn error_type(err: &RError) -> ErrorType {
    match err {
      &RError::WrongPassword => ErrorType::WrongPassword,
      &RError::NotEnoughData => ErrorType::NotEnoughData,
      &RError::CantCalculateSeedSize(_, _) => ErrorType::CantCalculateSeedSize,
      &RError::DataError(_) => ErrorType::DataError,
      &RError::EntropyGeneratorError(_) => ErrorType::EntropyGeneratorError,
      &RError::InvalidSeedSize(_) => ErrorType::InvalidSeedSize,
      &RError::KeyDoesNotExist(_) => ErrorType::KeyDoesNotExist,
      &RError::KeyError(_, _) => ErrorType::KeyError,
      &RError::KeyPathError(_) => ErrorType::KeyPathError,
      &RError::MnemonicError(_) => ErrorType::MnemonicError
    }
  }

  pub fn new(err: &RError) -> Self {
    Self {
      error_type: Self::error_type(err),
      message: err.description().to_cstr()
    }
  }

  pub fn free(&mut self) {
    if !self.message.is_null() {
      unsafe { free(self.message as *mut c_void); }
    }
    self.message = std::ptr::null_mut();
  }
}

#[no_mangle]
pub unsafe extern "C" fn delete_error(error: &mut Error) {
  error.free();
}

pub trait CResult<T> {
  fn response(&self, val: &mut T, error: &mut Error) -> bool;
}

impl<T: Copy> CResult<T> for Result<T, RError> {
  fn response(&self, val: &mut T, error: &mut Error) -> bool {
    match self {
      Err(err) => {
        *error = Error::new(err);
        false
      },
      Ok(value) => {
        *val = *value;
        true
      }
    }
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Data {
  ptr: *const u8,
  len: usize 
}

impl Data {
  pub fn free(&mut self) {
    if !self.ptr.is_null() {
      unsafe { free(self.ptr as *mut c_void); }
    }
    self.ptr = std::ptr::null_mut();
  }
}

impl From<&[u8]> for Data {
  fn from(data: &[u8]) -> Self {
    let dataptr = unsafe { malloc(data.len()) as *mut u8 };
    let slice = unsafe { std::slice::from_raw_parts_mut(dataptr, data.len()) };
    slice.copy_from_slice(data);
    Self { ptr: dataptr, len: data.len() }
  }
}

impl From<Vec<u8>> for Data {
  fn from(data: Vec<u8>) -> Self {
    let dataptr = unsafe { malloc(data.len()) as *mut u8 };
    let slice = unsafe { std::slice::from_raw_parts_mut(dataptr, data.len()) };
    slice.copy_from_slice(data.as_ref());
    Self { ptr: dataptr, len: data.len() }
  }
}

impl From<&Vec<u8>> for Data {
  fn from(data: &Vec<u8>) -> Self {
    let dataptr = unsafe { malloc(data.len()) as *mut u8 };
    let slice = unsafe { std::slice::from_raw_parts_mut(dataptr, data.len()) };
    slice.copy_from_slice(data.as_ref());
    Self { ptr: dataptr, len: data.len() }
  }
}

#[no_mangle]
pub unsafe extern "C" fn delete_data(data: &mut Data) {
  data.free();
}

pub type PChar = *const c_char;

pub trait ToCString {
  fn to_cstr(&self) -> PChar; 
}

impl ToCString for &str {
  fn to_cstr(&self) -> PChar {
    let len = self.len() + 1;
    let ptr = unsafe { malloc(len) as *mut u8 };
    let ref mut slice = unsafe { std::slice::from_raw_parts_mut(ptr, len) };
    slice.copy_from_slice(self.as_bytes());
    slice[len-1] = b'\0';
    ptr as PChar
  } 
}

impl ToCString for String {
  fn to_cstr(&self) -> PChar {
    let slice: &str = self.as_ref();
    slice.to_cstr()
  } 
}