use traits::messaging::{Message as MessageTrait, NoResponse};
use super::error::{Message as Error};
use errors::WalletLoadError;

pub struct Response {
  id: u32
}

impl Response {
  pub fn new(id: u32) -> Self {
    Response { id: id }
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
  wallet_id: &'a str,
  password: &'a str
}

impl<'a> Message<'a> {
  pub fn new(id: u32, wallet_id: &'a str, password: &'a str) -> Self {
    Message {
      id: id,
      wallet_id: wallet_id,
      password: password
    }
  }

  pub fn get_wallet_id(&self) -> &'a str {
    self.wallet_id
  }

  pub fn get_password(&self) -> &'a str {
    self.password
  }
}

impl<'a> MessageTrait for Message<'a> {
  type Response = Result<Response, Error<WalletLoadError<'a>>>;

  fn get_id(&self) -> u32 {
    self.id
  }
}