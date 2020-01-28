use super::ptr::ArrayPtr;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DataPtr {
  ptr: *const u8,
  len: usize
}

impl ArrayPtr for DataPtr {
  type Element = u8;

  fn from_ptr(ptr: *const u8, len: usize) -> DataPtr {
    Self { ptr, len }
  }

  fn get_ptr(&self) -> *const u8 {
    self.ptr
  }

  fn get_count(&self) -> usize {
    self.len
  }

  fn set_ptr(&mut self, ptr: *const u8) {
    self.ptr = ptr;
  }
}

#[no_mangle]
pub unsafe extern "C" fn delete_data(data: &mut DataPtr) {
  data.free();
}
