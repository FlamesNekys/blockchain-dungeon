use game::{
    Action, BASE_DAMAGE, Direction, Event, GameError, GameState, POTION, POTION_HEAL, PlayerId,
    PlayerState, Position, START_HEALTH, SWORD, SWORD_DAMAGE, Tile, World, apply,
};

const ANNA: PlayerId = PlayerId(1);
const BORIS: PlayerId = PlayerId(2);

/// Три клетки в ряд: Анна на сундуке с мечом, Борис рядом, третья клетка свободна.
fn duel() -> GameState {
    let mut world = World::new(3, 1);
    world.set_tile(Position { x: 0, y: 0 }, Tile::Chest(String::from(SWORD)));
    let mut state = GameState::new(world);
    state
        .add_player(ANNA, "Anna", Position { x: 0, y: 0 })
        .expect("cell is free");
    state
        .add_player(BORIS, "Boris", Position { x: 1, y: 0 })
        .expect("cell is free");
    state
}

fn attack(target: PlayerId) -> Action {
    Action::Attack { target }
}

fn kill_boris(state: &mut GameState) {
    for _ in 0..START_HEALTH / BASE_DAMAGE {
        apply(state, ANNA, attack(BORIS)).expect("target is near and alive");
    }
}

#[test]
fn bare_hands_deal_base_damage() {
    let mut state = duel();
    let events = apply(&mut state, ANNA, attack(BORIS));
    let left = START_HEALTH - BASE_DAMAGE;
    assert_eq!(
        events,
        Ok(vec![Event::Damaged {
            player: BORIS,
            by: ANNA,
            amount: BASE_DAMAGE,
            health: left
        }])
    );
    assert_eq!(
        state.player(BORIS).and_then(game::Player::health),
        Some(left)
    );
}

#[test]
fn sword_in_hand_deals_sword_damage() {
    let mut state = duel();
    apply(&mut state, ANNA, Action::PickUp).expect("chest is here");
    apply(
        &mut state,
        ANNA,
        Action::UseItem {
            item: String::from(SWORD),
        },
    )
    .expect("item is in the list");
    let events = apply(&mut state, ANNA, attack(BORIS));
    let left = START_HEALTH - SWORD_DAMAGE;
    assert_eq!(
        events,
        Ok(vec![Event::Damaged {
            player: BORIS,
            by: ANNA,
            amount: SWORD_DAMAGE,
            health: left
        }])
    );
}

#[test]
fn sword_in_the_list_but_not_in_hand_does_not_affect() {
    let mut state = duel();
    apply(&mut state, ANNA, Action::PickUp).expect("chest is here");
    apply(&mut state, ANNA, attack(BORIS)).expect("target is near");
    assert_eq!(
        state.player(BORIS).and_then(game::Player::health),
        Some(START_HEALTH - BASE_DAMAGE)
    );
}

#[test]
fn the_last_blow_kills_and_records_the_turn() {
    let mut state = duel();
    for _ in 0..START_HEALTH / BASE_DAMAGE - 1 {
        apply(&mut state, ANNA, attack(BORIS)).expect("target is near");
    }
    let turn_of_the_last_blow = state.turn();
    let events = apply(&mut state, ANNA, attack(BORIS));
    assert_eq!(
        events,
        Ok(vec![
            Event::Damaged {
                player: BORIS,
                by: ANNA,
                amount: BASE_DAMAGE,
                health: 0
            },
            Event::Died { player: BORIS }
        ])
    );
    let boris = state.player(BORIS).expect("dead players stay in the state");
    assert_eq!(
        boris.state,
        PlayerState::Dead {
            died_at_turn: turn_of_the_last_blow
        }
    );
    assert_eq!(state.turn(), turn_of_the_last_blow + 1);
}

#[test]
fn the_dead_do_not_act() {
    let mut state = duel();
    kill_boris(&mut state);
    let before = state.clone();
    for action in [
        Action::Wait,
        Action::Move(Direction::East),
        Action::PickUp,
        attack(ANNA),
    ] {
        assert_eq!(
            apply(&mut state, BORIS, action),
            Err(GameError::PlayerDead(BORIS))
        );
    }
    assert_eq!(state, before);
}

#[test]
fn the_dead_cannot_be_attacked_or_given_items() {
    let mut state = duel();
    apply(&mut state, ANNA, Action::PickUp).expect("chest is here");
    kill_boris(&mut state);
    let before = state.clone();
    assert_eq!(
        apply(&mut state, ANNA, attack(BORIS)),
        Err(GameError::PlayerDead(BORIS))
    );
    let gift = Action::Give {
        to: BORIS,
        item: String::from(SWORD),
    };
    assert_eq!(
        apply(&mut state, ANNA, gift),
        Err(GameError::PlayerDead(BORIS))
    );
    assert_eq!(state, before);
}

#[test]
fn the_dead_do_not_block_the_cell_and_are_not_drawn() {
    let mut state = duel();
    kill_boris(&mut state);
    assert_eq!(game::render(&state), "1..\n");
    let events = apply(&mut state, ANNA, Action::Move(Direction::East));
    assert_eq!(
        events,
        Ok(vec![Event::Moved {
            player: ANNA,
            to: Position { x: 1, y: 0 }
        }])
    );
}

#[test]
fn attack_is_refused_for_every_broken_rule() {
    let mut state = duel();
    let before = state.clone();
    assert_eq!(
        apply(&mut state, ANNA, attack(ANNA)),
        Err(GameError::SelfTarget)
    );
    assert_eq!(
        apply(&mut state, ANNA, attack(PlayerId(9))),
        Err(GameError::UnknownPlayer(PlayerId(9)))
    );
    assert_eq!(state, before);

    apply(&mut state, BORIS, Action::Move(Direction::East)).expect("cell is free");
    let moved_away = state.clone();
    assert_eq!(
        apply(&mut state, ANNA, attack(BORIS)),
        Err(GameError::TooFar)
    );
    assert_eq!(state, moved_away);
}

#[test]
fn potion_heals_a_wounded_player_by_the_agreed_amount() {
    let mut world = World::new(2, 1);
    world.set_tile(Position { x: 0, y: 0 }, Tile::Chest(String::from(POTION)));
    let mut state = GameState::new(world);
    state
        .add_player(ANNA, "Anna", Position { x: 0, y: 0 })
        .expect("cell is free");
    state
        .add_player(BORIS, "Boris", Position { x: 1, y: 0 })
        .expect("cell is free");
    apply(&mut state, ANNA, Action::PickUp).expect("chest is here");
    for _ in 0..5 {
        apply(&mut state, BORIS, attack(ANNA)).expect("target is near");
    }
    let wounded = START_HEALTH - 5 * BASE_DAMAGE;
    assert_eq!(
        state.player(ANNA).and_then(game::Player::health),
        Some(wounded)
    );

    let events = apply(
        &mut state,
        ANNA,
        Action::UseItem {
            item: String::from(POTION),
        },
    );
    let healed = wounded + POTION_HEAL;
    assert_eq!(
        events,
        Ok(vec![Event::Healed {
            player: ANNA,
            health: healed
        }])
    );
    assert_eq!(
        state.player(ANNA).and_then(game::Player::health),
        Some(healed)
    );
}
