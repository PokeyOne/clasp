use super::*;

#[test]
fn digit_value_for_base_ten() {
    let units: Vec<(u8, char)> = vec![
        (0, '0'),
        (1, '1'),
        (2, '2'),
        (3, '3'),
        (4, '4'),
        (5, '5'),
        (6, '6'),
        (7, '7'),
        (8, '8'),
        (9, '9'),
    ];

    for unit in units {
        let result = NumberBase::digit_value(unit.1).unwrap();

        assert_eq!(unit.0, result);
    }
}

#[test]
fn digit_value_for_hex() {
    let units: Vec<(u8, char)> = vec![
        (10, 'A'),
        (11, 'B'),
        (12, 'C'),
        (13, 'D'),
        (14, 'E'),
        (15, 'F'),
    ];

    for unit in units {
        let result = NumberBase::digit_value(unit.1).unwrap();

        assert_eq!(unit.0, result);
    }
}

#[test]
fn number_base_ideal_decimal() {
    let value = NumberBase::DECIMAL.parse_number("37");
    assert_eq!(Ok(37u64), value);
}

#[test]
fn number_base_invalid_decimal() {
    let a = NumberBase::DECIMAL.parse_number("0d40");
    let b = NumberBase::DECIMAL.parse_number("boris");

    println!("a = {:?}, b = {:?}", a, b);
    assert!(a.is_err());
    assert!(b.is_err());
}

#[test]
fn number_base_negative_number_should_fail() {}

#[test]
fn number_base_empty_string_should_fail() {
    let bases = vec![
        NumberBase::BINARY,
        NumberBase::OCTAL,
        NumberBase::DECIMAL,
        NumberBase::HEX,
    ];

    for base in bases {
        assert!(base.parse_number("").is_err());
    }
}

#[test]
fn number_base_ideal_hex() {
    let values = vec![
        NumberBase::HEX.parse_number("4f"),
        NumberBase::HEX.parse_number("4F"),
        NumberBase::HEX.parse_number("2"),
        NumberBase::HEX.parse_number("0b3F"),
    ];

    assert_eq!(Ok(0x4f), values[0]);
    assert_eq!(Ok(0x4f), values[1]);
    assert_eq!(Ok(0x2), values[2]);
    assert_eq!(Ok(0x0b3f), values[3]);
}

#[test]
fn number_base_invalid_hex() {
    let values = vec![
        "f34dddeg",
        "g",
        "0xFF",
        "FFFFFFFFFFFFFFFF0", // overflow
    ];

    for val in values {
        assert!(NumberBase::HEX.parse_number(&val).is_err());
    }
}

#[test]
fn prefixed_ideal_hex() {
    assert_eq!(Ok(0xFFu64), parse_number("0xFF"));
}

#[test]
fn prefixed_ideal_binary() {
    assert_eq!(Ok(0xFFu64), parse_number("0b11111111"));
}

#[test]
fn prefixed_ideal_octal() {
    assert_eq!(Ok(0xFFu64), parse_number("0o377"));
}

#[test]
fn prefixed_ideal_decimal() {
    assert_eq!(Ok(0xFFu64), parse_number("0d255"));
}

#[test]
fn ideal_decimal() {
    assert_eq!(Ok(0xFFu64), parse_number("255"));
}
