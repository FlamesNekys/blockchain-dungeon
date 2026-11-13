use game::{Player, Position, Tile, World, pick_up};

#[test]
fn take_item_empties_the_chest() {
    let mut world = World::new(2, 1);
    let chest = Position { x: 1, y: 0 };
    world.set_tile(chest, Tile::Chest(String::from("sword")));
    assert_eq!(world.take_item(chest), Some(String::from("sword")));
    assert_eq!(world.tile(chest), Some(&Tile::Floor));
    assert_eq!(world.take_item(chest), None);
}

#[test]
fn take_item_leaves_walls_and_floor_untouched() {
    let mut world = World::new(2, 1);
    world.set_tile(Position { x: 1, y: 0 }, Tile::Wall);
    let before = world.clone();
    assert_eq!(world.take_item(Position { x: 0, y: 0 }), None);
    assert_eq!(world.take_item(Position { x: 1, y: 0 }), None);
    assert_eq!(world.take_item(Position { x: 5, y: 5 }), None);
    assert_eq!(world, before);
}

#[test]
fn pick_up_moves_the_item_into_the_inventory() {
    let mut world = World::new(2, 1);
    let chest = Position { x: 1, y: 0 };
    world.set_tile(chest, Tile::Chest(String::from("potion")));
    let mut player = Player::new("Anna", chest);
    assert!(pick_up(&mut world, &mut player));
    assert_eq!(player.inventory.items(), ["potion"]);
    assert_eq!(world.tile(chest), Some(&Tile::Floor));
}

#[test]
fn pick_up_on_plain_floor_does_nothing() {
    let mut world = World::new(2, 1);
    let before = world.clone();
    let mut player = Player::new("Anna", Position { x: 0, y: 0 });
    assert!(!pick_up(&mut world, &mut player));
    assert_eq!(player.inventory.items(), [] as [String; 0]);
    assert_eq!(world, before);
}

#[test]
fn second_pick_up_finds_nothing() {
    let mut world = World::new(1, 1);
    let chest = Position { x: 0, y: 0 };
    world.set_tile(chest, Tile::Chest(String::from("key")));
    let mut player = Player::new("Anna", chest);
    assert!(pick_up(&mut world, &mut player));
    assert!(!pick_up(&mut world, &mut player));
    assert_eq!(player.inventory.items(), ["key"]);
}
