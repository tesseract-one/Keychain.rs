use super::object::IntoRObject;
use super::result::JResult;
use crate::java_class::JavaClass;
use jni::objects::JObject;
use jni::JNIEnv;

pub fn handle_result<F: FnOnce() -> JResult<R>, R>(func: F) -> R {
  func().unwrap()
}

pub unsafe fn handle_ref<F: FnOnce(&mut O) -> JResult<R>, O: JavaClass, R>(
  env: &JNIEnv, object: JObject, func: F
) -> R {
  object.into_ref::<O>(env).and_then(|object| func(object)).unwrap()
}

pub unsafe fn handle_owned<F: FnOnce(O) -> JResult<R>, O: JavaClass, R>(
  env: &JNIEnv, object: JObject, func: F
) -> R {
  object.into_owned::<O>(env).and_then(|object| func(object)).unwrap()
}
