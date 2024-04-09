// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

//! This module provides feature to upgrade deno executable

use crate::deno_cli::args::Flags;
use crate::deno_cli::args::UpgradeFlags;
use crate::deno_cli::colors;
use crate::deno_cli::factory::CliFactory;
use crate::deno_cli::http_util::HttpClient;
use crate::deno_cli::standalone::binary::unpack_into_dir;
use crate::deno_cli::util::progress_bar::ProgressBar;
use crate::deno_cli::util::progress_bar::ProgressBarStyle;
use crate::deno_cli::util::time;
use crate::deno_cli::version;

use async_trait::async_trait;
use deno_core::anyhow::bail;
use deno_core::anyhow::Context;
use deno_core::error::AnyError;
use deno_core::unsync::spawn;
use deno_semver::Version;
use once_cell::sync::Lazy;
use std::borrow::Cow;
use std::env;
use std::fs;
use std::io::IsTerminal;
use std::ops::Sub;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use std::time::Duration;

const RELEASE_URL: &str = "https://github.com/denoland/deno/releases";

pub static ARCHIVE_NAME: Lazy<String> = Lazy::new(|| format!("deno-{}.zip", "TARGET"));

// How often query server for new version. In hours.
const UPGRADE_CHECK_INTERVAL: i64 = 24;

const UPGRADE_CHECK_FETCH_DELAY: Duration = Duration::from_millis(500);

/// Environment necessary for doing the update checker.
/// An alternate trait implementation can be provided for testing purposes.
trait UpdateCheckerEnvironment: Clone {
  fn read_check_file(&self) -> String;
  fn write_check_file(
    &self,
    text: &str,
  );
  fn current_time(&self) -> chrono::DateTime<chrono::Utc>;
}

#[derive(Clone)]
struct RealUpdateCheckerEnvironment {
  cache_file_path: PathBuf,
  current_time: chrono::DateTime<chrono::Utc>,
}

impl RealUpdateCheckerEnvironment {
  pub fn new(cache_file_path: PathBuf) -> Self {
    Self {
      cache_file_path,
      // cache the current time
      current_time: time::utc_now(),
    }
  }
}

impl UpdateCheckerEnvironment for RealUpdateCheckerEnvironment {
  fn read_check_file(&self) -> String {
    std::fs::read_to_string(&self.cache_file_path).unwrap_or_default()
  }

  fn write_check_file(
    &self,
    text: &str,
  ) {
    let _ = std::fs::write(&self.cache_file_path, text);
  }

  fn current_time(&self) -> chrono::DateTime<chrono::Utc> {
    self.current_time
  }
}

#[derive(Debug, Copy, Clone)]
enum UpgradeCheckKind {
  Execution,
  Lsp,
}

#[async_trait(?Send)]
trait VersionProvider: Clone {
  fn is_canary(&self) -> bool;
  async fn latest_version(&self) -> Result<String, AnyError>;
  fn current_version(&self) -> Cow<str>;

  fn release_kind(&self) -> UpgradeReleaseKind {
    if self.is_canary() {
      UpgradeReleaseKind::Canary
    } else {
      UpgradeReleaseKind::Stable
    }
  }
}

#[derive(Clone)]
struct RealVersionProvider {
  http_client: Arc<HttpClient>,
  check_kind: UpgradeCheckKind,
}

impl RealVersionProvider {
  pub fn new(
    http_client: Arc<HttpClient>,
    check_kind: UpgradeCheckKind,
  ) -> Self {
    Self {
      http_client,
      check_kind,
    }
  }
}

#[async_trait(?Send)]
impl VersionProvider for RealVersionProvider {
  fn is_canary(&self) -> bool {
    version::is_canary()
  }

  async fn latest_version(&self) -> Result<String, AnyError> {
    get_latest_version(&self.http_client, self.release_kind(), self.check_kind).await
  }

  fn current_version(&self) -> Cow<str> {
    Cow::Borrowed(version::release_version_or_canary_commit_hash())
  }
}

struct UpdateChecker<TEnvironment: UpdateCheckerEnvironment, TVersionProvider: VersionProvider> {
  env: TEnvironment,
  version_provider: TVersionProvider,
  maybe_file: Option<CheckVersionFile>,
}

