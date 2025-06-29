// @ts-check

import * as path from "node:path";
import * as fs from "node:fs";
import * as stream from "node:stream";
import * as url from "node:url";
import { execFileSync } from "node:child_process";
import * as tar from "./utils/tar.cjs";
import { Paths } from "./utils/paths.mjs";

const LIBNODE_VERSION = "24.3.0";

const __filename = url.fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const __root = path.dirname(__dirname);

const OS =
  process.env.os ||
  {
    darwin: "macos",
    linux: "linux",
    win32: "windows",
  }[process.platform];

const ARCH =
  process.env.arch ||
  {
    arm64: "arm64",
    x64: "amd64",
  }[process.arch];

const OS_ARCH = `${OS}-${ARCH}`;

const PROFILE = process.env.profile || "debug";

const CARGO_PROFILE = PROFILE !== "debug" ? PROFILE : "dev";
const CARGO_TARGET = {
  "linux-amd64": "x86_64-unknown-linux-musl",
  "linux-arm64": "aarch64-unknown-linux-musl",
  "macos-amd64": "x86_64-apple-darwin",
  "macos-arm64": "aarch64-apple-darwin",
  "windows-amd64": "x86_64-pc-windows-msvc",
  "windows-arm64": "aarch64-pc-windows-msvc",
}[`${OS}-${ARCH}`];

const CARGO_BIN_NAME =
  OS === "windows" ? "mach_bundler_cli.exe" : "mach_bundler_cli";
const BIN_NAME = OS === "windows" ? "mach.exe" : "mach";

if (fs.existsSync(Paths.target.os_arch.profile.url)) {
  rmrf(Paths.target.os_arch.profile.url);
}

// Download libs
if (!fs.existsSync(Paths.target.os_arch.lib.url)) {
  console.log("Downloading libnode");
  create_dir_all(Paths.target.os_arch.lib.url);
  const url = `https://github.com/alshdavid/libnode-prebuilt/releases/download/v${LIBNODE_VERSION}/libnode-${OS}-${ARCH}.tar.gz`;
  const response = await globalThis.fetch(url);
  if (!response.ok) {
    throw new Error("Unable to fetch binary");
  }
  const body = await response.bytes();
  const file = new stream.Duplex();

  file.push(body);
  file.push(null);

  // @ts-expect-error
  let writable = tar.x({
    C: Paths.target.os_arch.lib.url,
    chmod: true,
  });

  file.pipe(writable);
  await new Promise((res) => writable.on("close", res));
}

if (fs.existsSync(Paths.target.os_arch.profile.url)) {
  rmrf(Paths.target.os_arch.profile.url);
}

$("cargo", ["build", "--profile", CARGO_PROFILE, "--target", CARGO_TARGET]);

if (!fs.existsSync(Paths.target.os_arch.profile.url)) {
  create_dir_all(Paths.target.os_arch.profile.url);
}

fs.linkSync(
  Paths.target[".cargo"].target.profile.bin.url,
  Paths.target.os_arch.profile.bin.url,
);
create_dir_all(Paths.target.os_arch.profile.lib.url);

// Link libs
for (const entry of fs.readdirSync(Paths.target.os_arch.lib.url)) {
  fs.linkSync(
    path.join(Paths.target.os_arch.lib.url, entry),
    path.join(Paths.target.os_arch.profile.lib.url, entry),
  );
}

// Utils
function rmrf(target) {
  fs.rmSync(target, { recursive: true, force: true });
}

function create_dir_all(target) {
  fs.mkdirSync(target, { recursive: true });
}

function $(command, args = []) {
  execFileSync(command, args, { shell: true, stdio: "inherit" });
}
