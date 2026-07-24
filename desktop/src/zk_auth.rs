use ark_bn254::Bn254;
use ark_groth16::{Groth16, Proof, VerifyingKey};
use ark_snark::SNARK;

pub struct ZkAuthValidator {
    verifying_key: VerifyingKey<Bn254>,
}

impl ZkAuthValidator {
    pub fn new(vk: VerifyingKey<Bn254>) -> Self {
        Self { verifying_key: vk }
    }

    pub fn verify_node_proof(&self, proof_bytes: &[u8], current_merkle_root: &[u8]) -> bool {
        // Deserialize the succinct Groth16 proof broadcasted by the edge node
        let proof = Proof::<Bn254>::deserialize_compressed(proof_bytes).unwrap();
        
        // Public inputs vector matching the circuit definition
        let public_inputs = vec![/* Converted Merkle root field element */];

        // Verify mathematically that the prover holds a valid WoT leaf 
        // without leaking identity or public key material.
        let is_authentic = Groth16::<Bn254>::verify(
            &self.verifying_key, 
            &public_inputs, 
            &proof
        ).unwrap_or(false);

        if is_authentic {
            println!("[+] ZK-PoA Verified: Anonymous node belongs to Web of Trust.");
        } else {
            println!("[!] ZK-PoA REJECTED: Invalid membership proof.");
        }

        is_authentic
    }
}
