use std::collections::HashMap;
use traits::network::Network;
use traits::messaging::*;
use traits::storage::{Storage, StorageLoadError};
use messages::error;
use futures::prelude::*;
use futures::future;
use std::error::Error;
use errors::WalletLoadError;
use aes;
use multi;

pub struct Tesseract<'a> {
  networks:Vec<Box<Network>>,
  error_handler: &'a Handler<error::Message<String>>,
  storage: &'a Storage,
  wallets: HashMap<String, multi::MultiWallet>
}
impl<'a> AcceptsMessages for Tesseract<'a> {}

impl<'a> Tesseract<'a> {
  const DATA_HEADER: [u8; 16] = [0xff; 16];

  pub fn register_network(&mut self, network: Box<Network>) {
    self.networks.push(network);
  }

  pub fn error(&self, error: &error::Message<String>) {
    self.error_handler.message(error, self);
  }

  pub fn has_wallet(&self, id: &str) -> Box<Future<Item = bool, Error = Box<Error>>> {
    self.storage.has_bytes(id)
  }

  pub fn is_wallet_loaded(&self, id: &str) -> bool {
    self.wallets.contains_key(id)
  }

  // pub fn load_wallet<'b>(&self, id: &'b str, password: &'b str) -> Box<Future<Item = (), Error = WalletLoadError<'b>> + 'b> {
  //   let ref mut wallets = &self.wallets;
  //   let future = self.storage.load_bytes(id)
  //     .then(move |res| {
  //       match res {
  //         Err(err) => match err {
  //           StorageLoadError::NoKey(key) => future::err(WalletLoadError::NoWallet(key)),
  //           StorageLoadError::StorageError(key, err) => future::err(WalletLoadError::UnknownError(key, err))
  //         },
  //         Ok(bytes) => {
  //           if bytes.len() < Self::DATA_HEADER.len() {
  //             return future::err(WalletLoadError::BadData(id));
  //           }
  //           let mut header = aes::decrypt(&bytes, password);
  //           let data = header.split_off(Self::DATA_HEADER.len());
  //           if header != Self::DATA_HEADER {
  //             return future::err(WalletLoadError::WrongPassword(id))
  //           }
  //           if let Some(wallet) = multi::MultiWallet::from_data(&data) {
  //             wallets.insert(String::from(id), wallet);
  //             return future::ok(());
  //           }
            
  //           return future::err(WalletLoadError::BadData(id));
  //         }
  //       }
  //     });
  //   Box::new(future)
  // }

  pub fn new(error_handler: &'a Handler<error::Message<String>>, storage: &'a Storage) -> Self {
    Tesseract {
      networks: Vec::new(),
      error_handler: error_handler,
      storage: storage,
      wallets: HashMap::new()
    }
  }
}