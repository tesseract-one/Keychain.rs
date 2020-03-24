use crate::utils::array::IntoJArray;
use crate::utils::handler::handle_ref;
use jni::objects::JObject;
use jni::sys::jobjectArray;
use jni::JNIEnv;
use keychain::Keychain;

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_Keychain_networks(
  env: JNIEnv, object: JObject
) -> jobjectArray {
  handle_ref(&env, object, |keychain: &mut Keychain| keychain.networks().into_jarray(&env))
}
