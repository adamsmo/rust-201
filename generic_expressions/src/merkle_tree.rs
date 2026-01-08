use crate::bool_assertions::{Assert, IsTrue, is_power_of_two, log2};

pub struct MerkleTree<const LEAVES: usize>
where
    // LEAVES must be power of 2 for MerkleTree to be zk-snarks friendly
    Assert<{ is_power_of_two(LEAVES) }>: IsTrue,
    // Total nodes, it is called well-formedness bound,
    // this is forcing the compiler to evaluate the expression
    [(); 2 * LEAVES - 1]:,
{
    nodes: [[u8; 32]; 2 * LEAVES - 1],
}

impl<const LEAVES: usize> MerkleTree<LEAVES>
where
    Assert<{ is_power_of_two(LEAVES) }>: IsTrue,
    [(); 2 * LEAVES - 1]:,
{
    pub const DEPTH: usize = log2(LEAVES);
    pub const NODE_COUNT: usize = 2 * LEAVES - 1;

    pub fn new() -> Self {
        Self {
            nodes: [[0u8; 32]; 2 * LEAVES - 1],
        }
    }

    pub fn depth(&self) -> usize {
        Self::DEPTH
    }

    pub fn leaf_count(&self) -> usize {
        LEAVES
    }

    pub fn total_nodes(&self) -> usize {
        Self::NODE_COUNT
    }
}

/// merkle proof that can be used for zk-proofs
pub struct MerkleProof<const LEAVES: usize>
where
    Assert<{ is_power_of_two(LEAVES) }>: IsTrue,
    [(); log2(LEAVES)]:,
{
    /// Sibling hashes along the merkle tree path
    siblings: [[u8; 32]; log2(LEAVES)],
    /// Binary path encoded with bits (0 = left branch, 1 = right branch)
    path: [bool; log2(LEAVES)],
}

#[test]
fn test_merkle_tree() {
    let tree = MerkleTree::<8>::new();
    assert_eq!(tree.depth(), 3);
    assert_eq!(tree.leaf_count(), 8);
    assert_eq!(tree.total_nodes(), 15);

    let big_tree = MerkleTree::<1024>::new();
    assert_eq!(big_tree.depth(), 10);
    assert_eq!(big_tree.total_nodes(), 2047);

    // example merkle proof with compile time checks for path description
    MerkleProof::<8> {
        siblings: [
            [
                42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
                42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
            ],
            [
                42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
                42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
            ],
            [
                42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
                42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
            ],
        ],
        path: [true, false, true],
    };

    // This does not compile
    // let bad_mpt = MerkleTree::<42>::new();
}
