#!/data/data/com.termux/files/usr/bin/bash
termux-wake-lock

# Set environment paths
export PREFIX="/data/data/com.termux/files/usr"
export HOME="/data/data/com.termux/files/home"
APP_DIR="$HOME/the-remote-viewer"

cd "$APP_DIR" || exit 1

echo "[*] Boot sequence triggered: Launching The Remote Viewer..."

# 1. Start the Termux SDR Python daemon in the background
nohup python3 mobile/daemon/sdr_stream.py > "$HOME/.local/share/remote-viewer/sdr.log" 2>&1 &

# 2. Start the compiled Rust Master Orchestrator daemon
nohup ./target/release/remote-viewer-orchestrator > "$HOME/.local/share/remote-viewer/daemon.log" 2>&1 &

echo "[+] Sovereign daemons running."
