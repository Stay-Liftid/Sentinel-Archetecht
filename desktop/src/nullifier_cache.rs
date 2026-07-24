use std::collections::HashSet;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

const EPOCH_DURATION_SECS: u64 = 60;

pub struct NullifierLedger {
    seen_nullifiers: Mutex<HashSet<String>>,
    current_epoch: Mutex<u64>,
}

impl NullifierLedger {
    pub fn new() -> Self {
        Self {
            seen_nullifiers: Mutex::new(HashSet::new()),
            current_epoch: Mutex::new(Self::get_active_epoch()),
        }
    }

    pub fn get_active_epoch() -> u64 {
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        start / EPOCH_DURATION_SECS
    }

    pub fn validate_and_register(&self, nullifier: &str) -> bool {
        let active_epoch = Self::get_active_epoch();
        let mut current_epoch = self.current_epoch.lock().unwrap();
        let mut set = self.seen_nullifiers.lock().unwrap();

        // Flush cache if we entered a new time epoch
        if active_epoch != *current_epoch {
            *current_epoch = active_epoch;
            set.clear();
            println!("[*] Transitioned to epoch {}. Flushed old ZK nullifiers.", active_epoch);
        }

        // Evaluate nullifier uniqueness
        if set.contains(nullifier) {
            println!("[!] REJECTED: Replay attack detected! Nullifier already spent: {}", nullifier);
            return false;
        }

        set.insert(nullifier.to_string());
        println!("[+] Nullifier accepted: {}", nullifier);
        true
    }
}
