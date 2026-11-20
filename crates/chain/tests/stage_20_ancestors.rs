use chain::{Block, Hash, Ledger, MemoryStorage, Transaction};
use game::{Action, Direction, PlayerId};

fn tx(sender: u32, nonce: u64, action: Action) -> Transaction {
    Transaction {
        sender: PlayerId(sender),
        nonce,
        action,
    }
}

// Цепочка: начальный блок и три блока поверх него.
fn ledger_with_three_blocks() -> (Ledger<MemoryStorage>, Vec<Hash>) {
    let mut ledger = Ledger::new(MemoryStorage::new()).expect("пустое хранилище годится");
    let mut hashes = vec![ledger.tip()];
    let batches = [
        vec![
            tx(1, 0, Action::Move(Direction::East)),
            tx(2, 0, Action::Wait),
        ],
        vec![tx(2, 1, Action::Move(Direction::West))],
        vec![
            tx(1, 1, Action::Move(Direction::East)),
            tx(1, 2, Action::PickUp),
        ],
    ];
    for batch in batches {
        let (block, rejected) = ledger.build_block(&batch);
        assert_eq!(rejected.len(), 0);
        hashes.push(block.hash());
        ledger.add_block(block).expect("блок верен");
    }
    (ledger, hashes)
}

#[test]
fn ancestors_go_from_block_down_to_genesis() {
    let (ledger, hashes) = ledger_with_three_blocks();
    let seen: Vec<Hash> = ledger.ancestors(ledger.tip()).map(Block::hash).collect();
    assert_eq!(seen, [hashes[3], hashes[2], hashes[1], hashes[0]]);
    let heights: Vec<u64> = ledger
        .ancestors(hashes[2])
        .map(|block| block.header().height)
        .collect();
    assert_eq!(heights, [2, 1, 0]);
}

#[test]
fn ancestors_of_genesis_is_genesis_alone() {
    let (ledger, hashes) = ledger_with_three_blocks();
    assert_eq!(ledger.ancestors(hashes[0]).count(), 1);
}

#[test]
fn ancestors_is_lazy_and_composes_with_adapters() {
    let (ledger, hashes) = ledger_with_three_blocks();
    let newest_two: Vec<Hash> = ledger
        .ancestors(ledger.tip())
        .take(2)
        .map(Block::hash)
        .collect();
    assert_eq!(newest_two, [hashes[3], hashes[2]]);
    let total: usize = ledger
        .ancestors(ledger.tip())
        .map(|block| block.transactions().len())
        .sum();
    assert_eq!(total, 5);
}

#[test]
fn block_is_found_by_height() {
    let (ledger, hashes) = ledger_with_three_blocks();
    assert_eq!(ledger.block_at_height(0).map(Block::hash), Some(hashes[0]));
    assert_eq!(ledger.block_at_height(2).map(Block::hash), Some(hashes[2]));
    assert_eq!(ledger.block_at_height(3).map(Block::hash), Some(hashes[3]));
    assert_eq!(ledger.block_at_height(4), None);
}

#[test]
fn common_ancestor_of_blocks_on_one_line_is_the_lower_one() {
    let (ledger, hashes) = ledger_with_three_blocks();
    assert_eq!(
        ledger.common_ancestor(hashes[3], hashes[1]),
        Some(hashes[1])
    );
    assert_eq!(
        ledger.common_ancestor(hashes[1], hashes[3]),
        Some(hashes[1])
    );
    assert_eq!(
        ledger.common_ancestor(hashes[2], hashes[2]),
        Some(hashes[2])
    );
    assert_eq!(
        ledger.common_ancestor(hashes[2], Hash::from_bytes([9; 32])),
        None
    );
}

#[test]
fn transactions_of_player_go_from_newest_to_oldest() {
    let (ledger, _) = ledger_with_three_blocks();
    let of_first: Vec<u64> = ledger
        .transactions_of(PlayerId(1))
        .map(|tx| tx.nonce)
        .collect();
    assert_eq!(of_first, [2, 1, 0]);
    let of_second: Vec<&Action> = ledger
        .transactions_of(PlayerId(2))
        .map(|tx| &tx.action)
        .collect();
    assert_eq!(of_second, [&Action::Move(Direction::West), &Action::Wait]);
    assert_eq!(ledger.transactions_of(PlayerId(9)).count(), 0);
}