impl<TEnvironment: UpdateCheckerEnvironment, TVersionProvider: VersionProvider>
  UpdateChecker<TEnvironment, TVersionProvider>
{
  pub fn new(
    env: TEnvironment,
    version_provider: TVersionProvider,
  ) -> Self {
    let maybe_file = CheckVersionFile::parse(env.read_check_file());
    Self {
      env,
      version_provider,
      maybe_file,
    }
  }

  pub fn should_check_for_new_version(&self) -> bool {
    match &self.maybe_file {
      Some(file) => {
        let last_check_age = self
          .env
          .current_time()
          .signed_duration_since(file.last_checked);
        last_check_age > chrono::Duration::hours(UPGRADE_CHECK_INTERVAL)
      }
      None => true,
    }
  }

  /// Returns the version if a new one is available and it should be prompted about.
  pub fn should_prompt(&self) -> Option<String> {
    let file = self.maybe_file.as_ref()?;
    // If the current version saved is not the actually current version of the binary
    // It means
    // - We already check for a new version today
    // - The user have probably upgraded today
    // So we should not prompt and wait for tomorrow for the latest version to be updated again
    let current_version = self.version_provider.current_version();
    if file.current_version != current_version {
      return None;
    }
    if file.latest_version == current_version {
      return None;
    }

    if let Ok(current) = Version::parse_standard(&current_version) {
      if let Ok(latest) = Version::parse_standard(&file.latest_version) {
        if current >= latest {
          return None;
        }
      }
    }

    let last_prompt_age = self
      .env
      .current_time()
      .signed_duration_since(file.last_prompt);
    if last_prompt_age > chrono::Duration::hours(UPGRADE_CHECK_INTERVAL) {
      Some(file.latest_version.clone())
    } else {
      None
    }
  }

  /// Store that we showed the update message to the user.
  pub fn store_prompted(self) {
    if let Some(file) = self.maybe_file {
      self
        .env
        .write_check_file(&file.with_last_prompt(self.env.current_time()).serialize());
    }
  }
}

fn get_minor_version(version: &str) -> &str {
  version.rsplitn(2, '.').collect::<Vec<&str>>()[1]
}

fn print_release_notes(
  current_version: &str,
  new_version: &str,
) {
  if get_minor_version(current_version) != get_minor_version(new_version) {
    log::info!(
      "{}{}",
      "Release notes: https://github.com/denoland/deno/releases/tag/v",
      &new_version,
    );
    log::info!(
      "{}{}",
      "Blog post: https://deno.com/blog/v",
      get_minor_version(new_version)
    );
  }
}

pub fn upgrade_check_enabled() -> bool {
  matches!(
    env::var("DENO_NO_UPDATE_CHECK"),
    Err(env::VarError::NotPresent)
  )
}

