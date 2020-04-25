package one.tesseract.keychain;

class Network extends RustObject {
  public Network(long ptr) {
    super(ptr);
  }
  
  @Override
  public native void free();
}
