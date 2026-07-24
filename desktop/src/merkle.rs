use sha2::{Sha256, Digest};
use std::collections::HashMap;

pub struct StateMerkleTree {
    pub root_hash: String,
    leaves: HashMap<String, Vec<u8>>, // Hash -> Raw Telemetry Packet
}

impl StateMerkleTree {
    pub fn new() -> Self {
        Self {
            root_hash: String::new(),
            leaves: HashMap::new(),
        }
    }

    pub fn insert_packet(&mut self, packet_data: &[u8]) {
        // Hash the incoming packet
        let mut hasher = Sha256::new();
        hasher.update(packet_data);
        let packet_hash = hex::encode(hasher.finalize());

        // Store the leaf
        self.leaves.insert(packet_hash.clone(), packet_data.to_vec());

        // Recalculate root (Simplified: In a real implementation, 
        // you would rebuild the binary tree structure here).
        self.recalculate_root();
    }

    fn recalculate_root(&mut self) {
        // A naive root calculation for demonstration: hashing all leaf hashes together.
        // A true Merkle tree requires a structured binary pair-hashing.
        let mut all_hashes: Vec<String> = self.leaves.keys().cloned().collect();
        all_hashes.sort(); 

        let mut hasher = Sha256::new();
        for h in all_hashes {
            hasher.update(h.as_bytes());
        }
        self.root_hash = hex::encode(hasher.finalize());
    }
}
