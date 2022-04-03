use super::*;
use crate::tokenization::tokenize;

#[test]
fn test_empty() -> Result<(), Error> {
    let result = parse(vec![])?;
    let expected = Ast {
        root: Expression::Series(Vec::new())
    };

    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn test_empty_series() -> Result<(), Error> {
    let input = tokenize("()").expect("tokenization error");
    let result = parse(input)?;
    let expected = Ast {
        root: Expression::Series(Vec::new())
    };

    assert_eq!(result, expected);

    Ok(())
}
