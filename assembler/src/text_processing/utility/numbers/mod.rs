use ascii::AsciiChar;

#[cfg(test)]
mod tests;

/// Represents a number's base, such as base 10 for regular decimal, or base 16
/// for hex numbers.
pub struct NumberBase {
    place_value: u8
}

impl NumberBase {
    /// Base 2 such as 0b10101011
    pub const BINARY: Self = Self { place_value: 2 };
    /// Base 8 such as 0o1723
    pub const OCTAL: Self = Self { place_value: 8 };
    /// Base 10 such 1503829
    pub const DECIMAL: Self = Self { place_value: 10 };
    /// Base 16 such as 0xFE350D
    pub const HEX: Self = Self { place_value: 16 };

    /// Parses a number assuming this base, returns Err(String) with the reason
    /// if cannot parse. Input **must not** include the base prefix, for that
    /// call the general method `text_processing::utility::parse_number`.
    pub fn parse_number(&self, val: &str) -> Result<u64, String> {
        if val == "" {
            return Err(String::from("Empty String"));
        }

        let mut result: u64 = 0;

        let mut i = 0;
        loop {
            match val.chars().nth(i) {
                Some(val) => match Self::digit_value(val) {
                    Some(n) if n < self.place_value => {
                        let mul = result.checked_mul(self.place_value as u64);
                        if mul.is_none() {
                            return Err(String::from("Integer Overflow"))
                        }
                        result = mul.unwrap() + (n as u64);
                    },
                    _ => return Err(String::from("Invalid digit"))
                },
                None => break
            };

            i += 1;
        }

        Ok(result)
    }

    /// Given a single alpha numeric digit 0-9 or a-f, this will return its
    /// value as an unsigned integer.
    ///
    /// # Examples
    /// ```
    /// # use clasm_compiler::text_processing::utility::numbers::NumberBase;
    /// let four = NumberBase::digit_value('4');
    /// let eleven = NumberBase::digit_value('b');
    /// let fifteen = NumberBase::digit_value('F');
    /// let none = NumberBase::digit_value('G');
    ///
    /// assert_eq!(4, four.unwrap());
    /// assert_eq!(11, eleven.unwrap());
    /// assert_eq!(15, fifteen.unwrap());
    /// assert!(none.is_none());
    /// ```
    pub fn digit_value(val: char) -> Option<u8> {
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
///
/// Below is the ebnf that describes the numbers
/// ```ebnf
/// number ::= prefixed_number | staight_decimal
/// prefixed_number ::= '0x' straight_hex | '0b' straight_binary | '0o' straight_octal | '0d' straight_decimal
/// straight_hex ::= hex_digit+
/// straight_binary ::= binary_digit+
/// straight_octal ::= octal_digit+
/// straight_decimal ::= decimal_digit+
/// binary_digit ::= '0' | '1'
/// octal_digit ::= binary_digit | '2' | '3' | '4' | '5' | '6' | '7'
/// decimal_digit ::= octal_digit | '8' | '9'
/// hex_digit ::= decimal_digit | lower_hex_digit | upper_hex_digit
/// lower_hex_digit ::= 'a' | 'b' | 'c' | 'd' | 'e' | 'f'
/// upper_hex_digit ::= 'A' | 'B' | 'C' | 'D' | 'E' | 'F'
/// ```
///
/// # Examples
///
/// Passing simple decimals should work.
/// ```
/// # use clasm_compiler::text_processing::utility::numbers::parse_number;
/// let a = parse_number("193");
/// assert_eq!(193, a.unwrap());
/// ```
///
/// Other types of integers work as well.
/// ```
/// # use clasm_compiler::text_processing::utility::numbers::parse_number;
/// let a = parse_number("0d193");
/// assert_eq!(193, a.unwrap());
///
/// let a = parse_number("0xB4F");
/// assert_eq!(0xB4F, a.unwrap());
///
/// let a = parse_number("0b10101110");
/// assert_eq!(0b10101110, a.unwrap());
///
/// let a = parse_number("0o17");
/// assert_eq!(0o17, a.unwrap());
/// ```
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
        None => NumberBase::DECIMAL.parse_number(val)
    }
}
