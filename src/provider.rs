use futures::prelude::*;
use futures::future;
use util::future::*;
use std::collections::HashMap;
use std::sync::{ Arc, Mutex };
use rand::rngs::OsRng;

use storage::{ Storage, Error as StorageError };
use wallet::HDWallet;
use network::{ Network, MnemonicError };
use network_type::NetworkType;
use networks::all_networks;
use key_storage::KeyStorage;
use mnemonic::{ generate as generate_mnemonic, Language };
use util::crypt;
use data::{ VersionedData, WalletDataV1 };

#[derive(Debug)]
pub enum Error {
  WalletDoesNotExist(String),
  StorageError(String, Box<std::error::Error>),
  WrongPassword(String),
  NotEnoughData(String),
  CantCalculateSeedSize,
  InvalidSeedSize(usize),
  PrivateKeyDoesNotExist(String, NetworkType),
  DataParseError(String, serde_cbor::error::Error),
  MnemonicError(String),
  UnknownError
}

impl From<StorageError> for Error {
  fn from(err: StorageError) -> Self {
    match err {
      StorageError::KeyDoesNotExist(name) => Error::WalletDoesNotExist(name),
      StorageError::InternalError(name, err) => Error::StorageError(name, err),
    }
  }
}

impl Error {
  fn from_decrypt_error(name: String, err: crypt::DecryptError) -> Self {
    match err {
      crypt::DecryptError::NotEnoughData => Error::NotEnoughData(name),
      crypt::DecryptError::DecryptionFailed => Error::WrongPassword(name),
    }
  }
}

pub type Networks = Arc<HashMap<NetworkType, Box<Network>>>;

pub struct HDWalletProvider {
  storage: Arc<Storage>,
  networks: Networks,
  random: Arc<Mutex<OsRng>>,
  seed_size: usize
}

impl HDWalletProvider {
  pub fn new(storage: Box<Storage>) -> Self {
    Self::with_network_objs(storage, all_networks()).unwrap() // It's safe. Our network key sizes should align
  }

  pub fn with_networks(storage: Box<Storage>, networks: &[NetworkType]) -> Self {
    let filtered: Vec<Box<Network>> = all_networks()
      .into_iter()
      .filter(|network| {
        let ntype = network.get_type();
        networks.iter().position(|nt| nt == &ntype).is_some()
      })
      .collect();
    Self::with_network_objs(storage, filtered).unwrap() // It's safe. Our network key sizes should align
  }

  #[cfg(feature = "custom-networks")]
  pub fn with_custom_networks(
    storage: Box<Storage>,
    networks: Vec<Box<Network>>
  ) -> Result<Self, Error> {
    Self::with_network_objs(storage, networks)
  }

  pub fn has_network(&self, nt: &NetworkType) -> bool {
    self.networks.contains_key(nt)
  }

