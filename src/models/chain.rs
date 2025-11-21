use crate::models::block::Block;

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

    }
}

