#!/bin/bash
# Creates a macOS .app bundle from the vibe-idler binary + assets
# Usage: ./scripts/bundle-macos.sh <binary-path> <output-dir>

set -e

BINARY="$1"
OUTPUT_DIR="$2"
VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')

if [ -z "$BINARY" ] || [ -z "$OUTPUT_DIR" ]; then
    echo "Usage: $0 <binary-path> <output-dir>"
    exit 1
fi

APP_NAME="Vibe Idler"
APP_BUNDLE="$OUTPUT_DIR/${APP_NAME}.app"
CONTENTS="$APP_BUNDLE/Contents"
MACOS="$CONTENTS/MacOS"
RESOURCES="$CONTENTS/Resources"

rm -rf "$APP_BUNDLE"
mkdir -p "$MACOS" "$RESOURCES"

# Copy binary and assets
cp "$BINARY" "$MACOS/vibe-idler-bin"
cp -R assets "$MACOS/assets"

# Create launcher script that opens in Terminal
cat > "$MACOS/vibe-idler" << 'LAUNCHER'
#!/bin/bash
DIR="$(cd "$(dirname "$0")" && pwd)"

# If we're already in a terminal, just run
if [ -t 0 ] && [ -t 1 ]; then
    cd "$DIR"
    exec "$DIR/vibe-idler-bin"
fi

# Otherwise, open Terminal.app and run there
osascript -e "
tell application \"Terminal\"
    activate
    do script \"cd \\\"$DIR\\\" && ./vibe-idler-bin; exit\"
end tell
"
LAUNCHER
chmod +x "$MACOS/vibe-idler"

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
    <key>CFBundlePackagetype</key>
    <string>APPL</string>
    <key>CFBundleSignature</key>
    <string>????</string>
    <key>LSMinimumSystemVersion</key>
    <string>12.0</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>LSUIElement</key>
    <false/>
</dict>
</plist>
PLIST

echo "Created: $APP_BUNDLE"
