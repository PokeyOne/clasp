use super::*;

#[test]
fn parse_single_number_literal() -> Result<(), AstConstructionError> {
    let input = vec![Token::Literal(TokenLiteral::Number(1.0))];
    let expected = Ast::new(vec![
        Expression::Literal(AstLiteral::Number(1.0))
    ]);

    assert_eq!(expected, parse_tree(input)?);

    Ok(())
}