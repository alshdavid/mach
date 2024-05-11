/*
  To interact with Nodejs, Mach spawns a child Nodejs process.
  That process spawns Nodejs worker threads, each connect to the host
  via an IPC channel.

  The IPC channel uses native pipes/sockets, the specific type depends
  on the platform APIs.

  There is very little communication overhead with this approach, though
  types still need to be serialized when they go from Rust to Nodejs via
  napi so JavaScript plugins will never be as fast as native plugins.
*/
mod nodejs_adapter;
mod nodejs_instance;
mod nodejs_manager;
mod nodejs_worker;

pub use nodejs_adapter::*;
pub use nodejs_instance::*;
pub use nodejs_manager::*;
pub use nodejs_worker::*;
