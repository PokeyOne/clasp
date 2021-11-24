use super::*;

#[test]
fn word_to_bytes() {
    let word: Word = 0x0123_4567_89AB_CDEF;
    let expected_bytes: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];

    assert_eq!(word.to_bytes(), expected_bytes);
}
