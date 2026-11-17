use game::{GameError, GameState, PlayerId, Position, Tile, World, render};

#[test]
fn from_rows_reads_walls_and_floor() {
    let world = World::from_rows(&["#.#", "..."]).expect("picture is valid");
    assert_eq!(world.width(), 3);
    assert_eq!(world.height(), 2);
    assert_eq!(world.tile(Position { x: 0, y: 0 }), Some(&Tile::Wall));
    assert_eq!(world.tile(Position { x: 1, y: 0 }), Some(&Tile::Floor));
    assert_eq!(world.tile(Position { x: 2, y: 1 }), Some(&Tile::Floor));
}

#[test]
fn from_rows_rejects_bad_pictures() {
    assert_eq!(World::from_rows(&[]), None);
    assert_eq!(World::from_rows(&[""]), None);
    assert_eq!(World::from_rows(&["..", "..."]), None);
    assert_eq!(World::from_rows(&["...", ".."]), None);
    assert_eq!(World::from_rows(&[".x."]), None);
}

#[test]
fn new_game_is_empty_and_starts_at_turn_zero() {
    let state = GameState::new(World::new(2, 2));
    assert_eq!(state.turn(), 0);
    assert!(state.players().is_empty());
    assert_eq!(state.player(PlayerId(1)), None);
}

#[test]
fn add_player_puts_a_living_player_on_the_map() {
    let mut state = GameState::new(World::new(2, 2));
    assert_eq!(
        state.add_player(PlayerId(1), "Anna", Position { x: 1, y: 0 }),
        Ok(())
    );
    let anna = state.player(PlayerId(1)).expect("player was added");
    assert_eq!(anna.name, "Anna");
    assert_eq!(anna.position, Position { x: 1, y: 0 });
    assert!(anna.is_alive());
    assert_eq!(
        state.living_player_at(Position { x: 1, y: 0 }),
        Some(PlayerId(1))
    );
    assert_eq!(state.living_player_at(Position { x: 0, y: 0 }), None);
}

#[test]
fn add_player_refuses_bad_requests_and_changes_nothing() {
    let mut world = World::new(3, 1);
    world.set_tile(Position { x: 2, y: 0 }, Tile::Wall);
    let mut state = GameState::new(world);
    state
        .add_player(PlayerId(1), "Anna", Position { x: 0, y: 0 })
        .expect("cell is free");
    let before = state.clone();

    assert_eq!(
        state.add_player(PlayerId(1), "Clone", Position { x: 1, y: 0 }),
        Err(GameError::PlayerExists(PlayerId(1)))
    );
    assert_eq!(
        state.add_player(PlayerId(2), "Boris", Position { x: 3, y: 0 }),
        Err(GameError::OutOfBounds)
    );
    assert_eq!(
        state.add_player(PlayerId(2), "Boris", Position { x: 2, y: 0 }),
        Err(GameError::Wall)
    );
    assert_eq!(
        state.add_player(PlayerId(2), "Boris", Position { x: 0, y: 0 }),
        Err(GameError::Occupied { by: PlayerId(1) })
    );
    assert_eq!(state, before);
}

#[test]
fn players_are_listed_in_order_of_their_numbers() {
    let mut state = GameState::new(World::new(3, 1));
    state
        .add_player(PlayerId(9), "Zed", Position { x: 0, y: 0 })
        .expect("cell is free");
    state
        .add_player(PlayerId(2), "Boris", Position { x: 1, y: 0 })
        .expect("cell is free");
    state
        .add_player(PlayerId(5), "Eve", Position { x: 2, y: 0 })
        .expect("cell is free");
    let order: Vec<PlayerId> = state.players().keys().copied().collect();
    assert_eq!(order, [PlayerId(2), PlayerId(5), PlayerId(9)]);
}

#[test]
fn genesis_matches_the_agreed_picture() {
    let state = GameState::genesis();
    let expected = "\
#######
#1.$..#
#.#.#.#
#..$.2#
#######
";
    assert_eq!(render(&state), expected);
    assert_eq!(state.turn(), 0);
    assert_eq!(
        state.world().tile(Position { x: 3, y: 1 }),
        Some(&Tile::Chest(String::from("sword")))
    );
    assert_eq!(
        state.world().tile(Position { x: 3, y: 3 }),
        Some(&Tile::Chest(String::from("potion")))
    );
    assert_eq!(
        state.player(PlayerId(1)).map(|player| player.name.as_str()),
        Some("Anna")
    );
    assert_eq!(
        state.player(PlayerId(2)).map(|player| player.name.as_str()),
        Some("Boris")
    );
}

#[test]
fn genesis_is_the_same_every_time() {
    assert_eq!(GameState::genesis(), GameState::genesis());
}

#[test]
fn render_shows_the_last_digit_of_a_player_number() {
    let mut state = GameState::new(World::new(2, 1));
    state
        .add_player(PlayerId(27), "Far", Position { x: 1, y: 0 })
        .expect("cell is free");
    assert_eq!(render(&state), ".7\n");
}

#[test]
fn errors_explain_themselves() {
    assert_eq!(
        GameError::UnknownPlayer(PlayerId(4)).to_string(),
        "игрока #4 не существует"
    );
    assert_eq!(
        GameError::NoSuchItem(String::from("axe")).to_string(),
        "в инвентаре нет предмета «axe»"
    );
    assert_eq!(
        GameError::Occupied { by: PlayerId(2) }.to_string(),
        "клетка занята игроком #2"
    );
}
