package one.tesseract.keychain;

public interface IKeyPath {
  int purpose();
  int coin();
  int account();
  int change();
  int address();
}
