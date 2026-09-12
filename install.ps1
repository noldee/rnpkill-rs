$ErrorActionPreference = "Stop"

$Repo = "noldee/rnpkill-rs"
$BinName = "rnpkill-rs"
$InstallDir = "$env:USERPROFILE\.rnpkill-rs\bin"

$Target = "x86_64-pc-windows-msvc"
$Asset = "$BinName-$Target.zip"
$Url = "https://github.com/$Repo/releases/latest/download/$Asset"

Write-Host "Descargando $Asset..."
$TmpZip = Join-Path $env:TEMP $Asset
Invoke-WebRequest -Uri $Url -OutFile $TmpZip

Write-Host "Extrayendo..."
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
Expand-Archive -Path $TmpZip -DestinationPath $InstallDir -Force
Remove-Item $TmpZip

$ExePath = Join-Path $InstallDir "$BinName.exe"
Write-Host "rnpkill-rs instalado en $ExePath"

$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
    Write-Host "Se agregó $InstallDir a tu PATH de usuario. Abre una nueva terminal para usarlo."
} else {
    Write-Host "$InstallDir ya estaba en tu PATH."
}