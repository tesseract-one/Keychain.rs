package one.tesseract.keychain;

public abstract class RustObject {
  private long ptr;

  static {
    System.loadLibrary("rust_keychain_java");
  }

  public RustObject(long ptr) {
    this.ptr = ptr;
  }

  public long getPtr(boolean isOwned) {
    long ptr = this.ptr;
    if (isOwned) {
      this.ptr = 0;
    }
    return ptr;
  }

  public abstract void free();
}
