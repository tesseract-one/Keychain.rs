use key::{ Key as IKey, Error };
use key_path::{ Error as KPError, KeyPath, BIP44_PURPOSE, BIP44_SOFT_UPPER_BOUND };
use super::key_path::BIP44_COIN_TYPE;
use ed25519_bip32::{ XPrv, DerivationScheme, Signature, SIGNATURE_SIZE, XPRV_SIZE, PrivateKeyError };
use cryptoxide::sha2::Sha512;
use cryptoxide::digest::Digest;
use network::Network;
use bip39;
use std::fmt;

const D_SCHEME: DerivationScheme = DerivationScheme::V2;

#[derive(Debug)]
pub enum KeyError {
  LengthInvalid(usize),
  HighestBitsInvalid,
  LowestBitsInvalid,
}

impl fmt::Display for KeyError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &KeyError::LengthInvalid(len) => write!(f, "Invalid data length {}", len),
      &KeyError::HighestBitsInvalid => write!(f, "Highest bits is invalid"),
      &KeyError::LowestBitsInvalid => write!(f, "Lowest bits is invalid"),
    }
  }
}

impl std::error::Error for KeyError {}

impl From<PrivateKeyError> for KeyError {
  fn from(err: PrivateKeyError) -> Self {
    match err {
      PrivateKeyError::LengthInvalid(len) => KeyError::LengthInvalid(len),
      PrivateKeyError::HighestBitsInvalid => KeyError::HighestBitsInvalid,
      PrivateKeyError::LowestBitsInvalid => KeyError::LowestBitsInvalid
    }
  }
}

pub struct Key {
  xprv: XPrv
}

impl Key {
  pub fn from_data(data: &[u8]) -> Result<Self, Error> {
    let mut arr: [u8; XPRV_SIZE] = [0; XPRV_SIZE];
    if data.len() < XPRV_SIZE {
      return Err(Error::InvalidKeySize(data.len(), XPRV_SIZE));
    }
    arr.copy_from_slice(data);
    XPrv::from_bytes_verified(arr)
      .map(|xprv| {
        Self { xprv: xprv.derive(D_SCHEME, BIP44_PURPOSE).derive(D_SCHEME, BIP44_COIN_TYPE) }
      })
      .map_err(|err| {
        let key_err: KeyError = err.into();
        Error::InvalidKeyData(Box::new(key_err))
      })
  }

  pub fn data_from_seed(seed: &bip39::Seed) -> Result<Vec<u8>, Error> {
    let mut out = [0u8; XPRV_SIZE];

    let mut hasher = Sha512::new();
    hasher.input(&seed.as_ref()[0..32]);
    hasher.result(&mut out[0..64]);
    out[0] &= 248;
    out[31] &= 63;
    out[31] |= 64;
    out[31] &= 0b1101_1111; // set 3rd highest bit to 0 as per the spec
    out[64..96].clone_from_slice(&seed.as_ref()[32..64]);

    XPrv::from_bytes_verified(out)
      .map_err(|err| {
        let key_err: KeyError = err.into();
        Error::InvalidKeyData(Box::new(key_err))
      })
      .map(|xprv| Vec::from(xprv.as_ref()))
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

impl IKey for Key {
  fn network(&self) -> Network {
    Network::CARDANO
  }

  fn address(&self, path: &KeyPath) -> Result<Vec<u8>, Error> {
    self.pub_key(path)
  }

  fn pub_key(&self, path: &KeyPath) -> Result<Vec<u8>, Error> {
    self.derive_private(path)
      .map_err(|err| err.into())
      .map(|pk| Vec::from(pk.public().as_ref()))
  }

  fn sign(&self, data: &[u8], path: &KeyPath) -> Result<Vec<u8>, Error> {
    self.derive_private(path)
      .map(|pk| {
        let signature: Signature<Vec<u8>> = pk.sign(data);
        Vec::from(signature.as_ref())
      })
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
}