use crate::models::block::Block;
//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
struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,

}

impl Blockchain{
    pub fn new(difficulty: usize)-> Self{
        let genesis_block = Block::genesis(difficulty);
        let chain = vec![genesis_block]; 

        Blockchain{
            chain,
            difficulty,
        }
    }

    pub fn add_block(&mut self, block_data: String){
        // Get the last block hash
        let last_block = self.chain.last().expect("The chain does not have a genesis block");
        let last_hash = last_block.get_hash().clone();
        
        // Create and mine the new block
        let mut new_block = Block::new(block_data.to_string(), last_hash);
        new_block.mine(self.difficulty);

        // Add the new block to the chain
        self.chain.push(new_block);
    
    }
}


#[cfg(test)]
mod tests{
    use super::Blockchain;

    #[test]
    fn test_add_block(){
        // Create a new blockchain
        let mut blockchain = Blockchain::new(12);
        
        // Add a new block to the blockchain
        blockchain.add_block("Test adding a new block".to_string());

        // Print the block string
        let last_block =  blockchain.chain.last().expect("chain is empty");
        println!("Last lock: {:?}", last_block);

    }
}
