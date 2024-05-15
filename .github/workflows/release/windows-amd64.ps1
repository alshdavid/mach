$ErrorActionPreference = "Stop"
$RootPath = @(Split-Path @(Split-Path @(Split-Path $PSScriptRoot)))
$Job = "windows-amd64"

& "$PSScriptRoot\..\platform\windows\setup.ps1"

$env:MACH_SKIP_POST_INSTALL = 'true'

New-Item -ItemType "directory" -Force -Path "$RootPath\artifacts" | Out-Null

rustup target add x86_64-pc-windows-msvc

just build-publish

cd "${RootPath}/target/${Job}"
mv release mach
tar -czvf "mach-${Job}.tar.gz" mach
Move-Item "mach-${Job}.tar.gz" -Destination "${RootPath}/artifacts" | Out-Null

cd "${RootPath}/npm/mach-os-arch"
npm pack
Move-Item *.tgz -Destination "npm-mach-${Job}.tgz"
Move-Item *.tgz -Destination "${RootPath}/artifacts/npm-mach-${Job}.tgz"
