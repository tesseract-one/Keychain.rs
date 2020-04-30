package one.tesseract.keychain;

public class GenericKeyPath extends RustObject implements IKeyPath {
  public GenericKeyPath(long ptr) {
    super(ptr);
  }

  public static native GenericKeyPath fromString(String string);

  @Override
  public native void free();

  @Override
  public native int purpose();

  @Override
  public native int coin();

  @Override
  public native int account();

  @Override
  public native int change();

  @Override
  public native int address();
}
