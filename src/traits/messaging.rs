use std;

pub trait Message {
  type Response : Message;

  fn get_id(&self) -> u32;
}

pub struct NoResponse;
impl Message for NoResponse {
  type Response = NoResponse;

  fn get_id(&self) -> u32 {
    std::u32::MAX
  }
}

impl<VM, EM> Message for Result<VM, EM> where VM: Message, EM: Message {
  type Response = NoResponse;

  fn get_id(&self) -> u32 {
    match self {
      Ok(m) => m.get_id(),
      Err(m) => m.get_id()
    }
  }
}

pub trait AcceptsMessages {}

pub trait Handler<M>: AcceptsMessages  where M : Message {
  fn message(&self, message: &M, from: &Handler<M::Response>);
}

impl<T: AcceptsMessages> Handler<NoResponse> for T {
  fn message(&self, _: &NoResponse, _: &Handler<NoResponse>) {}
}
