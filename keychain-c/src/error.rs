use crate::utils::ptr::UnsizedPtr;
use crate::utils::string::{CharPtr, ToCString};
use keychain::Error as RError;

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

impl UnsizedPtr for ErrorPtr {
  type Type = str;

  unsafe fn get_ref(&self) -> &str {
    (&self.message as &dyn UnsizedPtr<Type = str>).get_ref()
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
