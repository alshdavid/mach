New-Item -ItemType "directory" -Force -Path "$HOME\.local" | Out-Null
New-Item -ItemType "directory" -Force -Path "$HOME\.local\just" | Out-Null

Invoke-WebRequest 'https://github.com/casey/just/releases/download/1.25.2/just-1.25.2-x86_64-pc-windows-msvc.zip' -OutFile $HOME\.local\just\just.zip
Expand-Archive $HOME\.local\just\just.zip -DestinationPath $HOME\.local\just

$env:Path = $HOME + '\.local\just;' + $env:Path
Write-Output "${HOME}\.local\just" >> $env:GITHUB_PATH
