use jni::{objects::{JValue, JClass, JString, JObject}, JNIEnv};

pub struct Logger<'a> {
  env: JNIEnv<'a>,
  log_class: JClass<'a>,
  tag: JString<'a>,
}

impl<'a> Logger<'a> {
  pub fn new(env: JNIEnv<'a>, tag: &str) -> Result<Self, jni::errors::Error> {
      Ok(Self {
          env,
          log_class: env.find_class("android/util/Log")?,
          tag: env.new_string(tag)?,
      })
  }

  pub fn d(&self, message: impl AsRef<str>) -> Result<(), jni::errors::Error> {
      self.env.call_static_method(
          self.log_class,
          "d",
          "(Ljava/lang/String;Ljava/lang/String;)I",
          &[
              JValue::Object(JObject::from(self.tag)),
              JValue::Object(JObject::from(self.env.new_string(message)?))
          ]
      )?;
      Ok(())
  }
}