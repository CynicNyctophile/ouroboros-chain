pub struct Block {
    timestamp: i64,
    transactions: Vec<Transaction>,
    pre_block_hash: String,
    hash: String,
    height: usize,
    nonce: i64,
}

impl Block {
    pub fn new_block(pre_block_hash: String, transactions: &[Transaction], height: usize) -> Block {
        let mut block = Block {
            timestamp: crate::current_timestamp(),
            pre_block_hash,
            hash: String::new(),
            transactions: transactions.to_vec(),
            nonce: 0,
            height,
        };
    }
}
