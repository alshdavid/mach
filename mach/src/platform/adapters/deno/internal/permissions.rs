use std::path::Path;

use deno_core::{error::AnyError, url::Url};
use deno_fs::OpenOptions;
use deno_node::NodePermissions;
pub struct AppPermissions {}

impl NodePermissions for AppPermissions {
  fn check_net_url(
    &mut self,
    url: &deno_core::url::Url,
    api_name: &str,
  ) -> Result<(), AnyError> {
    Ok(())
  }

  fn check_read_with_api_name(
    &self,
    path: &std::path::Path,
    api_name: Option<&str>,
  ) -> Result<(), AnyError> {
    Ok(())
  }

  fn check_sys(
    &self,
    kind: &str,
    api_name: &str,
  ) -> Result<(), AnyError> {
    Ok(())
  }

  fn check_write_with_api_name(
    &self,
    path: &std::path::Path,
    api_name: Option<&str>,
  ) -> Result<(), AnyError> {
    Ok(())
  }
}

impl deno_web::TimersPermission for AppPermissions {
  fn allow_hrtime(&mut self) -> bool {
    true
  }
}

impl deno_fetch::FetchPermissions for AppPermissions {
  fn check_net_url(
    &mut self,
    _url: &Url,
    _api_name: &str,
  ) -> Result<(), AnyError> {
    Ok(())
  }

  fn check_read(
    &mut self,
    _p: &Path,
    _api_name: &str,
  ) -> Result<(), AnyError> {
    Ok(())
  }
}

impl deno_net::NetPermissions for AppPermissions {
  fn check_net<T: AsRef<str>>(
    &mut self,
    _host: &(T, Option<u16>),
    _api_name: &str,
  ) -> Result<(), AnyError> {
    Ok(())
  }

  fn check_read(
    &mut self,
    _path: &Path,
    _api_name: &str,
  ) -> Result<(), AnyError> {
    Ok(())
  }

  fn check_write(
    &mut self,
    _path: &Path,
    _api_name: &str,
  ) -> Result<(), AnyError> {
    Ok(())
  }
}

impl deno_websocket::WebSocketPermissions for AppPermissions {
  fn check_net_url(
    &mut self,
    _url: &Url,
    _api_name: &str,
  ) -> Result<(), AnyError> {
    Ok(())
  }
}

impl deno_fs::FsPermissions for AppPermissions {
  fn check_read(
    &mut self,
    _path: &Path,
    _api_name: &str,
  ) -> Result<(), AnyError> {
    Ok(())
  }

  fn check_read_all(
    &mut self,
    _api_name: &str,
  ) -> Result<(), AnyError> {
    Ok(())
  }

  fn check_read_blind(
    &mut self,
    _path: &Path,
    _display: &str,
    _api_name: &str,
  ) -> Result<(), AnyError> {
    Ok(())
  }

  fn check_write(
    &mut self,
    _path: &Path,
    _api_name: &str,
  ) -> Result<(), AnyError> {
    Ok(())
  }

  fn check_write_partial(
    &mut self,
    _path: &Path,
    _api_name: &str,
  ) -> Result<(), AnyError> {
    Ok(())
  }

  fn check_write_all(
    &mut self,
    _api_name: &str,
  ) -> Result<(), AnyError> {
    Ok(())
  }

  fn check_write_blind(
    &mut self,
    _p: &Path,
    _display: &str,
    _api_name: &str,
  ) -> Result<(), AnyError> {
    Ok(())
  }

  fn check(
    &mut self,
    _open_options: &OpenOptions,
    _path: &Path,
    _api_name: &str,
  ) -> Result<(), AnyError> {
    Ok(())
  }
}

impl deno_napi::NapiPermissions for AppPermissions {
    fn check(&mut self, path: Option<&Path>)
        -> std::result::Result<(), AnyError> {
        Ok(())
    }
}
