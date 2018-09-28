use futures::prelude::*;
use futures::future;
use util::future::*;
use std::collections::HashMap;
use std::sync::Arc;

use storage::*;
use rand::Random;
use wallet::HDWallet;
use network::{ Network, NetworkType };
use networks::all_networks;
use key_storage::KeyStorage;
use mnemonic;
use crypt;
use data::{ VersionedData, WalletDataV1 };

pub enum Error {
  WalletDoesNotExist(String),
  StorageError(String, Box<std::error::Error>),
  WrongPassword(String),
  NotEnoughData(String),
  PrivateKeyDoesNotExist(String, NetworkType),
  DataParseError(String, serde_cbor::error::Error),
  UnknownError
}

impl Error {
  fn from_storage_error(err: StorageLoadError) -> Self {
    match err {
      StorageLoadError::NoKey(name) => Error::WalletDoesNotExist(name),
      StorageLoadError::StorageError(name, err) => Error::StorageError(name, err),
      _ => Error::UnknownError
    }
  }

  fn from_decrypt_error(name: String, err: crypt::DecryptError) -> Self {
    match err {
      crypt::DecryptError::NotEnoughData => Error::NotEnoughData(name),
      crypt::DecryptError::WrongPassword => Error::WrongPassword(name),
      _ => Error::UnknownError
    }
  }
}

pub type Networks = Arc<HashMap<NetworkType, Box<Network>>>;

pub struct HDWalletProvider {
  storage: Box<Storage>,
  networks: Networks,
  random: Box<Random>
}

impl HDWalletProvider {
  pub fn new(storage: Box<Storage>, random: Box<Random>) -> Self {
    Self::new_with_networks(storage, random, all_networks())
  }

  pub fn new_with_networks(storage: Box<Storage>, random: Box<Random>, networks: &[Box<Network>]) -> Self {
    let map: HashMap<NetworkType, Box<Network>> = networks.iter().map(|nt| { (nt.get_type(), *nt) }).collect();
    Self { storage, random, networks: Arc::new(map) }
  }

  pub fn has_network(&self, nt: &NetworkType) -> bool {
    self.networks.contains_key(nt)
  }

  pub fn get_network<'a>(&'a self, nt: &NetworkType) -> &'a Network { // TODO: Error handling
    self.networks.get(nt).unwrap().as_ref()
  }

  pub fn has_wallet(&self, name: &str) -> Box<Future<Item = bool, Error = Error>> {
    self.storage.has_bytes(name).map_err(Error::from_storage_error).into_box()
  }

  pub fn load_wallet(&self, name: &str, password: &str) -> Box<Future<Item = HDWallet, Error = Error>> {
    let sname = name.to_owned();
    let networks = self.networks;
    self.load_wallet_data(sname, password.to_owned())
      .map(|pks| HDWallet::new(&sname, pks, networks))
      .into_box()
  }

  pub fn create_wallet(&self, name: &str, password: &str) -> Box<Future<Item = (HDWallet, String), Error = Error>> {
    let networks: Vec<&Network> = self.networks.values().map(|b| b.as_ref()).collect();
    let mnemonic = mnemonic::create_mnemonic(
      &networks,
      self.random.as_ref()
    );
    
  }

  pub fn restore_wallet(&self, name: &str, mnemonic: &str, password: &str) -> Box<Future<Item = HDWallet, Error = Error>> {
    
  }

  pub fn rename_wallet(&self, old_name: &str, new_name: &str, password: &str) -> Box<Future<Item = (), Error = Error>> {

  }

  pub fn remove_wallet(&self, name: &str, password: &str) -> Box<Future<Item = (), Error = Error>> {
    let sname = name.to_owned();
    let storage = self.storage;
    self.load_wallet_data_raw(sname, password.to_owned())
      .and_then(|_| storage.remove_bytes(&sname).map_err(Error::from_storage_error))
      .into_box()
  }

  pub fn private_key(&self, name: &str, network: &NetworkType, password: &str) -> Box<Future<Item = Vec<u8>, Error = Error>> {
    let sname = name.to_owned();
    let snetwork = network.to_owned();
    self.load_wallet_data(sname, password.to_owned())
      .and_then(|pks| {
        match pks.key(&snetwork) {
          Some(key) => future::ok(key),
          None => future::err(Error::PrivateKeyDoesNotExist(sname, snetwork))
        }
      })
      .into_box()
  }
}

// Private methods
impl HDWalletProvider {
  fn save_wallet_data(&self, name: &str, password: &str, keys: &KeyStorage) -> Box<Future<Item = (), Error = Error>> {
    let wdata = WalletDataV1 { private_keys: keys.clone() };
    let data = VersionedData::new(&wdata).and_then(|vdata| vdata.to_bytes());
    if let Err(err) = data {
      return future::err(Error::DataParseError(name.to_owned(), err)).into_box();
    }
    let crypted = crypt::encrypt(&data.unwrap(), password, self.random.as_ref());
    self.storage.save_bytes(name, &crypted)
      .map_err(Error::from_storage_error)
      .into_box()
  }

  fn load_wallet_data_raw(&self, name: String, password: String) ->  Box<Future<Item = Vec<u8>, Error = Error>> {
    self.storage.load_bytes(&name)
      .map_err(Error::from_storage_error)
      .and_then(|data| {
        match crypt::decrypt(&data, &password) {
          Ok(decrypted) => future::ok(decrypted),
          Err(err) => future::err(Error::from_decrypt_error(name, err))
        }
      })
      .into_box()
  }

  fn load_wallet_data(&self, name: String, password: String) -> Box<Future<Item = KeyStorage, Error = Error>> {
    self.load_wallet_data_raw(name, password)
      .and_then(|data| { 
        match VersionedData::from_bytes(&data).and_then(|vdata| vdata.get_data()) {
          Ok(wdata) => future::ok(wdata.private_keys),
          Err(err) => future::err(Error::DataParseError(name, err))
        }
      }).into_box()
  }
}