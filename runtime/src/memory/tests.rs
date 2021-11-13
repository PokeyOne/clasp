use super::*;

#[test]
fn word_from_bytes() {
    let expected: Word = 0xBACD93FE0102E493;
    let bytes: [Byte; 8] = [0xBA, 0xCD, 0x93, 0xFE, 0x01, 0x02, 0xE4, 0x93];
    let word: Word = Word::from_bytes(&bytes);

    assert_eq!(expected, word);
}

#[test]
fn bytes_from_word() {
    let input_word: Word = 0xBACD93FE0102E493;
    let expected_bytes: [Byte; 8] = [0xBA, 0xCD, 0x93, 0xFE, 0x01, 0x02, 0xE4, 0x93];
    let result: [Byte; 8] = input_word.get_bytes();

    for i in 0..8 {
        assert_eq!(
            expected_bytes[i], result[i],
            "Byte {} should match, expected {:#X} -> got {:#X}",
            i, expected_bytes[i], result[i]
        );
    }
}

#[test]
fn initial_memory_should_be_zero_and_readable() {
    let memory: Memory = Memory::new(1000); // 1kb

    for i in 0..(1000 - 8) {
        println!("{}", i);
        let value: Word = match memory.read(i as MemoryLocation) {
            Ok(value) => value,
            Err(_err_type) => {
                panic!();
            }
        };

        assert_eq!(0, value);
    }

    assert_eq!(memory.size(), 1000);
}

#[test]
fn writing_to_memory() {
    let mut memory: Memory = Memory::new(8);

    memory.write(0, 0x3F);

    assert_eq!(
        0x3F,
        match memory.read(0) {
            Ok(value) => value,
            Err(_err_type) => {
                panic!();
            }
        }
    );
}

#[test]
fn writing_and_reading_to_register() {
    let mut memory: Memory = Memory::new(16);

    let test_value = 0x0123456789ABCDEF;
    match memory.write(GA_LOC, test_value) {
        Ok(_) => {}
        Err(err_type) => {
            panic!("Could not write to memory with error: {:?}", err_type);
        }
    };

    assert_eq!(
        test_value,
        match memory.read(GA_LOC) {
            Ok(value) => value,
            Err(_err_type) => {
                panic!();
            }
        }
    );
}
