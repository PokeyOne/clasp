use super::*;

mod literals;

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
    assert_eq!(result.0, vec!["cool".to_string(), "test".to_string(), "of the system".to_string()]);
    assert_eq!(result.1, expected);
}

#[test]
fn preprocess_string_with_spaces_and_escaped_characters() {
    let input = "this is a \"cool\" \"test\" \" of \\\"the\\\"\\n system\"".to_string();
    let expected = "this is a  \"0\"   \"1\"   \"2\" ".to_string();

    let result = preprocess_strings(&input);
    assert_eq!(result.0, vec!["cool".to_string(), "test".to_string(), " of \"the\"\n system".to_string()]);
    assert_eq!(result.1, expected);
}
