use keychain::{ Error as RError };
use std::os::raw::c_uchar;
use libc::{ malloc, free, c_void };

#[repr(C)]
pub struct Error {
  id: PChar,
  message: PChar
}

impl Error {
  pub fn new(_err: &RError) -> Self {
    Self { id: std::ptr::null_mut(), message: std::ptr::null_mut() }
  }

  pub fn free(&mut self) {
    if !self.id.is_null() {
      unsafe { free(self.id as *mut c_void); }
    }
    if !self.message.is_null() {
      unsafe { free(self.message as *mut c_void); }
    }
    self.id = std::ptr::null_mut();
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

pub type PChar = *const c_uchar;

pub trait ToCString {
  fn to_cstr(&self) -> PChar; 
}

impl ToCString for String {
  fn to_cstr(&self) -> PChar {
    let len = self.len() + 1;
    let ptr = unsafe { malloc(len) as *mut c_uchar };
    let ref mut slice = unsafe { std::slice::from_raw_parts_mut(ptr, len) };
    slice.copy_from_slice(self.as_bytes());
    slice[len-1] = b'\0';
    ptr
  } 
}