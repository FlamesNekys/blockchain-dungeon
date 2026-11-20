use chain::Encode;
use game::{
    Action, Direction, GameState, Inventory, Player, PlayerId, PlayerState, Position, Tile, World,
    apply,
};

#[test]
fn small_game_types_have_exact_bytes() {
    assert_eq!(PlayerId(7).to_bytes(), [7, 0, 0, 0]);
    assert_eq!(
        Position { x: 1, y: -1 }.to_bytes(),
        [1, 0, 0, 0, 255, 255, 255, 255]
    );
    assert_eq!(Direction::North.to_bytes(), [0]);
    assert_eq!(Direction::South.to_bytes(), [1]);
    assert_eq!(Direction::East.to_bytes(), [2]);
    assert_eq!(Direction::West.to_bytes(), [3]);
}

#[test]
fn tiles_have_exact_bytes() {
    assert_eq!(Tile::Wall.to_bytes(), [0]);
    assert_eq!(Tile::Floor.to_bytes(), [1]);
    assert_eq!(
        Tile::Chest(String::from("ab")).to_bytes(),
        [2, 2, 0, 0, 0, b'a', b'b']
    );
}

#[test]
fn actions_have_exact_bytes() {
    assert_eq!(Action::Move(Direction::East).to_bytes(), [0, 2]);
    assert_eq!(
        Action::Attack {
            target: PlayerId(2)
        }
        .to_bytes(),
        [1, 2, 0, 0, 0]
    );
    assert_eq!(Action::PickUp.to_bytes(), [2]);
    assert_eq!(
        Action::UseItem {
            item: String::from("p")
        }
        .to_bytes(),
        [3, 1, 0, 0, 0, b'p']
    );
    assert_eq!(
        Action::Give {
            to: PlayerId(2),
            item: String::from("s")
        }
        .to_bytes(),
        [4, 2, 0, 0, 0, 1, 0, 0, 0, b's']
    );
    assert_eq!(Action::Wait.to_bytes(), [5]);
}

#[test]
fn world_is_size_and_tiles_row_by_row() {
    let world = World::from_rows(&["#.", ".#"]).expect("рисунок правильный");
    assert_eq!(
        world.tiles(),
        [Tile::Wall, Tile::Floor, Tile::Floor, Tile::Wall]
    );
    assert_eq!(
        world.to_bytes(),
        [2, 0, 0, 0, 2, 0, 0, 0, 4, 0, 0, 0, 0, 1, 1, 0]
    );
}

#[test]
fn inventory_is_list_and_item_in_hands() {
    let mut inventory = Inventory::default();
    assert_eq!(inventory.to_bytes(), [0, 0, 0, 0, 0]);
    inventory.add(String::from("a"));
    inventory.add(String::from("b"));
    assert!(inventory.equip("a"));
    assert_eq!(
        inventory.to_bytes(),
        [1, 0, 0, 0, 1, 0, 0, 0, b'b', 1, 1, 0, 0, 0, b'a']
    );
}

#[test]
fn player_is_name_position_state_inventory() {
    let mut player = Player::new("A", Position { x: 1, y: 2 });
    let alive = [
        1, 0, 0, 0, b'A', 1, 0, 0, 0, 2, 0, 0, 0, 0, 100, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    assert_eq!(player.to_bytes(), alive);
    player.state = PlayerState::Dead { died_at_turn: 3 };
    let dead = [
        1, 0, 0, 0, b'A', 1, 0, 0, 0, 2, 0, 0, 0, 1, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    assert_eq!(player.to_bytes(), dead);
}

#[test]
fn genesis_state_has_known_sum() {
    let state = GameState::genesis();
    assert_eq!(state.to_bytes().len(), 139);
    assert_eq!(
        state.digest().to_string(),
        "bd310bfe30cd853b36132ce4f6713eacd4edcba59bb0e50371b4f2bf8a6b6063"
    );
}
#[test]
fn sum_follows_every_change_of_state() {
    let mut state = GameState::genesis();
    let before = state.digest();
    assert_eq!(
        apply(&mut state, PlayerId(1), Action::Move(Direction::North)),
        Err(game::GameError::Wall)
    );
    assert_eq!(
        state.digest(),
        before,
        "отказ не меняет состояние, значит и сумму"
    );
    apply(&mut state, PlayerId(1), Action::Wait).expect("ждать можно всегда");
    assert_ne!(state.digest(), before, "счётчик ходов входит в состояние");
}
