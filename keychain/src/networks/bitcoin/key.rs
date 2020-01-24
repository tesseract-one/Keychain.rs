use super::key_path::COIN_TYPE;
use bip39;
use crate::key::{Error, Key as IKey};
use crate::key_path::{Error as KPError, KeyPath, BIP44_SOFT_UPPER_BOUND};
use crate::network::Network;

use secp256k1_bip32::XPrv;

pub struct Key {
  xprv: XPrv
}

impl Key {
  pub fn from_data(data: &[u8]) -> Result<Self, Error> {
    let xprv = XPrv::from_data(data).map_err(|err| Error::from(err))?;
    Ok(Self { xprv })
  }

  pub fn data_from_seed(seed: &bip39::Seed) -> Result<Vec<u8>, Error> {
    let xprv = XPrv::from_seed(seed).map_err(|err| Error::from(err))?;
    Ok(xprv.serialize())
  }

  fn derive_private(&self, path: &dyn KeyPath) -> Result<XPrv, Error> {
    if path.coin() != COIN_TYPE {
      return Err(KPError::InvalidCoin(path.coin(), COIN_TYPE).into());
    }
    if path.account() < BIP44_SOFT_UPPER_BOUND {
      return Err(KPError::InvalidAccount(path.account()).into());
    }
    if path.change() != 0 && path.change() != 1 {
      return Err(KPError::InvalidChange(path.change()).into());
    }
    if path.address() >= BIP44_SOFT_UPPER_BOUND {
      return Err(KPError::InvalidAddress(path.address()).into());
    }
    self
      .xprv
      .derive(path.purpose())
      .and_then(|pk| pk.derive(path.coin()))
      .and_then(|pk| pk.derive(path.account()))
      .and_then(|pk| pk.derive(path.change()))
      .and_then(|pk| pk.derive(path.address()))
      .map_err(|err| err.into())
  }
}

impl IKey for Key {
  fn network(&self) -> Network {
    Network::BITCOIN
  }

  fn pub_key(&self, path: &dyn KeyPath) -> Result<Vec<u8>, Error> {
    self.derive_private(path).map(|pk| pk.public().serialize())
  }

  fn sign(&self, data: &[u8], path: &dyn KeyPath) -> Result<Vec<u8>, Error> {
    self.derive_private(path)?.sign(data).map_err(|err| Error::from_secp_sign_error(err))
  }

  fn verify(&self, data: &[u8], signature: &[u8], path: &dyn KeyPath) -> Result<bool, Error> {
    self.derive_private(path)?.public().verify(data, signature).map_err(|err| err.into())
  }
}
