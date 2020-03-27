use crate::utils::array::{IntoJByteArray, IntoVec};
use crate::utils::handler::*;
use crate::utils::object::{IntoJObject, IntoRObject};
use crate::utils::ptr::Ptr;
use crate::utils::result::{IntoResult, Zip};
use crate::utils::string::{IntoJString, IntoString};
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jbyteArray, jobject, jstring};
use jni::JNIEnv;
use keychain::KeychainManager;

#[no_mangle]
pub extern "system" fn Java_one_tesseract_keychain_KeychainManager_newKeychainManager(
  env: JNIEnv, _: JClass
) -> jobject {
  handle_result(|| {
    KeychainManager::new().into_result().and_then(|manager| manager.into_jobject(&env))
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_KeychainManager_hasNetwork(
  env: JNIEnv, manager: JObject, network: JObject
) -> jboolean {
  handle_ref(&env, manager, |manager: &mut KeychainManager| {
    network.into_ref(&env).map(|network| manager.has_network(network) as jboolean)
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_KeychainManager_generateMnemonic(
  env: JNIEnv, manager: JObject, language: JObject
) -> jstring {
  handle_ref(&env, manager, |manager: &mut KeychainManager| {
    language
      .into_owned(&env)
      .and_then(|language| manager.generate_mnemonic(Some(language)).into_result())
      .and_then(|mnemonic| mnemonic.into_jstring(&env))
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_KeychainManager_keychainDataFromSeed(
  env: JNIEnv, manager: JObject, seed: jbyteArray, password: JString
) -> jbyteArray {
  handle_ref(&env, manager, |manager: &mut KeychainManager| {
    password
      .into_string(&env)
      .zip(seed.into_vec(&env))
      .and_then(|(password, seed)| manager.keychain_data_from_seed(&seed, &password).into_result())
      .and_then(|data| data.into_jbyte_array(&env))
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_KeychainManager_keychainDataFromMnemonic(
  env: JNIEnv, manager: JObject, mnemonic: JString, password: JString, language: JObject
) -> jbyteArray {
  handle_ref(&env, manager, |manager: &mut KeychainManager| {
    mnemonic
      .into_string(&env)
      .zip(password.into_string(&env))
      .zip(language.into_owned(&env))
      .and_then(|((mnemonic, password), language)| {
        manager.keychain_data_from_mnemonic(&mnemonic, &password, Some(language)).into_result()
      })
      .and_then(|data| data.into_jbyte_array(&env))
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_KeychainManager_keychainFromData(
  env: JNIEnv, manager: JObject, data: jbyteArray, password: JString
) -> jobject {
  handle_ref(&env, manager, |manager: &mut KeychainManager| {
    data
      .into_vec(&env)
      .zip(password.into_string(&env))
      .and_then(|(data, password)| manager.keychain_from_data(&data, &password).into_result())
      .and_then(|keychain| keychain.into_jobject(&env))
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_KeychainManager_addNetwork(
  env: JNIEnv, manager: JObject, encrypted: jbyteArray, password: JString, network: JObject
) -> jbyteArray {
  handle_ref(&env, manager, |manager: &mut KeychainManager| {
    encrypted
      .into_vec(&env)
      .zip(password.into_string(&env))
      .zip(network.into_owned(&env))
      .and_then(|((encrypted, password), network)| {
        manager.add_network(&encrypted, &password, network).into_result()
      })
      .and_then(|data| data.into_jbyte_array(&env))
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_KeychainManager_changePassword(
  env: JNIEnv, manager: JObject, encrypted: jbyteArray, old_password: JString,
  new_password: JString
) -> jbyteArray {
  handle_ref(&env, manager, |manager: &mut KeychainManager| {
    encrypted
      .into_vec(&env)
      .zip(old_password.into_string(&env))
      .zip(new_password.into_string(&env))
      .and_then(|((encrypted, old_password), new_password)| {
        manager.change_password(&encrypted, &old_password, &new_password).into_result()
      })
      .and_then(|data| data.into_jbyte_array(&env))
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_KeychainManager_free(
  env: JNIEnv, manager: JObject
) {
  handle_result(|| manager.free::<KeychainManager>(&env))
}
