use crate::error::ErrorPtr;
use crate::utils::panic::handle_exception_result;
use crate::utils::ptr::UnsizedPtr;
use crate::utils::result::CResult;
use crate::utils::string::CharPtr;
use keychain::{GenericKeyPath, KeyPath as IKeyPath};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KeyPath {
  purpose: u32,
  coin: u32,
  account: u32,
  change: u32,
  address: u32
}

impl IKeyPath for KeyPath {
  fn purpose(&self) -> u32 {
    self.purpose
  }
  fn coin(&self) -> u32 {
    self.coin
  }
  fn account(&self) -> u32 {
    self.account
  }
  fn change(&self) -> u32 {
    self.change
  }
  fn address(&self) -> u32 {
    self.address
  }
}

impl From<&dyn IKeyPath> for KeyPath {
  fn from(path: &dyn IKeyPath) -> Self {
    Self {
      purpose: path.purpose(),
      coin: path.coin(),
      account: path.account(),
      change: path.change(),
      address: path.address()
    }
  }
}

#[no_mangle]
pub unsafe extern "C" fn keypath_from_string(
  string: CharPtr, key_path: &mut KeyPath, error: &mut ErrorPtr
) -> bool {
  handle_exception_result(|| {
    GenericKeyPath::from(string.get_ref())
      .map_err(|err| err.into())
      .map(|path| (&path as &dyn IKeyPath).into())
  })
  .response(key_path, error)
}
