#!/bin/bash

# Set directory containing your rnostr binary
APP_DIR="ndb.AppDir"

# Create the AppDir if it doesn't exist
mkdir -p "$APP_DIR"

# Copy the rnostr binary
cp target/release/ndb "$APP_DIR/AppRun"

# Include necessary libraries (replace paths if they differ on your system)
cp /lib64/libgcc_s.so.1 "$APP_DIR/libgcc_s.so.1"
cp /lib64/libm.so.6 "$APP_DIR/libm.so.6"
cp /lib64/libc.so.6 "$APP_DIR/libc.so.6"

# Include the dynamic linker (may vary depending on your system)
#cp /lib64/ld-linux-x86_64.so.2 "$APP_DIR/usr/lib/x86_64-linux-gnu/ld-linux-x86_64.so.2"

# Add additional dependencies (if any)
# Use ldd target/release/rnostr to identify other required libraries

touch "$APP_DIR/icon.png"

# Create a basic desktop file (replace with your application details)
cat > "$APP_DIR/rnostr.desktop" << EOF
[Desktop Entry]
Name=ndb
GenericName=ndb
Comment=Your Rnostr DB cli tool
Exec=./AppRun
Icon=icon
Terminal=false
Type=Application
Categories=Utility
EOF

# Define the runtime file name and URL
RUNTIME_FILE="runtime-x86_64"
RUNTIME_URL="https://github.com/AppImage/type2-runtime/releases/download/continuous/$RUNTIME_FILE"

# Check if the runtime file exists
if [ ! -f "$RUNTIME_FILE" ]; then
  echo "Runtime file '$RUNTIME_FILE' not found. Downloading..."
  # Download the runtime file with wget (ensure wget is installed)
  wget "$RUNTIME_URL" || {
    echo "Error downloading runtime file. Please check your internet connection."
    exit 1
  }
fi

# AppImage name (change if desired)
APPIMAGE_NAME="ndb-$(git rev-parse --short HEAD).AppImage"  # Add version control integration (optional)

# Create the AppImage
appimagetool-x86_64.appimage --runtime-file runtime-x86_64 "$APP_DIR" "$APPIMAGE_NAME" #&& rm -r "$APP_DIR"

echo "AppImage created: $APPIMAGE_NAME"
