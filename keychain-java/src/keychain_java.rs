use crate::utils::array::{IntoJArray, IntoJByteArray, IntoVec};
use crate::utils::handler::*;
use crate::utils::object::IntoRObject;
use crate::utils::ptr::Ptr;
use crate::utils::result::{IntoResult, Zip};
use jni::objects::JObject;
use jni::sys::{jboolean, jbyteArray, jobjectArray};
use jni::JNIEnv;
use keychain::{GenericKeyPath, Keychain};

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_Keychain_networks(
  env: JNIEnv, keychain: JObject
) -> jobjectArray {
  handle_ref(&env, keychain, |keychain: &mut Keychain| keychain.networks().into_jarray(&env))
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_Keychain_pubKey(
  env: JNIEnv, keychain: JObject, network: JObject, path: JObject
) -> jbyteArray {
  handle_ref(&env, keychain, |keychain: &mut Keychain| {
    network
      .into_ref(&env)
      .zip(path.into_ref(&env))
      .and_then(|(network, path): (_, &mut GenericKeyPath)| {
        keychain.pub_key(network, path).into_result()
      })
      .and_then(|data| data.into_jbyte_array(&env))
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_Keychain_sign(
  env: JNIEnv, keychain: JObject, network: JObject, data: jbyteArray, path: JObject
) -> jbyteArray {
  handle_ref(&env, keychain, |keychain: &mut Keychain| {
    network
      .into_ref(&env)
      .zip(data.into_vec(&env))
      .zip(path.into_ref(&env))
      .and_then(|((network, data), path): ((_, _), &mut GenericKeyPath)| {
        keychain.sign(network, &data, path).into_result()
      })
      .and_then(|data| data.into_jbyte_array(&env))
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_Keychain_verify(
  env: JNIEnv, keychain: JObject, network: JObject, data: jbyteArray, signature: jbyteArray,
  path: JObject
) -> jboolean {
  handle_ref(&env, keychain, |keychain: &mut Keychain| {
    network
      .into_ref(&env)
      .zip(data.into_vec(&env))
      .zip(signature.into_vec(&env))
      .zip(path.into_ref(&env))
      .and_then(|(((network, data), signature), path): ((_, _), &mut GenericKeyPath)| {
        keychain.verify(network, &data, &signature, path).into_result()
      })
      .map(|result| result as jboolean)
  })
}

#[no_mangle]
pub unsafe extern "system" fn Java_one_tesseract_keychain_Keychain_free(
  env: JNIEnv, keychain: JObject
) {
  handle_result(|| keychain.free::<Keychain>(&env))
}
