& "$PSScriptRoot\install-nodejs.ps1"
node -v

& "$PSScriptRoot\install-rust.ps1"
cargo --version
