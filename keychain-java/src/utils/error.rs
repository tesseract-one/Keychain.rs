use std::fmt::Display;

#[derive(Debug)]
pub struct JavaError {
  message: String
}

impl JavaError {
  pub fn from<E: Display>(err: E) -> Self {
    JavaError { message: format!("{}", err) }
  }
}
