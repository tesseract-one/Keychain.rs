use bip39;
use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use cryptoxide::digest::Digest;
use cryptoxide::hmac::Hmac;
use cryptoxide::mac::Mac;
use cryptoxide::sha2::{Sha256, Sha512};
use cryptoxide::sha3::Sha3;
use ripemd160::{Digest as RipeDigest, Ripemd160};
use secp256k1::{sign, util, Message, PublicKey, SecretKey};

use super::error::KeyError;
use super::public::XPub;

const HMAC_KEY: &[u8] = b"Bitcoin seed";
const BIP44_SOFT_UPPER_BOUND: u32 = 0x80000000;

mod data_layout {
  pub const DEPTH_SIZE: usize = 1;
  pub const FINGERPRINT_SIZE: usize = 4;
  pub const INDEX_SIZE: usize = 4;
  pub const CHAIN_CODE_SIZE: usize = 32;
  pub const KEY_SIZE: usize = super::util::SECRET_KEY_SIZE;
  pub const CHECKSUM_SIZE: usize = 4;

  pub const KEY_DATA_SIZE: usize =
    (DEPTH_SIZE + FINGERPRINT_SIZE + INDEX_SIZE + CHAIN_CODE_SIZE + KEY_SIZE + CHECKSUM_SIZE);

  pub const ENTROPY_SIZE: usize = CHAIN_CODE_SIZE + KEY_SIZE;

  pub const DEPTH_START: usize = 0;
  pub const DEPTH_END: usize = DEPTH_START + DEPTH_SIZE;
  pub const FINGERPRINT_START: usize = DEPTH_END;
  pub const FINGERPRINT_END: usize = FINGERPRINT_START + FINGERPRINT_SIZE;
  pub const INDEX_START: usize = FINGERPRINT_END;
  pub const INDEX_END: usize = INDEX_START + INDEX_SIZE;
  pub const CHAIN_CODE_START: usize = INDEX_END;
  pub const CHAIN_CODE_END: usize = CHAIN_CODE_START + CHAIN_CODE_SIZE;
  pub const KEY_START: usize = CHAIN_CODE_END;
  pub const KEY_END: usize = KEY_START + KEY_SIZE;
  pub const CHECKSUM_START: usize = KEY_END;
  pub const CHECKSUM_END: usize = CHECKSUM_START + CHECKSUM_SIZE;
}

pub struct XPrv {
  key: SecretKey,
  chaincode: [u8; data_layout::CHAIN_CODE_SIZE],
  parent_fingerprint: [u8; data_layout::FINGERPRINT_SIZE],
  depth: u8,
  index: u32
}

impl XPrv {
  pub fn from_data(data: &[u8]) -> Result<Self, KeyError> {
    use self::data_layout::*;

    if data.len() != KEY_DATA_SIZE {
      return Err(KeyError::InvalidDataSize(data.len(), KEY_DATA_SIZE));
    }

    let mut sha256 = Sha256::new();
    sha256.input(&data[DEPTH_START..KEY_END]);
    let mut hash = [0u8; 32];
    sha256.result(&mut hash);
    sha256.reset();
    sha256.input(&hash);
    sha256.result(&mut hash);

    if data[CHECKSUM_START..CHECKSUM_END] != hash[0..CHECKSUM_SIZE] {
      return Err(KeyError::InvalidSecretKey);
    }

    let depth: u8 = data[DEPTH_START];
    let mut parent_fingerprint = [0u8; FINGERPRINT_SIZE];
    parent_fingerprint.copy_from_slice(&data[FINGERPRINT_START..FINGERPRINT_END]);
    let index: u32 = BigEndian::read_u32(&data[INDEX_START..INDEX_END]);
    let mut chaincode = [0u8; CHAIN_CODE_SIZE];
    chaincode.copy_from_slice(&data[CHAIN_CODE_START..CHAIN_CODE_END]);

    let pk = SecretKey::parse_slice(&data[KEY_START..KEY_END]).map_err(|e| KeyError::from(e))?;

    Ok(Self { depth, parent_fingerprint, index, chaincode, key: pk })
  }

