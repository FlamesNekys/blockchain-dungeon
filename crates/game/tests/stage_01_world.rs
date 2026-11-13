use game::{Position, Tile, World};

#[test]
fn new_world_is_all_floor() {
    let world = World::new(3, 2);
    assert_eq!(world.width(), 3);
    assert_eq!(world.height(), 2);
    for y in 0..2 {
        for x in 0..3 {
            assert_eq!(world.tile(Position { x, y }), Some(&Tile::Floor));
        }
    }
}

#[test]
fn tile_outside_the_map_is_none() {
    let world = World::new(3, 2);
    assert_eq!(world.tile(Position { x: 3, y: 0 }), None);
    assert_eq!(world.tile(Position { x: 0, y: 2 }), None);
    assert_eq!(world.tile(Position { x: -1, y: 0 }), None);
    assert_eq!(world.tile(Position { x: 0, y: -1 }), None);
}

#[test]
fn set_tile_replaces_exactly_one_cell() {
    let mut world = World::new(3, 2);
    assert!(world.set_tile(Position { x: 2, y: 1 }, Tile::Wall));
    assert_eq!(world.tile(Position { x: 2, y: 1 }), Some(&Tile::Wall));
    assert_eq!(world.tile(Position { x: 1, y: 1 }), Some(&Tile::Floor));
    assert_eq!(world.tile(Position { x: 2, y: 0 }), Some(&Tile::Floor));
}

#[test]
fn set_tile_outside_the_map_changes_nothing() {
    let mut world = World::new(3, 2);
    let before = world.clone();
    assert!(!world.set_tile(Position { x: 3, y: 0 }, Tile::Wall));
    assert!(!world.set_tile(Position { x: 0, y: -1 }, Tile::Wall));
    assert_eq!(world, before);
}

#[test]
fn position_is_copied_implicitly() {
    let first = Position { x: 1, y: 2 };
    let second = first;
    assert_eq!(first, second);
}

#[test]
fn empty_world_has_no_tiles() {
    let world = World::new(0, 0);
    assert_eq!(world.tile(Position { x: 0, y: 0 }), None);
}
