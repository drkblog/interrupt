# package.ps1 - Package Interrupt as a portable Windows ZIP distribution

$ErrorActionPreference = "Stop"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host " Building Interrupt Portable Package " -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

# 1. Read version from Cargo.toml
$cargoToml = Get-Content -Path "$PSScriptRoot\Cargo.toml" -Raw
if ($cargoToml -match 'version\s*=\s*"([^"]+)"') {
    $version = $Matches[1]
} else {
    $version = "1.0.0"
}

Write-Host "[1/4] Target Version: v$version" -ForegroundColor Green

# 2. Build release binary
Write-Host "[2/4] Compiling release binary with Cargo..." -ForegroundColor Green
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Error "Cargo build failed!"
    exit 1
}

# 3. Create staging directory
$distDir = "$PSScriptRoot\dist"
$folderName = "interrupt-v$version-windows-x64"
$stagingDir = "$distDir\$folderName"
$zipPath = "$distDir\$folderName.zip"

if (Test-Path $stagingDir) { Remove-Item -Recurse -Force $stagingDir }
if (Test-Path $zipPath) { Remove-Item -Force $zipPath }

New-Item -ItemType Directory -Path $stagingDir -Force | Out-Null

# 4. Copy distribution files
Write-Host "[3/4] Copying files to staging directory..." -ForegroundColor Green
Copy-Item "$PSScriptRoot\target\release\interrupt.exe" "$stagingDir\interrupt.exe"
Copy-Item "$PSScriptRoot\README.md" "$stagingDir\README.md"
Copy-Item "$PSScriptRoot\CHANGELOG.md" "$stagingDir\CHANGELOG.md"
Copy-Item "$PSScriptRoot\LICENSE" "$stagingDir\LICENSE"

# 5. Compress to ZIP archive
Write-Host "[4/4] Creating ZIP archive ($zipPath)..." -ForegroundColor Green
Compress-Archive -Path "$stagingDir" -DestinationPath "$zipPath" -Force

# Clean up staging directory
Remove-Item -Recurse -Force $stagingDir

$zipItem = Get-Item $zipPath
$sizeMb = [math]::Round($zipItem.Length / 1MB, 2)

Write-Host "========================================" -ForegroundColor Cyan
Write-Host " SUCCESS! Package created successfully:" -ForegroundColor Green
Write-Host " Path: $($zipItem.FullName)" -ForegroundColor Yellow
Write-Host " Size: $sizeMb MB" -ForegroundColor Yellow
Write-Host "========================================" -ForegroundColor Cyan
