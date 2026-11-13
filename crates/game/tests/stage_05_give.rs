use game::{Player, Position, give_item};

fn two_players() -> Vec<Player> {
    let mut anna = Player::new("Anna", Position { x: 0, y: 0 });
    anna.inventory.add(String::from("sword"));
    anna.inventory.add(String::from("rope"));
    let boris = Player::new("Boris", Position { x: 1, y: 0 });
    vec![anna, boris]
}

#[test]
fn item_moves_from_one_player_to_another() {
    let mut players = two_players();
    assert!(give_item(&mut players, 0, 1, "sword"));
    assert_eq!(players[0].inventory.items(), ["rope"]);
    assert_eq!(players[1].inventory.items(), ["sword"]);
}

#[test]
fn item_can_travel_back() {
    let mut players = two_players();
    assert!(give_item(&mut players, 0, 1, "sword"));
    assert!(give_item(&mut players, 1, 0, "sword"));
    assert_eq!(players[0].inventory.items(), ["rope", "sword"]);
    assert_eq!(players[1].inventory.items(), [] as [String; 0]);
}

#[test]
fn giving_to_yourself_is_refused() {
    let mut players = two_players();
    let before = players.clone();
    assert!(!give_item(&mut players, 0, 0, "sword"));
    assert_eq!(players, before);
}

#[test]
fn giving_a_missing_item_is_refused() {
    let mut players = two_players();
    let before = players.clone();
    assert!(!give_item(&mut players, 1, 0, "sword"));
    assert!(!give_item(&mut players, 0, 1, "axe"));
    assert_eq!(players, before);
}

#[test]
fn wrong_player_numbers_are_refused_and_nothing_is_lost() {
    let mut players = two_players();
    let before = players.clone();
    assert!(!give_item(&mut players, 0, 2, "sword"));
    assert!(!give_item(&mut players, 2, 0, "sword"));
    assert_eq!(players, before);
}
