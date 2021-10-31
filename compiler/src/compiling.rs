mod tokenization;

use tokenization::ast::AbstractSyntaxTree;

pub fn compile_text(input: String) -> String {
    let _ast: AbstractSyntaxTree = tokenization::tokenize_text(input);
    // TODO: this

    return "nop\nend\n".to_string();
}
