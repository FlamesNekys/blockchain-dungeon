use chain::{ChainState, Encode, Transaction, TxError};
use game::{Action, Direction, Event, GameError, GameState, PlayerId, Position};

fn tx(sender: u32, nonce: u64, action: Action) -> Transaction {
    Transaction {
        sender: PlayerId(sender),
        nonce,
        action,
    }
}

#[test]
fn transaction_has_exact_bytes() {
    let step = tx(1, 0, Action::Move(Direction::East));
    assert_eq!(step.to_bytes(), [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2]);
    assert_eq!(
        step.digest().to_string(),
        "a48b39e7f3b5888ba0a4150b40d10c5a825f0d18907858a4f5cb0d0d5362daae"
    );
}

#[test]
fn genesis_holds_game_genesis_and_no_counters() {
    let state = ChainState::genesis();
    assert_eq!(state.game(), &GameState::genesis());
    assert_eq!(state.next_nonce(PlayerId(1)), 0);
    assert_eq!(state.next_nonce(PlayerId(99)), 0);
    assert_eq!(
        state.digest().to_string(),
        "38180e2c7806b0aa37045b575273c80ae587c4f788467f4762a876fa67485ec7"
    );
}

#[test]
fn applied_transaction_moves_game_and_counter() {
    let mut state = ChainState::genesis();
    let events = state
        .apply(&tx(1, 0, Action::Move(Direction::East)))
        .expect("Ход разрешён");
    assert_eq!(
        events,
        vec![Event::Moved {
            player: PlayerId(1),
            to: Position { x: 2, y: 1 }
        }]
    );
    assert_eq!(state.next_nonce(PlayerId(1)), 1);
    assert_eq!(state.next_nonce(PlayerId(2)), 0);
    assert_eq!(state.game().turn(), 1);
    assert_eq!(
        state.digest().to_string(),
        "7d5135a9b309c8daca8bf7004583348bbe962102e2adb1e2aedd6f95cd7a9c62"
    );
}

#[test]
fn counters_are_separate_for_each_player() {
    let mut state = ChainState::genesis();
    state
        .apply(&tx(1, 0, Action::Wait))
        .expect("первый ход первого игрока");
    state
        .apply(&tx(2, 0, Action::Wait))
        .expect("первый ход второго игрока");
    state
        .apply(&tx(1, 1, Action::Wait))
        .expect("второй ход первого игрока");
    assert_eq!(state.next_nonce(PlayerId(1)), 2);
    assert_eq!(state.next_nonce(PlayerId(2)), 1);
}

#[test]
fn repeated_transaction_is_rejected() {
    let mut state = ChainState::genesis();
    let step = tx(1, 0, Action::Wait);
    state.apply(&step).expect("в первый раз ход проходит");
    let before = state.clone();
    assert_eq!(
        state.apply(&step),
        Err(TxError::BadNonce {
            expected: 1,
            got: 0
        })
    );
    assert_eq!(state, before);
}

#[test]
fn skipped_number_is_rejected() {
    let mut state = ChainState::genesis();
    let before = state.clone();
    assert_eq!(
        state.apply(&tx(1, 1, Action::Wait)),
        Err(TxError::BadNonce {
            expected: 0,
            got: 1
        })
    );
    assert_eq!(state, before);
}

#[test]
fn game_refusal_keeps_state_and_counter() {
    let mut state = ChainState::genesis();
    let before = state.clone();
    let into_wall = tx(1, 0, Action::Move(Direction::North));
    assert_eq!(state.apply(&into_wall), Err(TxError::Game(GameError::Wall)));
    assert_eq!(state, before);
    assert_eq!(state.next_nonce(PlayerId(1)), 0);
}

#[test]
fn unknown_player_is_rejected_by_game_rules() {
    let mut state = ChainState::genesis();
    let stranger = tx(9, 0, Action::Wait);
    assert_eq!(
        state.apply(&stranger),
        Err(TxError::Game(GameError::UnknownPlayer(PlayerId(9))))
    );
    assert_eq!(state.next_nonce(PlayerId(9)), 0);
}

#[test]
fn counters_are_part_of_the_sum() {
    let mut first = ChainState::genesis();
    let mut second = ChainState::genesis();
    first.apply(&tx(1, 0, Action::Wait)).expect("ход разрешён");
    second.apply(&tx(2, 0, Action::Wait)).expect("ход разрешён");
    assert_eq!(first.game().turn(), second.game().turn());
    assert_ne!(first.digest(), second.digest());
}
