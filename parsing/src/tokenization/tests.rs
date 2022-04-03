use super::*;

type TestResult = Result<(), Error>;

#[test]
fn test_empty() -> TestResult {
    let result = tokenize("")?;

    assert_eq!(0, result.len());

    Ok(())
}

#[test]
fn test_just_brackets() -> TestResult {
    let result = tokenize("(())")?;

    let expected = vec![OpenBracket, OpenBracket, CloseBracket, CloseBracket];

    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn test_identifier() -> TestResult {
    let result = tokenize("test")?;

    let expected = vec![Identifier("test".to_string())];

    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn test_identifier_in_brackets() -> TestResult {
    let result = tokenize("(test)")?;

    let expected = vec![OpenBracket, Identifier("test".to_string()), CloseBracket];

    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn test_multiple_identifiers() -> TestResult {
    let result = tokenize("(food bar)")?;

    let expected = vec![
        OpenBracket,
        Identifier("food".to_string()),
        Identifier("bar".to_string()),
        CloseBracket,
    ];

    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn test_string_literals() -> TestResult {
    let result = tokenize("(println \"hello\")")?;

    let expected = vec![
        OpenBracket,
        Identifier("println".to_string()),
        StringLiteral("hello".to_string()),
        CloseBracket,
    ];

    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn test_string_with_escapes() -> TestResult {
    let result = tokenize("\"this is a\ntest of \\\"escaped\"")?;

    let expected = vec![StringLiteral("this is a\ntest of \"escaped".to_string())];

    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn test_unterminated_string() {
    match tokenize("\"blah") {
        Ok(_) => panic!("Unterminated string literal should fail"),
        Err(Error { line, col, kind }) => {
            assert_eq!(kind, ErrorKind::UnterminatedString);
            assert_eq!(line, 1);
            assert_eq!(col, 5);
        }
    }
}
