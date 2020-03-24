package one.tesseract.keychain;

class Keychain extends RustObject {
  public Keychain(long ptr) {
    super(ptr);
  }

  public native Network[] networks();
}
