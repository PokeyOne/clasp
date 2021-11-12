use super::*;

#[test]
fn number_base_ideal_decimal() {
    let value = NumberBase::DECIMAL.parse_number("37");
    assert_eq!(Ok(37u64), value);
}

#[test]
fn number_base_invalid_decimal() {
    let a = NumberBase::DECIMAL.parse_number("0d40");
    let b = NumberBase::DECIMAL.parse_number("boris");

    assert!(a.is_err());
    assert!(b.is_err());
}

#[test]
fn number_base_negative_number_should_fail() {
    
}

#[test]
fn number_base_empty_string_should_fail() {
    let bases = vec![
        NumberBase::BINARY,
        NumberBase::OCTAL,
        NumberBase::DECIMAL,
        NumberBase::HEX
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
        NumberBase::HEX.parse_number("0b3F")
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
        "FFFFFFFFFFFFFFFF0" // overflow
    ];

    for val in values {
        assert!(NumberBase::HEX.parse_number(&val).is_err());
    }
}
