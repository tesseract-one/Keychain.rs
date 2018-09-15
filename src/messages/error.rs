use traits::messaging::{Message as MessageTrait, NoResponse};

#[derive(Debug)]
pub struct Message<T: Clone> {
  id: u32,
  error: T
}

impl<T> Message<T> where T: Clone {
  pub fn new(id: u32, error: &T) -> Self  {
    Message {
      error: error.clone(),
      id: id
    }
  }

  pub fn get_error(&self) -> &T {
    &self.error
  }
}

impl<T> MessageTrait for Message<T> where T: Clone {
  type Response = NoResponse;

  fn get_id(&self) -> u32 {
    self.id
  }
}