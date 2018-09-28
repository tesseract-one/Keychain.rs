use network::{ Network as INetwork, NetworkType, PrivateKey as IPrivateKey, SeedSize, Error, MnemonicError };
use super::cardano::hdwallet::{ XPrv };
use super::bip39::{ Seed, MnemonicString, dictionary };
use super::private_key::PrivateKey;

const UNIQUE_SEED_VALUE: &[u8] = b"CARDANO_seed_VALUE";

pub struct Network;

impl INetwork for Network {
  fn new() -> Self {
    Network {}
  }

  fn get_type(&self) -> NetworkType {
    NetworkType::Cardano
  }

  fn get_seed_size(&self) -> SeedSize {
    SeedSize { min: 96, max: 256 }
  }

  fn key_from_data(&self, data: &[u8]) -> Result<Box<IPrivateKey>, Error> {
    PrivateKey::from_data(data).map(|pk| pk.boxed())
  }

  fn key_data_from_mnemonic(&self, mnemonic: &str) -> Result<Vec<u8>, MnemonicError> {
    let seed_size = self.get_seed_size();
    let words = mnemonic.split(" ").filter(|part| part.len() > 0).collect::<Vec<&str>>();
    if words.len() > seed_size.max_words() {
      return Err(MnemonicError::ToLong);
    }
    if words.len() < seed_size.min_words() {
      return Err(MnemonicError::ToShort);
    }
    if words.len() % 3 != 0 {
      return Err(MnemonicError::BadWordCount);
    }
    MnemonicString::new(&dictionary::ENGLISH, mnemonic.to_owned())
      .map_err(|_| MnemonicError::UnsupportedWordFound)
      .map(|mstring| {
        let seed = Seed::from_mnemonic_string(&mstring, UNIQUE_SEED_VALUE);
        let xprv = XPrv::generate_from_bip39(&seed);
        Vec::from(xprv.as_ref())
      })
  }
}