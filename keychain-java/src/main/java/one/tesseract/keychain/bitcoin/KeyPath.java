package one.tesseract.keychain.bitcoin;

import one.tesseract.keychain.IKeyPath;
import one.tesseract.keychain.RustObject;

public class KeyPath extends RustObject implements IKeyPath {
  public KeyPath(long ptr) {
    super(ptr);
  }

  public static native KeyPath bip44(boolean testnet, int account, int change, int address);
  public static native KeyPath bip84(boolean testnet, int account, int change, int address);
  public static native KeyPath bip49(boolean testnet, int account, int change, int address);

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
