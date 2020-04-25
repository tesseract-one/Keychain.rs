use crate::keychain::KeyPath as IKeyPath;
use crate::utils::handler::*;
use crate::utils::object::IntoJObject;
use crate::utils::ptr::Ptr;
use crate::utils::result::IntoResult;
use jni::objects::{JClass, JObject};
use jni::sys::{jint, jobject};
use jni::JNIEnv;
use keychain::networks::ethereum::KeyPath;

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_ethereum_KeyPath_newKeyPath(
  env: JNIEnv, _: JClass, account: jint
) -> jobject {
  handle_result(|| {
    KeyPath::new(account as u32).into_result().and_then(|key_path| key_path.into_jobject(&env))
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_ethereum_KeyPath_newMetamask(
  env: JNIEnv, _: JClass, account: jint
) -> jobject {
  handle_result(|| {
    KeyPath::new_metamask(account as u32)
      .into_result()
      .and_then(|key_path| key_path.into_jobject(&env))
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_ethereum_KeyPath_purpose(
  env: JNIEnv, ethereum_key_path: JObject
) -> jint {
  handle_ref(&env, ethereum_key_path, |ethereum_key_path: &mut KeyPath| {
    Ok(ethereum_key_path.purpose() as jint)
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_ethereum_KeyPath_coin(
  env: JNIEnv, ethereum_key_path: JObject
) -> jint {
  handle_ref(&env, ethereum_key_path, |ethereum_key_path: &mut KeyPath| {
    Ok(ethereum_key_path.coin() as jint)
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_ethereum_KeyPath_account(
  env: JNIEnv, ethereum_key_path: JObject
) -> jint {
  handle_ref(&env, ethereum_key_path, |ethereum_key_path: &mut KeyPath| {
    Ok(ethereum_key_path.account() as jint)
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_ethereum_KeyPath_change(
  env: JNIEnv, ethereum_key_path: JObject
) -> jint {
  handle_ref(&env, ethereum_key_path, |ethereum_key_path: &mut KeyPath| {
    Ok(ethereum_key_path.change() as jint)
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_ethereum_KeyPath_address(
  env: JNIEnv, ethereum_key_path: JObject
) -> jint {
  handle_ref(&env, ethereum_key_path, |ethereum_key_path: &mut KeyPath| {
    Ok(ethereum_key_path.address() as jint)
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_ethereum_KeyPath_free(
  env: JNIEnv, ethereum_key_path: JObject
) {
  handle_result(|| ethereum_key_path.free::<KeyPath>(&env))
}
