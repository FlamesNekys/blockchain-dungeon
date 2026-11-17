use game::{Direction, Player, PlayerId, PlayerState, Position, START_HEALTH};

#[test]
fn neighbor_moves_one_cell() {
    let start = Position { x: 2, y: 2 };
    assert_eq!(
        start.neighbor(Direction::North),
        Some(Position { x: 2, y: 1 })
    );
    assert_eq!(
        start.neighbor(Direction::South),
        Some(Position { x: 2, y: 3 })
    );
    assert_eq!(
        start.neighbor(Direction::East),
        Some(Position { x: 3, y: 2 })
    );
    assert_eq!(
        start.neighbor(Direction::West),
        Some(Position { x: 1, y: 2 })
    );
}

#[test]
fn neighbor_refuses_to_overflow() {
    let corner = Position {
        x: i32::MAX,
        y: i32::MIN,
    };
    assert_eq!(corner.neighbor(Direction::East), None);
    assert_eq!(corner.neighbor(Direction::North), None);
    assert_eq!(
        corner.neighbor(Direction::West),
        Some(Position {
            x: i32::MAX - 1,
            y: i32::MIN
        })
    );
}

#[test]
fn distance_counts_steps_without_diagonals() {
    let start = Position { x: 1, y: 1 };
    assert_eq!(start.distance(start), 0);
    assert_eq!(start.distance(Position { x: 2, y: 1 }), 1);
    assert_eq!(start.distance(Position { x: 2, y: 2 }), 2);
    assert_eq!(start.distance(Position { x: -3, y: 4 }), 7);
}

#[test]
fn distance_does_not_overflow() {
    let far_left = Position {
        x: i32::MIN,
        y: i32::MIN,
    };
    let far_right = Position {
        x: i32::MAX,
        y: i32::MAX,
    };
    assert_eq!(far_left.distance(far_right), u32::MAX);
}

#[test]
fn position_and_player_id_print_for_people() {
    assert_eq!(Position { x: 3, y: -1 }.to_string(), "(3, -1)");
    assert_eq!(PlayerId(7).to_string(), "#7");
}

#[test]
fn player_ids_are_ordered_by_number() {
    assert!(PlayerId(1) < PlayerId(2));
    assert_eq!(PlayerId(5), PlayerId(5));
}

#[test]
fn new_player_is_alive_with_full_health() {
    let player = Player::new("Anna", Position { x: 1, y: 1 });
    assert_eq!(
        player.state,
        PlayerState::Alive {
            health: START_HEALTH
        }
    );
    assert!(player.is_alive());
    assert_eq!(player.health(), Some(START_HEALTH));
}

#[test]
fn dead_player_has_no_health() {
    let mut player = Player::new("Anna", Position { x: 1, y: 1 });
    player.state = PlayerState::Dead { died_at_turn: 3 };
    assert!(!player.is_alive());
    assert_eq!(player.health(), None);
}
