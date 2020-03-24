use crate::utils::array::{IntoJByteArray, IntoVec};
use crate::utils::handler::*;
use crate::utils::object::{IntoJObject, IntoRObject};
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