  pub fn from_seed(seed: &bip39::Seed) -> Result<Self, KeyError> {
    use self::data_layout::*;

    let mut hmac = Hmac::new(Sha512::new(), HMAC_KEY);
    hmac.input(seed.as_ref());
    let result = hmac.result();
    let entropy = result.code();
    if entropy.len() < ENTROPY_SIZE {
      return Err(KeyError::InvalidEntropySize(entropy.len()));
    }

    let mut i_l = [0u8; KEY_SIZE];
    let mut i_r = [0u8; CHAIN_CODE_SIZE];
    i_l.copy_from_slice(&entropy[0..KEY_SIZE]);
    i_r.copy_from_slice(&entropy[KEY_SIZE..(KEY_SIZE + CHAIN_CODE_SIZE)]);

    let pk = SecretKey::parse_slice(&i_l).map_err(|err| KeyError::from(err))?;

    Ok(Self {
      key: pk,
      chaincode: i_r,
      depth: 0,
      index: 0,
      parent_fingerprint: [0u8; FINGERPRINT_SIZE]
    })
  }

  pub fn public(&self) -> XPub {
    XPub::new(PublicKey::from_secret_key(&self.key))
  }

  pub fn serialize(&self) -> Vec<u8> {
    use self::data_layout::*;

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

    data.extend_from_slice(&hash[0..CHECKSUM_SIZE]);

    data
  }

  pub fn sign(&self, data: &[u8]) -> Result<Vec<u8>, KeyError> {
    let mut sha3 = Sha3::keccak256();
    sha3.input(data);
    let mut out = [0u8; util::MESSAGE_SIZE];
    sha3.result(&mut out);

    let message = Message::parse(&out);

    let (signature, recovery) = sign(&message, &self.key);

    let rec_id = recovery.serialize();
    if rec_id != 0 && rec_id != 1 {
      return Err(KeyError::InvalidRecoveryId);
    }

    let mut data = Vec::with_capacity(util::SIGNATURE_SIZE + 1);
    data.extend_from_slice(&signature.r.b32());
    data.extend_from_slice(&signature.s.b32());
    data.push(rec_id);
    Ok(data)
  }

  pub fn derive(&self, index: u32) -> Result<Self, KeyError> {
    use self::data_layout::*;

    if self.depth == std::u8::MAX {
      return Err(KeyError::DeriveDepthTooBig);
    }
    let mut hmac = Hmac::new(Sha512::new(), &self.chaincode);

    let hardened = index >= BIP44_SOFT_UPPER_BOUND;

    if hardened {
      let mut input = Vec::with_capacity(util::SECRET_KEY_SIZE + INDEX_SIZE + 1);
      input.push(0x00);
      input.extend_from_slice(&self.key.serialize());
      if input.write_u32::<BigEndian>(index).is_err() {
        return Err(KeyError::InternalError);
      }
      hmac.input(&input);
    } else {
      let mut input = Vec::with_capacity(util::COMPRESSED_PUBLIC_KEY_SIZE + INDEX_SIZE);
      input.extend_from_slice(&self.public().serialize_compressed());
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

    let mut chaincode = [0u8; CHAIN_CODE_SIZE];
    chaincode.copy_from_slice(&entropy[KEY_SIZE..(KEY_SIZE + CHAIN_CODE_SIZE)]);

    let tweak_pk =
      SecretKey::parse_slice(&entropy[0..KEY_SIZE]).map_err(|err| KeyError::from(err))?;

    let mut newpk = self.key.clone();

    if newpk.tweak_add_assign(&tweak_pk).is_err() {
      if (hardened && index < std::u32::MAX) || (!hardened && index < BIP44_SOFT_UPPER_BOUND) {
        return self.derive(index + 1);
      } else {
        return Err(KeyError::TweakOutOfRange);
      }
    }

    let mut hasher = Ripemd160::new();
    hasher.input(&self.public().compressed_sha256());
    let mut fingerprint = [0u8; FINGERPRINT_SIZE];
    fingerprint.copy_from_slice(&hasher.result()[0..FINGERPRINT_SIZE]);

    Ok(Self {
      key: newpk,
      depth: self.depth + 1,
      chaincode,
      index,
      parent_fingerprint: fingerprint
    })
  }
}
