use keychain::{Keychain, KeychainManager, Language, Network};

pub trait JavaClass {
  fn class_name() -> &'static str;
}

impl JavaClass for Keychain {
  fn class_name() -> &'static str {
    "one/tesseract/keychain/Keychain"
  }
}

impl JavaClass for KeychainManager {
  fn class_name() -> &'static str {
    "one/tesseract/keychain/KeychainManager"
  }
}

impl JavaClass for Network {
  fn class_name() -> &'static str {
    "one/tesseract/keychain/Network"
  }
}

impl JavaClass for Language {
  fn class_name() -> &'static str {
    "one/tesseract/keychain/Language"
  }
}
