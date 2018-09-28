use key_path::Bip44_KeyPath;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum NetworkType {
  Cardano = 1815,
  Ethereum = 60
}

impl NetworkType {
  pub fn all<'a>() -> &'a [NetworkType] {
    &[NetworkType::Cardano, NetworkType::Ethereum]
  }
}

#[derive(Debug, Copy, Clone)]
pub enum Error {
  WrongKeyPath,
  WrongKeySize,
  BadKeyData,
  SignError
}

#[derive(Debug, Copy, Clone)]
pub enum MnemonicError {
  ToShort,
  ToLong,
  BadWordCount,
  UnsupportedWordFound,
  Unknown
}

#[derive(Debug, Copy, Clone)]
pub struct SeedSize {
  pub min: usize,
  pub max: usize
}

impl SeedSize {
  pub fn min_words(&self) -> usize {
    self.min / 32 * 3
  }

  pub fn max_words(&self) -> usize {
    self.max / 32 * 3
  }
}

pub trait PrivateKey {
  fn from_data(data: &[u8]) -> Result<Self, Error> where Self: Sized;

  fn pub_key(&self, path: &Bip44_KeyPath) -> Result<Vec<u8>, Error>;

  fn sign(&self, data: &[u8], path: &Bip44_KeyPath) -> Result<Vec<u8>, Error>;

  fn boxed(self) -> Box<PrivateKey> where Self: Sized + 'static {
    Box::new(self)
  }
}

pub trait Network {
  fn new() -> Self where Self: Sized;

  fn get_type(&self) -> NetworkType;

  fn get_seed_size(&self) -> SeedSize;

  fn key_from_data(&self, data: &[u8]) -> Result<Box<PrivateKey>, Error>;

  fn key_data_from_mnemonic(&self, mnemonic: &str) -> Result<Vec<u8>, MnemonicError>;

  fn boxed() -> Box<Self> where Self: Sized {
    Box::new(Self::new())
  }
}
