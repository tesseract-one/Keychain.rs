use crate::java_class::JavaClass;
use crate::utils::array::IntoJByteArray;
use crate::utils::error::JavaError;
use crate::utils::object::IntoJObject;
use jni::objects::JValue;
use jni::JNIEnv;

pub trait IntoJValue<'a> {
  fn into_jvalue(self, env: &JNIEnv) -> Result<JValue<'a>, JavaError>;
}

impl<'a, T: JavaClass> IntoJValue<'a> for T {
  fn into_jvalue(self, env: &JNIEnv) -> Result<JValue<'a>, JavaError> {
    self.into_jobject(env).map(|object| object.into())
  }
}

impl<'a> IntoJValue<'a> for Vec<u8> {
  fn into_jvalue(self, env: &JNIEnv) -> Result<JValue<'a>, JavaError> {
    self.into_jbyte_array(env).map(|data| data.into())
  }
}
