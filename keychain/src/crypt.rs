use cryptoxide::chacha20poly1305::ChaCha20Poly1305;
use cryptoxide::hmac::Hmac;
use cryptoxide::pbkdf2::pbkdf2;
use cryptoxide::sha2::Sha512;
use entropy::Entropy;
use std::error;
use std::iter::repeat;

mod password_encryption_parameter {
  pub const ITER: u32 = 19_162;
  pub const SALT_SIZE: usize = 32;
  pub const NONCE_SIZE: usize = 12;
  pub const KEY_SIZE: usize = 32;
  pub const TAG_SIZE: usize = 16;

  pub const METADATA_SIZE: usize = SALT_SIZE + NONCE_SIZE + TAG_SIZE;

  pub const SALT_START: usize = 0;
  pub const SALT_END: usize = SALT_START + SALT_SIZE;
  pub const NONCE_START: usize = SALT_END;
  pub const NONCE_END: usize = NONCE_START + NONCE_SIZE;
  pub const TAG_START: usize = NONCE_END;
  pub const TAG_END: usize = TAG_START + TAG_SIZE;
  pub const ENCRYPTED_START: usize = TAG_END;
}

#[derive(Debug)]
pub enum DecryptError {
  NotEnoughData,
  DecryptionFailed
}

impl std::fmt::Display for DecryptError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      &DecryptError::NotEnoughData => write!(f, "Not enough data for decryption."),
      &DecryptError::DecryptionFailed => write!(f, "Decryption failed. Check your key")
    }
  }
}

impl error::Error for DecryptError {}

pub fn encrypt(data: &[u8], password: &str, entropy: &Entropy) -> Vec<u8> {
  use self::password_encryption_parameter::*;
  let mut salt: [u8; SALT_SIZE] = [0; SALT_SIZE];
  let mut nonce: [u8; NONCE_SIZE] = [0; NONCE_SIZE];

  entropy.fill_bytes(&mut salt);
  entropy.fill_bytes(&mut nonce);

  let key = {
    let mut mac = Hmac::new(Sha512::new(), password.as_bytes());
    let mut key: Vec<u8> = repeat(0).take(KEY_SIZE).collect();
    pbkdf2(&mut mac, &salt[..], ITER, &mut key);
    key
  };

  let mut tag = [0; TAG_SIZE];
  let mut encrypted: Vec<u8> = repeat(0).take(data.len()).collect();

  ChaCha20Poly1305::new(&key, &nonce, &[]).encrypt(&data, &mut encrypted, &mut tag);

  let mut output = Vec::with_capacity(data.len() + METADATA_SIZE);
  output.extend_from_slice(&salt);
  output.extend_from_slice(&nonce);
  output.extend_from_slice(&tag);
  output.extend_from_slice(&encrypted);
  output
}

pub fn decrypt(data: &[u8], password: &str) -> Result<Vec<u8>, DecryptError> {
  use self::password_encryption_parameter::*;

  if data.len() <= METADATA_SIZE {
    // not enough input to decrypt.
    return Err(DecryptError::NotEnoughData);
  }

  let salt = &data[SALT_START..SALT_END];
  let nonce = &data[NONCE_START..NONCE_END];
  let tag = &data[TAG_START..TAG_END];
  let encrypted = &data[ENCRYPTED_START..];

  let key = {
    let mut mac = Hmac::new(Sha512::new(), password.as_bytes());
    let mut key: Vec<u8> = repeat(0).take(KEY_SIZE).collect();
    pbkdf2(&mut mac, &salt[..], ITER, &mut key);
    key
  };

  let mut decrypted: Vec<u8> = repeat(0).take(encrypted.len()).collect();
  let decryption_succeed =
    ChaCha20Poly1305::new(&key, &nonce, &[]).decrypt(&encrypted, &mut decrypted, &tag);

  if decryption_succeed {
    Ok(decrypted)
  } else {
    Err(DecryptError::DecryptionFailed)
  }
}
