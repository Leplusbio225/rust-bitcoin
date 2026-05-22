use crate::U256;
use uuid::Uuid;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

pub struct BlockHeader {
    pub timestamp: u64,
    pub nonce: u64,
    pub target: U256,
    pub prev_block_hash: [u8; 32],
    pub merkle_root: [u8; 32],
}

pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

pub struct TransactionInput {
    pub prev_transaction_output_hash: [u8; 32],
    pub signature: [u8; 64],
}

pub struct TransactionOutput {
    pub value: u64,
    pub unique_id: Uuid,
    pub pubkey: [u8; 33],
}

impl Blockchain {
    // constructeur - initialise la blockchain avec des données vides
    pub fn new() -> Self {
        Blockchain { blocks: vec![] }
    }

    // Méthode pour ajouter un block à la liste de blocks
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}

impl Block {
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        Block {
            header: header,
            transactions: transactions,
        }
    }

    pub fn hash(&self) -> ! {
        unimplemented!()
    }
}

impl BlockHeader {
    pub fn new(
        timestamp: u64,
        nonce: u64,
        target: U256,
        merkle_root: [u8; 32],
        prev_block_hash: [u8; 32],
    ) -> Self {
        BlockHeader {
            timestamp,
            nonce,
            target,
            merkle_root,
            prev_block_hash,
        }
    }

    pub fn hash(&self) -> ! {
        unimplemented!()
    }
}
