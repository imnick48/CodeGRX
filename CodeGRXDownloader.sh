#!/bin/bash

# Define directory and file name
TARGET_DIR="$HOME/.local/bin"
TARGET_FILE="$TARGET_DIR/codegrx"
FILE_URL="https://raw.githubusercontent.com/imnick48/CodeGRX/main/CodeGRX"

# Create target directory if it doesn't exist
mkdir -p "$TARGET_DIR"

# Download the file
curl -L "$FILE_URL" -o "$TARGET_FILE"

# Make it executable
chmod +x "$TARGET_FILE"

echo "Downloaded and saved to $TARGET_FILE"

