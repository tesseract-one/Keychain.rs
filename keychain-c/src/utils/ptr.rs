pub trait Ptr<T: ?Sized> {
  unsafe fn rust_ref(&self) -> &T;
  unsafe fn free(&mut self);
}
