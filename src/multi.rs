use traits::wallet::Wallet;
use traits::network::NetworkType;
use traits::errors::NotFoundError;
use traits::account::Account;

pub struct MultiWallet {
  wallets: Vec<Box<Wallet>>
}

impl MultiWallet {
  fn get_wallet_for_network<'a>(&'a self, network: NetworkType) -> Result<&'a Wallet, NotFoundError> {
    for wallet in &self.wallets {
      if wallet.get_network_type() == network {
        return Ok(wallet.as_ref());
      }
    }
    Err(NotFoundError { what: Box::new(network) })
  }

  pub fn get_supported_networks(&self) -> Vec<NetworkType> {
    self.wallets.iter().map(|w| w.get_network_type()).collect()
  }

  pub fn get_account(&self, network: NetworkType, id: u8) -> Result<Option<&Account>, NotFoundError> {
    match self.get_wallet_for_network(network) {
      Ok(wallet) => Ok(wallet.get_account(id)),
      Err(not_found) => Err(not_found)
    }
  }

  pub fn new(wallets: Vec<Box<Wallet>>) -> Self {
    MultiWallet {
      wallets: wallets
    }
  }

  pub fn from_data(data: &[u8]) -> Option<Self> {
    None
  }
}