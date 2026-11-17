use game::{Action, Direction, Event, GameError, GameState, PlayerId, Position, apply};

const ANNA: PlayerId = PlayerId(1);
const BORIS: PlayerId = PlayerId(2);

#[test]
fn move_changes_position_and_reports_it() {
    let mut state = GameState::genesis();
    let events = apply(&mut state, ANNA, Action::Move(Direction::East));
    assert_eq!(
        events,
        Ok(vec![Event::Moved {
            player: ANNA,
            to: Position { x: 2, y: 1 }
        }])
    );
    assert_eq!(
        state.player(ANNA).map(|player| player.position),
        Some(Position { x: 2, y: 1 })
    );
}

#[test]
fn every_successful_action_advances_the_turn_counter() {
    let mut state = GameState::genesis();
    assert_eq!(
        apply(&mut state, ANNA, Action::Wait),
        Ok(vec![Event::Waited { player: ANNA }])
    );
    assert_eq!(state.turn(), 1);
    apply(&mut state, BORIS, Action::Move(Direction::West)).expect("cell is free");
    assert_eq!(state.turn(), 2);
}

#[test]
fn refused_action_does_not_advance_the_turn_counter() {
    let mut state = GameState::genesis();
    assert_eq!(
        apply(&mut state, ANNA, Action::Move(Direction::North)),
        Err(GameError::Wall)
    );
    assert_eq!(state.turn(), 0);
}

#[test]
fn walls_stop_the_player() {
    let mut state = GameState::genesis();
    let before = state.clone();
    assert_eq!(
        apply(&mut state, ANNA, Action::Move(Direction::North)),
        Err(GameError::Wall)
    );
    assert_eq!(
        apply(&mut state, ANNA, Action::Move(Direction::West)),
        Err(GameError::Wall)
    );
    assert_eq!(state, before);
}

#[test]
fn the_edge_of_the_map_stops_the_player() {
    let mut state = GameState::new(game::World::new(1, 1));
    state
        .add_player(ANNA, "Anna", Position { x: 0, y: 0 })
        .expect("cell is free");
    let before = state.clone();
    for direction in [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ] {
        assert_eq!(
            apply(&mut state, ANNA, Action::Move(direction)),
            Err(GameError::OutOfBounds)
        );
    }
    assert_eq!(state, before);
}

#[test]
fn a_living_player_blocks_the_cell() {
    let mut state = GameState::new(game::World::new(2, 1));
    state
        .add_player(ANNA, "Anna", Position { x: 0, y: 0 })
        .expect("cell is free");
    state
        .add_player(BORIS, "Boris", Position { x: 1, y: 0 })
        .expect("cell is free");
    let before = state.clone();
    assert_eq!(
        apply(&mut state, ANNA, Action::Move(Direction::East)),
        Err(GameError::Occupied { by: BORIS })
    );
    assert_eq!(state, before);
}

#[test]
fn unknown_player_cannot_act() {
    let mut state = GameState::genesis();
    let before = state.clone();
    assert_eq!(
        apply(&mut state, PlayerId(99), Action::Wait),
        Err(GameError::UnknownPlayer(PlayerId(99)))
    );
    assert_eq!(state, before);
}

#[test]
fn a_chest_cell_can_be_entered() {
    let mut state = GameState::genesis();
    apply(&mut state, ANNA, Action::Move(Direction::East)).expect("cell is free");
    let events = apply(&mut state, ANNA, Action::Move(Direction::East));
    assert_eq!(
        events,
        Ok(vec![Event::Moved {
            player: ANNA,
            to: Position { x: 3, y: 1 }
        }])
    );
}
