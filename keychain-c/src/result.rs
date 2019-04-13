
#[repr(C)]
pub enum CResult<T: Sized> {
  Err,
  Ok(T)
}