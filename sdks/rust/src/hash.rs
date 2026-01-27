//! Hashing utilities for Makoto attestations.
//!
//! Provides SHA-256 hashing, Merkle tree construction, and proof generation.

use crate::error::{MakotoError, Result};
use crate::types::HashAlgorithm;
use sha2::{Digest as Sha2Digest, Sha256};

/// Compute SHA-256 hash of data and return as hex string.
pub fn sha256_hex(data: &[u8]) -> String {
    let hash = Sha256::digest(data);
    hex::encode(hash)
}

/// Compute SHA-256 hash of a string and return as hex string.
pub fn sha256_str(s: &str) -> String {
    sha256_hex(s.as_bytes())
}

/// A Merkle tree for efficient integrity verification.
#[derive(Debug, Clone)]
pub struct MerkleTree {
    /// Leaf hashes (level 0).
    leaves: Vec<[u8; 32]>,
    /// Internal nodes organized by level (level 1 = parents of leaves, etc.).
    levels: Vec<Vec<[u8; 32]>>,
    /// Hash algorithm used.
    algorithm: HashAlgorithm,
}

impl MerkleTree {
    /// Create a new Merkle tree from leaf data.
    ///
    /// Each item in `leaves` is hashed to create the leaf nodes.
    pub fn from_leaves(leaves: &[&[u8]]) -> Self {
        let leaf_hashes: Vec<[u8; 32]> = leaves
            .iter()
            .map(|data| {
                let hash = Sha256::digest(data);
                let mut arr = [0u8; 32];
                arr.copy_from_slice(&hash);
                arr
            })
            .collect();

        Self::from_leaf_hashes(leaf_hashes)
    }

    /// Create a new Merkle tree from pre-computed leaf hashes.
    pub fn from_leaf_hashes(leaf_hashes: Vec<[u8; 32]>) -> Self {
        if leaf_hashes.is_empty() {
            return Self {
                leaves: vec![],
                levels: vec![],
                algorithm: HashAlgorithm::Sha256,
            };
        }

        let mut levels = Vec::new();
        let mut current_level = leaf_hashes.clone();

        // Build tree bottom-up
        while current_level.len() > 1 {
            let mut next_level = Vec::new();

            for chunk in current_level.chunks(2) {
                let hash = if chunk.len() == 2 {
                    hash_pair(&chunk[0], &chunk[1])
                } else {
                    // Odd number of nodes: duplicate the last one
                    hash_pair(&chunk[0], &chunk[0])
                };
                next_level.push(hash);
            }

            levels.push(current_level);
            current_level = next_level;
        }

        // Add root level
        levels.push(current_level);

        Self {
            leaves: leaf_hashes,
            levels,
            algorithm: HashAlgorithm::Sha256,
        }
    }

    /// Get the root hash of the tree.
    pub fn root(&self) -> Option<[u8; 32]> {
        self.levels.last().and_then(|level| level.first().copied())
    }

    /// Get the root hash as a hex string.
    pub fn root_hex(&self) -> Option<String> {
        self.root().map(|r| hex::encode(r))
    }

    /// Get the number of leaves.
    pub fn leaf_count(&self) -> usize {
        self.leaves.len()
    }

    /// Get the height of the tree.
    pub fn height(&self) -> usize {
        self.levels.len()
    }

    /// Get the hash algorithm used.
    pub fn algorithm(&self) -> HashAlgorithm {
        self.algorithm
    }

    /// Generate a Merkle proof for a leaf at the given index.
    pub fn proof(&self, leaf_index: usize) -> Result<MerkleProof> {
        if leaf_index >= self.leaves.len() {
            return Err(MakotoError::MerkleError(format!(
                "Leaf index {} out of bounds (tree has {} leaves)",
                leaf_index,
                self.leaves.len()
            )));
        }

        let mut siblings = Vec::new();
        let mut positions = Vec::new();
        let mut index = leaf_index;

        for level in &self.levels[..self.levels.len().saturating_sub(1)] {
            let sibling_index = if index % 2 == 0 { index + 1 } else { index - 1 };

            if sibling_index < level.len() {
                siblings.push(level[sibling_index]);
                positions.push(if index % 2 == 0 {
                    SiblingPosition::Right
                } else {
                    SiblingPosition::Left
                });
            } else {
                // Odd number of nodes: sibling is self
                siblings.push(level[index]);
                positions.push(SiblingPosition::Right);
            }

            index /= 2;
        }

        Ok(MerkleProof {
            leaf_index,
            leaf_hash: self.leaves[leaf_index],
            siblings,
            positions,
        })
    }

    /// Verify that a proof is valid for this tree.
    pub fn verify_proof(&self, proof: &MerkleProof) -> bool {
        let computed_root = proof.compute_root();
        self.root() == Some(computed_root)
    }
}

/// Position of a sibling node in a Merkle proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SiblingPosition {
    Left,
    Right,
}