  pub fn get_network<'a>(&'a self, nt: &NetworkType) -> Option<&'a Network> {
    self.networks.get(nt).map(|n| n.as_ref())
  }

  pub fn has_wallet(&self, name: &str) -> Box<Future<Item = bool, Error = Error>> {
    self.storage.has_bytes(name).from_err().into_box()
  }

  pub fn load_wallet(&self, name: &str, password: &str) -> Box<Future<Item = HDWallet, Error = Error>> {
    let sname = name.to_owned();
    let networks = self.networks.clone();
    self.load_wallet_data(name, password)
      .map(move |pks| HDWallet::new(&sname, pks, networks))
      .into_box()
  }

  pub fn create_wallet(&self, name: &str, password: &str, language: Option<Language>) -> Box<Future<Item = (HDWallet, String), Error = Error>> {
    let lang = language.unwrap_or(Language::default());
    match generate_mnemonic(self.seed_size, lang, &mut self.random.lock().unwrap()) {
      Ok(mnemonic) => self.restore_wallet(name, &mnemonic, password).map(|wallet| (wallet, mnemonic)).into_box(),
      Err(_) => future::err(Error::InvalidSeedSize(self.seed_size)).into_box()
    }
  }

  pub fn restore_wallet(&self, name: &str, mnemonic: &str, password: &str) -> Box<Future<Item = HDWallet, Error = Error>> {
    let sname = name.to_owned();
    let start: Result<Vec<(NetworkType, Vec<u8>)>, MnemonicError> = Ok(Vec::with_capacity(self.networks.len()));
    let keys = self.networks.as_ref().into_iter()
      .fold(start, |res, (nt, network)| {
        match res {
          Err(err) => Err(err),
          Ok(mut vec) => network.key_data_from_mnemonic(mnemonic).map(|data| {
            vec.push((*nt, data));
            vec
          })
        }
      });
    if let Err(err) = keys {
      return future::err(Error::MnemonicError(sname)).into_box();
    }
    let key_storage = KeyStorage::new(&keys.unwrap());
    let networks = Arc::clone(&self.networks);
    self.save_wallet_data(name, password, &key_storage)
      .map(move |_| HDWallet::new(&sname, key_storage, networks))
      .into_box()
  }

  pub fn rename_wallet(&self, name: &str, to_name: &str, password: &str) -> Box<Future<Item = (), Error = Error>> {
    let sname = name.to_owned();
    let sto_name = to_name.to_owned();
    let spassword = password.to_owned();
    let storage1 = Arc::clone(&self.storage);
    let storage2 = Arc::clone(&self.storage);
    let random = Arc::clone(&self.random);
    self.load_wallet_data_raw(name, password)
      .and_then(move |data| {
        let crypted = crypt::encrypt(&data, &spassword, &mut random.lock().unwrap());
        storage1.save_bytes(&sto_name, &crypted).from_err()
      })
      .and_then(move |_| storage2.remove_bytes(&sname).from_err())
      .into_box()
  }

  pub fn change_wallet_password(&self, name: &str, oldpwd: &str, newpwd: &str) -> Box<Future<Item = (), Error = Error>> {
    let sname = name.to_owned();
    let snewpwd = newpwd.to_owned();
    let storage = Arc::clone(&self.storage);
    let random = Arc::clone(&self.random);
    self.load_wallet_data_raw(name, oldpwd)
      .and_then(move |data| {
        let crypted = crypt::encrypt(&data, &snewpwd, &mut random.lock().unwrap());
        storage.save_bytes(&sname, &crypted).from_err()
      })
      .into_box()
  }

  pub fn remove_wallet(&self, name: &str, password: &str) -> Box<Future<Item = (), Error = Error>> {
    let sname = name.to_owned();
    let storage = Arc::clone(&self.storage);
    self.load_wallet_data_raw(name, password)
      .and_then(move |_| storage.remove_bytes(&sname).from_err())
      .into_box()
  }

  #[cfg(feature = "backup")]
  pub fn backup_wallet_keys(&self, name: &str, password: &str) -> Box<Future<Item = Vec<(NetworkType, Vec<u8>)>, Error = Error>> {
    self.load_wallet_data(name, password)
      .map(|pks| pks.keys())
      .into_box()
  }
}

// Private methods
impl HDWalletProvider {
  fn with_network_objs(storage: Box<Storage>, networks: Vec<Box<Network>>) -> Result<Self, Error> {
    match Self::calculate_seed_size(&networks) {
      Some(seed_size) => OsRng::new().map(|random| {
        let map: HashMap<NetworkType, Box<Network>> = networks.into_iter().map(|nt| { (nt.get_type(), nt) }).collect();
        
        Self { 
          storage: Arc::from(storage),
          seed_size,
          random: Arc::new(Mutex::new(random)),
          networks: Arc::new(map)
        }
      }).map_err(|_| Error::UnknownError),
      None => Err(Error::CantCalculateSeedSize)
    }
  }

  fn save_wallet_data(&self, name: &str, password: &str, keys: &KeyStorage) -> Box<Future<Item = (), Error = Error>> {
    let wdata = WalletDataV1 { private_keys: keys.clone() };
    let data = VersionedData::new(&wdata).and_then(|vdata| vdata.to_bytes());
    if let Err(err) = data {
      return future::err(Error::DataParseError(name.to_owned(), err)).into_box();
    }
    let crypted = crypt::encrypt(&data.unwrap(), password, &mut self.random.lock().unwrap());
    self.storage.save_bytes(name, &crypted)
      .from_err()
      .into_box()
  }

  fn load_wallet_data_raw(&self, name: &str, password: &str) ->  Box<Future<Item = Vec<u8>, Error = Error>> {
    let sname = name.to_owned();
    let spassword = password.to_owned();
    self.storage.load_bytes(name)
      .from_err()
      .and_then(move |data| {
        match crypt::decrypt(&data, &spassword) {
          Ok(decrypted) => future::ok(decrypted),
          Err(err) => future::err(Error::from_decrypt_error(sname, err))
        }
      })
      .into_box()
  }

  fn load_wallet_data(&self, name: &str, password: &str) -> Box<Future<Item = KeyStorage, Error = Error>> {
    let sname = name.to_owned();
    self.load_wallet_data_raw(name, password)
      .and_then(|data| { 
        match VersionedData::from_bytes(&data).and_then(|vdata| vdata.get_data()) {
          Ok(wdata) => future::ok(wdata.private_keys),
          Err(err) => future::err(Error::DataParseError(sname, err))
        }
      }).into_box()
  }

  fn calculate_seed_size(networks: &[Box<Network>]) -> Option<usize> {
    let mut min = 0;
    let mut max = std::usize::MAX;
    for network in networks.into_iter() {
      let ssize = network.get_seed_size();
      min = min.max(ssize.min);
      max = max.min(ssize.max);
    }
    if min == 0 {
      return None;
    }
    if max >= min { Some(min) } else { None }
  }
}