use sodiumoxide::crypto::sign;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
pub struct HybridLogicalClock {
    physical_time: u64,
    logical_counter: u32,
}

impl HybridLogicalClock {
    pub fn new() -> Self {
        let start = SystemTime::now();
        let physical_time = start.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        Self {
            physical_time,
            logical_counter: 0,
        }
    }

    pub fn tick(&mut self) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        if now > self.physical_time {
            self.physical_time = now;
            self.logical_counter = 0;
        } else {
            self.logical_counter += 1;
        }
    }
}

fn main() {
    sodiumoxide::init().unwrap();
    let (pk, sk) = sign::gen_keypair();
    
    let mut hlc = HybridLogicalClock::new();
    hlc.tick();

    let message = format!("telemetry_payload_node_alpha_{}", hlc.physical_time);
    let signature = sign::sign(message.as_bytes(), &sk);

    println!("[*] Daemon online. Node Public Key: {:?}", pk);
    println!("[*] Generated HLC Timestamp: {:?}", hlc);
    println!("[+] Signed telemetry packet securely via libsodium.");
}
