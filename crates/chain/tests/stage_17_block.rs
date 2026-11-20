use chain::{Block, BlockHeader, ChainState, Encode, Hash, Transaction};
use game::{Action, Direction, PlayerId};

fn step() -> Transaction {
    Transaction {
        sender: PlayerId(1),
        nonce: 0,
        action: Action::Move(Direction::East),
    }
}

fn header() -> BlockHeader {
    BlockHeader {
        parent: Hash::from_bytes([1; 32]),
        height: 5,
        state_root: Hash::from_bytes([2; 32]),
        tx_root: Hash::from_bytes([3; 32]),
        nonce: 9,
    }
}

#[test]
fn header_is_five_fields_in_order() {
    let bytes = header().to_bytes();
    assert_eq!(bytes.len(), 32 + 8 + 32 + 32 + 8);
    assert_eq!(bytes[..32], [1; 32]);
    assert_eq!(bytes[32..40], [5, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(bytes[40..72], [2; 32]);
    assert_eq!(bytes[72..104], [3; 32]);
    assert_eq!(bytes[104..], [9, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn block_hash_is_header_sum() {
    let block = Block::new(header(), vec![step()]);
    assert_eq!(block.hash(), header().digest());
    assert_eq!(block.header(), &header());
    assert_eq!(block.transactions(), [step()]);
}

#[test]
fn every_header_field_changes_block_hash() {
    let base = Block::new(header(), Vec::new()).hash();
    let mut changed = header();
    changed.parent = Hash::ZERO;
    assert_ne!(Block::new(changed, Vec::new()).hash(), base);
    let mut changed = header();
    changed.height = 6;
    assert_ne!(Block::new(changed, Vec::new()).hash(), base);
    let mut changed = header();
    changed.state_root = Hash::ZERO;
    assert_ne!(Block::new(changed, Vec::new()).hash(), base);
    let mut changed = header();
    changed.tx_root = Hash::ZERO;
    assert_ne!(Block::new(changed, Vec::new()).hash(), base);
    let mut changed = header();
    changed.nonce = 10;
    assert_ne!(Block::new(changed, Vec::new()).hash(), base);
}

#[test]
fn tx_root_is_sum_of_encoded_list() {
    assert_eq!(
        Block::tx_root(&[]).to_string(),
        "df3f619804a92fdb4057192dc43dd748ea778adc52bc498ce80524c014b81119"
    );
    let one = Block::tx_root(&[step()]);
    assert_eq!(one, vec![step()].digest());
    let mut other = step();
    other.nonce = 1;
    assert_ne!(Block::tx_root(&[other]), one);
}

#[test]
fn block_bytes_are_header_then_transactions() {
    let block = Block::new(header(), vec![step()]);
    let mut expected = header().to_bytes();
    vec![step()].encode(&mut expected);
    assert_eq!(block.to_bytes(), expected);
}

#[test]
fn genesis_block_is_the_same_for_everyone() {
    let genesis = Block::genesis();
    assert_eq!(genesis.header().parent, Hash::ZERO);
    assert_eq!(genesis.header().height, 0);
    assert_eq!(genesis.header().state_root, ChainState::genesis().digest());
    assert_eq!(genesis.header().tx_root, Block::tx_root(&[]));
    assert_eq!(genesis.header().nonce, 0);
    assert_eq!(genesis.transactions().len(), 0);
    assert_eq!(
        genesis.hash().to_string(),
        "07980648143f1e6eab0bd4838dd5a9d05c31cbd6579af0c391f6cc6eda9cf0ec"
    );
}
