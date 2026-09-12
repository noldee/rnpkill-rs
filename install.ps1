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
# Creamos una carpeta temporal para extraer el contenido estructurado del ZIP
$TmpExtract = Join-Path $env:TEMP "rnpkill_extract"
if (Test-Path $TmpExtract) { Remove-Item $TmpExtract -Recurse -Force }
New-Item -ItemType Directory -Force -Path $TmpExtract | Out-Null

Expand-Archive -Path $TmpZip -DestinationPath $TmpExtract -Force

# Buscamos el .exe de forma recursiva dentro de lo extraído
$FoundExe = Get-ChildItem -Path $TmpExtract -Filter "$BinName.exe" -Recurse | Select-Object -First 1

if ($FoundExe) {
    # Creamos la carpeta bin definitiva y movemos SOLO el archivo .exe ahí
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
    Move-Item -Path $FoundExe.FullName -Destination $InstallDir -Force
} else {
    throw "No se encontró el ejecutable $BinName.exe dentro del archivo ZIP."
}

# Limpieza de archivos temporales
Remove-Item $TmpZip -Force
Remove-Item $TmpExtract -Recurse -Force

$ExePath = Join-Path $InstallDir "$BinName.exe"
Write-Host "rnpkill-rs instalado exitosamente en $ExePath"

# Configuración del PATH
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
$CleanInstallDir = $InstallDir.TrimEnd('\')

# Expresión regular para buscar la ruta exacta en el PATH (evita duplicados parciales)
if ($UserPath -split ';' -notcontains $CleanInstallDir) {
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$CleanInstallDir", "User")
    Write-Host "Se agregó $InstallDir a tu PATH de usuario. Abre una nueva terminal para usarlo."
} else {
    Write-Host "$InstallDir ya estaba en tu PATH."
}
