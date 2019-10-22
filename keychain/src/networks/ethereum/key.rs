use super::key_path::BIP44_COIN_TYPE;
use bip39;
use key::{Error, Key as IKey};
use key_path::{Error as KPError, KeyPath, BIP44_PURPOSE, BIP44_SOFT_UPPER_BOUND};
use network::Network;

use secp256k1_bip32::XPrv;

pub struct Key {
  xprv: XPrv
}

impl Key {
  pub fn from_data(data: &[u8]) -> Result<Self, Error> {
    XPrv::from_data(data)
      .and_then(|pk| pk.derive(BIP44_PURPOSE))
      .and_then(|pk| pk.derive(BIP44_COIN_TYPE))
      .map_err(|err| err.into())
      .map(|pk| Self { xprv: pk })
  }

  pub fn data_from_seed(seed: &bip39::Seed) -> Result<Vec<u8>, Error> {
    let xprv = XPrv::from_seed(seed).map_err(|err| Error::from(err))?;
    Ok(xprv.serialize())
  }

  fn derive_private(&self, path: &dyn KeyPath) -> Result<XPrv, Error> {
    if path.purpose() != BIP44_PURPOSE {
      return Err(KPError::InvalidPurpose(path.purpose(), BIP44_PURPOSE).into());
    }
    if path.coin() != BIP44_COIN_TYPE {
      return Err(KPError::InvalidCoin(path.coin(), BIP44_COIN_TYPE).into());
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
      .derive(path.account())
      .and_then(|pk| pk.derive(path.change()))
      .and_then(|pk| pk.derive(path.address()))
      .map_err(|err| err.into())
  }
}

impl IKey for Key {
  fn network(&self) -> Network {
    Network::ETHEREUM
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
