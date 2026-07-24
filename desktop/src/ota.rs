use sha2::{Sha256, Digest};
use sodiumoxide::crypto::sign;
use std::fs::File;
use std::io::Read;

const ESP_NOW_MAX_PAYLOAD: usize = 200; // Leaving room for header overhead

pub struct OtaManager {
    firmware_binary: Vec<u8>,
    pub file_hash: String,
    pub signature: [u8; sign::SIGNATUREBYTES],
}

impl OtaManager {
    pub fn new(bin_path: &str, secret_key: &sign::SecretKey) -> Self {
        let mut file = File::open(bin_path).expect("Missing firmware binary");
        let mut firmware_binary = Vec::new();
        file.read_to_end(&mut firmware_binary).unwrap();

        // 1. Hash the entire binary
        let mut hasher = Sha256::new();
        hasher.update(&firmware_binary);
        let hash_result = hasher.finalize();
        let file_hash = hex::encode(hash_result);

        // 2. Sign the binary hash
        let signature = sign::sign_detached(&hash_result, secret_key);
        let mut sig_array = [0u8; sign::SIGNATUREBYTES];
        sig_array.copy_from_slice(&signature);

        Self {
            firmware_binary,
            file_hash,
            signature: sig_array,
        }
    }

    pub fn generate_chunks(&self) -> Vec<Vec<u8>> {
        self.firmware_binary.chunks(ESP_NOW_MAX_PAYLOAD)
            .map(|chunk| chunk.to_vec())
            .collect()
    }
}