pub fn check_for_upgrades(
  http_client: Arc<HttpClient>,
  cache_file_path: PathBuf,
) {
  if !upgrade_check_enabled() {
    return;
  }

  let env = RealUpdateCheckerEnvironment::new(cache_file_path);
  let version_provider = RealVersionProvider::new(http_client, UpgradeCheckKind::Execution);
  let update_checker = UpdateChecker::new(env, version_provider);

  if update_checker.should_check_for_new_version() {
    let env = update_checker.env.clone();
    let version_provider = update_checker.version_provider.clone();
    // do this asynchronously on a separate task
    spawn(async move {
      // Sleep for a small amount of time to not unnecessarily impact startup
      // time.
      tokio::time::sleep(UPGRADE_CHECK_FETCH_DELAY).await;

      fetch_and_store_latest_version(&env, &version_provider).await;

      // text is used by the test suite
      log::debug!("Finished upgrade checker.")
    });
  }

  // Print a message if an update is available
  if let Some(upgrade_version) = update_checker.should_prompt() {
    if log::log_enabled!(log::Level::Info) && std::io::stderr().is_terminal() {
      if version::is_canary() {
        eprint!(
          "{} ",
          colors::green("A new canary release of Deno is available.")
        );
        eprintln!(
          "{}",
          colors::italic_gray("Run `deno upgrade --canary` to install it.")
        );
      } else {
        eprint!(
          "{} {} → {} ",
          colors::green("A new release of Deno is available:"),
          colors::cyan(version::deno()),
          colors::cyan(&upgrade_version)
        );
        eprintln!(
          "{}",
          colors::italic_gray("Run `deno upgrade` to install it.")
        );
      }

      update_checker.store_prompted();
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LspVersionUpgradeInfo {
  pub latest_version: String,
  pub is_canary: bool,
}

pub async fn check_for_upgrades_for_lsp(
  http_client: Arc<HttpClient>
) -> Result<Option<LspVersionUpgradeInfo>, AnyError> {
  if !upgrade_check_enabled() {
    return Ok(None);
  }

  let version_provider = RealVersionProvider::new(http_client, UpgradeCheckKind::Lsp);
  check_for_upgrades_for_lsp_with_provider(&version_provider).await
}

async fn check_for_upgrades_for_lsp_with_provider(
  version_provider: &impl VersionProvider
) -> Result<Option<LspVersionUpgradeInfo>, AnyError> {
  let latest_version = version_provider.latest_version().await?;
  let current_version = version_provider.current_version();
  if current_version == latest_version {
    Ok(None) // nothing to upgrade
  } else if version_provider.is_canary() {
    Ok(Some(LspVersionUpgradeInfo {
      latest_version,
      is_canary: true,
    }))
  } else {
    if let Ok(current) = Version::parse_standard(&current_version) {
      if let Ok(latest) = Version::parse_standard(&latest_version) {
        if current >= latest {
          return Ok(None); // nothing to upgrade
        }
      }
    }
    Ok(Some(LspVersionUpgradeInfo {
      latest_version,
      is_canary: false,
    }))
  }
}

async fn fetch_and_store_latest_version<
  TEnvironment: UpdateCheckerEnvironment,
  TVersionProvider: VersionProvider,
>(
  env: &TEnvironment,
  version_provider: &TVersionProvider,
) {
  // Fetch latest version or commit hash from server.
  let latest_version = match version_provider.latest_version().await {
    Ok(latest_version) => latest_version,
    Err(_) => return,
  };

  env.write_check_file(
    &CheckVersionFile {
      // put a date in the past here so that prompt can be shown on next run
      last_prompt: env
        .current_time()
        .sub(chrono::Duration::hours(UPGRADE_CHECK_INTERVAL + 1)),
      last_checked: env.current_time(),
      current_version: version_provider.current_version().to_string(),
      latest_version,
    }
    .serialize(),
  );
}

pub async fn upgrade(
  flags: Flags,
  upgrade_flags: UpgradeFlags,
) -> Result<(), AnyError> {
  let factory = CliFactory::from_flags(flags).await?;
  let client = factory.http_client();
  let current_exe_path = std::env::current_exe()?;
  let output_exe_path = upgrade_flags.output.as_ref().unwrap_or(&current_exe_path);

  let permissions = if let Ok(metadata) = fs::metadata(output_exe_path) {
    let permissions = metadata.permissions();
    if permissions.readonly() {
      bail!(
        "You do not have write permission to {}",
        output_exe_path.display()
      );
    }
    #[cfg(unix)]
    if std::os::unix::fs::MetadataExt::uid(&metadata) == 0
      && !nix::unistd::Uid::effective().is_root()
    {
      bail!(
        concat!(
          "You don't have write permission to {} because it's owned by root.\n",
          "Consider updating deno through your package manager if its installed from it.\n",
          "Otherwise run `deno upgrade` as root.",
        ),
        output_exe_path.display()
      );
    }
    permissions
  } else {
    fs::metadata(&current_exe_path)?.permissions()
  };

  let install_version = match upgrade_flags.version {
    Some(passed_version) => {
      let re_hash = lazy_regex::regex!("^[0-9a-f]{40}$");
      let passed_version = passed_version
        .strip_prefix('v')
        .unwrap_or(&passed_version)
        .to_string();

      if upgrade_flags.canary && !re_hash.is_match(&passed_version) {
        bail!("Invalid commit hash passed");
      } else if !upgrade_flags.canary && Version::parse_standard(&passed_version).is_err() {
        bail!("Invalid version passed");
      }

      let current_is_passed = if upgrade_flags.canary {
        crate::deno_cli::version::GIT_COMMIT_HASH == passed_version
      } else if !crate::deno_cli::version::is_canary() {
        crate::deno_cli::version::deno() == passed_version
      } else {
        false
      };

      if !upgrade_flags.force && upgrade_flags.output.is_none() && current_is_passed {
        log::info!(
          "Version {} is already installed",
          crate::deno_cli::version::deno()
        );
        return Ok(());
      }

      passed_version
    }
    None => {
      let release_kind = if upgrade_flags.canary {
        log::info!("Looking up latest canary version");
        UpgradeReleaseKind::Canary
      } else {
        log::info!("Looking up latest version");
        UpgradeReleaseKind::Stable
      };

      let latest_version =
        get_latest_version(client, release_kind, UpgradeCheckKind::Execution).await?;

      let current_is_most_recent = if upgrade_flags.canary {
        let latest_hash = &latest_version;
        crate::deno_cli::version::GIT_COMMIT_HASH == latest_hash
      } else if !crate::deno_cli::version::is_canary() {
        let current = Version::parse_standard(crate::deno_cli::version::deno()).unwrap();
        let latest = Version::parse_standard(&latest_version).unwrap();
        current >= latest
      } else {
        false
      };

      if !upgrade_flags.force && upgrade_flags.output.is_none() && current_is_most_recent {
        log::info!(
          "Local deno version {} is the most recent release",
          if upgrade_flags.canary {
            crate::deno_cli::version::GIT_COMMIT_HASH
          } else {
            crate::deno_cli::version::deno()
          }
        );
        return Ok(());
      } else {
        log::info!("Found latest version {}", latest_version);
        latest_version
      }
    }
  };

  let download_url = if upgrade_flags.canary {
    format!(
      "https://dl.deno.land/canary/{}/{}",
      install_version, *ARCHIVE_NAME
    )
  } else {
    format!(
      "{}/download/v{}/{}",
      RELEASE_URL, install_version, *ARCHIVE_NAME
    )
  };

  let archive_data = download_package(client, &download_url)
    .await
    .with_context(|| format!("Failed downloading {download_url}. The version you requested may not have been built for the current architecture."))?;

  log::info!("Deno is upgrading to version {}", &install_version);

  let temp_dir = tempfile::TempDir::new()?;
  let new_exe_path = unpack_into_dir(
    "deno",
    &ARCHIVE_NAME,
    archive_data,
    cfg!(windows),
    &temp_dir,
  )?;
  fs::set_permissions(&new_exe_path, permissions)?;
  check_exe(&new_exe_path)?;

  if upgrade_flags.dry_run {
    fs::remove_file(&new_exe_path)?;
    log::info!("Upgraded successfully (dry run)");
    if !upgrade_flags.canary {
      print_release_notes(version::deno(), &install_version);
    }
  } else {
    let output_exe_path = upgrade_flags.output.as_ref().unwrap_or(&current_exe_path);
    let output_result = if *output_exe_path == current_exe_path {
      replace_exe(&new_exe_path, output_exe_path)
    } else {
      fs::rename(&new_exe_path, output_exe_path)
        .or_else(|_| fs::copy(&new_exe_path, output_exe_path).map(|_| ()))
    };
    if let Err(err) = output_result {
      const WIN_ERROR_ACCESS_DENIED: i32 = 5;
      if cfg!(windows) && err.raw_os_error() == Some(WIN_ERROR_ACCESS_DENIED) {
        return Err(err).with_context(|| {
          format!(
            concat!(
              "Could not replace the deno executable. This may be because an ",
              "existing deno process is running. Please ensure there are no ",
              "running deno processes (ex. Stop-Process -Name deno ; deno {}), ",
              "close any editors before upgrading, and ensure you have ",
              "sufficient permission to '{}'."
            ),
            // skip the first argument, which is the executable path
            std::env::args().skip(1).collect::<Vec<_>>().join(" "),
            output_exe_path.display(),
          )
        });
      } else {
        return Err(err.into());
      }
    }
    log::info!("Upgraded successfully");
    if !upgrade_flags.canary {
      print_release_notes(version::deno(), &install_version);
    }
  }

  drop(temp_dir); // delete the temp dir
  Ok(())
}

#[derive(Debug, Clone, Copy)]
enum UpgradeReleaseKind {
  Stable,
  Canary,
}

async fn get_latest_version(
  client: &HttpClient,
  release_kind: UpgradeReleaseKind,
  check_kind: UpgradeCheckKind,
) -> Result<String, AnyError> {
  let url = get_url(release_kind, "TARGET", check_kind);
  let text = client.download_text(url).await?;
  Ok(normalize_version_from_server(release_kind, &text))
}

fn normalize_version_from_server(
  release_kind: UpgradeReleaseKind,
  text: &str,
) -> String {
  let text = text.trim();
  match release_kind {
    UpgradeReleaseKind::Stable => text.trim_start_matches('v').to_string(),
    UpgradeReleaseKind::Canary => text.to_string(),
  }
}

fn get_url(
  release_kind: UpgradeReleaseKind,
  target_tuple: &str,
  check_kind: UpgradeCheckKind,
) -> String {
  let file_name = match release_kind {
    UpgradeReleaseKind::Stable => Cow::Borrowed("release-latest.txt"),
    UpgradeReleaseKind::Canary => Cow::Owned(format!("canary-{target_tuple}-latest.txt")),
  };
  let query_param = match check_kind {
    UpgradeCheckKind::Execution => "",
    UpgradeCheckKind::Lsp => "?lsp",
  };
  format!("{}/{}{}", base_upgrade_url(), file_name, query_param)
}

fn base_upgrade_url() -> Cow<'static, str> {
  // this is used by the test suite
  if let Ok(url) = env::var("DENO_DONT_USE_INTERNAL_BASE_UPGRADE_URL") {
    Cow::Owned(url)
  } else {
    Cow::Borrowed("https://dl.deno.land")
  }
}

async fn download_package(
  client: &HttpClient,
  download_url: &str,
) -> Result<Vec<u8>, AnyError> {
  log::info!("Downloading {}", &download_url);
  let maybe_bytes = {
    let progress_bar = ProgressBar::new(ProgressBarStyle::DownloadBars);
    // provide an empty string here in order to prefer the downloading
    // text above which will stay alive after the progress bars are complete
    let progress = progress_bar.update("");
    client
      .download_with_progress(download_url, &progress)
      .await?
  };
  match maybe_bytes {
    Some(bytes) => Ok(bytes),
    None => {
      log::error!("Download could not be found, aborting");
      std::process::exit(1)
    }
  }
}

fn replace_exe(
  from: &Path,
  to: &Path,
) -> Result<(), std::io::Error> {
  if cfg!(windows) {
    // On windows you cannot replace the currently running executable.
    // so first we rename it to deno.old.exe
    fs::rename(to, to.with_extension("old.exe"))?;
  } else {
    fs::remove_file(to)?;
  }
  // Windows cannot rename files across device boundaries, so if rename fails,
  // we try again with copy.
  fs::rename(from, to).or_else(|_| fs::copy(from, to).map(|_| ()))?;
  Ok(())
}

fn check_exe(exe_path: &Path) -> Result<(), AnyError> {
  let output = Command::new(exe_path)
    .arg("-V")
    .stderr(std::process::Stdio::inherit())
    .output()?;
  assert!(output.status.success());
  Ok(())
}

#[derive(Debug)]
struct CheckVersionFile {
  pub last_prompt: chrono::DateTime<chrono::Utc>,
  pub last_checked: chrono::DateTime<chrono::Utc>,
  pub current_version: String,
  pub latest_version: String,
}

impl CheckVersionFile {
  pub fn parse(content: String) -> Option<Self> {
    let split_content = content.split('!').collect::<Vec<_>>();

    if split_content.len() != 4 {
      return None;
    }

    let latest_version = split_content[2].trim().to_owned();
    if latest_version.is_empty() {
      return None;
    }
    let current_version = split_content[3].trim().to_owned();
    if current_version.is_empty() {
      return None;
    }

    let last_prompt = chrono::DateTime::parse_from_rfc3339(split_content[0])
      .map(|dt| dt.with_timezone(&chrono::Utc))
      .ok()?;
    let last_checked = chrono::DateTime::parse_from_rfc3339(split_content[1])
      .map(|dt| dt.with_timezone(&chrono::Utc))
      .ok()?;

    Some(CheckVersionFile {
      last_prompt,
      last_checked,
      current_version,
      latest_version,
    })
  }

  fn serialize(&self) -> String {
    format!(
      "{}!{}!{}!{}",
      self.last_prompt.to_rfc3339(),
      self.last_checked.to_rfc3339(),
      self.latest_version,
      self.current_version,
    )
  }

  fn with_last_prompt(
    self,
    dt: chrono::DateTime<chrono::Utc>,
  ) -> Self {
    Self {
      last_prompt: dt,
      ..self
    }
  }
}
