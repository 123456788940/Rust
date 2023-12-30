use sha2::{Digest, Sha256};

struct MerkleTree {
    leaves: Vec<String>,
    root: String,
}

impl MerkleTree {
    fn new(leaves: Vec<String>) -> Self {
        let root = MerkleTree::compute_root(&leaves);
        MerkleTree {leaves, root}
    }

    fn compute_root(leaves: &[String]) -> String {
        if leaves.is_empty() {
            return String::from("No leaves");
        }

        let mut current_level: Vec<String> = leaves.to_vec();

        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            for chunk in current_level.chunks(2) {
                let mut hasher = Sha256::new();
                let concatenated = format!("{} {}", chunk[0], chunk.get(1).unwrap_or(&"".to_string()));
                hasher.update(concatenated);
                let result = hasher.finalize();
                let hex_string: String = result.iter().map(|byte| format!("{:02x}", byte)).collect();
                next_level.push(hex_string);
            }
            current_level = next_level;
        }
        current_level.pop().unwrap_or("No_Root".to_string())
    }
}

fn main() {
    let leaves = vec![
        "leaf1".to_string(), 
    "leaf2".to_string(),
     "leaf3".to_string(),
      "leaf4".to_string(), 
      "leaf5".to_string()];
    let merkle_tree = MerkleTree::new(leaves.clone());
    println!("leaves: {:?}", merkle_tree.leaves);
    println!("root: {}", merkle_tree.root);

}
