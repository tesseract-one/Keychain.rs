use network::Network;
use std::cmp;
use bip39;
use rand::Random;

pub fn calculate_seed_size(networks: &[&Network]) -> Option<usize> {
  let mut min = 0;
  let mut max = std::usize::MAX;
  for network in networks.into_iter() {
    let ssize = network.get_seed_size();
    min = cmp::max(min, ssize.min);
    max = cmp::min(max, ssize.max);
  }
  if min == 0 {
    return None;
  }
  if max >= min { Some(min) } else { None }
}

pub fn create_mnemonic_with_size<'a>(size: usize, random: &Random) -> bip39::Result<&'a str> {
  bip39::Type::from_entropy_size(size).map(|etype| {
    &*bip39::Entropy::generate(etype, || random.random()).to_mnemonics().to_string(&bip39::dictionary::ENGLISH)
  })
}

pub fn create_mnemonic<'a>(networks: &[&Network], random: &Random) -> Option<&'a str> {
  calculate_seed_size(networks).and_then(|size| {
    match create_mnemonic_with_size(size, random) {
      Ok(s) => Some(s),
      _ => None
    }
  })
}