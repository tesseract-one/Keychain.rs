package one.tesseract.keychain;

class RustObject {
  private final long ptr;

  static {
    System.loadLibrary("rust_keychain_java");
  }

  public RustObject(long ptr) {
    this.ptr = ptr;
  }

  public long getPtr() {
    return this.ptr;
  }
}
