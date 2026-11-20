use std::collections::BTreeSet;

use game::{GameState, PlayerId};
use thiserror::Error;

use crate::{
    Block, BlockHeader, ChainError::WrongTxRoot, ChainState, Encode, Hash, Storage, StorageError,
    Transaction, TxError, rules::Rule,
};

#[derive(Debug, PartialEq, Eq, Clone, Error)]
pub enum ChainError {
    #[error("первый блок хранилища не начальный")]
    BadGenesis,
    #[error("неизвестный родитель: {0}")]
    UnknownParent(Hash),
    #[error("not on tip: {tip} {parent}")]
    NotOnTip { tip: Hash, parent: Hash },
    #[error("неверная высота, ожидалось {expected} получил {got}")]
    WrongHeight { expected: u64, got: u64 },
    #[error("неверный корень транзакции")]
    WrongTxRoot,
    #[error("неверный корень состояния")]
    WrongStateRoot,
    #[error("транзакция {index} отклонена: {source}")]
    Transaction { index: usize, source: TxError },
    #[error("{index} {rule} {reason}")]
    Rule {
        index: usize,
        rule: String,
        reason: String,
    },
    #[error("отказ хранилища: {0}")]
    Storage(#[from] StorageError),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rejected {
    pub transaction: Transaction,
    pub reason: ChainError,
}

pub struct Ledger<S: Storage> {
    storage: S,
    state: ChainState,
    tip: Hash,
    height: u64,
    rules: Vec<Box<dyn Rule>>,
}

impl<S: Storage> Ledger<S> {
    /// Создаёт родительский журнал
    ///
    /// # Errors
    /// Возвращает `ChainError` при неверных полях блока
    pub fn new(storage: S) -> Result<Ledger<S>, ChainError> {
        Ledger::with_rules(storage, Vec::new())
    }

    /// Создаёт родительский журнал
    ///
    /// # Errors
    /// Возвращает `ChainError` при неверных полях блока
    pub fn with_rules(storage: S, rules: Vec<Box<dyn Rule>>) -> Result<Ledger<S>, ChainError> {
        let genesis = Block::genesis();
        let mut ledger = Ledger {
            storage,
            state: ChainState::genesis(),
            tip: genesis.hash(),
            height: 0,
            rules,
        };
        match ledger.storage.get_at(0) {
            None => ledger.storage.put(genesis)?,
            Some(first) if *first == genesis => {}
            Some(_) => return Err(ChainError::BadGenesis),
        }
        for block in (1..).map_while(|position| ledger.storage.get_at(position)) {
            let state = ledger.validate(block)?;
            ledger.tip = block.hash();
            ledger.height = block.header().height;
            ledger.state = state;
        }
        Ok(ledger)
    }

    /// Добавляет блок в журнал
    ///
    /// # Errors
    /// Возвращает `ChainError` при неверных полях блока
    pub fn add_block(&mut self, block: Block) -> Result<(), ChainError> {
        let state = self.validate(&block)?;
        let height = block.header().height;
        let tip = block.hash();
        self.storage.put(block)?;
        self.height = height;
        self.tip = tip;
        self.state = state;
        Ok(())
    }

    #[must_use]
    pub fn build_block(&self, candidates: &[Transaction]) -> (Block, Vec<Rejected>) {
        let mut state = self.state().clone();
        let mut included: Vec<Transaction> = Vec::new();
        let mut rejected = Vec::new();
        for tx in candidates {
            let index = included.len();
            let applied = self
                .check_rules(state.game(), &included, index, tx)
                .and_then(|()| {
                    state
                        .apply(tx)
                        .map_err(|source| ChainError::Transaction { index, source })
                });
            match applied {
                Ok(_) => included.push(tx.clone()),
                Err(err) => {
                    rejected.push(Rejected {
                        transaction: tx.clone(),
                        reason: err,
                    });
                }
            }
        }
        let header = BlockHeader {
            parent: self.tip(),
            height: self.height().saturating_add(1),
            state_root: state.digest(),
            tx_root: Block::tx_root(&included),
            nonce: 0,
        };
        (Block::new(header, included), rejected)
    }

    #[must_use]
    pub fn tip(&self) -> Hash {
        self.tip
    }

    #[must_use]
    pub fn height(&self) -> u64 {
        self.height
    }

    #[must_use]
    pub fn state(&self) -> &ChainState {
        &self.state
    }

    #[must_use]
    pub fn storage(&self) -> &S {
        &self.storage
    }

    fn check_rules(
        &self,
        game: &GameState,
        earlier: &[Transaction],
        index: usize,
        tx: &Transaction,
    ) -> Result<(), ChainError> {
        for rule in &self.rules {
            rule.check(game, earlier, tx)
                .map_err(|err| ChainError::Rule {
                    index,
                    rule: rule.name().to_owned(),
                    reason: err,
                })?;
        }
        Ok(())
    }

    /// Проверяет поля блока
    ///
    /// # Errors
    /// Возвращает `ChainError` при неверных полях блока
    pub fn validate(&self, block: &Block) -> Result<ChainState, ChainError> {
        let header = block.header();
        if header.parent != self.tip {
            if self.storage.get(&header.parent).is_some() {
                return Err(ChainError::NotOnTip {
                    tip: self.tip,
                    parent: header.parent,
                });
            }
            return Err(ChainError::UnknownParent(header.parent));
        }
        let expected_height = self.height.checked_add(1);
        if expected_height != Some(header.height) {
            return Err(ChainError::WrongHeight {
                expected: self.height + 1,
                got: header.height,
            });
        }
        if Block::tx_root(block.transactions()) != header.tx_root {
            return Err(WrongTxRoot);
        }
        let mut state = self.state.clone();
        for (index, tx) in block.transactions().iter().enumerate() {
            self.check_rules(
                state.game(),
                block.transactions().get(..index).unwrap_or_default(),
                index,
                tx,
            )?;
            state
                .apply(tx)
                .map_err(|source| ChainError::Transaction { index, source })?;
        }
        if state.digest() != header.state_root {
            return Err(ChainError::WrongStateRoot);
        }
        Ok(state)
    }

    #[must_use]
    pub fn ancestors(&self, from: Hash) -> Ancestors<'_, S> {
        Ancestors {
            storage: &self.storage,
            next: Some(from),
        }
    }

    #[must_use]
    pub fn block_at_height(&self, height: u64) -> Option<&Block> {
        self.ancestors(self.tip)
            .find(|block| block.header().height == height)
    }

    #[must_use]
    pub fn common_ancestor(&self, first: Hash, second: Hash) -> Option<Hash> {
        let seen: BTreeSet<Hash> = self.ancestors(first).map(Block::hash).collect();
        self.ancestors(second)
            .map(Block::hash)
            .find(|hash| seen.contains(hash))
    }

    pub fn transactions_of(&self, player: PlayerId) -> impl Iterator<Item = &Transaction> {
        self.ancestors(self.tip)
            .flat_map(|block| block.transactions().iter().rev())
            .filter(move |tx| tx.sender == player)
    }
}

pub struct Ancestors<'a, S: Storage> {
    storage: &'a S,
    next: Option<Hash>,
}

impl<'a, S: Storage> Iterator for Ancestors<'a, S> {
    type Item = &'a Block;

    fn next(&mut self) -> Option<Self::Item> {
        let hash = self.next.take()?;
        let block = self.storage.get(&hash)?;
        if block.header().height != 0 {
            self.next = Some(block.header().parent);
        }
        Some(block)
    }
}
