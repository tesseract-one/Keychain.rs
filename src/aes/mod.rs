extern crate cryptoxide;

mod aessafe;
mod cryptoutil;
mod step_by;
mod simd;
mod blockmodes;

use self::cryptoxide::pbkdf2;

pub fn encrypt(data: &[u8], password: &str) -> Vec<u8> {
  // aessafe::AesSafe256Encryptor
  return Vec::new();
}

pub fn decrypt(data: &[u8], password: &str) ->  Vec<u8> {
  return Vec::new();
}