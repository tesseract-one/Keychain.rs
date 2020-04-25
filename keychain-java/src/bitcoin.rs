use crate::utils::handler::handle_result;
use crate::utils::handler::*;
use crate::utils::object::IntoJObject;
use crate::utils::ptr::Ptr;
use crate::utils::result::IntoResult;
use jni::objects::{JClass, JObject};
use jni::sys::{jboolean, jint, jobject};
use jni::JNIEnv;
use keychain::networks::bitcoin::KeyPath;
use keychain::KeyPath as IKeyPath;

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_bitcoin_KeyPath_free(
  env: JNIEnv, bitcoin_key_path: JObject
) {
  handle_result(|| bitcoin_key_path.free::<KeyPath>(&env))
}

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

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_bitcoin_KeyPath_purpose(
  env: JNIEnv, bitcoin_key_path: JObject
) -> jint {
  handle_ref(&env, bitcoin_key_path, |bitcoin_key_path: &mut KeyPath| {
    Ok(bitcoin_key_path.purpose() as jint)
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_bitcoin_KeyPath_coin(
  env: JNIEnv, bitcoin_key_path: JObject
) -> jint {
  handle_ref(&env, bitcoin_key_path, |bitcoin_key_path: &mut KeyPath| {
    Ok(bitcoin_key_path.coin() as jint)
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_bitcoin_KeyPath_account(
  env: JNIEnv, bitcoin_key_path: JObject
) -> jint {
  handle_ref(&env, bitcoin_key_path, |bitcoin_key_path: &mut KeyPath| {
    Ok(bitcoin_key_path.account() as jint)
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_bitcoin_KeyPath_change(
  env: JNIEnv, bitcoin_key_path: JObject
) -> jint {
  handle_ref(&env, bitcoin_key_path, |bitcoin_key_path: &mut KeyPath| {
    Ok(bitcoin_key_path.change() as jint)
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_bitcoin_KeyPath_address(
  env: JNIEnv, bitcoin_key_path: JObject
) -> jint {
  handle_ref(&env, bitcoin_key_path, |bitcoin_key_path: &mut KeyPath| {
    Ok(bitcoin_key_path.address() as jint)
  })
}
