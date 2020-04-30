package one.tesseract.keychain;

public class Network extends RustObject {
  public Network(long ptr) {
    super(ptr);
  }
  
  @Override
  public native void free();
}
