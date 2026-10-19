use sha2::{Sha256, Digest};

pub struct KaspaBlockHeader {
    pub version: u16,
    pub parents: Vec<String>,
    pub timestamp: u64,
    pub blue_score: u64,
}

impl KaspaBlockHeader {
    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.blue_score.to_le_bytes());
        format!("{:x}", hasher.finalize())
    }
}

fn main() {
    let header = KaspaBlockHeader {
        version: 1,
        parents: vec!["parent_block_hash_sample".to_string()],
        timestamp: 1700000000,
        blue_score: 85000000,
    };
    println!("Kaspa GHOSTDAG Block Hash: {}", header.calculate_hash());
}
