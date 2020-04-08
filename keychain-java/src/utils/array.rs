use super::object::IntoJObject;
use super::result::{IntoResult, JResult};
use crate::java_class::JavaClass;
use jni::objects::JObject;
use jni::sys::{jbyteArray, jobjectArray, jsize};
use jni::JNIEnv;

pub trait IntoJArray {
  fn into_jarray(self, env: &JNIEnv) -> JResult<jobjectArray>;
}

impl<T: JavaClass> IntoJArray for Vec<T> {
  fn into_jarray(self, env: &JNIEnv) -> JResult<jobjectArray> {
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
          .collect::<JResult<()>>()
          .map(|()| array)
      })
  }
}

pub trait IntoJByteArray {
  fn into_jbyte_array(self, env: &JNIEnv) -> JResult<jbyteArray>;
}

impl IntoJByteArray for Vec<u8> {
  fn into_jbyte_array(self, env: &JNIEnv) -> JResult<jbyteArray> {
    env.byte_array_from_slice(&self).into_result()
  }
}

pub trait IntoVec {
  fn into_vec(self, env: &JNIEnv) -> JResult<Vec<u8>>;
}

impl IntoVec for jbyteArray {
  fn into_vec(self, env: &JNIEnv) -> JResult<Vec<u8>> {
    env.convert_byte_array(self).into_result()
  }
}
