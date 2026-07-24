use sodiumoxide::crypto::sign;
use std::collections::HashSet;

pub struct WebOfTrust {
    trusted_keys: HashSet<[u8; sign::PUBLICKEYBYTES]>,
}

impl WebOfTrust {
    pub fn new() -> Self {
        Self {
            trusted_keys: HashSet::new(),
        }
    }

    pub fn provision_node(&mut self, public_key: &[u8]) {
        if public_key.len() == sign::PUBLICKEYBYTES {
            let mut key_array = [0u8; sign::PUBLICKEYBYTES];
            key_array.copy_from_slice(public_key);
            self.trusted_keys.insert(key_array);
            println!("[*] Provisioned new trusted node key in WoT");
        }
    }

    pub fn verify_packet(&self, payload: &[u8], signature_bytes: &[u8], public_key_bytes: &[u8]) -> bool {
        if signature_bytes.len() != sign::SIGNATUREBYTES || public_key_bytes.len() != sign::PUBLICKEYBYTES {
            return false;
        }

        let mut key_array = [0u8; sign::PUBLICKEYBYTES];
        key_array.copy_from_slice(public_key_bytes);

        // 1. Is this node in our offline registry?
        if !self.trusted_keys.contains(&key_array) {
            println!("[!] REJECTED: Public key not found in local Web of Trust");
            return false;
        }

        let pk = sign::PublicKey(key_array);
        let mut sig_array = [0u8; sign::SIGNATUREBYTES];
        sig_array.copy_from_slice(signature_bytes);
        let sig = sign::Signature(sig_array);

        // 2. Is the signature cryptographically valid for this payload?
        let is_valid = sign::verify_detached(&sig, payload, &pk);
        if !is_valid {
            println!("[!] REJECTED: Cryptographic signature mismatch");
        }

        is_valid
    }
}
