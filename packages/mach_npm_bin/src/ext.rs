use napi::Env;
use napi::JsFunction;
use napi::JsObject;
use napi::JsString;
use napi::NapiRaw;
use napi::NapiValue;

pub fn console_log<V>(
  env: &Env,
  args: &[V],
) -> napi::Result<()>
where
  V: NapiRaw,
{
  let key_console = env.create_string("console")?;
  let key_log = env.create_string("log")?;

  env
    .get_global()?
    .get_property_unchecked::<JsString, JsObject>(key_console)?
    .get_property_unchecked::<JsString, JsFunction>(key_log)?
    .call(None, args)?;

  Ok(())
}

pub trait UtilsExt {
  /// Runs console.log() in the JavaScript context.
  /// useful for debugging [`NapiValue`] types
  #[allow(dead_code)]
  fn console_log<V>(
    &self,
    args: &[V],
  ) -> napi::Result<()>
  where
    V: NapiValue;
}

impl UtilsExt for Env {
  fn console_log<V>(
    &self,
    args: &[V],
  ) -> napi::Result<()>
  where
    V: NapiValue,
  {
    console_log(&self, args)
  }
}
