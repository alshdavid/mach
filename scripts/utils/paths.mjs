import * as path from "node:path";
import * as url from "node:url";
import {
  CARGO_TARGET,
  PROFILE,
  CARGO_BIN_NAME,
  OS_ARCH,
  BIN_NAME,
} from "./env.mjs";

const __filename = url.fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const __root = path.dirname(path.dirname(__dirname));

export const Paths = {
  root: __root,
  target: {
    url: path.join(__root, "target"),
    [".cargo"]: {
      url: path.join(__root, "target", ".cargo"),
      target: {
        profile: {
          bin: {
            url: path.join(
              __root,
              "target",
              ".cargo",
              CARGO_TARGET,
              PROFILE,
              CARGO_BIN_NAME,
            ),
          },
        },
      },
    },
    os_arch: {
      url: path.join(__root, "target", OS_ARCH),
      profile: {
        url: path.join(__root, "target", OS_ARCH, PROFILE),
        lib: {
          url: path.join(__root, "target", OS_ARCH, PROFILE, "lib"),
        },
        bin: {
          url: path.join(__root, "target", OS_ARCH, PROFILE, BIN_NAME),
        },
      },
      lib: {
        url: path.join(__root, "target", OS_ARCH, "lib"),
      },
    },
  },
};
