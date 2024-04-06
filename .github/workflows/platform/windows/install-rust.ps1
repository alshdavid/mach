New-Item -ItemType "directory" -Force -Path "$HOME\.local" | Out-Null
New-Item -ItemType "directory" -Force -Path "$HOME\.local\rust" | Out-Null
New-Item -ItemType "directory" -Force -Path "$HOME\.local\rust\rustup" | Out-Null
New-Item -ItemType "directory" -Force -Path "$HOME\.local\rust\cargo" | Out-Null

$env:RUSTUP_HOME = "${HOME}/.local/rust/rustup"
$env:CARGO_HOME = "${HOME}/.local/rust/cargo"
$env:Path = "$env:USERPROFILE\.cargo\bin;" + $env:Path

Invoke-WebRequest 'https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe' -OutFile $HOME\.local\rust\rustup-init.exe

& "$HOME\.local\rust\rustup-init.exe" --no-modify-path -y
