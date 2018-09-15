use super::network::{Network, NetworkType};
use super::account::{Account};

pub trait UnsignedTransaction { }
pub trait SignedTransaction { }

pub trait Wallet {
  fn get_network_type(&self) -> NetworkType;

  fn get_network(&self) -> &Network;

  fn get_account(&self, id: u8) -> Option<&Account>;

  fn new_account(&self, id: u8) -> &Account;

  fn remove_account(&self, id: u8);

  fn send_money_transaction<'a>(&self, to: &str, amount: u64) -> &'a UnsignedTransaction;

  fn sign_transaction<'a>(&self, tx: &'a UnsignedTransaction, with: u8) -> &'a SignedTransaction;
}
