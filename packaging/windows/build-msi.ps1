# PowerShell script to build Windows MSI installer
# Usage: .\build-msi.ps1 -Version "0.0.1"

param(
    [Parameter(Mandatory=$false)]
    [string]$Version = "0.0.1"
)

$ErrorActionPreference = "Stop"

Write-Host "Building MSI installer for Requiem $Version..." -ForegroundColor Green

# Check if WiX is installed
$wixInstalled = Get-Command candle.exe -ErrorAction SilentlyContinue
if (-not $wixInstalled) {
    Write-Host "Error: WiX Toolset not found!" -ForegroundColor Red
    Write-Host "Please install WiX from: https://wixtoolset.org/" -ForegroundColor Yellow
    Write-Host "Or run: dotnet tool install --global wix" -ForegroundColor Yellow
    exit 1
}

# Check if binary exists
if (-not (Test-Path "target\release\requiem.exe")) {
    Write-Host "Error: requiem.exe not found in target\release\" -ForegroundColor Red
    Write-Host "Please run 'cargo build --release' first" -ForegroundColor Yellow
    exit 1
}

# Create output directory
New-Item -ItemType Directory -Force -Path "dist" | Out-Null

# Update version in WXS file
$wxsContent = Get-Content "requiem.wxs" -Raw
$wxsContent = $wxsContent -replace 'Version="[^"]*"', "Version=`"$Version`""
$wxsContent | Set-Content "requiem-versioned.wxs"

Write-Host "Compiling WiX source..." -ForegroundColor Cyan

# Compile WiX source
candle.exe requiem-versioned.wxs -out dist\requiem.wixobj
if ($LASTEXITCODE -ne 0) {
    Write-Host "Error: candle.exe failed" -ForegroundColor Red
    exit 1
}

Write-Host "Linking MSI..." -ForegroundColor Cyan

# Link and create MSI
light.exe dist\requiem.wixobj `
    -ext WixUIExtension `
    -cultures:en-us `
    -out "dist\Requiem-$Version-Windows.msi"

if ($LASTEXITCODE -ne 0) {
    Write-Host "Error: light.exe failed" -ForegroundColor Red
    exit 1
}

# Clean up temporary files
Remove-Item "requiem-versioned.wxs" -ErrorAction SilentlyContinue
Remove-Item "dist\requiem.wixobj" -ErrorAction SilentlyContinue
Remove-Item "dist\Requiem-$Version-Windows.wixpdb" -ErrorAction SilentlyContinue

Write-Host ""
Write-Host "✅ MSI created successfully: dist\Requiem-$Version-Windows.msi" -ForegroundColor Green
Write-Host ""
Write-Host "File size:" -ForegroundColor Cyan
$fileSize = (Get-Item "dist\Requiem-$Version-Windows.msi").Length / 1MB
Write-Host ("{0:N2} MB" -f $fileSize) -ForegroundColor White
Write-Host ""
Write-Host "To test the installer, run:" -ForegroundColor Yellow
Write-Host "  msiexec /i dist\Requiem-$Version-Windows.msi" -ForegroundColor White
