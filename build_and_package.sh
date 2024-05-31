#!/bin/bash

set -e

# Configuration
APP_NAME="DecisionMaker"
EXECUTABLE_NAME="decision_maker"
VERSION="0.1.0"
MAINTAINER="Marvin Lerousse <marvin@donjon.com>"
DESCRIPTION="This is my Rust GUI application"
ICON_PATH="ui/favicon/marvin.png"
BUILD_DIR="target/release"
DEB_PACKAGE_DIR="$APP_NAME"
BIN_INSTALL_PATH="/usr/local/bin"
ICON_INSTALL_PATH="/usr/share/icons/hicolor/256x256/apps"
DESKTOP_INSTALL_PATH="/usr/share/applications"
DESKTOP_ENTRY="$APP_NAME.desktop"
DEB_FILE="$APP_NAME-$VERSION.deb"

# Build the release binary
echo "Building the release binary..."
cargo build --release

# Create necessary directory structure for the .deb package
echo "Creating directory structure for the .deb package..."
mkdir -p "$DEB_PACKAGE_DIR/DEBIAN"
mkdir -p "$DEB_PACKAGE_DIR$BIN_INSTALL_PATH"
mkdir -p "$DEB_PACKAGE_DIR$ICON_INSTALL_PATH"
mkdir -p "$DEB_PACKAGE_DIR$DESKTOP_INSTALL_PATH"

# Copy the binary, icon, and desktop entry file
echo "Copying files..."
cp "$BUILD_DIR/$EXECUTABLE_NAME" "$DEB_PACKAGE_DIR$BIN_INSTALL_PATH/"
cp "$ICON_PATH" "$DEB_PACKAGE_DIR$ICON_INSTALL_PATH/$APP_NAME.png"

# Create the desktop entry file
echo "Creating desktop entry file..."
cat > "$DEB_PACKAGE_DIR$DESKTOP_INSTALL_PATH/$DESKTOP_ENTRY" << EOL
[Desktop Entry]
Version=$VERSION
Name=$APP_NAME
Comment=$DESCRIPTION
Exec=$BIN_INSTALL_PATH/$EXECUTABLE_NAME
Icon=$ICON_INSTALL_PATH/$APP_NAME.png
Terminal=false
Type=Application
Categories=Utility;
EOL

# Create the control file
echo "Creating control file..."
cat > "$DEB_PACKAGE_DIR/DEBIAN/control" << EOL
Package: $APP_NAME
Version: $VERSION
Section: base
Priority: optional
Architecture: amd64
Maintainer: $MAINTAINER
Description: $DESCRIPTION
EOL

# Build the .deb package
echo "Building the .deb package..."
dpkg-deb --build "$DEB_PACKAGE_DIR"

# Move the .deb package to the current directory
mv "$DEB_PACKAGE_DIR.deb" "$DEB_FILE"

# Clean up
echo "Cleaning up..."
rm -rf "$DEB_PACKAGE_DIR"

echo "Package $DEB_FILE created successfully!"
