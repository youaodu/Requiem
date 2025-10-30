#!/bin/bash
# Script to convert icon.png to platform-specific formats
# Requires: ImageMagick (for .ico), iconutil (macOS, for .icns)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
ASSETS_DIR="$ROOT_DIR/assets"
SOURCE_PNG="$ASSETS_DIR/icon.png"

echo "🎨 Converting application icons..."
echo ""

# Check if source PNG exists
if [ ! -f "$SOURCE_PNG" ]; then
    echo "❌ Error: Source icon not found at $SOURCE_PNG"
    exit 1
fi

# Create assets directory if it doesn't exist
mkdir -p "$ASSETS_DIR"

# ============================================
# Windows Icon (.ico)
# ============================================
echo "📦 Creating Windows icon (.ico)..."

if command -v convert &> /dev/null; then
    # Using ImageMagick
    convert "$SOURCE_PNG" \
        -define icon:auto-resize=256,128,64,48,32,16 \
        "$ASSETS_DIR/icon.ico"
    echo "✅ Created icon.ico"
elif command -v magick &> /dev/null; then
    # Using ImageMagick v7+ (Windows)
    magick "$SOURCE_PNG" \
        -define icon:auto-resize=256,128,64,48,32,16 \
        "$ASSETS_DIR/icon.ico"
    echo "✅ Created icon.ico"
else
    echo "⚠️  ImageMagick not found. Please install it to create .ico file:"
    echo "   - macOS: brew install imagemagick"
    echo "   - Linux: sudo apt install imagemagick / sudo pacman -S imagemagick"
    echo "   - Windows: choco install imagemagick"
    echo ""
    echo "Or use an online converter: https://convertio.co/png-ico/"
fi

# ============================================
# macOS Icon (.icns)
# ============================================
echo ""
echo "🍎 Creating macOS icon (.icns)..."

if [[ "$OSTYPE" == "darwin"* ]]; then
    # Create iconset directory
    ICONSET_DIR="$ASSETS_DIR/icon.iconset"
    mkdir -p "$ICONSET_DIR"

    # Generate multiple sizes for macOS
    for size in 16 32 64 128 256 512; do
        size2x=$((size * 2))
        if command -v sips &> /dev/null; then
            sips -z $size $size "$SOURCE_PNG" --out "$ICONSET_DIR/icon_${size}x${size}.png" > /dev/null
            sips -z $size2x $size2x "$SOURCE_PNG" --out "$ICONSET_DIR/icon_${size}x${size}@2x.png" > /dev/null
        fi
    done

    # Convert iconset to icns
    if command -v iconutil &> /dev/null; then
        iconutil -c icns "$ICONSET_DIR" -o "$ASSETS_DIR/icon.icns"
        rm -rf "$ICONSET_DIR"
        echo "✅ Created icon.icns"
    else
        echo "⚠️  iconutil not found (only available on macOS)"
        rm -rf "$ICONSET_DIR"
    fi
else
    echo "⚠️  macOS icon creation is only supported on macOS"
    echo "   You can create .icns manually using:"
    echo "   - Online: https://cloudconvert.com/png-to-icns"
    echo "   - Or on macOS using the provided script"
fi

echo ""
echo "📁 Icon files in $ASSETS_DIR:"
ls -lh "$ASSETS_DIR"/*.{ico,icns,png} 2>/dev/null || echo "   (only PNG available)"

echo ""
echo "✅ Done! Icon conversion complete."
