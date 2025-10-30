# Test Windows MSI build locally
# Run this script on Windows to test MSI packaging

Write-Host "🔧 Testing Windows MSI Build..." -ForegroundColor Cyan

# Check prerequisites
Write-Host "`n1. Checking prerequisites..." -ForegroundColor Yellow

# Check Rust
if (Get-Command cargo -ErrorAction SilentlyContinue) {
    Write-Host "✅ Rust/Cargo found" -ForegroundColor Green
} else {
    Write-Host "❌ Rust/Cargo not found. Install from https://rustup.rs" -ForegroundColor Red
    exit 1
}

# Check WiX
if (Get-Command wix -ErrorAction SilentlyContinue) {
    Write-Host "✅ WiX Toolset found" -ForegroundColor Green
} else {
    Write-Host "⚠️  WiX Toolset not found. Installing..." -ForegroundColor Yellow
    dotnet tool install --global wix
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Failed to install WiX" -ForegroundColor Red
        exit 1
    }
}

# Check ImageMagick
if (Get-Command magick -ErrorAction SilentlyContinue) {
    Write-Host "✅ ImageMagick found" -ForegroundColor Green
} else {
    Write-Host "⚠️  ImageMagick not found. Install with: choco install imagemagick" -ForegroundColor Yellow
    $response = Read-Host "Continue without icon conversion? (y/n)"
    if ($response -ne 'y') {
        exit 1
    }
}

# Build the project
Write-Host "`n2. Building release binary..." -ForegroundColor Yellow
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Build failed" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Build successful" -ForegroundColor Green

# Convert icon if ImageMagick is available
if (Get-Command magick -ErrorAction SilentlyContinue) {
    Write-Host "`n3. Converting icon..." -ForegroundColor Yellow
    if (Test-Path "assets/icon.png") {
        magick assets/icon.png -define icon:auto-resize=256,128,64,48,32,16 assets/icon.ico
        if ($LASTEXITCODE -eq 0) {
            Write-Host "✅ Icon converted" -ForegroundColor Green
        } else {
            Write-Host "⚠️  Icon conversion failed" -ForegroundColor Yellow
        }
    } else {
        Write-Host "⚠️  assets/icon.png not found" -ForegroundColor Yellow
    }
} else {
    Write-Host "`n3. Skipping icon conversion" -ForegroundColor Yellow
}

# Create versioned WXS file
Write-Host "`n4. Creating versioned WXS file..." -ForegroundColor Yellow
$version = "0.0.1"  # Update this to match your version
$wxsPath = Join-Path $PWD "requiem-versioned.wxs"

# Read and update version
$wxsContent = Get-Content requiem.wxs -Raw -Encoding UTF8
$wxsContent = $wxsContent -replace 'Version="[^"]*"', "Version=`"$version`""

# Write with UTF8 encoding without BOM
$utf8NoBom = New-Object System.Text.UTF8Encoding $false
[System.IO.File]::WriteAllText($wxsPath, $wxsContent, $utf8NoBom)

# Verify file
if (Test-Path $wxsPath) {
    Write-Host "✅ WXS file created successfully" -ForegroundColor Green
    Write-Host "First few lines:"
    Get-Content $wxsPath -Head 5
} else {
    Write-Host "❌ Failed to create WXS file" -ForegroundColor Red
    exit 1
}

# Build MSI
Write-Host "`n5. Building MSI installer..." -ForegroundColor Yellow
New-Item -ItemType Directory -Force -Path "dist" | Out-Null
wix build requiem-versioned.wxs -out "dist\Requiem-$version-Windows.msi"

if ($LASTEXITCODE -eq 0) {
    Write-Host "`n✅ MSI build successful!" -ForegroundColor Green
    Write-Host "MSI location: dist\Requiem-$version-Windows.msi" -ForegroundColor Cyan

    # Show file size
    $msiFile = Get-Item "dist\Requiem-$version-Windows.msi"
    $sizeMB = [math]::Round($msiFile.Length / 1MB, 2)
    Write-Host "File size: $sizeMB MB" -ForegroundColor Cyan
} else {
    Write-Host "`n❌ MSI build failed" -ForegroundColor Red
    exit 1
}

# Clean up
Write-Host "`n6. Cleaning up..." -ForegroundColor Yellow
Remove-Item requiem-versioned.wxs -ErrorAction SilentlyContinue
Write-Host "✅ Done!" -ForegroundColor Green
