package one.tesseract.keychain;

class Keychain extends RustObject {
  public Keychain(long ptr) {
    super(ptr);
  }

  public native Network[] networks();
  public native byte[] pubKey(Network network, IKeyPath path);
  public native byte[] sign(Network network, byte[] data, IKeyPath path);
  public native boolean verify(Network network, byte[] data, byte[] signature, IKeyPath path);
  public native void free();
}
