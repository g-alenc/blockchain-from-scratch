use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Block {
    data: String,
    nonce: u64,
    previous_hash: [u8; 32],
    datestamp: u128,

    #[serde(skip)] // Skip the hash value for serialization
    hash: [u8; 32],
}

impl Block{
    pub fn new(data: String, previous_hash: [u8; 32]) -> Self{
        //Calculate the datestamp
        let now = SystemTime::now();
        let datestamp = now.duration_since(UNIX_EPOCH).unwrap().as_millis();

        //Initialize nonce to zero
        let nonce = 0;

        //Create and return the new Block
        Block {
            data,
            nonce,
            previous_hash,
            datestamp,
            hash : [0; 32], // Initialize the hash empty
        }
    }

    fn calculate_hash(&self) -> [u8; 32]{
        //Serialize the block data
        let serialized_block = serde_json::to_string(self).unwrap();
        
        // Creates a Sha256 object
        let mut hasher = Sha256::new();

        // Calculate the hash
        hasher.update(serialized_block.as_bytes());
        let hash = hasher.finalize();

        return hash.into()
    }

    pub fn mine(&mut self, difficulty: usize){
        // println!("Initializing mining of the block with difficulty {}", difficulty);
        
        loop{
            // Calculate the Hash
            let hash = self.calculate_hash();
            // Checks if the hash wins the difficulty
            let is_valid = check_difficulty(&hash, difficulty);
            
            if is_valid{
                self.hash = hash;
                // println!("Mining completed:\nhash: {:?}\nNonce: {}", hash, self.nonce);

                break
            }

            // Updates the nonce
            self.nonce += 1;
        }

    }

    // Return the first (mined) block of the blockchain
    pub fn genesis(difficulty: usize) -> Block{
        let mut block = Block::new("First block".to_string(), [0; 32]);
        block.mine(difficulty);

        return block
    }

    pub fn get_hash(&self)-> [u8; 32]{
        return self.hash;
    }

}

pub fn check_difficulty(hash: &[u8], difficulty: usize)-> bool{

    // Check if the difficulty is smaller then the hash len
    if difficulty > hash.len() * 8 {
            return false;
        }

    let full_bytes = difficulty / 8;
    let remaining_bits = difficulty % 8;

    // Check the full bytes
    for i in 0..full_bytes{
        if hash[i] != 0{
            return false
        }
    }

    // Check the remaining bits
    if remaining_bits > 0{
        // Acess the last byte
        let current_byte = hash[full_bytes];

        // Check just the remaining bits of the last byte
        if (current_byte >> (8 - remaining_bits)) != 0{
            return false
        }
    }

    return true
}

#[cfg(test)]
mod tests {
    // import the Block struct from parent module 
    use super::Block; 
    use serde_json;

    #[test]
    fn test_block_serialization(){
        // Create a sample Block
        let block = Block::new("Test Data".to_string(), [0; 32]);

        // Serialize the Block
        let serialized = serde_json::to_string(&block).unwrap();
        assert!(serialized.len() > 0);
        println!("Serialized Block: {}", serialized);

        //deserialize the Block
        let deserialized: Block = serde_json::from_str(&serialized).unwrap();

        // Assert equality
        assert_eq!(block, deserialized);
    }

    //TODO: Implement the test for mine fn
    #[test]
    fn test_block_mine(){
        let mut block = Block::new("Test Data".to_string(), [0; 32]);

        block.mine(20);

    }
}