use heed::{EnvOpenOptions, Database};
use heed::types::{Str, Bytes};
use std::fs;
use std::path::Path;

const DB_PATH: &str = "/data/data/com.termux/files/home/.local/share/remote-viewer/lmdb";

pub struct StorageEngine {
    env: heed::Env,
    telemetry_db: Database<Str, Bytes>,
}

impl StorageEngine {
    pub fn new() -> Self {
        // Ensure directory exists
        if !Path::new(DB_PATH).exists() {
            fs::create_dir_all(DB_PATH).expect("Failed to create LMDB directory");
        }

        // Initialize LMDB Environment with 1GB max map size
        let env = EnvOpenOptions::new()
            .map_size(1024 * 1024 * 1024) 
            .max_dbs(2)
            .open(DB_PATH)
            .expect("Failed to open LMDB environment");

        let mut wtxn = env.write_txn().unwrap();
        let telemetry_db = env.create_database(&mut wtxn, Some("telemetry")).unwrap();
        wtxn.commit().unwrap();

        Self { env, telemetry_db }
    }

    pub fn insert_telemetry(&self, packet_hash: &str, raw_packet: &[u8]) {
        let mut wtxn = self.env.write_txn().unwrap();
        
        self.telemetry_db
            .put(&mut wtxn, &packet_hash, &raw_packet)
            .expect("LMDB Write Failed");
            
        wtxn.commit().expect("LMDB Commit Failed");
        println!("[+] Committed {} bytes to LMDB -> {}", raw_packet.len(), packet_hash);
    }
}
