use super::error::JavaError;
use std::fmt::Display;

pub trait Zip<T1, T2, E> {
  fn zip(self, result: Result<T2, E>) -> Result<(T1, T2), E>;
}

impl<T1, T2, E> Zip<T1, T2, E> for Result<T1, E> {
  fn zip(self, result: Result<T2, E>) -> Result<(T1, T2), E> {
    self.and_then(|r1| result.map(|r2| (r1, r2)))
  }
}

pub trait IntoResult<T> {
  fn into_result(self) -> Result<T, JavaError>;
}

impl<T, E: Display> IntoResult<T> for Result<T, E> {
  fn into_result(self) -> Result<T, JavaError> {
    self.map_err(|err| JavaError::from(err))
  }
}
