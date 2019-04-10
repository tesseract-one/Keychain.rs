use bip39;
use entropy::Entropy;
use std::fmt;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Language {
  English,
  French,
  Japanese,
  Korean,
  ChineseSimplified,
  ChineseTraditional,
  Italian,
  Spanish
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
  UnknownError(Box<std::error::Error>)
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &Error::MnemonicToShort(size, min) => write!(f, "Mnemonic {} too short. Min: {}", size, min),
      &Error::MnemonicToLong(size, max) => write!(f, "Mnemonic {} too short. Max: {}", size, max),
      &Error::WrongNumberOfWords(count) => write!(f, "Wrong number of words {}", count),
      &Error::UnsupportedWordFound(ref word) => write!(f, "Unsupported word found '{}'. Maybe wrong dictionary?", word),
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
        &bip39::dictionary::Error::MnemonicWordNotFoundInDictionary(ref word) => 
          Error::UnsupportedWordFound(word.clone())
      },
      _ => Error::UnknownError(Box::new(err))
    }
  }
}


pub fn generate(size: usize, language: Language, entropy: &Entropy) -> Result<String, Error> {
  bip39::Type::from_entropy_size(size).map(|etype| {
    (*bip39::Entropy::generate(etype, || {
        let mut buf = [0u8];
        entropy.fill_bytes(&mut buf);
        buf[0]
      })
      .to_mnemonics()
      .to_string(language.to_dict())
    ).to_owned()
  }).map_err(|err| err.into())
}