package one.tesseract.keychain.cardano;

import one.tesseract.keychain.IKeyPath;
import one.tesseract.keychain.RustObject;

public class KeyPath extends RustObject implements IKeyPath {
  public KeyPath(long ptr) {
    super(ptr);
  }

  @Override
  public native void free();

  public static native KeyPath newKeyPath(boolean testnet, int account, int change, int address);

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
