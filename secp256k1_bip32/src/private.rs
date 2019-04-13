use bip39;
use cryptoxide::hmac::Hmac;
use cryptoxide::mac::Mac;
use cryptoxide::digest::Digest;
use cryptoxide::sha2::{ Sha512, Sha256 };
use cryptoxide::sha3::Sha3;
use ripemd160::{ Ripemd160, Digest as RipeDigest };
use secp256k1::{ SecretKey, PublicKey, Message, util, sign };
use byteorder::{ BigEndian, WriteBytesExt, ByteOrder };
use num_bigint::BigUint;
use num_traits::{ Num, Zero };

use super::error::KeyError;
use super::public::XPub;

const HMAC_KEY: &[u8] = b"Bitcoin seed";
const KEY_DATA_SIZE: usize = 77;
const ENTROPY_SIZE: usize = 64;
const BIP44_SOFT_UPPER_BOUND: u32 = 0x80000000;

lazy_static! {
  static ref CURVE_ORDER: BigUint = BigUint::from_str_radix(
    "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141", 16
  ).unwrap();
}


pub struct XPrv {
  key: SecretKey,
  chaincode: [u8; 32],
  parent_fingerprint: [u8; 4],
  depth: u8,
  index: u32
}

impl XPrv {
  pub fn from_data(data: &[u8]) -> Result<Self, KeyError> {
    if data.len() != KEY_DATA_SIZE {
      return Err(KeyError::InvalidDataSize(data.len(), KEY_DATA_SIZE));
    }

    let mut sha256 = Sha256::new();
    sha256.input(&data[0..72]);
    let mut hash = [0u8; 32];
    sha256.result(&mut hash);
    sha256.reset();
    sha256.input(&hash);
    sha256.result(&mut hash);

    if data[73..76] != hash[0..3] {
      return Err(KeyError::InvalidSecretKey);
    }

    let depth: u8 = data[0];
    let mut parent_fingerprint = [0u8; 4];
    parent_fingerprint.copy_from_slice(&data[1..4]);
    let index: u32 = BigEndian::read_u32(&data[5..8]);
    let mut chaincode = [0u8; 32];
    chaincode.copy_from_slice(&data[9..40]);

    SecretKey::parse_slice(&data[41..72])
      .map_err(|err| err.into())
      .and_then(|private_key| {
        let pub_key = PublicKey::from_secret_key(&private_key);
        if pub_key.serialize()[0] != 0x04 {
          return Err(KeyError::InvalidPublicKey);
        }
        Ok(private_key)
      })
      .map(|pk|
        Self {
          depth,
          parent_fingerprint,
          index,
          chaincode,
          key: pk
        }
      )
  }

  pub fn from_seed(seed: &bip39::Seed) -> Result<Self, KeyError> {
    let mut hmac = Hmac::new(Sha512::new(), HMAC_KEY);
    hmac.input(seed.as_ref());
    let result = hmac.result();
    let entropy = result.code();
    if entropy.len() < ENTROPY_SIZE {
      return Err(KeyError::InvalidEntropySize(entropy.len()));
    }

    let mut i_l = [0u8; 32];
    let mut i_r = [0u8; 32];
    i_l.copy_from_slice(&entropy[0..31]);
    i_r.copy_from_slice(&entropy[32..63]);

    SecretKey::parse(&i_l)
      .map_err(|err| err.into())
      .and_then(|private_key| {
        let pub_key = PublicKey::from_secret_key(&private_key);
        if pub_key.serialize()[0] != 0x04 {
          return Err(KeyError::InvalidPublicKey);
        }
        Ok(private_key)
      })
      .map(|pk| Self { key: pk, chaincode: i_r, depth: 0, index: 0, parent_fingerprint: [0u8; 4] })
  }

  pub fn public(&self) -> XPub {
    XPub::new(PublicKey::from_secret_key(&self.key))
  }

