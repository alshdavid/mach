// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use super::collectors::TestCollector;
use super::definitions::TestModule;
use super::execution::TestRun;
use super::lsp_custom;

use crate::deno_cli::lsp::client::Client;
use crate::deno_cli::lsp::client::TestingNotification;
use crate::deno_cli::lsp::config;
use crate::deno_cli::lsp::documents::DocumentsFilter;
use crate::deno_cli::lsp::language_server::StateSnapshot;
use crate::deno_cli::lsp::performance::Performance;

use deno_ast::swc::visit::VisitWith;
use deno_core::error::AnyError;
use deno_core::parking_lot::Mutex;
use deno_core::serde_json::json;
use deno_core::serde_json::Value;
use deno_core::ModuleSpecifier;
use deno_runtime::tokio_util::create_basic_runtime;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;
use std::thread;
use tokio::sync::mpsc;
use tower_lsp::jsonrpc::Error as LspError;
use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::lsp_types as lsp;

fn as_delete_notification(uri: ModuleSpecifier) -> TestingNotification {
  TestingNotification::DeleteModule(
    lsp_custom::TestModuleDeleteNotificationParams {
      text_document: lsp::TextDocumentIdentifier { uri },
    },
  )
}

/// The main structure which handles requests and sends notifications related
/// to the Testing API.
#[derive(Debug)]
pub struct TestServer {
  client: Client,
  performance: Arc<Performance>,
  /// A channel for handling run requests from the client
  run_channel: mpsc::UnboundedSender<u32>,
  /// A map of run ids to test runs
  runs: Arc<Mutex<HashMap<u32, TestRun>>>,
  /// Tests that are discovered from a versioned document
  tests: Arc<Mutex<HashMap<ModuleSpecifier, TestModule>>>,
  /// A channel for requesting that changes to documents be statically analyzed
  /// for tests
  update_channel: mpsc::UnboundedSender<Arc<StateSnapshot>>,
}

impl TestServer {
  pub fn new(
    client: Client,
    performance: Arc<Performance>,
    maybe_root_uri: Option<ModuleSpecifier>,
  ) -> Self {
    let tests: Arc<Mutex<HashMap<ModuleSpecifier, TestModule>>> =
      Arc::new(Mutex::new(HashMap::new()));

    let (update_channel, mut update_rx) =
      mpsc::unbounded_channel::<Arc<StateSnapshot>>();
    let (run_channel, mut run_rx) = mpsc::unbounded_channel::<u32>();

    let server = Self {
      client,
      performance,
      run_channel,
      runs: Default::default(),
      tests,
      update_channel,
    };

    let tests = server.tests.clone();
    let client = server.client.clone();
    let performance = server.performance.clone();
    let mru = maybe_root_uri.clone();
    let _update_join_handle = thread::spawn(move || {
      let runtime = create_basic_runtime();

      runtime.block_on(async {
        loop {
          match update_rx.recv().await {
            None => break,
            Some(snapshot) => {
              let mark = performance.mark("lsp.testing_update");
              let mut tests = tests.lock();
              // we create a list of test modules we currently are tracking
              // eliminating any we go over when iterating over the document
              let mut keys: HashSet<ModuleSpecifier> =
                tests.keys().cloned().collect();
              for document in snapshot
                .documents
                .documents(DocumentsFilter::AllDiagnosable)
              {
                let specifier = document.specifier();
                if !snapshot.config.specifier_enabled_for_test(specifier) {
                  continue;
                }
                keys.remove(specifier);
                let script_version = document.script_version();
                let valid = if let Some(test) = tests.get(specifier) {
                  test.script_version == script_version
                } else {
                  false
                };
                if !valid {
                  if let Some(Ok(parsed_source)) =
                    document.maybe_parsed_source()
                  {
                    let was_empty = tests
                      .remove(specifier)
                      .map(|tm| tm.is_empty())
                      .unwrap_or(true);
                    let mut collector = TestCollector::new(
                      specifier.clone(),
                      script_version,
                      parsed_source.text_info().clone(),
                    );
                    parsed_source.module().visit_with(&mut collector);
                    let test_module = collector.take();
                    if !test_module.is_empty() {
                      client.send_test_notification(
                        test_module.as_replace_notification(mru.as_ref()),
                      );
                    } else if !was_empty {
                      client.send_test_notification(as_delete_notification(
                        specifier.clone(),
                      ));
                    }
                    tests.insert(specifier.clone(), test_module);
                  }
                }
              }
              for key in keys {
                client.send_test_notification(as_delete_notification(key));
              }
              performance.measure(mark);
            }
          }
        }
      })
    });

    let client = server.client.clone();
    let runs = server.runs.clone();
    let _run_join_handle = thread::spawn(move || {
      let runtime = create_basic_runtime();

      runtime.block_on(async {
        loop {
          match run_rx.recv().await {
            None => break,
            Some(id) => {
              let maybe_run = {
                let runs = runs.lock();
                runs.get(&id).cloned()
              };
              if let Some(run) = maybe_run {
                match run.exec(&client, maybe_root_uri.as_ref()).await {
                  Ok(_) => (),
                  Err(err) => {
                    client.show_message(lsp::MessageType::ERROR, err);
                  }
                }
                client.send_test_notification(TestingNotification::Progress(
                  lsp_custom::TestRunProgressParams {
                    id,
                    message: lsp_custom::TestRunProgressMessage::End,
                  },
                ));
                runs.lock().remove(&id);
              }
            }
          }
        }
      })
    });

    server
  }

  fn enqueue_run(&self, id: u32) -> Result<(), AnyError> {
    self.run_channel.send(id).map_err(|err| err.into())
  }

  /// A request from the client to cancel a test run.
  pub fn run_cancel_request(
    &self,
    params: lsp_custom::TestRunCancelParams,
  ) -> LspResult<Option<Value>> {
    if let Some(run) = self.runs.lock().get(&params.id) {
      run.cancel();
      Ok(Some(json!(true)))
    } else {
      Ok(Some(json!(false)))
    }
  }

  /// A request from the client to start a test run.
  pub fn run_request(
    &self,
    params: lsp_custom::TestRunRequestParams,
    workspace_settings: config::WorkspaceSettings,
  ) -> LspResult<Option<Value>> {
    let test_run =
      { TestRun::new(&params, self.tests.clone(), workspace_settings) };
    let enqueued = test_run.as_enqueued();
    {
      let mut runs = self.runs.lock();
      runs.insert(params.id, test_run);
    }
    self.enqueue_run(params.id).map_err(|err| {
      log::error!("cannot enqueue run: {}", err);
      LspError::internal_error()
    })?;
    Ok(Some(json!({ "enqueued": enqueued })))
  }

  pub(crate) fn update(
    &self,
    snapshot: Arc<StateSnapshot>,
  ) -> Result<(), AnyError> {
    self.update_channel.send(snapshot).map_err(|err| err.into())
  }
}
