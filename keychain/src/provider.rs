use futures::prelude::*;
use futures::future;
use util::future::*;
use std::collections::HashMap;
use std::sync::Arc;

use storage::{ Storage };
use wallet::{ HDWallet };
use error::Error;
use network::Network;
use network_type::NetworkType;
use networks::all_networks;
use key_storage::KeyStorage;
use entropy::{ Entropy, OsEntropy };
use mnemonic::{ generate as generate_mnemonic, Language };
use util::crypt;
use data::{ VersionedData, WalletDataV1 };

pub type Networks = Arc<HashMap<NetworkType, Box<Network>>>;

pub struct HDWalletProvider {
  storage: Arc<Storage>,
  networks: Networks,
  random: Arc<Entropy>,
  seed_size: usize
}

impl HDWalletProvider {
  pub fn new(storage: Box<Storage>) -> Result<Self, Error> {
    Self::with_network_objs(storage, all_networks())
  }

  pub fn with_networks(storage: Box<Storage>, networks: &[NetworkType]) -> Result<Self, Error> {
    let filtered: Vec<Box<Network>> = all_networks()
      .into_iter()
      .filter(|network| {
        let ntype = network.get_type();
        networks.iter().position(|nt| nt == &ntype).is_some()
      })
      .collect();
    Self::with_network_objs(storage, filtered)
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
    let networks = Arc::clone(&self.networks);
    self.load_wallet_data(name, password)
      .and_then(move |pks|
        HDWallet::new(&sname, pks, networks)
          .map_err(|err| Error::from_wallet_error(&sname, err))
          .into_future()
      )
      .into_box()
  }

  pub fn create_wallet(&self, name: &str, password: &str, language: Option<Language>) -> Box<Future<Item = (HDWallet, String), Error = Error>> {
    let lang = language.unwrap_or(Language::default());
    match generate_mnemonic(self.seed_size, lang, self.random.as_ref()) {
      Ok(mnemonic) => self.restore_wallet(name, &mnemonic, password).map(|wallet| (wallet, mnemonic)).into_box(),
      Err(_) => future::err(Error::InvalidSeedSize(name.to_owned(), self.seed_size)).into_box()
    }
  }

  pub fn restore_wallet(&self, name: &str, mnemonic: &str, password: &str) -> Box<Future<Item = HDWallet, Error = Error>> {
    let sname = name.to_owned();
    let networks = Arc::clone(&self.networks);
    match HDWallet::key_storage_for_mnemonic(mnemonic, self.networks.as_ref()) {
      Err(err) => future::err(Error::from_wallet_error(name, err)).into_box(),
      Ok(key_storage) => 
        self.save_wallet_data(name, password, &key_storage)
          .and_then(move |_| 
            HDWallet::new(&sname, key_storage, networks)
              .map_err(|err| Error::from_wallet_error(&sname, err))
              .into_future()
          )
          .into_box()
    }
  }

  pub fn restore_wallet_from_keys(&self, name: &str, password: &str, keys: &[(NetworkType, Vec<u8>)]) -> Box<Future<Item = HDWallet, Error = Error>> {
    let filtered: Vec<(NetworkType, Vec<u8>)> = keys.into_iter()
      .filter(|(nt, _)| self.has_network(nt))
      .cloned()
      .collect();
    let key_storage = KeyStorage::new(&filtered);
    match HDWallet::new(name, key_storage.clone(), Arc::clone(&self.networks)) {
      Err(err) => future::err(Error::from_wallet_error(name, err)).into_box(),
      Ok(wallet) => self.save_wallet_data(name, password, &key_storage).map(move |_| wallet).into_box()
    }
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
        let crypted = crypt::encrypt(&data, &spassword, random.as_ref());
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
        let crypted = crypt::encrypt(&data, &snewpwd, random.as_ref());
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
    Self::calculate_seed_size(&networks)
      .and_then(|seed| OsEntropy::new().map(|rnd| (seed, rnd)).map_err(|err| Error::EntropyGeneratorError(err)))
      .map(|(seed_size, random)| {
        let map: HashMap<NetworkType, Box<Network>> =
          networks.into_iter().map(|nt| { (nt.get_type(), nt) }).collect();
        Self { 
          storage: Arc::from(storage),
          seed_size,
          random: Arc::new(random),
          networks: Arc::new(map)
        }
      })
  }

  fn save_wallet_data(&self, name: &str, password: &str, keys: &KeyStorage) -> Box<Future<Item = (), Error = Error>> {
    let wdata = WalletDataV1 { private_keys: keys.clone() };
    let data = VersionedData::new(&wdata).and_then(|vdata| vdata.to_bytes());
    if let Err(err) = data {
      return future::err(Error::DataParseError(name.to_owned(), err)).into_box();
    }
    let crypted = crypt::encrypt(&data.unwrap(), password, self.random.as_ref());
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
          Err(err) => future::err(Error::from_decrypt_error(&sname, err))
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

  fn calculate_seed_size(networks: &[Box<Network>]) -> Result<usize, Error> {
    let mut min = 0;
    let mut max = std::usize::MAX;
    for network in networks.into_iter() {
      let ssize = network.get_seed_size();
      min = min.max(ssize.min);
      max = max.min(ssize.max);
    }
    if min == 0 {
      return Err(Error::CantCalculateSeedSize(min, max));
    }
    if max >= min { Ok(min) } else { Err(Error::CantCalculateSeedSize(min, max)) }
  }
}