use super::error::JavaError;
use super::object::IntoJObject;
use super::result::IntoResult;
use crate::java_class::JavaClass;
use jni::objects::JObject;
use jni::sys::{jbyteArray, jobjectArray, jsize};
use jni::JNIEnv;

pub trait IntoJArray {
  fn into_jarray(self, env: &JNIEnv) -> Result<jobjectArray, JavaError>;
}

impl<T: JavaClass> IntoJArray for Vec<T> {
  fn into_jarray(self, env: &JNIEnv) -> Result<jobjectArray, JavaError> {
    env
      .find_class(T::class_name())
      .and_then(|class| env.new_object_array(self.len() as jsize, class, JObject::null()))
      .into_result()
      .and_then(|array| {
        self
          .into_iter()
          .enumerate()
          .map(|(i, object)| {
            object.into_jobject(&env).and_then(|object| {
              env.set_object_array_element(array, i as jsize, object).into_result()
            })
          })
          .collect::<Result<(), JavaError>>()
          .map(|()| array)
      })
  }
}

pub trait IntoJByteArray {
  fn into_jbyte_array(self, env: &JNIEnv) -> Result<jbyteArray, JavaError>;
}

impl IntoJByteArray for Vec<u8> {
  fn into_jbyte_array(self, env: &JNIEnv) -> Result<jbyteArray, JavaError> {
    env.byte_array_from_slice(&self).into_result()
  }
}

pub trait IntoVec {
  fn into_vec(self, env: &JNIEnv) -> Result<Vec<u8>, JavaError>;
}

impl IntoVec for jbyteArray {
  fn into_vec(self, env: &JNIEnv) -> Result<Vec<u8>, JavaError> {
    env.convert_byte_array(self).into_result()
  }
}
