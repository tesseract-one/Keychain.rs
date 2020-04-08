use super::object::IntoRObject;
use super::result::{IntoResult, JResult};
use crate::java_class::JavaClass;
use jni::objects::JObject;
use jni::sys::jlong;
use jni::JNIEnv;

pub trait Ptr {
  fn as_ptr(self, env: &JNIEnv, is_owned: bool) -> JResult<jlong>;
  unsafe fn free<T: JavaClass>(self, env: &JNIEnv) -> JResult<()>;
}

impl Ptr for JObject<'_> {
  fn as_ptr(self, env: &JNIEnv, is_owned: bool) -> JResult<jlong> {
    env
      .call_method(self, "getPtr", "(Z)J", &[is_owned.into()])
      .and_then(|value| value.j())
      .into_result()
  }

  unsafe fn free<T: JavaClass>(self, env: &JNIEnv) -> JResult<()> {
    self.into_owned(env).map(|_: T| ())
  }
}
