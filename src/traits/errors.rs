use std::fmt;
use std::error;

pub trait DisplayAndDebug: fmt::Display + fmt::Debug {}
impl<T> DisplayAndDebug for T where T: fmt::Display, T: fmt::Debug {}

#[derive(Debug)]
pub struct NotFoundError {
  pub what: Box<DisplayAndDebug>
}

impl fmt::Display for NotFoundError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} not found", self.what)
  }
}

impl error::Error for NotFoundError {}