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
