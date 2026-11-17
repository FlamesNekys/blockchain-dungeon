use game::{
    Action,
    Direction::{self, West},
    GameState, POTION, PlayerId, SWORD, apply, render,
};

const ANNA: PlayerId = PlayerId(1);
const BORIS: PlayerId = PlayerId(2);

/// Партия из разрешённых и запрещённых ходов на начальной карте.
fn script() -> Vec<(PlayerId, Action)> {
    let use_item = |item: &str| Action::UseItem {
        item: String::from(item),
    };
    vec![
        (ANNA, Action::Move(Direction::North)),
        (ANNA, Action::Move(Direction::East)),
        (ANNA, Action::Move(Direction::East)),
        (ANNA, Action::PickUp),
        (ANNA, Action::PickUp),
        (ANNA, use_item(SWORD)),
        (ANNA, use_item(SWORD)),
        (BORIS, Action::Move(West)),
        (BORIS, Action::Move(West)),
        (BORIS, Action::PickUp),
        (BORIS, Action::Attack { target: ANNA }),
        (BORIS, Action::Move(Direction::North)),
        (ANNA, Action::Move(Direction::South)),
        (ANNA, Action::Attack { target: BORIS }),
        (BORIS, Action::Attack { target: ANNA }),
        (BORIS, use_item(POTION)),
        (
            BORIS,
            Action::Give {
                to: ANNA,
                item: String::from(POTION),
            },
        ),
        (PlayerId(3), Action::Wait),
        (ANNA, Action::Attack { target: BORIS }),
        (ANNA, Action::Attack { target: BORIS }),
        (ANNA, Action::Attack { target: BORIS }),
        (ANNA, Action::Attack { target: BORIS }),
        (BORIS, Action::Wait),
        (ANNA, Action::Move(Direction::South)),
    ]
}

#[test]
fn the_same_actions_always_give_the_same_state() {
    let mut first = GameState::genesis();
    let mut second = GameState::genesis();
    for (who, action) in script() {
        let first_result = apply(&mut first, who, action.clone());
        let second_result = apply(&mut second, who, action);
        assert_eq!(first_result, second_result);
    }
    assert_eq!(first, second);
}

#[test]
fn a_refused_action_leaves_the_state_untouched() {
    let mut state = GameState::genesis();
    let mut refused = 0;
    for (who, action) in script() {
        let before = state.clone();
        if apply(&mut state, who, action).is_err() {
            assert_eq!(state, before);
            refused += 1;
        }
    }
    assert_eq!(refused, 8);
}

#[test]
fn the_script_ends_in_the_agreed_position() {
    let mut state = GameState::genesis();
    for (who, action) in script() {
        let _ = apply(&mut state, who, action);
    }
    let expected = "\
#######
#.....#
#.#1#.#
#.....#
#######
";
    assert_eq!(render(&state), expected);
    assert_eq!(state.turn(), 16);
    let anna = state.player(ANNA).expect("player exists");
    assert_eq!(anna.health(), Some(90));
    assert_eq!(anna.inventory.equipped(), Some(SWORD));
    assert!(!state.player(BORIS).expect("player exists").is_alive());
}
