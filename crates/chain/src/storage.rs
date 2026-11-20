use std::collections::BTreeMap;

use crate::{Block, Hash};

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum StorageError {
    #[error("блок {0} уже лежит в хранилище")]
    Duplicate(Hash),
}

pub trait Storage {
    /// Кладёт блок в хранилище
    ///
    /// # Errors
    ///
    /// Возвращает `StorageError` такой блок уже есть в хранилище
    fn put(&mut self, block: Block) -> Result<(), StorageError>;
    fn get(&self, hash: &Hash) -> Option<&Block>;
    fn get_at(&self, position: usize) -> Option<&Block>;
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct MemoryStorage {
    blocks: Vec<Block>,
    positions: BTreeMap<Hash, usize>,
}

impl Storage for MemoryStorage {
    fn put(&mut self, block: Block) -> Result<(), StorageError> {
        let hash = block.hash();
        if self.get(&hash).is_some() {
            return Err(StorageError::Duplicate(hash));
        }
        self.positions.insert(hash, self.len());
        self.blocks.push(block);
        Ok(())
    }

    fn get(&self, hash: &Hash) -> Option<&Block> {
        let pos = self.positions.get(hash)?;
        self.blocks.get(*pos)
    }

    fn get_at(&self, position: usize) -> Option<&Block> {
        self.blocks.get(position)
    }

    fn len(&self) -> usize {
        self.blocks.len()
    }
}

pub struct CountingStorage<S: Storage> {
    inner: S,
    puts: u64,
}

impl<S: Storage> Storage for CountingStorage<S> {
    fn put(&mut self, block: Block) -> Result<(), StorageError> {
        self.inner.put(block)?;
        self.puts = self.puts.saturating_add(1);
        Ok(())
    }

    fn get(&self, hash: &Hash) -> Option<&Block> {
        self.inner.get(hash)
    }

    fn get_at(&self, position: usize) -> Option<&Block> {
        self.inner.get_at(position)
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl MemoryStorage {
    #[must_use]
    pub fn new() -> MemoryStorage {
        MemoryStorage::default()
    }
}

impl<S: Storage> CountingStorage<S> {
    pub fn new(inner: S) -> CountingStorage<S> {
        CountingStorage { inner, puts: 0 }
    }

    pub fn puts(&self) -> u64 {
        self.puts
    }

    pub fn into_inner(self) -> S {
        self.inner
    }
}
