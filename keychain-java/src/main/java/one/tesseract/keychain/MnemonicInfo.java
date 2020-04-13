package one.tesseract.keychain;

public class MnemonicInfo {
  private final String mnemonic;
  private final Language language;

  public MnemonicInfo(String mnemonic, Language language) {
    this.mnemonic = mnemonic;
    this.language = language;
  }

  public String getMnemonic() {
    return mnemonic;
  }

  public Language getLanguage() {
    return language;
  }
}
