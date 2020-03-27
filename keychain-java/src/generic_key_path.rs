use crate::utils::handler::*;
use crate::utils::object::IntoJObject;
use crate::utils::ptr::Ptr;
use crate::utils::result::IntoResult;
use crate::utils::string::IntoString;
use jni::objects::{JClass, JObject, JString};
use jni::sys::jobject;
use jni::JNIEnv;
use keychain::GenericKeyPath;

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_GenericKeyPath_fromString(
  env: JNIEnv, _: JClass, string: JString
) -> jobject {
  handle_result(|| {
    string
      .into_string(&env)
      .and_then(|string| GenericKeyPath::from(&string).into_result())
      .and_then(|key_path| key_path.into_jobject(&env))
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_GenericKeyPath_free(
  env: JNIEnv, generic_key_path: JObject
) {
  handle_result(|| generic_key_path.free::<GenericKeyPath>(&env))
}