/// A Merkle proof for a single leaf.
#[derive(Debug, Clone)]
pub struct MerkleProof {
    /// Index of the leaf being proved.
    pub leaf_index: usize,
    /// Hash of the leaf.
    pub leaf_hash: [u8; 32],
    /// Sibling hashes from leaf to root.
    pub siblings: Vec<[u8; 32]>,
    /// Position of each sibling (left or right).
    pub positions: Vec<SiblingPosition>,
}

impl MerkleProof {
    /// Compute the root hash from this proof.
    pub fn compute_root(&self) -> [u8; 32] {
        let mut current = self.leaf_hash;

        for (sibling, position) in self.siblings.iter().zip(self.positions.iter()) {
            current = match position {
                SiblingPosition::Left => hash_pair(sibling, &current),
                SiblingPosition::Right => hash_pair(&current, sibling),
            };
        }

        current
    }

    /// Verify the proof against an expected root.
    pub fn verify(&self, expected_root: &[u8; 32]) -> bool {
        &self.compute_root() == expected_root
    }

    /// Convert to hex-encoded format for JSON serialization.
    pub fn to_hex(&self) -> MerkleProofHex {
        MerkleProofHex {
            leaf_index: self.leaf_index,
            leaf_hash: hex::encode(self.leaf_hash),
            siblings: self.siblings.iter().map(|s| hex::encode(s)).collect(),
            positions: self
                .positions
                .iter()
                .map(|p| match p {
                    SiblingPosition::Left => "left".to_string(),
                    SiblingPosition::Right => "right".to_string(),
                })
                .collect(),
        }
    }
}

/// Hex-encoded Merkle proof for JSON serialization.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MerkleProofHex {
    pub leaf_index: usize,
    pub leaf_hash: String,
    pub siblings: Vec<String>,
    pub positions: Vec<String>,
}

/// Hash two 32-byte values together.
fn hash_pair(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(left);
    hasher.update(right);
    let result = hasher.finalize();
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&result);
    arr
}

/// Compute hash from hex-encoded data.
pub fn hash_from_hex(hex_data: &str) -> Result<[u8; 32]> {
    let bytes = hex::decode(hex_data)
        .map_err(|e| MakotoError::InvalidAttestation(format!("Invalid hex: {}", e)))?;

    if bytes.len() != 32 {
        return Err(MakotoError::InvalidAttestation(format!(
            "Expected 32 bytes, got {}",
            bytes.len()
        )));
    }

    let mut arr = [0u8; 32];
    arr.copy_from_slice(&bytes);
    Ok(arr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_hex() {
        let hash = sha256_str("hello");
        assert_eq!(
            hash,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_merkle_tree_single_leaf() {
        let tree = MerkleTree::from_leaves(&[b"leaf1"]);
        assert_eq!(tree.leaf_count(), 1);
        assert!(tree.root().is_some());
    }

    #[test]
    fn test_merkle_tree_two_leaves() {
        let tree = MerkleTree::from_leaves(&[b"leaf1", b"leaf2"]);
        assert_eq!(tree.leaf_count(), 2);
        assert_eq!(tree.height(), 2);

        let root = tree.root_hex().unwrap();
        assert_eq!(root.len(), 64); // SHA-256 = 32 bytes = 64 hex chars
    }

    #[test]
    fn test_merkle_tree_four_leaves() {
        let tree = MerkleTree::from_leaves(&[b"a", b"b", b"c", b"d"]);
        assert_eq!(tree.leaf_count(), 4);
        assert_eq!(tree.height(), 3); // leaves, level1, root

        // Verify proof for each leaf
        for i in 0..4 {
            let proof = tree.proof(i).unwrap();
            assert!(tree.verify_proof(&proof));
        }
    }

    #[test]
    fn test_merkle_tree_odd_leaves() {
        let tree = MerkleTree::from_leaves(&[b"a", b"b", b"c"]);
        assert_eq!(tree.leaf_count(), 3);

        // Should still work with odd number
        for i in 0..3 {
            let proof = tree.proof(i).unwrap();
            assert!(tree.verify_proof(&proof));
        }
    }

    #[test]
    fn test_merkle_proof_verification() {
        let tree = MerkleTree::from_leaves(&[b"record1", b"record2", b"record3", b"record4"]);
        let root = tree.root().unwrap();

        let proof = tree.proof(2).unwrap();
        assert!(proof.verify(&root));

        // Tamper with the proof
        let mut bad_proof = proof.clone();
        bad_proof.leaf_hash[0] ^= 1;
        assert!(!bad_proof.verify(&root));
    }

    #[test]
    fn test_proof_out_of_bounds() {
        let tree = MerkleTree::from_leaves(&[b"a", b"b"]);
        assert!(tree.proof(5).is_err());
    }

    #[test]
    fn test_empty_tree() {
        let tree = MerkleTree::from_leaves(&[]);
        assert_eq!(tree.leaf_count(), 0);
        assert!(tree.root().is_none());
    }
}
