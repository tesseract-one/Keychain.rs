package one.tesseract.keychain;

class GenericKeyPath extends RustObject implements KeyPath {
  public GenericKeyPath(long ptr) {
    super(ptr);
  }

  public static native GenericKeyPath fromString(String string);
  public native void free();
}
