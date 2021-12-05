use super::*;

fn true_literal() -> Token {
    Token::new(TokenKind::Literal(LiteralKind::Bool), "true".to_string())
}

fn false_literal() -> Token {
    Token::new(TokenKind::Literal(LiteralKind::Bool), "false".to_string())
}

#[test]
fn single_op() -> Result<(), &'static str> {
    let input_tokens = vec![
        true_literal(),
        Token::try_from_kind(TokenKind::Gt).unwrap(),
        false_literal()
    ];

    let output_tokens = map_tokens(&input_tokens)?;
    println!("in: {:?}", input_tokens);
    println!("out: {:?}", output_tokens);
    // out should eq [Literal(Bool(true)), Operator(Gt), Literal(Bool(false))]
    // TODO: actually assert automated, for now just high jacking to have fun

    let ce = ConditionalExpression::from_tokens(&input_tokens);
    println!("ce: {:#?}", ce);

    Ok(())
}

#[test]
#[ignore = "Not implemented"]
fn multi_op_with_brackets() -> Result<(), &'static str> {
    /*
    let input_tokens = vec![
        Token::new(TokenKind::Bracket(BracketKind::Openning, "(".to_string())),
        true_literal(),
        Token::try_from_kind(TokenKind::Gt).unwrap(),

];*/
    Err("not implemented")
}
