use rand_os::rand_core::RngCore;
use rand_os::OsRng;
use std::sync::Mutex;

pub trait Entropy {
  fn fill_bytes(&self, into: &mut [u8]);
}

pub struct OsEntropy {
  random: Mutex<OsRng>
}

impl OsEntropy {
  pub fn new() -> Self {
    Self { random: Mutex::new(OsRng) }
  }
}

impl Entropy for OsEntropy {
  fn fill_bytes(&self, into: &mut [u8]) {
    let rand = &mut self.random.lock().unwrap();
    rand.fill_bytes(into);
  }
}
