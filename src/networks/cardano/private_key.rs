use super::cardano::hdwallet::{ XPrv, XPRV_SIZE, DerivationScheme, Signature };
use key_path::{ Bip44_KeyPath, BIP44_PURPOSE, BIP44_SOFT_UPPER_BOUND, Error as KPError };
use super::key_path::BIP44_COIN_TYPE;
use network::{ PrivateKey as IPrivateKey, Error as NError  };

const D_SCHEME: DerivationScheme = DerivationScheme::V2;

pub struct PrivateKey {
  xprv: XPrv
}

impl PrivateKey {
  fn derive_private(&self, path: &Bip44_KeyPath) -> Result<XPrv, KPError> {
    if path.purpose() != BIP44_PURPOSE {
      return Err(KPError::WrongPurpose);
    }
    if path.coin() != BIP44_COIN_TYPE {
      return Err(KPError::WrongCoin);
    }
    if path.account() < BIP44_SOFT_UPPER_BOUND {
      return Err(KPError::NonHardenedValueAtIndex(3))
    }
    if path.change() != 0 && path.change() != 1 {
      return Err(KPError::WrongChange);
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
  fn from_data(data: &[u8]) -> Result<Self, NError> {
    let arr: [u8; XPRV_SIZE] = [0; XPRV_SIZE];
    if data.len() < XPRV_SIZE {
      return Err(NError::WrongKeySize);
    }
    arr.copy_from_slice(data);
    XPrv::from_bytes_verified(arr)
      .map(|xprv| {
        Self { xprv: xprv.derive(D_SCHEME, BIP44_PURPOSE).derive(D_SCHEME, BIP44_COIN_TYPE) }
      })
      .map_err(|err| NError::BadKeyData)
  }

  fn pub_key(&self, path: &Bip44_KeyPath) -> Result<Vec<u8>, NError> {
    self.derive_private(path)
      .map(|pk| Vec::from(pk.public().as_ref()))
      .map_err(|err| NError::WrongKeyPath)
  }

  fn sign(&self, data: &[u8], path: &Bip44_KeyPath) -> Result<Vec<u8>, NError> {
    self.derive_private(path)
      .map(|pk| {
        let signature: Signature<Vec<u8>> = pk.sign(data);
        Vec::from(signature.as_ref())
      })
      .map_err(|err| NError::WrongKeyPath)
  }
}