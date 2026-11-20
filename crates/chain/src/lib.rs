//! Цепочка блоков для игры «Blockchain Dungeon».

mod block;
mod encode;
mod hash;
mod ledger;
mod rules;
mod state;
mod storage;

pub use block::{Block, BlockHeader};
pub use encode::Encode;
pub use hash::{Hash, WrongHashLength};
pub use ledger::{Ancestors, ChainError, Ledger, Rejected};
pub use rules::{MaxActionsPerPlayer, NoWaiting, Rule};
pub use state::{ChainState, Transaction, TxError};
pub use storage::{CountingStorage, MemoryStorage, Storage, StorageError};
