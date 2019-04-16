use std::panic;
use keychain::Error;
use result::ErrorPtr;

pub fn handle_exception<F: FnOnce() -> R + panic::UnwindSafe, R>(func: F) -> Result<R, ErrorPtr> {
  handle_exception_result(|| Ok(func()))
}

pub fn handle_exception_result<F: FnOnce() -> Result<R, Error> + panic::UnwindSafe, R>(func: F) -> Result<R, ErrorPtr> {
  match panic::catch_unwind(func) {
    Ok(res) => res.map_err(|err| ErrorPtr::new(&err)),
    Err(err) => {
      if let Some(string) = err.downcast_ref::<String>() {
        return Err(ErrorPtr::panic(&string));
      } else if let Some(string) = err.downcast_ref::<&'static str>() {
        return Err(ErrorPtr::panic(string));
      }
      return Err(ErrorPtr::panic(&format!("Reason: {:?}", err)));
    }
  }
}

pub fn hide_exceptions() {
  panic::set_hook(Box::new(|_| {}));
}