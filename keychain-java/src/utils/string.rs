use super::result::{IntoResult, JResult};
use jni::objects::JString;
use jni::sys::jstring;
use jni::JNIEnv;

pub trait IntoJString {
  fn into_jstring(self, env: &JNIEnv) -> JResult<jstring>;
}

impl IntoJString for String {
  fn into_jstring(self, env: &JNIEnv) -> JResult<jstring> {
    env.new_string(self).map(|string| string.into_inner() as jstring).into_result()
  }
}

pub trait IntoString {
  fn into_string(self, env: &JNIEnv) -> JResult<String>;
}

impl IntoString for JString<'_> {
  fn into_string(self, env: &JNIEnv) -> JResult<String> {
    env.get_string(self).map(|string| string.into()).into_result()
  }
}
