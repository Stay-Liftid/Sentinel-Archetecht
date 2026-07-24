pragma circom 2.0.0;

include "./node_modules/circomlib/circuits/poseidon.circom";
include "./node_modules/circomlib/circuits/merkleTree.circom";

template NodeMembership(levels) {
    // Private inputs (Known only to the authenticating node)
    signal input secret;
    
    // Public inputs (Known to all orchestrators and mesh nodes)
    signal input merkleRoot;
    signal input pathElements[levels];
    signal input pathIndices[levels];

    // 1. Compute public commitment from the private secret
    component commitmentHasher = Poseidon(1);
    commitmentHasher.inputs[0] <== secret;
    signal computedCommitment;
    computedCommitment <== commitmentHasher.out;

    // 2. Verify that the commitment exists within the public Merkle tree root
    component treeVerifier = MerkleTreeChecker(levels);
    treeVerifier.leaf <== computedCommitment;
    treeVerifier.root <== merkleRoot;
    for (var i = 0; i < levels; i++) {
        treeVerifier.pathElements[i] <== pathElements[i];
        treeVerifier.pathIndices[i] <== pathIndices[i];
    }
}

component main {public [merkleRoot]} = NodeMembership(10);
