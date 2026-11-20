use chain::{Block, BlockHeader, CountingStorage, Hash, MemoryStorage, Storage, StorageError};

fn block(height: u64) -> Block {
    let header = BlockHeader {
        parent: Hash::ZERO,
        height,
        state_root: Hash::ZERO,
        tx_root: Block::tx_root(&[]),
        nonce: 0,
    };
    Block::new(header, Vec::new())
}

fn check_any_storage<S: Storage>(mut storage: S) -> S {
    assert_eq!(storage.len(), 0);
    assert_eq!(storage.len(), 0);
    assert_eq!(storage.get_at(0), None);

    let first = block(1);
    let second = block(2);
    storage.put(first.clone()).expect("первый блок принимается");
    storage
        .put(second.clone())
        .expect("второй блок принимается");

    assert!(!storage.is_empty());
    assert_eq!(storage.len(), 2);
    assert_eq!(storage.get(&first.hash()), Some(&first));
    assert_eq!(storage.get(&second.hash()), Some(&second));
    assert_eq!(storage.get(&Hash::ZERO), None);
    assert_eq!(storage.get_at(0), Some(&first));
    assert_eq!(storage.get_at(1), Some(&second));
    assert_eq!(storage.get_at(2), None);

    assert_eq!(
        storage.put(first.clone()),
        Err(StorageError::Duplicate(first.hash()))
    );
    assert_eq!(storage.len(), 2);
    storage
}

#[test]
fn memory_storage_keeps_blocks() {
    check_any_storage(MemoryStorage::new());
}

#[test]
fn counting_storage_behaves_like_inner_one() {
    check_any_storage(CountingStorage::new(MemoryStorage::new()));
}

#[test]
fn counting_storage_counts_only_successful_puts() {
    let storage = check_any_storage(CountingStorage::new(MemoryStorage::new()));
    assert_eq!(storage.puts(), 2);
    let inner: MemoryStorage = storage.into_inner();
    assert_eq!(inner.len(), 2);
}

#[test]
fn wrappers_can_be_nested() {
    let storage = check_any_storage(CountingStorage::new(CountingStorage::new(
        MemoryStorage::new(),
    )));
    assert_eq!(storage.puts(), 2);
    assert_eq!(storage.into_inner().puts(), 2);
}

#[test]
fn storage_works_as_trait_object() {
    let mut storages: Vec<Box<dyn Storage>> = vec![
        Box::new(MemoryStorage::new()),
        Box::new(CountingStorage::new(MemoryStorage::new())),
    ];
    for storage in &mut storages {
        storage.put(block(1)).expect("блок принимается");
        assert_eq!(storage.len(), 1);
    }
}
