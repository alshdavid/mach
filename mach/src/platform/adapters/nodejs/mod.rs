/*
  To interact with Nodejs, Mach spawns a child Nodejs process.

  The child process spawns Nodejs worker threads, each worker imports a
  napi module that contains the client implementation to connect to 
  their corresponding host IPC channel.

  The messages the workers receive via IPC are then passed into JavaScript
  using the built-in serialization system offered by napi.

  The IPC channels uses native pipes/sockets, the specific type depends
  on the platform APIs.

  There is very little communication overhead with this approach, though
  types still need to be serialized when they go from Rust to Nodejs via
  napi so JavaScript plugins will never be as fast as native plugins.
*/
mod nodejs_adapter;
mod nodejs_instance;
mod nodejs_worker;

pub use nodejs_adapter::*;
pub use nodejs_instance::*;
pub use nodejs_worker::*;
