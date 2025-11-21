use crate::models::block::Block;
//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
struct blockchain {
    chain: Vec<Block>,
    difficulty: usize,

}

impl blockchain{
    pub fn new(difficulty: usize)-> Self{
        let genesis_block = Block::genesis(difficulty);
        let chain = vec![genesis_block]; 

        blockchain{
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

