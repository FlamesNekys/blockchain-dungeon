use chain::{
    Block, BlockHeader, ChainError, ChainState, Encode, Hash, Ledger, MemoryStorage, Storage,
    Transaction, TxError,
};
use game::{Action, Direction, GameError, PlayerId, Position};

fn tx(sender: u32, nonce: u64, action: Action) -> Transaction {
    Transaction {
        sender: PlayerId(sender),
        nonce,
        action,
    }
}

fn new_ledger() -> Ledger<MemoryStorage> {
    Ledger::new(MemoryStorage::new()).expect("пустое хранилище годится")
}

// Собирает блок вручную, чтобы в него можно было положить и неверные сведения.
fn block_on(
    ledger: &Ledger<MemoryStorage>,
    transactions: Vec<Transaction>,
) -> (BlockHeader, Vec<Transaction>) {
    let mut state = ledger.state().clone();
    for transaction in &transactions {
        state
            .apply(transaction)
            .expect("в этих тестах транзакции заготовки верны");
    }
    let header = BlockHeader {
        parent: ledger.tip(),
        height: ledger.height() + 1,
        state_root: state.digest(),
        tx_root: Block::tx_root(&transactions),
        nonce: 0,
    };
    (header, transactions)
}

#[test]
fn new_ledger_starts_with_genesis_block() {
    let ledger = new_ledger();
    assert_eq!(ledger.tip(), Block::genesis().hash());
    assert_eq!(ledger.height(), 0);
    assert_eq!(ledger.state(), &ChainState::genesis());
    assert_eq!(ledger.storage().len(), 1);
    assert_eq!(ledger.storage().get_at(0), Some(&Block::genesis()));
}

#[test]
fn built_block_is_accepted() {
    let mut ledger = new_ledger();
    let (block, rejected) = ledger.build_block(&[
        tx(1, 0, Action::Move(Direction::East)),
        tx(2, 0, Action::Wait),
    ]);
    assert_eq!(rejected.len(), 0);
    assert_eq!(block.header().parent, Block::genesis().hash());
    assert_eq!(block.header().height, 1);
    assert_eq!(block.transactions().len(), 2);
    assert_eq!(ledger.height(), 0, "сборка блока не меняет журнал");

    let hash = block.hash();
    ledger.add_block(block).expect("собранный блок верен");
    assert_eq!(ledger.tip(), hash);
    assert_eq!(ledger.height(), 1);
    assert_eq!(ledger.storage().len(), 2);
    let anna = ledger
        .state()
        .game()
        .player(PlayerId(1))
        .expect("игрок есть");
    assert_eq!(anna.position, Position { x: 2, y: 1 });
    assert_eq!(ledger.state().next_nonce(PlayerId(1)), 1);
}

#[test]
fn build_block_leaves_out_bad_transactions() {
    let ledger = new_ledger();
    let into_wall = tx(1, 0, Action::Move(Direction::North));
    let good = tx(1, 0, Action::Move(Direction::East));
    let too_early = tx(2, 5, Action::Wait);
    let (block, rejected) =
        ledger.build_block(&[into_wall.clone(), good.clone(), too_early.clone()]);
    assert_eq!(block.transactions(), [good]);
    assert_eq!(rejected.len(), 2);
    assert_eq!(rejected[0].transaction, into_wall);
    assert_eq!(
        rejected[0].reason,
        ChainError::Transaction {
            index: 0,
            source: TxError::Game(GameError::Wall)
        }
    );
    assert_eq!(
        rejected[1].reason,
        ChainError::Transaction {
            index: 1,
            source: TxError::BadNonce {
                expected: 0,
                got: 5
            }
        }
    );
}

#[test]
fn empty_block_is_allowed() {
    let mut ledger = new_ledger();
    let (block, _) = ledger.build_block(&[]);
    ledger.add_block(block).expect("блок без транзакций верен");
    assert_eq!(ledger.height(), 1);
    assert_eq!(ledger.state(), &ChainState::genesis());
}

#[test]
fn block_with_unknown_parent_is_rejected() {
    let mut ledger = new_ledger();
    let (mut header, transactions) = block_on(&ledger, Vec::new());
    header.parent = Hash::from_bytes([7; 32]);
    let outcome = ledger.add_block(Block::new(header, transactions));
    assert_eq!(
        outcome,
        Err(ChainError::UnknownParent(Hash::from_bytes([7; 32])))
    );
    assert_eq!(ledger.height(), 0);
}

