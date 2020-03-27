package one.tesseract.keychain;

class KeychainManager extends RustObject {
  public KeychainManager(long ptr) {
    super(ptr);
  }

  public static native KeychainManager newKeychainManager();
  public native boolean hasNetwork(Network network);
  public native String generateMnemonic(Language language);
  public native byte[] keychainDataFromSeed(byte[] seed, String password);
  public native byte[] keychainDataFromMnemonic(String mnemonic, String password, Language language);
  public native Keychain keychainFromData(byte[] data, String password);
  public native byte[] addNetwork(byte[] encrypted, String password, Network network);
  public native byte[] changePassword(byte[] encrypted, String oldPassword, String newPassword);
  public native void free();
}
