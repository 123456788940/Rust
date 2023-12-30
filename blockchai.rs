struct Block {
    index: u32,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,

}

impl Block {
    fn new(index: u32, timestamp: u64, data: String, previous_hash: String) -> Block {
        let hash = calculate_hash(index, timestamp, &data, &previous_hash);
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }

    }
}

fn calculate_hash(index: u32, timestamp: u64, data: &str, previous_hash: &str) -> String {
    let input = format!("{}{}{}{}", index, timestamp, data, previous_hash);
    format!("{:x}", md5::compute(input))
}

fn main() {
    let genesis_block = Block::new(0, 0, "Genesis Block".to_string(), "0".to_string());
    let block1 = Block::new(1, 1234567890, "Block 1 Data".to_string(), genesis_block.hash.clone());
    let block2 = Block::new(2, 1234567891, "Block 2 Data".to_string(), block1.hash.clone());

   
    print_block(&genesis_block);
    print_block(&block1);
    print_block(&block2);


}


fn print_block(block: &Block) {
    println!("Index: {}", block.index);

    println!("timestamp: {}", block.timestamp);
    println!("data: {}", block.data);
   
    println!("previous_hash: {}", block.previous_hash);
    println!("hash: {}", block.hash);
    
    
    
    
}
