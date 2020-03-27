use crate::utils::handler::*;
use crate::utils::ptr::Ptr;
use jni::objects::JObject;
use jni::JNIEnv;
use keychain::Network;

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_Network_free(
  env: JNIEnv, network: JObject
) {
  handle_result(|| network.free::<Network>(&env))
}
