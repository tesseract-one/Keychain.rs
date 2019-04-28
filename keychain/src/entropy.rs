use rand::rngs::OsRng;
use rand::RngCore;
use std::sync::Mutex;

pub trait Entropy {
  fn fill_bytes(&self, into: &mut [u8]);
}

pub struct OsEntropy {
  random: Mutex<OsRng>
}

pub type OsEntropyError = rand::Error;

impl OsEntropy {
  pub fn new() -> Result<Self, OsEntropyError> {
    OsRng::new().map(|rand| Self { random: Mutex::new(rand) })
  }
}

impl Entropy for OsEntropy {
  fn fill_bytes(&self, into: &mut [u8]) {
    let rand = &mut self.random.lock().unwrap();
    rand.fill_bytes(into);
  }
}
