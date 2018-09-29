use network::{ Network as INetwork, SeedSize };
use private_key::{ PrivateKey as IPrivateKey, Error};
use mnemonic::{ Error as MnemonicError };
use network_type::NetworkType;
use bip39::{ Seed, MnemonicString, dictionary };
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

  fn key_data_from_mnemonic(&self, mnemonic: &str) -> Result<Vec<u8>, Error> {
    let seed_size = self.get_seed_size();
    let words = mnemonic.split(" ").filter(|part| part.len() > 0).collect::<Vec<&str>>();
    if words.len() > seed_size.max_words() {
      return Err(MnemonicError::MnemonicToLong(words.len(), seed_size.max_words()).into());
    }
    if words.len() < seed_size.min_words() {
      return Err(MnemonicError::MnemonicToShort(words.len(), seed_size.min_words()).into());
    }
    if words.len() % 3 != 0 {
      return Err(MnemonicError::WrongNumberOfWords(words.len()).into());
    }
    MnemonicString::new(&dictionary::ENGLISH, mnemonic.to_owned())
      .map_err(|err| Into::<MnemonicError>::into(err).into())
      .map(|mstring| {
        let seed = Seed::from_mnemonic_string(&mstring, UNIQUE_SEED_VALUE);
        PrivateKey::data_from_seed(&seed)
      })
  }
}