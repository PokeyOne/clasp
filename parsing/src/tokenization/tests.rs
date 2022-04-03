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

    let expected = vec![
        OpenBracket,
        OpenBracket,
        CloseBracket,
        CloseBracket
    ];

    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn test_identifier() -> TestResult {
    let result = tokenize("test")?;

    let expected = vec![
        Identifier("test".to_string())
    ];

    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn test_identifier_in_brackets() -> TestResult {
    let result = tokenize("(test)")?;

    let expected = vec![
        OpenBracket,
        Identifier("test".to_string()),
        CloseBracket
    ];

    assert_eq!(result, expected);

    Ok(())
}
