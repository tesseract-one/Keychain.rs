
use key_path::{ KeyPath, BIP44_PURPOSE, BIP44_SOFT_UPPER_BOUND, Error as KPError };
use super::key_path::BIP44_COIN_TYPE;
use private_key::{ Error, PrivateKey as IPrivateKey };

use bip39;
use super::hdwallet::{ XPrv, XPRV_SIZE, DerivationScheme, Signature, SIGNATURE_SIZE };

const D_SCHEME: DerivationScheme = DerivationScheme::V2;

pub struct PrivateKey {
  xprv: XPrv
}

impl PrivateKey {
  pub fn data_from_seed(seed: &bip39::Seed) -> Vec<u8> {
    let xprv = XPrv::generate_from_bip39(seed);
    Vec::from(xprv.as_ref())
  }

  fn derive_private(&self, path: &KeyPath) -> Result<XPrv, KPError> {
    if path.purpose() != BIP44_PURPOSE {
      return Err(KPError::InvalidPurpose(path.purpose(), BIP44_PURPOSE));
    }
    if path.coin() != BIP44_COIN_TYPE {
      return Err(KPError::InvalidCoin(path.coin(), BIP44_COIN_TYPE));
    }
    if path.account() < BIP44_SOFT_UPPER_BOUND {
      return Err(KPError::InvalidAccount(path.account()));
    }
    if path.change() != 0 && path.change() != 1 {
      return Err(KPError::InvalidChange(path.change()));
    }
    if path.address() >= BIP44_SOFT_UPPER_BOUND {
      return Err(KPError::InvalidAddress(path.address()));
    }
    Ok(
      self.xprv
        .derive(D_SCHEME, path.account())
        .derive(D_SCHEME, path.change())
        .derive(D_SCHEME, path.address())
    )
  }
}

impl IPrivateKey for PrivateKey {
  fn from_data(data: &[u8]) -> Result<Self, Error> {
    let mut arr: [u8; XPRV_SIZE] = [0; XPRV_SIZE];
    if data.len() < XPRV_SIZE {
      return Err(Error::InvalidKeySize(data.len(), XPRV_SIZE));
    }
    arr.copy_from_slice(data);
    XPrv::from_bytes_verified(arr)
      .map(|xprv| {
        Self { xprv: xprv.derive(D_SCHEME, BIP44_PURPOSE).derive(D_SCHEME, BIP44_COIN_TYPE) }
      })
      .map_err(|err| Error::InvalidKeyData(Box::new(err)))
  }

  fn pub_key(&self, path: &KeyPath) -> Result<Vec<u8>, Error> {
    self.derive_private(path)
      .map(|pk| Vec::from(pk.public().as_ref()))
      .map_err(|err| err.into())
  }

  fn verify(&self, data: &[u8], signature: &[u8], path: &KeyPath) -> Result<bool, Error> {
    let mut sign: [u8; SIGNATURE_SIZE] = [0; SIGNATURE_SIZE];
    if signature.len() < SIGNATURE_SIZE {
      return Err(Error::InvalidSignatureSize(signature.len(), SIGNATURE_SIZE));
    }
    sign.copy_from_slice(signature);

    self.derive_private(path)
      .map(|pk| {
        let native_signature: Signature<Vec<u8>> = Signature::from_bytes(sign);
        pk.verify(data, &native_signature)
      })
      .map_err(|err| err.into())
  }

  fn sign(&self, data: &[u8], path: &KeyPath) -> Result<Vec<u8>, Error> {
    self.derive_private(path)
      .map(|pk| {
        let signature: Signature<Vec<u8>> = pk.sign(data);
        Vec::from(signature.as_ref())
      })
      .map_err(|err| err.into())
  }
}