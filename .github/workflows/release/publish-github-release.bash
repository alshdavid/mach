set -ev

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
ROOT_DIR=$(dirname $(dirname $(dirname $SCRIPT_DIR)))

gh release create $MACH_VERSION --draft --notes "Automatically built binaries"

if [ "$BRANCH_NAME" == "main" ]; then
  gh release edit $MACH_VERSION --title "🚀 Mach - $MACH_VERSION"
else
  gh release edit $MACH_VERSION --prerelease
  gh release edit $MACH_VERSION --title "🧪 Branch: $BRANCH_NAME"
fi

gh release upload $MACH_VERSION "$ROOT_DIR/artifacts/linux-amd64/mach-linux-amd64.tar.gz"
gh release upload $MACH_VERSION "$ROOT_DIR/artifacts/linux-arm64/mach-linux-arm64.tar.gz"
gh release upload $MACH_VERSION "$ROOT_DIR/artifacts/macos-amd64/mach-macos-amd64.tar.gz"
gh release upload $MACH_VERSION "$ROOT_DIR/artifacts/macos-arm64/mach-macos-arm64.tar.gz"
gh release upload $MACH_VERSION "$ROOT_DIR/artifacts/windows-amd64/mach-windows-amd64.tar.gz"
gh release upload $MACH_VERSION "$ROOT_DIR/artifacts/windows-arm64/mach-windows-arm64.tar.gz"

gh release edit $MACH_VERSION --draft=false

