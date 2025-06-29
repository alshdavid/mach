// @ts-check

import * as path from "node:path";
import * as fs from "node:fs";
import * as stream from "node:stream";
import { execFileSync } from "node:child_process";
import * as tar from "./utils/tar.cjs";
import { Paths } from "./utils/paths.mjs";
import { CARGO_PROFILE, CARGO_TARGET, OS, ARCH } from "./utils/env.mjs";

const LIBNODE_VERSION = "24.3.0";

async function main() {
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
}

main()
