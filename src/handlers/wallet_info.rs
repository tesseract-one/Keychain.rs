use traits::messaging::*;
use tesseract::Tesseract;
use messages::*;
use futures::prelude::*;

impl<'a> Handler<has_saved_wallet::Message<'a>> for Tesseract<'a> {
  fn message(&self, message: &has_saved_wallet::Message, from: &Handler<has_saved_wallet::Response>) {
    self.has_wallet(message.get_wallet_id()).map(|loaded| {
      from.message(&has_saved_wallet::Response::new(message.get_id(), false), self);
      loaded
    });
  }
}

impl<'a> Handler<is_wallet_loaded::Message<'a>> for Tesseract<'a> {
  fn message(&self, message: &is_wallet_loaded::Message, from: &Handler<is_wallet_loaded::Response>) {
    from.message(
      &is_wallet_loaded::Response::new(message.get_id(), self.is_wallet_loaded(message.get_wallet_id())),
      self
    ); 
  }
}