#[test]
fn block_on_old_parent_is_rejected() {
    let mut ledger = new_ledger();
    let genesis = ledger.tip();
    let (first, _) = ledger.build_block(&[tx(1, 0, Action::Wait)]);
    let (rival, _) = ledger.build_block(&[tx(2, 0, Action::Wait)]);
    let tip = first.hash();
    ledger.add_block(first).expect("первый блок верен");
    assert_eq!(
        ledger.add_block(rival),
        Err(ChainError::NotOnTip {
            tip,
            parent: genesis
        })
    );
}

#[test]
fn block_with_wrong_height_is_rejected() {
    let mut ledger = new_ledger();
    let (mut header, transactions) = block_on(&ledger, Vec::new());
    header.height = 2;
    let outcome = ledger.add_block(Block::new(header, transactions));
    assert_eq!(
        outcome,
        Err(ChainError::WrongHeight {
            expected: 1,
            got: 2
        })
    );
}

#[test]
fn changed_transaction_is_noticed() {
    let mut ledger = new_ledger();
    let (header, _) = block_on(&ledger, vec![tx(1, 0, Action::Move(Direction::East))]);
    // Заголовок от одного списка транзакций, а в блоке лежит другой.
    let forged = vec![tx(1, 0, Action::Move(Direction::South))];
    assert_eq!(
        ledger.add_block(Block::new(header, forged)),
        Err(ChainError::WrongTxRoot)
    );
}

#[test]
fn wrong_state_root_is_rejected() {
    let mut ledger = new_ledger();
    let (mut header, transactions) = block_on(&ledger, vec![tx(1, 0, Action::Wait)]);
    header.state_root = Hash::ZERO;
    assert_eq!(
        ledger.add_block(Block::new(header, transactions)),
        Err(ChainError::WrongStateRoot)
    );
    assert_eq!(ledger.state(), &ChainState::genesis());
}

#[test]
fn block_with_refused_transaction_is_rejected_whole() {
    let mut ledger = new_ledger();
    let transactions = vec![
        tx(2, 0, Action::Wait),
        tx(1, 0, Action::Move(Direction::North)),
    ];
    let header = BlockHeader {
        parent: ledger.tip(),
        height: 1,
        state_root: Hash::ZERO,
        tx_root: Block::tx_root(&transactions),
        nonce: 0,
    };
    let outcome = ledger.add_block(Block::new(header, transactions));
    assert_eq!(
        outcome,
        Err(ChainError::Transaction {
            index: 1,
            source: TxError::Game(GameError::Wall)
        })
    );
    assert_eq!(ledger.state(), &ChainState::genesis());
    assert_eq!(ledger.storage().len(), 1);
}

#[test]
fn reopened_storage_gives_the_same_ledger() {
    let mut ledger = new_ledger();
    for nonce in 0..3 {
        let (block, _) = ledger.build_block(&[tx(1, nonce, Action::Wait)]);
        ledger.add_block(block).expect("блок верен");
    }
    let mut copy = MemoryStorage::new();
    for position in 0..ledger.storage().len() {
        let block = ledger
            .storage()
            .get_at(position)
            .expect("блок с таким номером есть");
        copy.put(block.clone()).expect("в копии блока ещё нет");
    }
    let reopened = Ledger::new(copy).expect("цепочка в хранилище верна");
    assert_eq!(reopened.tip(), ledger.tip());
    assert_eq!(reopened.height(), 3);
    assert_eq!(reopened.state(), ledger.state());
}

#[test]
fn storage_with_foreign_first_block_is_rejected() {
    let header = BlockHeader {
        parent: Hash::ZERO,
        height: 0,
        state_root: Hash::ZERO,
        tx_root: Block::tx_root(&[]),
        nonce: 0,
    };
    let mut storage = MemoryStorage::new();
    storage
        .put(Block::new(header, Vec::new()))
        .expect("хранилище пустое");
    assert!(matches!(Ledger::new(storage), Err(ChainError::BadGenesis)));
}
