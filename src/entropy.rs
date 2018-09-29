use rand::rngs::OsRng;
use rand::RngCore;
use std::sync::Mutex;

pub trait Provider {
  fn fill_bytes(&self, into: &mut [u8]);
}

pub struct Entropy {
  random: Mutex<OsRng>
}

impl Entropy {
  pub fn new() -> Result<Self, rand::Error> {
    OsRng::new().map(|rand| Self { random: Mutex::new(rand) })
  }
}

impl Provider for Entropy {
  fn fill_bytes(&self, into: &mut [u8]) {
    let rand = &mut self.random.lock().unwrap();
    rand.fill_bytes(into);
  }
}