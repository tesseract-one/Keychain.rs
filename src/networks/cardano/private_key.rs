use super::cardano::hdwallet::{ XPrv, XPRV_SIZE, DerivationScheme, Signature };
use key_path::{ Bip44KeyPath, BIP44_PURPOSE, BIP44_SOFT_UPPER_BOUND, Error as KPError };
use super::key_path::BIP44_COIN_TYPE;
use private_key::{ Error, PrivateKey as IPrivateKey };

use bip39;
use cryptoxide::sha2::Sha512;
use cryptoxide::digest::Digest;

const D_SCHEME: DerivationScheme = DerivationScheme::V2;

pub struct PrivateKey {
  xprv: XPrv
}

impl PrivateKey {
  pub fn data_from_seed(seed: &bip39::Seed) -> Vec<u8> {
    let mut out = [0u8; XPRV_SIZE];
    let mut hasher = Sha512::new();
    hasher.input(&seed.as_ref()[0..32]);
    hasher.result(&mut out[0..64]);
    out[0] &= 248;
    out[31] &= 63;
    out[31] |= 64;
    out[31] &= 0b1101_1111; // set 3rd highest bit to 0 as per the spec
    out[64..96].clone_from_slice(&seed.as_ref()[32..64]);

    Vec::from(out.as_ref())
  }

  fn derive_private(&self, path: &Bip44KeyPath) -> Result<XPrv, KPError> {
    if path.purpose() != BIP44_PURPOSE {
      return Err(KPError::InvalidPurpose(path.purpose()));
    }
    if path.coin() != BIP44_COIN_TYPE {
      return Err(KPError::InvalidCoin(path.coin(), BIP44_COIN_TYPE));
    }
    if path.account() < BIP44_SOFT_UPPER_BOUND {
      return Err(KPError::NonHardenedValueAtIndex(3))
    }
    if path.change() != 0 && path.change() != 1 {
      return Err(KPError::InvalidChange(path.change()));
    }
    if path.address() >= BIP44_SOFT_UPPER_BOUND {
      return Err(KPError::HardenedValueAtIndex(5))
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

  fn pub_key(&self, path: &Bip44KeyPath) -> Result<Vec<u8>, Error> {
    self.derive_private(path)
      .map(|pk| Vec::from(pk.public().as_ref()))
      .map_err(|err| err.into())
  }

  fn sign(&self, data: &[u8], path: &Bip44KeyPath) -> Result<Vec<u8>, Error> {
    self.derive_private(path)
      .map(|pk| {
        let signature: Signature<Vec<u8>> = pk.sign(data);
        Vec::from(signature.as_ref())
      })
      .map_err(|err| err.into())
  }
}