use super::error::JavaError;
use super::object::IntoRObject;
use super::result::IntoResult;
use crate::java_class::JavaClass;
use jni::objects::JObject;
use jni::sys::jlong;
use jni::JNIEnv;

pub trait Ptr {
  fn as_ptr(self, env: &JNIEnv) -> Result<jlong, JavaError>;
  unsafe fn free<T: JavaClass>(self, env: &JNIEnv) -> Result<(), JavaError>;
}

impl Ptr for JObject<'_> {
  fn as_ptr(self, env: &JNIEnv) -> Result<jlong, JavaError> {
    env.call_method(self, "getPtr", "()J", &[]).and_then(|value| value.j()).into_result()
  }

  unsafe fn free<T: JavaClass>(self, env: &JNIEnv) -> Result<(), JavaError> {
    self.into_owned(env).map(|_: T| ())
  }
}
