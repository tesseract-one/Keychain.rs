use keychain::{networks::*, GenericKeyPath, Keychain, KeychainManager, Language, Network};

pub const PACKAGE: &str = "one/tesseract/keychain/";

pub trait JavaClass {
  fn class_name() -> String;
}

impl JavaClass for GenericKeyPath {
  fn class_name() -> String {
    PACKAGE.to_owned() + "GenericKeyPath"
  }
}

impl JavaClass for Keychain {
  fn class_name() -> String {
    PACKAGE.to_owned() + "Keychain"
  }
}

impl JavaClass for KeychainManager {
  fn class_name() -> String {
    PACKAGE.to_owned() + "KeychainManager"
  }
}

impl JavaClass for bitcoin::KeyPath {
  fn class_name() -> String {
    PACKAGE.to_owned() + "bitcoin/KeyPath"
  }
}

impl JavaClass for cardano::KeyPath {
  fn class_name() -> String {
    PACKAGE.to_owned() + "cardano/KeyPath"
  }
}

impl JavaClass for ethereum::KeyPath {
  fn class_name() -> String {
    PACKAGE.to_owned() + "ethereum/KeyPath"
  }
}

impl JavaClass for Language {
  fn class_name() -> String {
    PACKAGE.to_owned() + "Language"
  }
}

impl JavaClass for Network {
  fn class_name() -> String {
    PACKAGE.to_owned() + "Network"
  }
}
