pub struct Block {
    timestamp: i64,
    pre_block_hash: String,
    hash: String,
    transactions: Vec<Transaction>,
    nonce: i64,
    height: usize,
}

pub struct ProofOfWork {
    block: Block,
    target: BigInt,
}
