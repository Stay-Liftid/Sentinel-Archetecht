use std::os::unix::net::UnixListener;
use std::io::{BufRead, BufReader};
use std::fs;
use sodiumoxide::crypto::sign;
use std::path::Path;

const SOCKET_PATH: &str = "/data/data/com.termux/files/home/.local/share/remote-viewer/ingest.sock";

pub fn start_ipc_listener(secret_key: &sign::SecretKey) {
    // Ensure clean socket bind
    if Path::new(SOCKET_PATH).exists() {
        fs::remove_file(SOCKET_PATH).unwrap();
    }

    let listener = UnixListener::bind(SOCKET_PATH).expect("Failed to bind UDS pipe");
    println!("[*] Rust orchestrator listening for local telemetry on UDS...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut reader = BufReader::new(stream);
                let mut buffer = String::new();

                // Read newline-framed JSON from Python
                if let Ok(bytes_read) = reader.read_line(&mut buffer) {
                    if bytes_read > 0 {
                        println!("[+] Received raw metrics: {}", buffer.trim());
                        
                        // 1. Generate HLC Timestamp
                        // 2. Encode to Protobuf structure
                        // 3. Cryptographically sign the Protobuf byte array
                        
                        let signature = sign::sign_detached(buffer.as_bytes(), secret_key);
                        println!("[+] Ed25519 Signature generated: {:?}", signature);
                        
                        // 4. Push to P2P Mesh Gossip...
                    }
                }
            }
            Err(err) => println!("[!] IPC Stream failure: {}", err),
        }
    }
}
