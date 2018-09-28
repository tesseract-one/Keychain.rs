use bip39;
use rand::rngs::OsRng;
use rand::RngCore;

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

pub fn generate(size: usize, language: Language, random: &mut OsRng) -> bip39::Result<String> {
  bip39::Type::from_entropy_size(size).map(|etype| {
    (*bip39::Entropy::generate(etype, || random.next_u32() as u8)
      .to_mnemonics()
      .to_string(language.to_dict())
    ).to_owned()
  })
}