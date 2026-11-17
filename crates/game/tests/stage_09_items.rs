use game::{
    Action, Direction, Event, GameError, GameState, PlayerId, Position, START_HEALTH, Tile, World,
    apply,
};

const ANNA: PlayerId = PlayerId(1);
const BORIS: PlayerId = PlayerId(2);

/// Карта из трёх клеток в ряд: Анна стоит на сундуке с заданным предметом, Борис рядом, третья клетка свободна.
fn state_with_chest(item: &str) -> GameState {
    let mut world = World::new(3, 1);
    world.set_tile(Position { x: 0, y: 0 }, Tile::Chest(String::from(item)));
    let mut state = GameState::new(world);
    state
        .add_player(ANNA, "Anna", Position { x: 0, y: 0 })
        .expect("cell is free");
    state
        .add_player(BORIS, "Boris", Position { x: 1, y: 0 })
        .expect("cell is free");
    state
}

fn use_item(item: &str) -> Action {
    Action::UseItem {
        item: String::from(item),
    }
}

fn give(to: PlayerId, item: &str) -> Action {
    Action::Give {
        to,
        item: String::from(item),
    }
}

#[test]
fn pick_up_moves_the_item_from_the_chest_to_the_inventory() {
    let mut state = state_with_chest("sword");
    let events = apply(&mut state, ANNA, Action::PickUp);
    assert_eq!(
        events,
        Ok(vec![Event::PickedUp {
            player: ANNA,
            item: String::from("sword")
        }])
    );
    assert_eq!(
        state.world().tile(Position { x: 0, y: 0 }),
        Some(&Tile::Floor)
    );
    let anna = state.player(ANNA).expect("player exists");
    assert_eq!(anna.inventory.items(), ["sword"]);
}

#[test]
fn pick_up_on_plain_floor_is_refused() {
    let mut state = state_with_chest("sword");
    let before = state.clone();
    assert_eq!(
        apply(&mut state, BORIS, Action::PickUp),
        Err(GameError::NothingToPickUp)
    );
    assert_eq!(state, before);
}

#[test]
fn using_an_ordinary_item_takes_it_in_hand() {
    let mut state = state_with_chest("sword");
    apply(&mut state, ANNA, Action::PickUp).expect("chest is here");
    let events = apply(&mut state, ANNA, use_item("sword"));
    assert_eq!(
        events,
        Ok(vec![Event::Equipped {
            player: ANNA,
            item: String::from("sword")
        }])
    );
    let anna = state.player(ANNA).expect("player exists");
    assert_eq!(anna.inventory.equipped(), Some("sword"));
    assert_eq!(anna.inventory.items(), [] as [String; 0]);
}

#[test]
fn potion_is_consumed_and_health_never_exceeds_the_start_value() {
    let mut state = state_with_chest("potion");
    apply(&mut state, ANNA, Action::PickUp).expect("chest is here");
    let events = apply(&mut state, ANNA, use_item("potion"));
    assert_eq!(
        events,
        Ok(vec![Event::Healed {
            player: ANNA,
            health: START_HEALTH
        }])
    );
    let anna = state.player(ANNA).expect("player exists");
    assert_eq!(anna.health(), Some(START_HEALTH));
    assert_eq!(anna.inventory.items(), [] as [String; 0]);
    assert_eq!(anna.inventory.equipped(), None);
}

#[test]
fn using_a_missing_item_is_refused() {
    let mut state = state_with_chest("sword");
    let before = state.clone();
    assert_eq!(
        apply(&mut state, ANNA, use_item("potion")),
        Err(GameError::NoSuchItem(String::from("potion")))
    );
    assert_eq!(state, before);
}

#[test]
fn give_moves_the_item_to_a_neighbor() {
    let mut state = state_with_chest("key");
    apply(&mut state, ANNA, Action::PickUp).expect("chest is here");
    let events = apply(&mut state, ANNA, give(BORIS, "key"));
    assert_eq!(
        events,
        Ok(vec![Event::Gave {
            from: ANNA,
            to: BORIS,
            item: String::from("key")
        }])
    );
    assert_eq!(
        state.player(ANNA).expect("player exists").inventory.items(),
        [] as [String; 0]
    );
    assert_eq!(
        state
            .player(BORIS)
            .expect("player exists")
            .inventory
            .items(),
        ["key"]
    );
}

#[test]
fn give_is_refused_for_every_broken_rule() {
    let mut state = state_with_chest("key");
    apply(&mut state, ANNA, Action::PickUp).expect("chest is here");
    let before = state.clone();

    assert_eq!(
        apply(&mut state, ANNA, give(ANNA, "key")),
        Err(GameError::SelfTarget)
    );
    assert_eq!(
        apply(&mut state, ANNA, give(PlayerId(9), "key")),
        Err(GameError::UnknownPlayer(PlayerId(9)))
    );
    assert_eq!(
        apply(&mut state, ANNA, give(BORIS, "axe")),
        Err(GameError::NoSuchItem(String::from("axe")))
    );
    assert_eq!(state, before);

    apply(&mut state, BORIS, Action::Move(Direction::East)).expect("cell is free");
    let moved_away = state.clone();
    assert_eq!(
        apply(&mut state, ANNA, give(BORIS, "key")),
        Err(GameError::TooFar)
    );
    assert_eq!(state, moved_away);
}

#[test]
fn item_in_hand_cannot_be_given() {
    let mut state = state_with_chest("sword");
    apply(&mut state, ANNA, Action::PickUp).expect("chest is here");
    apply(&mut state, ANNA, use_item("sword")).expect("item is in the list");
    assert_eq!(
        apply(&mut state, ANNA, give(BORIS, "sword")),
        Err(GameError::NoSuchItem(String::from("sword")))
    );
}
