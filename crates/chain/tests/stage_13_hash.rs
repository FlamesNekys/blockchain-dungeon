use chain::{Hash, WrongHashLength};

#[test]
fn known_text_gives_known_sum() {
    let sum = Hash::of(b"abc");
    assert_eq!(
        sum.to_string(),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn same_bytes_give_same_sum_and_one_changed_bit_changes_it() {
    assert_eq!(Hash::of(b"dungeon"), Hash::of(b"dungeon"));
    assert_ne!(Hash::of(b"dungeon"), Hash::of(b"dungeoo"));
}

#[test]
fn zero_is_printed_as_sixty_four_zeros() {
    let text = Hash::ZERO.to_string();
    assert_eq!(text.len(), 64);
    assert!(text.chars().all(|sign| sign == '0'));
}

#[test]
fn debug_view_contains_hex_text() {
    let sum = Hash::of(b"abc");
    assert_eq!(format!("{sum:?}"), format!("Hash({sum})"));
}

#[test]
fn bytes_survive_round_trip() {
    let mut bytes = [0_u8; 32];
    bytes[0] = 0xab;
    bytes[31] = 0x01;
    let sum = Hash::from_bytes(bytes);
    assert_eq!(sum.as_bytes(), &bytes);
    assert!(sum.to_string().starts_with("ab00"));
    assert!(sum.to_string().ends_with("0001"));
}

#[test]
fn slice_of_right_length_becomes_hash() {
    let bytes = vec![7_u8; 32];
    let sum = Hash::try_from(bytes.as_slice()).expect("в срезе ровно 32 байта");
    assert_eq!(sum, Hash::from_bytes([7; 32]));
}

#[test]
fn slice_of_wrong_length_is_rejected() {
    let short = [1_u8; 31];
    assert_eq!(Hash::try_from(&short[..]), Err(WrongHashLength(31)));
    let long = [1_u8; 33];
    assert_eq!(Hash::try_from(&long[..]), Err(WrongHashLength(33)));
}

#[test]
fn hashes_are_ordered_by_bytes() {
    let small = Hash::from_bytes([1; 32]);
    let big = Hash::from_bytes([2; 32]);
    assert!(small < big);
    assert!(Hash::ZERO < small);
}
