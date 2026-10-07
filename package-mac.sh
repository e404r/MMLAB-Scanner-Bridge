#!/bin/bash
set -e

DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$DIR"

echo "🦀 Building release binary for macOS..."
cargo build --release

APP_NAME="MMLAB Scanner Bridge"
DIST_DIR="$DIR/dist"
APP_BUNDLE="$DIST_DIR/macOS/$APP_NAME.app"

echo "📦 Creating macOS App Bundle ($APP_NAME.app)..."
rm -rf "$DIST_DIR/macOS"
mkdir -p "$APP_BUNDLE/Contents/MacOS"
mkdir -p "$APP_BUNDLE/Contents/Resources"

# Copy binary
cp "$DIR/target/release/hr-scanner-bridge" "$APP_BUNDLE/Contents/MacOS/$APP_NAME"
chmod +x "$APP_BUNDLE/Contents/MacOS/$APP_NAME"

# Copy Icon
if [ -f "$DIR/AppIcon.icns" ]; then
    cp "$DIR/AppIcon.icns" "$APP_BUNDLE/Contents/Resources/AppIcon.icns"
fi

# Create Info.plist
cat << 'EOF' > "$APP_BUNDLE/Contents/Info.plist"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>MMLAB Scanner Bridge</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundleIdentifier</key>
    <string>ge.mmlab.scannerbridge</string>
    <key>CFBundleName</key>
    <string>MMLAB Scanner Bridge</string>
    <key>CFBundleDisplayName</key>
    <string>MMLAB Scanner Bridge</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>LSMinimumSystemVersion</key>
    <string>11.0</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSAppTransportSecurity</key>
    <dict>
        <key>NSAllowsArbitraryLoads</key>
        <true/>
    </dict>
</dict>
</plist>
EOF

echo "💿 Creating macOS DMG Installer..."
hdiutil create -volname "$APP_NAME" -srcfolder "$DIST_DIR/macOS" -ov -format UDZO "$DIST_DIR/MMLAB-Scanner-Bridge-macOS.dmg"

echo "🗜️ Creating ZIP Archive..."
cd "$DIST_DIR/macOS"
zip -r -q "$DIST_DIR/MMLAB-Scanner-Bridge-macOS.zip" "$APP_NAME.app"

echo "✅ SUCCESS!"
echo "📁 App Bundle: $APP_BUNDLE"
echo "💿 DMG File:   $DIST_DIR/MMLAB-Scanner-Bridge-macOS.dmg"
echo "🗜️ ZIP File:   $DIST_DIR/MMLAB-Scanner-Bridge-macOS.zip"
