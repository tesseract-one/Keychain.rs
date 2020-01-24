use crate::error::ErrorPtr;

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
