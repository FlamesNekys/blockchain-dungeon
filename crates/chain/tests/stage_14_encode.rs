use chain::Encode;

#[test]
fn numbers_are_written_low_byte_first() {
    assert_eq!(7_u8.to_bytes(), [7]);
    assert_eq!(1_u32.to_bytes(), [1, 0, 0, 0]);
    assert_eq!(0x0102_0304_u32.to_bytes(), [4, 3, 2, 1]);
    assert_eq!(258_u64.to_bytes(), [2, 1, 0, 0, 0, 0, 0, 0]);
    assert_eq!((-1_i32).to_bytes(), [255, 255, 255, 255]);
    assert_eq!((-2_i32).to_bytes(), [254, 255, 255, 255]);
}

#[test]
fn byte_array_is_written_as_is() {
    assert_eq!([9_u8, 8, 7].to_bytes(), [9, 8, 7]);
    let empty: [u8; 0] = [];
    assert_eq!(empty.to_bytes().len(), 0);
}

#[test]
fn text_is_written_as_length_and_bytes() {
    assert_eq!("hi".to_bytes(), [2, 0, 0, 0, b'h', b'i']);
    assert_eq!(String::from("hi").to_bytes(), "hi".to_bytes());
    assert_eq!("".to_bytes(), [0, 0, 0, 0]);
    // Длина считается в байтах, а не в буквах: русская буква занимает два байта.
    assert_eq!("я".to_bytes(), [2, 0, 0, 0, 0xd1, 0x8f]);
}

#[test]
fn list_is_written_as_count_and_items() {
    let numbers: Vec<u32> = vec![1, 2];
    assert_eq!(numbers.to_bytes(), [2, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0]);
    assert_eq!(numbers.as_slice().to_bytes(), numbers.to_bytes());
    let nothing: Vec<u64> = Vec::new();
    assert_eq!(nothing.to_bytes(), [0, 0, 0, 0]);
}

#[test]
fn nested_lists_work() {
    let words = vec![String::from("a"), String::from("bc")];
    assert_eq!(
        words.to_bytes(),
        [2, 0, 0, 0, 1, 0, 0, 0, b'a', 2, 0, 0, 0, b'b', b'c']
    );
}

#[test]
fn option_is_written_as_mark_and_value() {
    let nothing: Option<u32> = None;
    assert_eq!(nothing.to_bytes(), [0]);
    assert_eq!(Some(5_u32).to_bytes(), [1, 5, 0, 0, 0]);
    let borrowed: Option<&str> = Some("x");
    assert_eq!(borrowed.to_bytes(), [1, 1, 0, 0, 0, b'x']);
}

#[test]
fn encode_appends_to_existing_bytes() {
    let mut out = vec![0xff];
    1_u8.encode(&mut out);
    "a".encode(&mut out);
    assert_eq!(out, [0xff, 1, 1, 0, 0, 0, b'a']);
}

#[test]
fn digest_is_sum_of_bytes() {
    assert_eq!("abc".to_bytes(), [3, 0, 0, 0, b'a', b'b', b'c']);
    assert_eq!(
        "abc".digest(),
        chain::Hash::of(&[3, 0, 0, 0, b'a', b'b', b'c'])
    );
    assert_ne!("abc".digest(), "abd".digest());
}
