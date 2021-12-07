use super::*;
use crate::tokenization::tokens::Keyword;

mod brackets;
mod keywords;
mod literals;
mod symbols;

#[test]
fn preprocess_strings_should_not_change_when_no_strings() {
    let input = "this is a cool test".to_string();

    let result = preprocess_strings(&input);
    assert_eq!(result.0.len(), 0);
    assert_eq!(result.1, input);
}

#[test]
fn preprocess_string_should_change_single_token_string() {
    let input = "this is a \"cool\" test".to_string();
    let expected = "this is a  \"0\"  test".to_string();

    let result = preprocess_strings(&input);
    assert_eq!(result.0, vec!["cool".to_string()]);
    assert_eq!(result.1, expected);
}

#[test]
fn preprocess_string_should_change_multiple_token_string() {
    let input = "this is a \"cool\" \"test\" \"of the system\"".to_string();
    let expected = "this is a  \"0\"   \"1\"   \"2\" ".to_string();

    let result = preprocess_strings(&input);
    assert_eq!(
        result.0,
        vec![
            "cool".to_string(),
            "test".to_string(),
            "of the system".to_string()
        ]
    );
    assert_eq!(result.1, expected);
}

#[test]
fn preprocess_string_with_spaces_and_escaped_characters() {
    let input = "this is a \"cool\" \"test\" \" of \\\"the\\\"\\n system\"".to_string();
    let expected = "this is a  \"0\"   \"1\"   \"2\" ".to_string();

    let result = preprocess_strings(&input);
    assert_eq!(
        result.0,
        vec![
            "cool".to_string(),
            "test".to_string(),
            " of \"the\"\n system".to_string()
        ]
    );
    assert_eq!(result.1, expected);
}

#[test]
fn preprocess_string_should_separate_brackets() {
    let input = "this is a \"cool\" (\"test\")".to_string();
    let expected = "this is a  \"0\"   (  \"1\"  ) ".to_string();

    let result = preprocess_strings(&input);
    assert_eq!(result.0, vec!["cool".to_string(), "test".to_string()]);
    assert_eq!(result.1, expected);
}

#[test]
fn parse_a_bunch_of_symbols_stringed_together() {
    // TODO: Eventually there should be another test with this same thing wiht
    //       no spaces in between.
    let input = "fn (3 + 4) , { fn true} | fn || false ? this_is_a_variable ;".to_string();
    let expected: Vec<Token> = vec![
        Token::Keyword(Keyword::Fn),
        Token::OpenBracket,
        Token::Literal(Literal::Number(3.0)),
        Token::Symbol(Symbol::Plus),
        Token::Literal(Literal::Number(4.0)),
        Token::CloseBracket,
        Token::Symbol(Symbol::Comma),
        Token::OpenCurlyBracket,
        Token::Keyword(Keyword::Fn),
        boolean_literal!(true),
        Token::CloseCurlyBracket,
        Token::Symbol(Symbol::Bar),
        Token::Keyword(Keyword::Fn),
        Token::Symbol(Symbol::Or),
        boolean_literal!(false),
        Token::Symbol(Symbol::QuestionMark),
        Token::Identifier("this_is_a_variable".to_string()),
        Token::Symbol(Symbol::Semicolon),
    ];

    let result = tokenize(&input).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn ideal_case_main_function() {
    let input = String::from("(fn say_hello (println \"Hello, World!\"))");
    let expected: Vec<Token> = vec![
        Token::OpenBracket,
        Token::Keyword(Keyword::Fn),
        Token::Identifier("say_hello".to_string()),
        Token::OpenBracket,
        Token::Identifier("println".to_string()),
        Token::Literal(Literal::String("Hello, World!".to_string())),
        Token::CloseBracket,
        Token::CloseBracket,
    ];

    let result = tokenize(&input).unwrap();
    assert_eq!(result, expected);
}
