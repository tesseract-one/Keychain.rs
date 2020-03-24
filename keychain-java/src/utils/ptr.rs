use super::error::JavaError;
use super::result::IntoResult;
use jni::objects::JObject;
use jni::sys::jlong;
use jni::JNIEnv;

pub trait AsPtr {
  fn as_ptr(self, env: &JNIEnv) -> Result<jlong, JavaError>;
}

impl AsPtr for JObject<'_> {
  fn as_ptr(self, env: &JNIEnv) -> Result<jlong, JavaError> {
    env.call_method(self, "getPtr", "()J", &[]).and_then(|value| value.j()).into_result()
  }
}
