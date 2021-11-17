use super::*;

#[test]
fn single_op() -> Result<(), &'static str> {
    let input_tokens = vec![
        Token::new(TokenKind::Literal(LiteralKind::Bool), "true".to_string()),
        Token::try_from_kind(TokenKind::Gt).unwrap(),
        Token::new(TokenKind::Literal(LiteralKind::Bool), "false".to_string())
    ];

    let output_tokens = map_tokens(&input_tokens)?;
    println!("in: {:?}", input_tokens);
    println!("out: {:?}", output_tokens);
    // TODO: actually assert automated, for now just high jacking to have fun

    let ce = ConditionalExpression::from_tokens(&input_tokens);
    println!("ce: {:#?}", ce);

    Ok(())
}
