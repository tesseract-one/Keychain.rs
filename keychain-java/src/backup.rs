use super::java_class::{JavaClass, PACKAGE};
use super::jni_type::JniType;
use crate::utils::array::IntoVec;
use crate::utils::handler::*;
use crate::utils::map::IntoJMap;
use crate::utils::object::IntoJObject;
use crate::utils::result::{IntoResult, JResult, Zip};
use crate::utils::string::{IntoJString, IntoString};
use jni::objects::{JObject, JString};
use jni::sys::{jbyteArray, jobject};
use jni::JNIEnv;
use keychain::{KeychainManager, Language};

struct MnemonicInfo {}

impl JavaClass for MnemonicInfo {
  fn class_name() -> String {
    PACKAGE.to_owned() + "MnemonicInfo"
  }
}

impl MnemonicInfo {
  fn new(env: &JNIEnv, mnemonic: String, language: Language) -> JResult<jobject> {
    mnemonic.into_jstring(env).zip(language.into_jobject(env)).and_then(|(mnemonic, language)| {
      env
        .find_class(Self::class_name())
        .and_then(|class| {
          env.new_object(
            class,
            format!("({}{})V", String::jni_type_signature(), Language::jni_type_signature()),
            &[mnemonic.into(), language.into()]
          )
        })
        .map(|mnemonic_info| mnemonic_info.into_inner())
        .into_result()
    })
  }
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_KeychainManager_getKeysData(
  env: JNIEnv, manager: JObject, encrypted: jbyteArray, password: JString
) -> jobject {
  handle_ref(&env, manager, |manager: &mut KeychainManager| {
    encrypted
      .into_vec(&env)
      .zip(password.into_string(&env))
      .and_then(|(encrypted, password)| manager.get_keys_data(&encrypted, &password).into_result())
      .and_then(|keys_data| keys_data.into_iter().into_jmap(&env))
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_KeychainManager_retrieveMnemonic(
  env: JNIEnv, manager: JObject, encrypted: jbyteArray, password: JString
) -> jobject {
  handle_ref(&env, manager, |manager: &mut KeychainManager| {
    encrypted
      .into_vec(&env)
      .zip(password.into_string(&env))
      .and_then(|(encrypted, password)| {
        manager.retrieve_mnemonic(&encrypted, &password).into_result()
      })
      .and_then(|(mnemonic, language)| MnemonicInfo::new(&env, mnemonic, language))
  })
}
