use crate::utils::handler::handle_result;
use crate::utils::handler::*;
use crate::utils::object::IntoJObject;
use crate::utils::ptr::Ptr;
use crate::utils::result::IntoResult;
use jni::objects::{JClass, JObject};
use jni::sys::{jint, jobject};
use jni::JNIEnv;
use keychain::networks::cardano::KeyPath;
use keychain::KeyPath as IKeyPath;

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_cardano_KeyPath_free(
  env: JNIEnv, cardano_key_path: JObject
) {
  handle_result(|| cardano_key_path.free::<KeyPath>(&env))
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_cardano_KeyPath_newKeyPath(
  env: JNIEnv, _: JClass, account: jint, change: jint, address: jint
) -> jobject {
  handle_result(|| {
    KeyPath::new(account as u32, change as u32, address as u32)
      .into_result()
      .and_then(|key_path| key_path.into_jobject(&env))
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_cardano_KeyPath_purpose(
  env: JNIEnv, cardano_key_path: JObject
) -> jint {
  handle_ref(&env, cardano_key_path, |bitcoin_key_path: &mut KeyPath| {
    Ok(bitcoin_key_path.purpose() as jint)
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_cardano_KeyPath_coin(
  env: JNIEnv, cardano_key_path: JObject
) -> jint {
  handle_ref(&env, cardano_key_path, |bitcoin_key_path: &mut KeyPath| {
    Ok(bitcoin_key_path.coin() as jint)
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_cardano_KeyPath_account(
  env: JNIEnv, cardano_key_path: JObject
) -> jint {
  handle_ref(&env, cardano_key_path, |bitcoin_key_path: &mut KeyPath| {
    Ok(bitcoin_key_path.account() as jint)
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_cardano_KeyPath_change(
  env: JNIEnv, cardano_key_path: JObject
) -> jint {
  handle_ref(&env, cardano_key_path, |bitcoin_key_path: &mut KeyPath| {
    Ok(bitcoin_key_path.change() as jint)
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_cardano_KeyPath_address(
  env: JNIEnv, cardano_key_path: JObject
) -> jint {
  handle_ref(&env, cardano_key_path, |bitcoin_key_path: &mut KeyPath| {
    Ok(bitcoin_key_path.address() as jint)
  })
}
