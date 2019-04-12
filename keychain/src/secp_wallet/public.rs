use cryptoxide::sha3::Sha3;
use cryptoxide::sha2::Sha256;
use cryptoxide::digest::Digest;
use super::error::KeyError;
use secp256k1;

pub struct XPub(secp256k1::PublicKey);

impl XPub {
  pub(crate) fn new(key: secp256k1::PublicKey) -> Self {
    Self(key)
  }

  pub fn serialize(&self) -> Vec<u8> {
    let bytes: &[u8] = &self.0.serialize();
    Vec::from(bytes)
  }

  pub fn address(&self) -> Vec<u8> {
    let mut sha3 = Sha3::keccak256();
    sha3.input(&self.0.serialize());
    let mut out = [0u8; 32];
    sha3.result(&mut out);
    Vec::from(&out[12..31])
  }

  pub fn verify(&self, data: &[u8], signature: &[u8]) -> Result<bool, KeyError> {
    if signature.len() != secp256k1::util::SIGNATURE_SIZE {
      return Err(KeyError::InvalidSignature(signature.len(), secp256k1::util::SIGNATURE_SIZE));
    }
    let mut sha3 = Sha3::keccak256();
    sha3.input(data);
    let mut out = [0u8; secp256k1::util::MESSAGE_SIZE];
    sha3.result(&mut out);

    let message = secp256k1::Message::parse(&out);

    secp256k1::Signature::parse_slice(signature)
      .map(|signature| {
        secp256k1::verify(&message, &signature, &self.0)
      })
      .map_err(|err| err.into() )
  }

  pub fn sha256(&self) -> [u8; 32] {
    let mut hasher = Sha256::new();
    let mut out = [0u8; 32];
    hasher.input(&self.0.serialize());
    hasher.result(&mut out);
    out
  }
}
