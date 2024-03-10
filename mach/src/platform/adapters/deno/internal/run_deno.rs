


use deno_core::{FastString, PollEventLoopOptions};
use deno_http::DefaultHttpPropertyExtractor;
// use npm::CliNpmResolverByonmCreateOptions;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use deno_core::url::Url;
use deno_runtime::*;

use super::permissions::AppPermissions;

pub struct RunDenoOptions {
  pub cwd: PathBuf,
  pub specifier: String,
  pub eval: String,
}

pub async fn run_deno(options: RunDenoOptions) {
  let main_module = Url::from_file_path(&options.cwd.join(&options.specifier)).unwrap();

  let deno_fs = Arc::new(deno_fs::RealFs);

  // let npm_resolver = npm::create_byonm_npm_resolver(CliNpmResolverByonmCreateOptions{
  //     fs: deno_fs.clone(),
  //     root_node_modules_dir: nm_dir,
  //   })
  //   .into_npm_resolver();

  let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
    module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
    extensions: vec![
      deno_webidl::deno_webidl::init_ops(),
      deno_console::deno_console::init_ops(),
      deno_url::deno_url::init_ops(),
      deno_web::deno_web::init_ops::<AppPermissions>(
          Arc::new(deno_web::BlobStore::default()),
          None,
      ),
      deno_webgpu::deno_webgpu::init_ops(),
      deno_canvas::deno_canvas::init_ops(),
      deno_fetch::deno_fetch::init_ops::<AppPermissions>(deno_fetch::Options {
          user_agent: "Mach/0.0.0".to_string(),
          root_cert_store_provider: None,
          ..Default::default()
      }),
      deno_websocket::deno_websocket::init_ops::<AppPermissions>(
          "Mach/0.0.0".to_string(),
          None,
          None,
      ),
      deno_crypto::deno_crypto::init_ops(None),
      deno_broadcast_channel::deno_broadcast_channel::init_ops(
        deno_broadcast_channel::InMemoryBroadcastChannel::default(),
      ),
      deno_net::deno_net::init_ops::<AppPermissions>(None, None),
      deno_napi::deno_napi::init_ops_and_esm::<AppPermissions>(),
      deno_tls::deno_tls::init_ops(),
      deno_http::deno_http::init_ops::<DefaultHttpPropertyExtractor>(),
      deno_io::deno_io::init_ops(Some(Default::default())),
      deno_fs::deno_fs::init_ops::<AppPermissions>(deno_fs.clone()),
      // deno_node::deno_node::init_ops::<AppPermissions>(Some(npm_resolver.clone()), deno_fs.clone()),
      deno_node::deno_node::init_ops::<AppPermissions>(None, deno_fs.clone()),
    ],
    ..Default::default()
  });

  let code: FastString = options.eval.into();
  let mod_id = js_runtime.load_main_es_module_from_code(&main_module, code)
    .await
    .unwrap();

  let result = js_runtime.mod_evaluate(
    mod_id
  );

  js_runtime
    .run_event_loop(PollEventLoopOptions {
      wait_for_inspector: false,
      pump_v8_message_loop: false,
    })
    .await.unwrap();

  result.await.unwrap();
}