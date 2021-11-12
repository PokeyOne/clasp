use ascii::AsciiChar;

#[cfg(test)]
mod tests;

struct NumberBase {
    place_value: u8
}

impl NumberBase {
    const BINARY: Self = Self { place_value: 2 };
    const OCTAL: Self = Self { place_value: 8 };
    const DECIMAL: Self = Self { place_value: 10 };
    const HEX: Self = Self { place_value: 16 };

    fn new(place_value: u8) -> Self {
        Self { place_value: place_value }
    }

    /// Parses a number assuming this base, returns Err(String) with the reason
    /// if cannot parse. Input **must not** include the base prefix, for that
    /// call the general method `text_processing::utility::parse_number`.
    fn parse_number(&self, val: &str) -> Result<u64, String> {
        if val == "" {
            return Err(String::from("Empty String"));
        }

        let mut result: u64 = 0;

        let mut i = 0;
        loop {
            match val.chars().nth(i) {
                Some(val) => match Self::digit_value(val) {
                    Some(n) if n < self.place_value => {
                        result *= self.place_value as u64;
                        result += n as u64;
                    },
                    _ => return Err(String::from("Invalid digit"))
                },
                None => break
            };

            i += 1;
        }

        Ok(result)
    }

    fn digit_value(val: char) -> Option<u8> {
        let ascii_char = AsciiChar::from_ascii(val).ok()?.to_ascii_uppercase();

        const a_byte: u8 = AsciiChar::A.as_byte();
        const z_byte: u8 = AsciiChar::Z.as_byte();

        if ascii_char.is_ascii_hexdigit() {
            match ascii_char.as_byte() {
                0x30..=0x39 => Some(ascii_char.as_byte() - 0x30),
                a_byte..=z_byte => Some(ascii_char.as_byte() - AsciiChar::A.as_byte() + 10),
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
pub fn parse_number(val: &str) -> Result<u64, String> {
    let result: Option<Result<u64, String>> = match val.chars().nth(0) {
        Some('0') => match val.chars().nth(1) {
            Some('b') => Some(NumberBase::BINARY.parse_number(&val[2..])),
            Some('o') => Some(NumberBase::OCTAL.parse_number(&val[2..])),
            Some('d') => Some(NumberBase::DECIMAL.parse_number(&val[2..])),
            Some('x') => Some(NumberBase::HEX.parse_number(&val[2..])),
            _ => None
        },
        _ => None
    };

    match result {
        Some(res) => res,
        None => NumberBase::BINARY.parse_number(val)
    }
}
