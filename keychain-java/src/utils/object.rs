use super::error::JavaError;
use super::ptr::Ptr;
use super::result::IntoResult;
use crate::java_class::JavaClass;
use jni::objects::JObject;
use jni::sys::{jlong, jobject};
use jni::JNIEnv;

pub trait IntoJObject {
  fn into_jobject(self, env: &JNIEnv) -> Result<jobject, JavaError>;
}

impl<T: JavaClass> IntoJObject for T {
  fn into_jobject(self, env: &JNIEnv) -> Result<jobject, JavaError> {
    let ptr = Box::into_raw(Box::new(self)) as jlong;
    env
      .find_class(T::class_name())
      .and_then(|class| env.new_object(class, "(J)V", &[ptr.into()]))
      .map(|object| object.into_inner())
      .into_result()
  }
}

pub trait IntoRObject<'a> {
  unsafe fn into_ref<T: JavaClass>(self, env: &JNIEnv) -> Result<&'a mut T, JavaError>;
  unsafe fn into_owned<T: JavaClass>(self, env: &JNIEnv) -> Result<T, JavaError>;
}

impl<'a> IntoRObject<'a> for JObject<'a> {
  unsafe fn into_ref<T: JavaClass>(self, env: &JNIEnv) -> Result<&'a mut T, JavaError> {
    self.as_ptr(env).map(|ptr| &mut *(ptr as *mut T))
  }

  unsafe fn into_owned<T: JavaClass>(self, env: &JNIEnv) -> Result<T, JavaError> {
    self.as_ptr(env).map(|ptr| *Box::from_raw(ptr as *mut T))
  }
}
