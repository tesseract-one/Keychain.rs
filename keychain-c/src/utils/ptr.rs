use std::ffi::c_void;

pub trait UnsizedPtr {
  type Type: ?Sized;

  unsafe fn get_ref(&self) -> &Self::Type;
  unsafe fn free(&mut self);
}

pub trait SizedPtr {
  type Type;

  fn from_ptr(ptr: *mut c_void) -> Self;
  fn get_ptr(&self) -> *mut c_void;
  fn set_ptr(&mut self, ptr: *mut c_void);

  fn new(val: Self::Type) -> Self
  where
    Self: Sized
  {
    Self::from_ptr(Box::into_raw(Box::new(val)) as *mut c_void)
  }

  unsafe fn get_ref(&self) -> &Self::Type {
    (self.get_ptr() as *mut Self::Type).as_ref().unwrap()
  }

  unsafe fn free(&mut self) {
    if self.get_ptr().is_null() {
      return;
    }
    let _: Box<Self::Type> = Box::from_raw(self.get_ptr() as *mut Self::Type);
    self.set_ptr(std::ptr::null_mut());
  }
}

pub trait ArrayPtr {
  type Element;

  fn from_ptr(ptr: *const Self::Element, count: usize) -> Self;
  fn get_ptr(&self) -> *const Self::Element;
  fn get_count(&self) -> usize;
  fn set_ptr(&mut self, ptr: *const Self::Element);

  fn new(val: Vec<Self::Element>) -> Self
  where
    Self: Sized
  {
    let count = val.len();
    let mut slice = val.into_boxed_slice();
    let out = slice.as_mut_ptr();
    std::mem::forget(slice);
    Self::from_ptr(out, count)
  }

  unsafe fn get_ref(&self) -> &[Self::Element] {
    std::slice::from_raw_parts(self.get_ptr(), self.get_count())
  }

  unsafe fn free(&mut self) {
    if self.get_ptr().is_null() {
      return;
    }
    let _ =
      Vec::from_raw_parts(self.get_ptr() as *mut Self::Element, self.get_count(), self.get_count());
    self.set_ptr(std::ptr::null_mut());
  }
}

pub trait IntoArrayPtr<T>: Sized {
  fn into_array_ptr(self) -> T;
}

pub trait FromArrayPtr<T>: Sized {
  fn from_array_ptr(_: T) -> Self;
}

impl<T, P> IntoArrayPtr<P> for T
where
  P: FromArrayPtr<T>
{
  fn into_array_ptr(self) -> P {
    P::from_array_ptr(self)
  }
}

impl<E, T> FromArrayPtr<Vec<E>> for T
where
  T: ArrayPtr<Element = E>
{
  fn from_array_ptr(vec: Vec<E>) -> Self {
    T::new(vec)
  }
}

impl<E, T> FromArrayPtr<&[E]> for T
where
  E: Clone,
  T: ArrayPtr<Element = E>
{
  fn from_array_ptr(slice: &[E]) -> Self {
    FromArrayPtr::from_array_ptr(Vec::from(slice))
  }
}
