use super::value::IntoJValue;
use crate::jni_type::JniType;
use crate::utils::error::JavaError;
use crate::utils::result::{IntoResult, Zip};
use jni::objects::JObject;
use jni::sys::jobject;
use jni::JNIEnv;
use std::iter::Iterator;

fn create_map<'a>(env: &JNIEnv<'a>) -> Result<JObject<'a>, JavaError> {
  env
    .find_class("java/util/HashMap")
    .into_result()
    .and_then(|class| env.new_object(class, "()V", &[]).into_result())
}

fn put<'a, K, V>(env: &JNIEnv<'a>, map: JObject<'a>, key: K, value: V) -> Result<(), JavaError>
where
  K: IntoJValue<'a> + JniType,
  V: IntoJValue<'a> + JniType
{
  key
    .into_jvalue(env)
    .zip(value.into_jvalue(&env))
    .and_then(|(key, value)| {
      env
        .call_method(
          map,
          "put",
          format!(
            "({}{}){}",
            K::jni_type_signature(),
            V::jni_type_signature(),
            V::jni_type_signature()
          ),
          &[key, value]
        )
        .into_result()
    })
    .map(|_| ())
}

pub trait IntoJMap<'a> {
  fn into_jmap(self, env: &JNIEnv<'a>) -> Result<jobject, JavaError>;
}

impl<'a, T, K, V> IntoJMap<'a> for T
where
  K: IntoJValue<'a> + JniType,
  V: IntoJValue<'a> + JniType,
  T: Iterator<Item = (K, V)>
{
  fn into_jmap(self, env: &JNIEnv<'a>) -> Result<jobject, JavaError> {
    create_map(&env).and_then(|map| {
      self
        .into_iter()
        .map(|(key, value)| put(env, map, key, value))
        .collect::<Result<(), JavaError>>()
        .map(|_| map.into_inner())
    })
  }
}
