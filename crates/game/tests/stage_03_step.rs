use game::{Direction, Player, Position, Tile, World, step};

fn world_with_wall() -> World {
    let mut world = World::new(3, 3);
    world.set_tile(Position { x: 2, y: 1 }, Tile::Wall);
    world
}

#[test]
fn new_player_stands_where_asked_with_empty_inventory() {
    let player = Player::new("Anna", Position { x: 1, y: 1 });
    assert_eq!(player.name, "Anna");
    assert_eq!(player.position, Position { x: 1, y: 1 });
    assert_eq!(player.inventory.items(), [] as [String; 0]);
}

#[test]
fn player_walks_in_all_four_directions() {
    let world = World::new(3, 3);
    let mut player = Player::new("Anna", Position { x: 1, y: 1 });
    assert!(step(&world, &mut player, Direction::North));
    assert_eq!(player.position, Position { x: 1, y: 0 });
    assert!(step(&world, &mut player, Direction::South));
    assert_eq!(player.position, Position { x: 1, y: 1 });
    assert!(step(&world, &mut player, Direction::East));
    assert_eq!(player.position, Position { x: 2, y: 1 });
    assert!(step(&world, &mut player, Direction::West));
    assert_eq!(player.position, Position { x: 1, y: 1 });
}

#[test]
fn player_cannot_walk_through_wall() {
    let world = world_with_wall();
    let mut player = Player::new("Anna", Position { x: 1, y: 1 });
    assert!(!step(&world, &mut player, Direction::East));
    assert_eq!(player.position, Position { x: 1, y: 1 });
}

#[test]
fn player_cannot_leave_the_map() {
    let world = World::new(2, 2);
    let mut player = Player::new("Anna", Position { x: 0, y: 0 });
    assert!(!step(&world, &mut player, Direction::North));
    assert!(!step(&world, &mut player, Direction::West));
    assert_eq!(player.position, Position { x: 0, y: 0 });

    let mut other = Player::new("Boris", Position { x: 1, y: 1 });
    assert!(!step(&world, &mut other, Direction::South));
    assert!(!step(&world, &mut other, Direction::East));
    assert_eq!(other.position, Position { x: 1, y: 1 });
}

#[test]
fn player_can_stand_on_a_chest() {
    let mut world = World::new(2, 1);
    world.set_tile(Position { x: 1, y: 0 }, Tile::Chest(String::from("sword")));
    let mut player = Player::new("Anna", Position { x: 0, y: 0 });
    assert!(step(&world, &mut player, Direction::East));
    assert_eq!(player.position, Position { x: 1, y: 0 });
}