  pub fn serialize(&self) -> Vec<u8> {
    let mut data = Vec::with_capacity(KEY_DATA_SIZE);

    data.push(self.depth);
    data.extend_from_slice(&self.parent_fingerprint);
    data.write_u32::<BigEndian>(self.index).unwrap();
    data.extend_from_slice(&self.chaincode);
    data.extend_from_slice(&self.key.serialize());

    let mut sha256 = Sha256::new();
    sha256.input(data.as_slice());
    let mut hash = [0u8; 32];
    sha256.result(&mut hash);
    sha256.reset();
    sha256.input(&hash);
    sha256.result(&mut hash);

    data.extend_from_slice(&hash[0..3]);

    data
  }

  pub fn sign(&self, data: &[u8]) -> Result<Vec<u8>, KeyError> {
    let mut sha3 = Sha3::keccak256();
    sha3.input(data);
    let mut out = [0u8; util::MESSAGE_SIZE];
    sha3.result(&mut out);

    let message = Message::parse(&out);

    sign(&message, &self.key)
      .map_err(|err| err.into())
      .and_then(|(signature, recovery)| {
        let rec_id = recovery.serialize();
        if rec_id != 0 && rec_id != 1 {
          return Err(KeyError::InvalidRecoveryId);
        }
        let mut data = Vec::with_capacity(util::SIGNATURE_SIZE + 1);
        data.extend_from_slice(&signature.r.b32());
        data.extend_from_slice(&signature.s.b32());
        data.push(rec_id);
        Ok(data)
      })
  }

  pub fn derive(&self, index: u32) -> Result<Self, KeyError> {
    if self.depth == std::u8::MAX {
      return Err(KeyError::DeriveDepthTooBig);
    }
    let mut hmac = Hmac::new(Sha512::new(), &self.chaincode);

    if index >= BIP44_SOFT_UPPER_BOUND {
      let mut input = Vec::with_capacity(util::SECRET_KEY_SIZE + 1 + 4);
      input.push(0x00);
      input.extend_from_slice(&self.key.serialize());
      if input.write_u32::<BigEndian>(index).is_err() {
        return Err(KeyError::InternalError);
      }
      hmac.input(&input);
    } else {
      let mut input = Vec::with_capacity(util::SECRET_KEY_SIZE + 4);
      input.extend_from_slice(&self.key.serialize());
      if input.write_u32::<BigEndian>(index).is_err() {
        return Err(KeyError::InternalError);
      }
      hmac.input(&input);
    }

    let result = hmac.result();
    let entropy = result.code();

    if entropy.len() < ENTROPY_SIZE {
      return Err(KeyError::InvalidEntropySize(entropy.len()));
    }

    let mut chaincode = [0u8; 32];
    chaincode.copy_from_slice(&entropy[0..31]);

    let bn = BigUint::from_bytes_be(&entropy[32..63]);
    let curve_order = CURVE_ORDER.clone();

    if bn > curve_order {
      if index < std::u32::MAX {
        return self.derive(index + 1);
      } else {
        return Err(KeyError::TweakOutOfRange)
      }
    }

    let new_pk = (bn + BigUint::from_bytes_be(&self.key.serialize())) % curve_order;
    if new_pk.is_zero() {
      if index < std::u32::MAX {
        return self.derive(index + 1);
      } else {
        return Err(KeyError::TweakOutOfRange)
      }
    }
    let pk_bytes = new_pk.to_bytes_be();
    let pk: Vec<u8> = if pk_bytes.len() == util::SECRET_KEY_SIZE {
      pk_bytes
    } else {
      let mut vec = Vec::with_capacity(util::SECRET_KEY_SIZE);
      vec.extend_from_slice(&vec![0u8; util::SECRET_KEY_SIZE - pk_bytes.len()]);
      vec.extend_from_slice(&pk_bytes);
      vec
    };

    SecretKey::parse_slice(&pk)
      .map_err(|err| err.into())
      .and_then(|private_key| {
        let pub_key = PublicKey::from_secret_key(&private_key);
        if pub_key.serialize()[0] != 0x04 {
          return Err(KeyError::InvalidPublicKey)
        }
        Ok(private_key)
      })
      .map(|pk| {
        let mut hasher = Ripemd160::new();
        hasher.input(&self.public().sha256());
        let mut fingerprint = [0u8; 4];
        fingerprint.copy_from_slice(&hasher.result()[0..3]);

        Self {
          key: pk,
          depth: self.depth + 1,
          chaincode,
          index,
          parent_fingerprint: fingerprint
        }
      })
  }
}
