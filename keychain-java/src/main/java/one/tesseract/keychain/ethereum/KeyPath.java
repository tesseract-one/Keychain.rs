package one.tesseract.keychain.ethereum;

import one.tesseract.keychain.IKeyPath;
import one.tesseract.keychain.RustObject;

public class KeyPath extends RustObject implements IKeyPath{
  public KeyPath(long ptr) {
    super(ptr);
  }

  public static native KeyPath newKeyPath(int account);
  public static native KeyPath newMetamask(int account);

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

  @Override
  public native void free();
}
