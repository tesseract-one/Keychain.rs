use traits::messaging::{Message as MessageTrait, NoResponse};

pub struct Response {
  id: u32,
  wallet_loaded: bool
}

impl Response {
  pub fn new(id: u32, wallet_loaded: bool) -> Self {
    Response {
      wallet_loaded: wallet_loaded,
      id: id
    }
  }

  pub fn get_wallet_loaded(&self) -> bool {
    self.wallet_loaded
  }
}

impl MessageTrait for Response {
  type Response = NoResponse;

  fn get_id(&self) -> u32 {
    self.id
  }
}

pub struct Message<'a> {
  id: u32,
  wallet_id: &'a str
}

impl<'a> Message<'a> {
  pub fn new(id: u32, wallet_id: &'a str) -> Self {
    Message {
      id: id,
      wallet_id: wallet_id
    }
  }

  pub fn get_wallet_id(&self) -> &'a str {
    self.wallet_id
  }
}

impl<'a> MessageTrait for Message<'a> {
  type Response = Response;

  fn get_id(&self) -> u32 {
    self.id
  }
}