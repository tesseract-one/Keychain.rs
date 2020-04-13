use super::java_class::JavaClass;

pub trait JniType {
  fn jni_type_signature() -> String;
}

impl<T: JavaClass> JniType for T {
  fn jni_type_signature() -> String {
    format!("L{};", T::class_name())
  }
}

impl<T: JniType> JniType for Vec<T> {
  fn jni_type_signature() -> String {
    format!("[{}", T::jni_type_signature())
  }
}

impl JniType for u8 {
  fn jni_type_signature() -> String {
    "B".to_owned()
  }
}

impl JniType for String {
  fn jni_type_signature() -> String {
    String::from("Ljava/lang/String;")
  }
}
