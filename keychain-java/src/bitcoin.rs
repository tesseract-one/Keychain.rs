use crate::utils::handler::handle_result;
use crate::utils::object::IntoJObject;
use crate::utils::result::IntoResult;
use jni::objects::JClass;
use jni::sys::{jboolean, jint, jobject};
use jni::JNIEnv;
use keychain::networks::bitcoin::KeyPath;

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_bitcoin_KeyPath_bip44(
  env: JNIEnv, _: JClass, testnet: jboolean, account: jint, change: jint, address: jint
) -> jobject {
  handle_result(|| {
    KeyPath::bip44(testnet != 0, account as u32, change as u32, address as u32)
      .into_result()
      .and_then(|key_path| key_path.into_jobject(&env))
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_bitcoin_KeyPath_bip49(
  env: JNIEnv, _: JClass, testnet: jboolean, account: jint, change: jint, address: jint
) -> jobject {
  handle_result(|| {
    KeyPath::bip49(testnet != 0, account as u32, change as u32, address as u32)
      .into_result()
      .and_then(|key_path| key_path.into_jobject(&env))
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_bitcoin_KeyPath_bip84(
  env: JNIEnv, _: JClass, testnet: jboolean, account: jint, change: jint, address: jint
) -> jobject {
  handle_result(|| {
    KeyPath::bip84(testnet != 0, account as u32, change as u32, address as u32)
      .into_result()
      .and_then(|key_path| key_path.into_jobject(&env))
  })
}
