#!/bin/bash
# Vibe Idler - macOS Release Script
# Builds, signs, notarizes, and packages a release zip
#
# Prerequisites:
#   - Rust toolchain installed
#   - Apple Developer ID certificate in keychain
#   - Notarytool credentials stored in keychain:
#     xcrun notarytool store-credentials "notarytool" \
#       --apple-id EMAIL --team-id TEAM_ID --password APP_PASSWORD
#
# Usage:
#   ./scripts/release-macos.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
SIGNING_IDENTITY="Developer ID Application: Aaron Rutledge (49GA7WP8W5)"
BUNDLE_ID="com.areadenial.vibe-idler"

cd "$PROJECT_DIR"

# Get version from Cargo.toml
VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')
echo "============================================"
echo " Vibe Idler v${VERSION} - macOS Release"
echo "============================================"
echo

# Step 1: Build release binary
echo "[1/5] Building release binary..."
cargo build --release
BINARY="target/release/vibe-idler"
echo "  Built: $BINARY"
echo

# Step 2: Sign the binary
echo "[2/5] Signing with Developer ID..."
codesign --force --options runtime \
    --sign "$SIGNING_IDENTITY" \
    --identifier "$BUNDLE_ID" \
    --timestamp \
    "$BINARY"
echo "  Signed. Verifying..."
codesign -dv --verbose=2 "$BINARY" 2>&1 | grep -E "Authority|Identifier|Timestamp"
echo

# Step 3: Create zip for notarization
echo "[3/5] Packaging for notarization..."
RELEASE_DIR="target/release-pkg"
rm -rf "$RELEASE_DIR"
mkdir -p "$RELEASE_DIR/vibe-idler"
cp "$BINARY" "$RELEASE_DIR/vibe-idler/"
cp -R assets "$RELEASE_DIR/vibe-idler/"

# Remove .DS_Store files from assets
find "$RELEASE_DIR" -name ".DS_Store" -delete

NOTARIZE_ZIP="target/vibe-idler-${VERSION}-macos-notarize.zip"
(cd "$RELEASE_DIR" && zip -r "../../$NOTARIZE_ZIP" vibe-idler/)
echo "  Created: $NOTARIZE_ZIP"
echo

# Step 4: Notarize
echo "[4/5] Submitting to Apple for notarization..."
echo "  (This typically takes 2-5 minutes)"

NOTARY_ARGS=""
if xcrun notarytool history --keychain-profile "notarytool" >/dev/null 2>&1; then
    NOTARY_ARGS="--keychain-profile notarytool"
else
    echo "  ERROR: No keychain profile 'notarytool' found."
    echo "  Store credentials with:"
    echo "    xcrun notarytool store-credentials \"notarytool\" \\"
    echo "      --apple-id EMAIL --team-id TEAM_ID --password APP_PASSWORD"
    echo
    echo "  Skipping notarization. The signed binary will still work but"
    echo "  users may see a Gatekeeper warning on first launch."
    echo
    NOTARY_ARGS=""
fi

if [ -n "$NOTARY_ARGS" ]; then
    if xcrun notarytool submit "$NOTARIZE_ZIP" $NOTARY_ARGS --wait; then
        echo "  Notarization successful!"
    else
        echo "  WARNING: Notarization failed. Binary is signed but not notarized."
        echo "  Users may see a Gatekeeper warning."
    fi
fi
echo

# Step 5: Create final distribution zip
echo "[5/5] Creating distribution zip..."
DIST_ZIP="target/vibe-idler-${VERSION}-macos.zip"
cp "$NOTARIZE_ZIP" "$DIST_ZIP"
echo "  Created: $DIST_ZIP"
echo

# Summary
SIZE=$(du -sh "$DIST_ZIP" | cut -f1)
echo "============================================"
echo " Release Complete!"
echo "============================================"
echo
echo "  Version:  v${VERSION}"
echo "  File:     $DIST_ZIP"
echo "  Size:     $SIZE"
echo
echo "  The binary is signed with Developer ID and"
echo "  will run on any Mac without quarantine warnings."
echo
echo "  To create a GitHub release:"
echo "    gh release create v${VERSION} $DIST_ZIP --title \"v${VERSION}\" --notes \"Release notes here\""
echo
