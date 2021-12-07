#[cfg(test)]
mod tests;

use crate::parsing::ast::{Expression, Statement, Ast, Literal};

pub enum AssemblyGenerationError {
    #[allow(dead_code)]
    NotImplemented
}

pub struct AssemblyBuilder {
    instructions: Vec<InstructionBuilder>,
    data_segment: DataSegmentBuilder
}

struct InstructionBuilder {
    operator: String,
    operands: Vec<String>
}

struct DataSegmentBuilder {
    data: Vec<(String, String)> // name, value
}

impl AssemblyBuilder {
    pub fn new() -> AssemblyBuilder {
        AssemblyBuilder {
            instructions: Vec::new(),
            data_segment: DataSegmentBuilder::new()
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn build_source(self) -> String {
        let mut source = String::new();

        for instruction in self.instructions {
            source.push_str(&format!("{}\n", instruction.build_source()));
        }

        source
    }

    fn format_literal(&mut self, literal: Literal) -> String {
        match literal {
            Literal::Number(number) => format!("{}", number),
            Literal::String(string) => format!("\"{}\"", string),
            Literal::Bool(bool) => format!("{}", bool)
        }
    }
}

impl InstructionBuilder {
    pub fn new(operator: String, operands: Vec<String>) -> InstructionBuilder {
        InstructionBuilder { operator, operands }
    }

    pub fn build_source(self) -> String {
        format!("{} {}", self.operator, self.operands.join(" "))
    }
}

impl DataSegmentBuilder {
    pub fn new() -> DataSegmentBuilder {
        DataSegmentBuilder {
            data: Vec::new()
        }
    }

    pub fn add_data(&mut self, name: String, value: String) {
        self.data.push((name, value));
    }

    pub fn build_source(self) -> String {
        if self.data.is_empty() {
            return String::new();
        }

        let mut source = String::new();
        source.push_str(".section data\n");

        for (name, value) in self.data {
            source.push_str(&format!("  .{} \"{}\"\n", name, value));
        }

        source
    }
}

pub fn generate_assembly(ast: Ast) -> Result<AssemblyBuilder, AssemblyGenerationError> {
    let mut builder = AssemblyBuilder::new();

    for expression in ast.into_expressions() {
        generate_expression_assembly(&mut builder, expression)?;
    }

    Ok(builder)
}

// TODO: Remove the idea of a top-level expression, only statements.
fn generate_expression_assembly(
    builder: &mut AssemblyBuilder,
    expression: Expression
) -> Result<(), AssemblyGenerationError> {
    match expression {
        Expression::Literal(literal) => {
            builder.add_instruction(InstructionBuilder::new(
                "push".to_string(),
                vec![format_literal(literal)]
            ));
        },
        _ => Err(AssemblyGenerationError::NotImplemented)
    }

    Ok(())
}

