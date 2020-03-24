use super::error::JavaError;
use super::result::IntoResult;
use jni::objects::JString;
use jni::sys::jstring;
use jni::JNIEnv;

pub trait IntoJString {
  fn into_jstring(self, env: &JNIEnv) -> Result<jstring, JavaError>;
}

impl IntoJString for String {
  fn into_jstring(self, env: &JNIEnv) -> Result<jstring, JavaError> {
    env.new_string(self).map(|string| string.into_inner() as jstring).into_result()
  }
}

pub trait IntoString {
  fn into_string(self, env: &JNIEnv) -> Result<String, JavaError>;
}

impl IntoString for JString<'_> {
  fn into_string(self, env: &JNIEnv) -> Result<String, JavaError> {
    env.get_string(self).map(|string| string.into()).into_result()
  }
}
