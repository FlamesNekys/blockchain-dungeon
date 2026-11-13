use game::Inventory;

fn inventory_with(names: &[&str]) -> Inventory {
    let mut inventory = Inventory::default();
    for name in names {
        inventory.add(String::from(*name));
    }
    inventory
}

#[test]
fn new_inventory_is_empty() {
    let inventory = Inventory::default();
    assert_eq!(inventory.items(), [] as [String; 0]);
    assert_eq!(inventory.equipped(), None);
    assert_eq!(inventory.longest_item_name(), None);
}

#[test]
fn add_keeps_the_order() {
    let inventory = inventory_with(&["rope", "torch", "key"]);
    assert_eq!(inventory.items(), ["rope", "torch", "key"]);
}

#[test]
fn remove_takes_the_first_matching_item() {
    let mut inventory = inventory_with(&["rope", "torch", "rope"]);
    assert_eq!(inventory.remove("rope"), Some(String::from("rope")));
    assert_eq!(inventory.items(), ["torch", "rope"]);
}

#[test]
fn remove_of_a_missing_item_changes_nothing() {
    let mut inventory = inventory_with(&["rope"]);
    assert_eq!(inventory.remove("sword"), None);
    assert_eq!(inventory.items(), ["rope"]);
}

#[test]
fn longest_item_name_prefers_the_earlier_item_on_a_tie() {
    let inventory = inventory_with(&["key", "torch", "sword", "map"]);
    assert_eq!(inventory.longest_item_name(), Some("torch"));
}

#[test]
fn equip_moves_the_item_from_the_list_into_the_hand() {
    let mut inventory = inventory_with(&["rope", "sword"]);
    assert!(inventory.equip("sword"));
    assert_eq!(inventory.equipped(), Some("sword"));
    assert_eq!(inventory.items(), ["rope"]);
}

#[test]
fn equip_returns_the_previous_item_to_the_end_of_the_list() {
    let mut inventory = inventory_with(&["sword", "rope", "axe"]);
    assert!(inventory.equip("sword"));
    assert!(inventory.equip("axe"));
    assert_eq!(inventory.equipped(), Some("axe"));
    assert_eq!(inventory.items(), ["rope", "sword"]);
}

#[test]
fn equip_of_a_missing_item_changes_nothing() {
    let mut inventory = inventory_with(&["rope"]);
    let before = inventory.clone();
    assert!(!inventory.equip("sword"));
    assert_eq!(inventory, before);
}
