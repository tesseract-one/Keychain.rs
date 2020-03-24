package one.tesseract.keychain;

class KeychainManager extends RustObject {
  public KeychainManager(long ptr) {
    super(ptr);
  }

  public static native KeychainManager newKeychainManager();
  public native boolean hasNetwork(Network network);
  public native String generateMnemonic(Language language);
  public native byte[] keychainDataFromSeed(byte[] seed, String password);
}
