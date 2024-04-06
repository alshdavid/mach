echo hi$RootPath = @(Split-Path @(Split-Path @(Split-Path $pwd.Path)))
$Job = "windows-arm64"

"../../../"
& "$PSScriptRoot\..\platform\windows\setup.ps1"

pnpm install
rustup target add aarch64-pc-windows-msvc

just build-publish

cd "${RootPath}/target/${Job}"
mv release mach
tar -czvf "mach-${Job}.tar.gz" mach
Move-Item "mach-${Job}.tar.gz" -Destination "${RootPath}/artifacts" | Out-Null

cd "${RootPath}/npm/mach-os-arch"
npm pack
Move-Item *.tgz -Destination "npm-mach-${Job}.tgz"
Move-Item *.tgz -Destination "${RootPath}/artifacts/npm-mach-${Job}.tgz"
