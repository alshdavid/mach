use std::fmt::Debug;
use std::sync::mpsc::Sender;

use napi::*;
use serde::de::DeserializeOwned;

pub fn await_promise<T>(
  env: Env,
  result: JsUnknown,
  tx: Sender<T>,
) -> napi::Result<()>
where
  T: DeserializeOwned + Send + Debug + 'static,
{
  if !result.is_promise()? {
    let res = env.from_js_value(result)?;
    tx.send(res).expect("send failure");
    return Ok(());
  }

  let result: JsObject = result.try_into()?;
  let then: JsFunction = result.get_named_property("then")?;

  let cb = env.create_function_from_closure("callback", move |ctx| {
    let v = ctx.get::<JsUnknown>(0)?;
    let res = ctx.env.from_js_value(v).expect("msg");
    tx.send(res).expect("send failure");
    ctx.env.get_undefined()
  })?;

  let eb = env.create_function_from_closure("error_callback", move |ctx| {
    let err = ctx.env.from_js_value(ctx.get::<JsUnknown>(0)?)?;
    println!("Failed {:?}", err);
    ctx.env.get_undefined()
  })?;

  then.call(Some(&result), &[cb, eb])?;
  Ok(())
}
