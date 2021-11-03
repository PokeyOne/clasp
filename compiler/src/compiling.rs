mod tokenization;

use tokenization::token::Token;

pub fn compile_text(input: String) -> String {
    let _ast: Vec<Token> = tokenization::tokenize_text(input);
    // TODO: this

    return "nop\nend\n".to_string();
}
