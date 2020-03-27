package one.tesseract.keychain;

class Keychain extends RustObject {
  public Keychain(long ptr) {
    super(ptr);
  }

  public native Network[] networks();
  public native byte[] pubKey(Network network, KeyPath path);
  public native byte[] sign(Network network, byte[] data, KeyPath path);
  public native boolean verify(Network network, byte[] data, byte[] signature, KeyPath path);
  public native void free();
}
