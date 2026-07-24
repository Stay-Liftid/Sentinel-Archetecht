#!/usr/bin/env bash
set -euo pipefail

echo "[*] Initializing The Remote Viewer local environment..."

# Detect OS and Architecture
OS="$(uname -s | tr '[:upper:]' '[:lower:c]')"
ARCH="$(uname -m)"

echo "[*] Detected platform: $OS-$ARCH"

# Dependency checks
for cmd in curl tar git; do
    if ! command -v "$cmd" &> /dev/null; then
        echo "[!] Error: Required tool '$cmd' is missing." >&2
        exit 1
    fi
done

# Create local state directory
mkdir -p ~/.local/share/remote-viewer/storage

# Pull latest release binary mapping or compile locally
if [ "$OS" = "linux" ]; then
    echo "[*] Setting up Linux/Android Termux runtime parameters..."
elif [ "$OS" = "darwin" ]; then
    echo "[*] Setting up macOS runtime parameters..."
fi

echo "[+] Initialization complete. Run './daemon --start' to launch."
