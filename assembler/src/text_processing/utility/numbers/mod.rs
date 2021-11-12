use ascii::AsciiChar;

#[cfg(test)]
mod tests;

struct NumberBase {
    place_value: u8
}

impl NumberBase {
    const BINARY: Self = new(2);
    const OCTAL: Self = new(8);
    const DECIMAL: Self = new(10);
    const HEX: Self = new(16);

    fn new(place_value: u8) -> Self {
        Self { place_value: place_value }
    }

    /// Parses a number assuming this base, returns Err(String) with the reason
    /// if cannot parse. Input **must not** include the base prefix, for that
    /// call the general method `text_processing::utility::parse_number`.
    fn parse_number(&self, val: &str) -> Result<u64, String> {
        if val == "" {
            return Err("Empty String");
        }

        let mut result: u64 = 0;

        let mut i = 0;
        loop {
            match val.chars().nth(i) {
                Some(val) => match digit_value(val) {
                    Some(c) if c < self.place_value => {
                        result *= self.place_value;
                        result += c as u64;
                    },
                    _ => return Err(String::from("Invalid digit"))
                },
                None => break
            };

            i += 1;
        }
    }

    fn digit_value(val: char) -> Option<u8> {
        let ascii_char = AsciiChar::from_ascii(val)?.to_ascii_uppercase();

        if ascii_char.is_ascii_hexdigit() {
            match ascii_char.as_byte() {
                0x30..=0x39 => Some(ascii_char.as_byte() - 0x30),
                AsciiChar::A.as_byte()..=AsciiChar::Z.as_byte() => Some(ascii_char.as_byte() - AsciiChar::A.as_byte()),
                _ => None
            }
        } else {
            None
        }
    }
}

/// Parse the string representation of an unsigned integer value. Supports only
/// unsigned values and if 'n' is the value then `0 <= n < 2^64`. The number may
/// be prefixed with '0b', '0o', '0d', or '0x' to indicate binary, octal,
/// decimal, or hexadecimal respectively. No prefix will be assumed decimal.
/// Suffixes such as the exponential suffix (i.e. `1.2e7`) are not supported,
/// nor are decimals.
pub fn parse_number(val: &str) -> Option<u64> {
    match val.chars().nth(0) {
        Some('0') => match val.chars().nth(1) {
            Some('b') => NumberBase::BINARY.parse_number(val[2..]),
            Some('o') => NumberBase::OCTAL.parse_number(val[2..]),
            Some('d') => NumberBase::DECIMAL.parse_number(val[2..]),
            Some('x') => NumberBase::HEX.parse_number(val[2..]),
        },
        _ => NumberBase::BINARY.parse_number(val)
    }
}
