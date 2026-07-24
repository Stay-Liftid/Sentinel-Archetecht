#!/data/data/com.termux/files/usr/bin/bash
TARGET_SCRIPT="mobile/daemon/sdr_stream.py"
LOG_PATH="$HOME/.local/share/remote-viewer/sdr.log"

while true; do
    if ! pgrep -f "$TARGET_SCRIPT" > /dev/null; then
        echo "[!] SDR daemon crashed or stopped. Restarting..." >> "$LOG_PATH"
        termux-wake-lock
        python3 "$TARGET_SCRIPT" &
    fi
    sleep 10
done
