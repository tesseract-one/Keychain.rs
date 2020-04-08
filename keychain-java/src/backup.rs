use crate::utils::array::IntoVec;
use crate::utils::handler::*;
use crate::utils::map::IntoJMap;
use crate::utils::result::{IntoResult, Zip};
use crate::utils::string::IntoString;
use jni::objects::{JObject, JString};
use jni::sys::{jbyteArray, jobject};
use jni::JNIEnv;
use keychain::KeychainManager;

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
