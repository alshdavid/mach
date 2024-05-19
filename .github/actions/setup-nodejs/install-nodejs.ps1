New-Item -ItemType "directory" -Force -Path "$HOME\.local" | Out-Null
New-Item -ItemType "directory" -Force -Path "$HOME\.local\nodejs" | Out-Null
New-Item -ItemType "directory" -Force -Path "$HOME\.local\nodejs\prefix" | Out-Null
New-Item -ItemType "directory" -Force -Path "$HOME\.local\nodejs\cache" | Out-Null
New-Item -ItemType "directory" -Force -Path "$HOME\.local\nodejs\pnpm-store" | Out-Null

Invoke-WebRequest "https://nodejs.org/download/release/v${NODE_VERSION}/node-v${NODE_VERSION}-win-x64.zip" -OutFile $HOME\.local\nodejs\node.zip

Expand-Archive $HOME\.local\nodejs\node.zip -DestinationPath $HOME\.local\nodejs
Move-Item "$HOME\.local\nodejs\node-v${NODE_VERSION}-win-x64\*" $HOME\.local\nodejs

$env:Path = $HOME + '\.local\nodejs;' + $env:Path
$env:Path = $HOME + '\.local\nodejs\prefix;' + $env:Path
$env:NPM_CONFIG_PREFIX = $HOME + '\.local\nodejs\prefix'

Write-Output "${HOME}\.local\nodejs" >> $env:GITHUB_PATH
Write-Output "${HOME}\.local\nodejs\prefix" >> $env:GITHUB_PATH
Write-Output "NPM_CONFIG_PREFIX=${NPM_CONFIG_PREFIX}" >> $env:GITHUB_ENV

npm install -g pnpm npm

npm -v
node -v
pnpm -v

pnpm config set store-dir $HOME\.local\nodejs\pnpm-store
