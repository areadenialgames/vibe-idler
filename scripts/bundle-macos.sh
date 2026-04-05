#!/bin/bash
# Creates a macOS .app bundle from the vibe-idler binary + assets
# Usage: ./scripts/bundle-macos.sh <binary-path> <output-dir> [arch]

set -e

BINARY="$1"
OUTPUT_DIR="$2"
FORCE_ARCH="${3:-}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')

if [ -z "$BINARY" ] || [ -z "$OUTPUT_DIR" ]; then
    echo "Usage: $0 <binary-path> <output-dir> [arch]"
    exit 1
fi

if [ -n "$FORCE_ARCH" ]; then
    ARCH="$FORCE_ARCH"
else
    ARCH=$(lipo -archs "$BINARY" 2>/dev/null || echo "arm64")
fi
echo "Building .app bundle for architecture: $ARCH"

APP_NAME="Vibe Idler"
APP_BUNDLE="$OUTPUT_DIR/${APP_NAME}.app"
CONTENTS="$APP_BUNDLE/Contents"
MACOS="$CONTENTS/MacOS"
RESOURCES="$CONTENTS/Resources"

rm -rf "$APP_BUNDLE"
mkdir -p "$MACOS" "$RESOURCES"

# Copy the game binary and assets together (binary expects assets/ as sibling)
cp "$BINARY" "$MACOS/vibe-idler-bin"
cp -R assets "$RESOURCES/assets"
find "$RESOURCES" -name ".DS_Store" -delete

# Compile the Swift launcher for the target architecture
echo "Compiling launcher..."
swiftc -target "${ARCH}-apple-macosx12.0" \
    -O -o "$MACOS/vibe-idler" \
    "$SCRIPT_DIR/launcher.swift"

# Create Info.plist
cat > "$CONTENTS/Info.plist" << PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>vibe-idler</string>
    <key>CFBundleIdentifier</key>
    <string>com.areadenial.vibe-idler</string>
    <key>CFBundleName</key>
    <string>${APP_NAME}</string>
    <key>CFBundleDisplayName</key>
    <string>${APP_NAME}</string>
    <key>CFBundleVersion</key>
    <string>${VERSION}</string>
    <key>CFBundleShortVersionString</key>
    <string>${VERSION}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleSignature</key>
    <string>????</string>
    <key>LSMinimumSystemVersion</key>
    <string>12.0</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>CFBundleDocumentTypes</key>
    <array/>
</dict>
</plist>
PLIST

echo "Created: $APP_BUNDLE"
