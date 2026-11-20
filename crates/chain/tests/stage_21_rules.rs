use chain::{ChainError, Ledger, MaxActionsPerPlayer, MemoryStorage, NoWaiting, Rule, Transaction};
use game::{Action, Direction, GameState, PlayerId};

fn tx(sender: u32, nonce: u64, action: Action) -> Transaction {
    Transaction {
        sender: PlayerId(sender),
        nonce,
        action,
    }
}

#[test]
fn limit_rule_counts_earlier_transactions_of_same_player() {
    let rule = MaxActionsPerPlayer { limit: 2 };
    let state = GameState::genesis();
    let earlier = [
        tx(1, 0, Action::Wait),
        tx(2, 0, Action::Wait),
        tx(1, 1, Action::Wait),
    ];
    assert_eq!(rule.name(), "max-actions-per-player");
    assert!(
        rule.check(&state, &earlier[..1], &tx(1, 1, Action::Wait))
            .is_ok()
    );
    assert!(
        rule.check(&state, &earlier, &tx(1, 2, Action::Wait))
            .is_err()
    );
    assert!(
        rule.check(&state, &earlier, &tx(2, 1, Action::Wait))
            .is_ok()
    );
}

#[test]
fn no_waiting_rule_refuses_only_waiting() {
    let rule = NoWaiting;
    let state = GameState::genesis();
    assert_eq!(rule.name(), "no-waiting");
    assert!(rule.check(&state, &[], &tx(1, 0, Action::Wait)).is_err());
    assert!(rule.check(&state, &[], &tx(1, 0, Action::PickUp)).is_ok());
}

#[test]
fn ledger_without_rules_accepts_everything_valid() {
    let ledger = Ledger::new(MemoryStorage::new()).expect("пустое хранилище годится");
    let (block, rejected) = ledger.build_block(&[
        tx(1, 0, Action::Wait),
        tx(1, 1, Action::Wait),
        tx(1, 2, Action::Wait),
    ]);
    assert_eq!(rejected.len(), 0);
    assert_eq!(block.transactions().len(), 3);
}

#[test]
fn build_block_respects_rules() {
    let rules: Vec<Box<dyn Rule>> = vec![
        Box::new(MaxActionsPerPlayer { limit: 1 }),
        Box::new(NoWaiting),
    ];
    let ledger = Ledger::with_rules(MemoryStorage::new(), rules).expect("пустое хранилище годится");
    let candidates = [
        tx(1, 0, Action::Move(Direction::East)),
        tx(1, 1, Action::Move(Direction::East)),
        tx(2, 0, Action::Wait),
        tx(2, 0, Action::Move(Direction::West)),
    ];
    let (block, rejected) = ledger.build_block(&candidates);
    assert_eq!(
        block.transactions(),
        [candidates[0].clone(), candidates[3].clone()]
    );
    assert_eq!(rejected.len(), 2);
    assert!(
        matches!(&rejected[0].reason, ChainError::Rule { index: 1, rule, .. } if rule == "max-actions-per-player")
    );
    assert!(
        matches!(&rejected[1].reason, ChainError::Rule { index: 1, rule, .. } if rule == "no-waiting")
    );
}

#[test]
fn block_breaking_a_rule_is_rejected() {
    // Блок собирает журнал без правил, а принимает журнал с правилом.
    let lenient = Ledger::new(MemoryStorage::new()).expect("пустое хранилище годится");
    let (block, _) = lenient.build_block(&[
        tx(1, 0, Action::Move(Direction::East)),
        tx(2, 0, Action::Wait),
    ]);

    let rules: Vec<Box<dyn Rule>> = vec![Box::new(NoWaiting)];
    let mut strict =
        Ledger::with_rules(MemoryStorage::new(), rules).expect("пустое хранилище годится");
    let outcome = strict.add_block(block);
    assert!(
        matches!(outcome, Err(ChainError::Rule { index: 1, ref rule, .. }) if rule == "no-waiting")
    );
    assert_eq!(strict.height(), 0);
}

// Правило можно написать и вне пакета chain: журнал о нём ничего не знает заранее.
struct OnlyPlayer(PlayerId);

impl Rule for OnlyPlayer {
    fn name(&self) -> &'static str {
        "only-player"
    }

    fn check(
        &self,
        _state: &GameState,
        _earlier: &[Transaction],
        tx: &Transaction,
    ) -> Result<(), String> {
        if tx.sender == self.0 {
            Ok(())
        } else {
            Err(format!("ходит только игрок {}", self.0))
        }
    }
}

#[test]
fn rule_from_another_package_plugs_in() {
    let rules: Vec<Box<dyn Rule>> = vec![Box::new(OnlyPlayer(PlayerId(2)))];
    let ledger = Ledger::with_rules(MemoryStorage::new(), rules).expect("пустое хранилище годится");
    let (block, rejected) = ledger.build_block(&[tx(1, 0, Action::Wait), tx(2, 0, Action::Wait)]);
    assert_eq!(block.transactions().len(), 1);
    assert_eq!(rejected.len(), 1);
    assert!(
        matches!(&rejected[0].reason, ChainError::Rule { index: 0, rule, reason } if rule == "only-player" && reason.contains("#2"))
    );
}
