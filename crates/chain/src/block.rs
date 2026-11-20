use crate::{ChainState, Encode, Hash, Transaction};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockHeader {
    pub parent: Hash,
    pub height: u64,
    pub state_root: Hash,
    pub tx_root: Hash,
    pub nonce: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    header: BlockHeader,
    transactions: Vec<Transaction>,
}

impl Encode for BlockHeader {
    fn encode(&self, out: &mut Vec<u8>) {
        self.parent.encode(out);
        self.height.encode(out);
        self.state_root.encode(out);
        self.tx_root.encode(out);
        self.nonce.encode(out);
    }
}

impl Encode for Block {
    fn encode(&self, out: &mut Vec<u8>) {
        self.header.encode(out);
        self.transactions.encode(out);
    }
}

impl Block {
    #[must_use]
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        Self {
            header,
            transactions,
        }
    }

    #[must_use]
    pub fn tx_root(transactions: &[Transaction]) -> Hash {
        transactions.digest()
    }

    #[must_use]
    pub fn header(&self) -> &BlockHeader {
        &self.header
    }

    #[must_use]
    pub fn transactions(&self) -> &[Transaction] {
        &self.transactions
    }

    #[must_use]
    pub fn hash(&self) -> Hash {
        self.header.digest()
    }

    #[must_use]
    pub fn genesis() -> Block {
        let header = BlockHeader {
            parent: Hash::ZERO,
            height: 0,
            state_root: ChainState::genesis().digest(),
            tx_root: Block::tx_root(&[]),
            nonce: 0,
        };
        Block::new(header, Vec::new())
    }
}
