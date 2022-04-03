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
        root: Expression::Series(vec![Expression::Series(vec![])])
    };

    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn test_series_of_idents() -> Result<(), Error> {
    let input = tokenize("(foo bar baz)").expect("tokenization error");
    let result = parse(input)?;
    let expected = Ast {
        root: Expression::Series(vec![Expression::Series(vec![
            Expression::Ident("foo".to_string()),
            Expression::Ident("bar".to_string()),
            Expression::Ident("baz".to_string()),
        ])])
    };

    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn test_nested_lists() -> Result<(), Error> {
    let input = tokenize("(add \"this\" (add this))").expect("tokenization error");
    let result = parse(input)?;
    let expected = Ast {
        root: Expression::Series(vec![Expression::Series(vec![
            Expression::Ident("add".to_string()),
            Expression::Literal(Literal::StringLiteral("this".to_string())),
            Expression::Series(vec![
                Expression::Ident("add".to_string()),
                Expression::Ident("this".to_string()),
            ]),
        ])])
    };

    assert_eq!(result, expected);

    Ok(())
}
