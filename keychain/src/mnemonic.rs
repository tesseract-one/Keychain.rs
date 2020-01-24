use bip39;
use crate::entropy::Entropy;
use std::fmt;

pub const SEED_SIZE: usize = bip39::SEED_SIZE;

#[derive(Primitive, Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Language {
  English = 0,
  French = 1,
  Japanese = 2,
  Korean = 3,
  ChineseSimplified = 4,
  ChineseTraditional = 5,
  Italian = 6,
  Spanish = 7
}

impl Language {
  pub fn to_dict<'a>(&self) -> &'a bip39::dictionary::DefaultDictionary {
    match self {
      &Language::English => &bip39::dictionary::ENGLISH,
      &Language::French => &bip39::dictionary::FRENCH,
      &Language::Japanese => &bip39::dictionary::JAPANESE,
      &Language::Korean => &bip39::dictionary::KOREAN,
      &Language::ChineseSimplified => &bip39::dictionary::CHINESE_SIMPLIFIED,
      &Language::ChineseTraditional => &bip39::dictionary::CHINESE_TRADITIONAL,
      &Language::Italian => &bip39::dictionary::ITALIAN,
      &Language::Spanish => &bip39::dictionary::SPANISH
    }
  }
}

impl Default for Language {
  fn default() -> Self {
    Language::English
  }
}

#[derive(Debug)]
pub enum Error {
  MnemonicToShort(usize, usize),
  MnemonicToLong(usize, usize),
  WrongNumberOfWords(usize),
  UnsupportedWordFound(String),
  InvalidEntropySize(usize),
  UnknownError(Box<dyn std::error::Error>)
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &Error::MnemonicToShort(size, min) => write!(f, "Mnemonic {} too short. Min: {}", size, min),
      &Error::MnemonicToLong(size, max) => write!(f, "Mnemonic {} too short. Max: {}", size, max),
      &Error::WrongNumberOfWords(count) => write!(f, "Wrong number of words {}", count),
      &Error::UnsupportedWordFound(ref word) => {
        write!(f, "Unsupported word found '{}'. Maybe wrong dictionary?", word)
      }
      &Error::InvalidEntropySize(size) => write!(f, "Invalid entropy size {}", size),
      &Error::UnknownError(ref err) => write!(f, "Unknown mnemonic error: {}", err)
    }
  }
}

impl std::error::Error for Error {}

impl From<bip39::Error> for Error {
  fn from(err: bip39::Error) -> Self {
    match err {
      bip39::Error::WrongNumberOfWords(words) => Error::WrongNumberOfWords(words),
      bip39::Error::WrongKeySize(size) => Error::InvalidEntropySize(size),
      bip39::Error::LanguageError(ref err) => match err {
        &bip39::dictionary::Error::MnemonicWordNotFoundInDictionary(ref word) => {
          Error::UnsupportedWordFound(word.clone())
        }
      },
      _ => Error::UnknownError(Box::new(err))
    }
  }
}

pub fn generate_entropy(size: usize, entropy: &dyn Entropy) -> Result<Vec<u8>, Error> {
  let bip_type = bip39::Type::from_entropy_size(size).map_err(|e| Error::from(e))?;
  let generated = bip39::Entropy::generate(bip_type, |bytes| entropy.fill_bytes(bytes));
  Ok(Vec::from(generated.as_ref()))
}

pub fn mnemonic_from_entropy(bytes: &[u8], language: Language) -> Result<String, Error> {
  bip39::Entropy::from_slice(bytes)
    .map(|ent| (*ent.to_mnemonics().to_string(language.to_dict())).to_owned())
    .map_err(|err| err.into())
}

pub fn seed_from_mnemonic(
  mnemonic: &str, unique: &str, size: usize, language: Language
) -> Result<Vec<u8>, Error> {
  let mnemonics =
    bip39::Mnemonics::from_string(language.to_dict(), mnemonic).map_err(|err| Error::from(err))?;
  let size_words = size / 32 * 3;
  let words_count = mnemonics.get_type().mnemonic_count();
  if size_words > words_count {
    return Err(Error::MnemonicToShort(words_count, size_words));
  }
  if size_words < words_count {
    return Err(Error::MnemonicToLong(words_count, size_words));
  }
  let mnemonic_string = mnemonics.to_string(language.to_dict());
  Ok(Vec::from(bip39::Seed::from_mnemonic_string(&mnemonic_string, unique.as_bytes()).as_ref()))
}
