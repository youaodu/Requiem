#!/bin/bash
# Script to create macOS DMG installer
# Usage: ./create-dmg.sh <version>

set -e

VERSION=${1:-"0.0.1"}
APP_NAME="Requiem"
DMG_NAME="${APP_NAME}-${VERSION}-macOS"
BINARY_PATH="target/release/requiem"

echo "Creating DMG for ${APP_NAME} ${VERSION}..."

# Check if binary exists
if [ ! -f "$BINARY_PATH" ]; then
    echo "Error: Binary not found at $BINARY_PATH"
    echo "Please run 'cargo build --release' first"
    exit 1
fi

# Clean up previous builds
rm -rf dist/dmg
rm -rf "dist/${APP_NAME}.app"
rm -f "dist/${DMG_NAME}.dmg"

# Create .app bundle structure
echo "Creating .app bundle..."
mkdir -p "dist/${APP_NAME}.app/Contents/MacOS"
mkdir -p "dist/${APP_NAME}.app/Contents/Resources/locales"

# Copy binary
cp "$BINARY_PATH" "dist/${APP_NAME}.app/Contents/MacOS/requiem"
chmod +x "dist/${APP_NAME}.app/Contents/MacOS/requiem"

# Copy Info.plist with version substitution
sed "s/0.0.1/${VERSION}/g" packaging/macos/Info.plist > "dist/${APP_NAME}.app/Contents/Info.plist"

# Copy resources
cp locales/*.json "dist/${APP_NAME}.app/Contents/Resources/locales/" 2>/dev/null || true
cp LICENSE "dist/${APP_NAME}.app/Contents/Resources/" 2>/dev/null || true

# Copy icon if it exists
# if [ -f "assets/icon.icns" ]; then
#     cp assets/icon.icns "dist/${APP_NAME}.app/Contents/Resources/"
# fi

echo ".app bundle created successfully"

# Create DMG staging directory
mkdir -p dist/dmg
cp -R "dist/${APP_NAME}.app" dist/dmg/

# Create symbolic link to Applications
ln -s /Applications dist/dmg/Applications

# Create DMG
echo "Creating DMG..."
hdiutil create -volname "${APP_NAME}" \
    -srcfolder dist/dmg \
    -ov -format UDZO \
    "dist/${DMG_NAME}.dmg"

echo "✅ DMG created successfully: dist/${DMG_NAME}.dmg"
echo ""
echo "File size:"
ls -lh "dist/${DMG_NAME}.dmg" | awk '{print $5}'

# Clean up staging
rm -rf dist/dmg

echo ""
echo "To test the DMG:"
echo "  open dist/${DMG_NAME}.dmg"
