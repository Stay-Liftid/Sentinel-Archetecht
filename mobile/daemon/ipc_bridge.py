import socket
import json
import os
import time

SOCKET_PATH = os.path.expanduser("~/.local/share/remote-viewer/ingest.sock")

def push_to_orchestrator(metrics_dict):
    """
    Pushes processed spectrum metrics into the Rust UDS pipe.
    Fails securely if the orchestrator daemon is not running.
    """
    if not os.path.exists(SOCKET_PATH):
        print("[!] Rust daemon offline. Dropping packet.")
        return

    try:
        with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as client:
            client.connect(SOCKET_PATH)
            
            # Serialize payload and append a newline for framing
            payload = json.dumps(metrics_dict).encode('utf-8') + b'\n'
            client.sendall(payload)
            
    except ConnectionRefusedError:
        print("[!] Connection refused by Rust IPC pipe.")

# Example Integration with the previous SDR loop
def example_ingest_loop():
    while True:
        # 1. Process IQ chunk...
        # 2. Compute density metrics...
        
        metrics = {
            "spectrum": "RF_ISM",
            "power_dbfs": -42.5,
            "timestamp": time.time_ns()
        }
        
        push_to_orchestrator(metrics)
        time.sleep(1)
