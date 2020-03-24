use super::error::JavaError;
use super::object::IntoRObject;
use crate::java_class::JavaClass;
use jni::objects::JObject;
use jni::JNIEnv;

pub fn handle_result<F: FnOnce() -> Result<R, JavaError>, R>(func: F) -> R {
  func().unwrap()
}

pub unsafe fn handle_ref<F: FnOnce(&mut O) -> Result<R, JavaError>, O: JavaClass, R>(
  env: &JNIEnv, object: JObject, func: F
) -> R {
  object.into_ref::<O>(env).and_then(|object| func(object)).unwrap()
}

pub unsafe fn handle_owned<F: FnOnce(O) -> Result<R, JavaError>, O: JavaClass, R>(
  env: &JNIEnv, object: JObject, func: F
) -> R {
  object.into_owned::<O>(env).and_then(|object| func(object)).unwrap()
}
