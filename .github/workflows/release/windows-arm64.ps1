echo hi$RootPath = @(Split-Path @(Split-Path @(Split-Path $pwd.Path)))
$Job = "windows-arm64"

"../../../"
& "$PSScriptRoot\..\platform\windows\setup.ps1"

npm install -g npm pnpm
pnpm install
rustup target add aarch64-pc-windows-msvc

just build-publish

cd $RootPath/target/$Job
mv release mach
tar -czvf mach-$Job.tar.gz mach
mv mach-$Job.tar.gz $RootPath/artifacts

cd ${{ github.workspace }}/npm/mach-os-arch
npm pack
mv *.tgz npm-mach-$Job.tgz
mv *.tgz $RootPath/artifacts/npm-mach-$Job.tgz
