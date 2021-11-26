use super::*;

#[test]
fn word_to_bytes() {
    let word: Word = 0x0123_4567_89AB_CDEF;
    let expected_bytes: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];

    assert_eq!(word.to_bytes(), expected_bytes);
}

#[test]
fn bytes_to_words() {
    let bytes: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    let expected_word: Word = 0x0102_0304_0506_0708;

    assert_eq!(Word::from_bytes(&bytes), expected_word);
    assert_eq!(Word::from_bytes_v(&bytes.to_vec()), expected_word);
}
