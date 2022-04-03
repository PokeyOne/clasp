#[cfg(test)]
mod tests;

mod tokens;

pub use tokens::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenizeError {
    InvalidToken(String)
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizeError> {
    todo!()
}

/// Determine if a character can be used as an identifier for a variable of
/// method.
///
/// # Examples
/// ```
/// use clasp_parsing::tokenization::is_valid_identifier_char;
///
/// // All a-z and A-Z are allowed anywhere in the identifier.
/// assert!(is_valid_identifier_char('c', true));
/// assert!(is_valid_identifier_char('c', false));
/// // Numbers are not allowed as the first character.
/// assert!(is_valid_identifier_char('0', false));
/// assert!(!is_valid_identifier_char('0', true));
/// ```
pub fn is_valid_identifier_char(c: char, is_first_char: bool) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '_' => true,
        '0'..='9' if !is_first_char => true,
        _ => false
    }
}
