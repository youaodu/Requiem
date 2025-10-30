#!/bin/bash
# Test Arch Linux package build locally using Docker

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "🏗️  Testing Arch Linux package build..."
echo "📁 Project root: $PROJECT_ROOT"
echo ""

# Check if Docker is available
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed. Please install Docker first."
    exit 1
fi

# Clean previous build artifacts
echo "🧹 Cleaning previous build artifacts..."
rm -f "$PROJECT_ROOT"/*.pkg.tar.zst
rm -rf "$PROJECT_ROOT/pkg"
rm -rf "$PROJECT_ROOT/src"

# Run build in Arch Linux container
echo "🐋 Starting Arch Linux container..."
docker run --rm -it \
    -v "$PROJECT_ROOT:/build:rw" \
    -w /build \
    archlinux:latest \
    bash -c '
        set -e

        echo "📦 Updating system..."
        pacman -Syu --noconfirm

        echo "📦 Installing dependencies..."
        pacman -S --noconfirm base-devel rust cargo git openssl fontconfig adobe-source-han-sans-otf-fonts namcap

        echo "👤 Creating build user..."
        useradd -m builder
        chown -R builder:builder /build

        echo "🔍 Validating PKGBUILD..."
        su builder -c "namcap PKGBUILD" || true

        echo "🔨 Building package..."
        su builder -c "makepkg --noconfirm -sf"

        echo "✅ Checking package quality..."
        PKGFILE=$(ls requiem-*.pkg.tar.zst)
        namcap "$PKGFILE" || true

        echo ""
        echo "✨ Package built successfully: $PKGFILE"

        echo ""
        echo "📊 Package info:"
        pacman -Qip "$PKGFILE"

        echo ""
        echo "📁 Package contents:"
        pacman -Qlp "$PKGFILE"

        echo ""
        echo "🔗 Package dependencies:"
        pacman -Qip "$PKGFILE" | grep "Depends On"
    '

echo ""
echo "✅ Build test completed!"
echo ""
echo "📦 Built package:"
ls -lh "$PROJECT_ROOT"/*.pkg.tar.zst 2>/dev/null || echo "No package found"
echo ""
echo "💡 To install locally: sudo pacman -U requiem-*.pkg.tar.zst"
