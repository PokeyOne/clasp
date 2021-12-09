#[cfg(test)]
mod tests;

mod build_functions;

use std::collections::HashMap;
use clasp_common::io::CCLASP_SIGNATURE;

/// A utility for building a program.
#[derive(Debug)]
pub struct ProgramBuilder {
    pub program: Vec<InstructionBuilder>
}

/// A utility for building an instruction.
#[derive(Debug)]
pub struct InstructionBuilder {
    pub instruction: InstructionKind,
    pub operands: Vec<Operand>,
}

/// A description of an instruction and its operands.
#[derive(Debug)]
pub struct InstructionKindDescription {
    pub name: &'static str,
    pub operand_types: Vec<OperandType>
}
/// An alias for an instruction kind description.
type IKD = InstructionKindDescription;

/// The kind of an instruction.
#[derive(Debug)]
pub enum InstructionKind {
    Nop,
    Mov,
    End
}

/// The type of operand that an instruction argument can have.
// TODO: Eventually this should not allow dead code, but for now it's fine
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum OperandType {
    /// The name o a register, such as "ga"
    Register,
    /// A literal value surrounded by round brackets, such as (1)
    Immediate,
    /// A raw number in assembly such as "0x1234"e,
    Address,
    /// An identifier that refers to a location in the program, starts with a ':'
    Label,
    /// An identifier in the data section such as ".d0001", starts with '.'
    Data,
    /// Register, Immediate, Address, or Data.
    Valuable,
    /// Register or Address.
    Destination,
    /// Any of the above.
    Any,
    /// A custom subset of the above.
    Some(Vec<OperandType>),
}

#[derive(Debug)]
pub enum Operand {
    Register(String),
    Immediate(u64),
    Address(u64),
    Label(String),
    Data(String)
}

#[derive(Debug)]
pub struct Instruction {
    kind: InstructionKind,
    operands: Vec<Operand>
}

#[derive(Debug)]
pub enum InstructionBuildError {
    #[allow(dead_code)]
    NotImplemented,
    WrongNumberOfOperands,
}
use InstructionBuildError::*;

impl ProgramBuilder {
    /// Creates a new program builder with no instructions.
    pub fn new() -> ProgramBuilder {
        ProgramBuilder {
            program: Vec::new()
        }
    }

    /// Adds an instruction to the program.
    pub fn add_instruction(&mut self, inst: InstructionBuilder) {
        self.program.push(inst);
    }

    /// Builds all the instructions in the program into a collection of
    /// instructions. To build the program binary, use the `build_binary`
    /// method.
    pub fn build(self) -> Result<Vec<Instruction>, InstructionBuildError> {
        if self.program.len() == 0 {
            return Ok(
                vec![
                    Instruction::new(InstructionKind::End, vec![])
                ]
            );
        }

        let mut instructions = Vec::new();
        instructions.reserve(self.program.len());
        for inst in self.program {
            instructions.push(inst.build()?);
        }

        Ok(instructions)
    }

    /// Builds the program into a binary.
    pub fn build_binary(self) -> Result<Vec<u8>, InstructionBuildError> {
        let instructions = self.build()?;
        let mut binary = Vec::new();
        binary.extend_from_slice(&CCLASP_SIGNATURE);
        // Length times 8 (64 bit) bytes is the bare minimum size of a binary.
        binary.reserve(instructions.len() * 8);
        for inst in instructions {
            binary.extend_from_slice(&inst.build()?);
        }

        Ok(binary)
    }
}

impl InstructionKindDescription {
    pub fn new(name: &'static str, operand_types: Vec<OperandType>) -> Self {
        Self {
            name,
            operand_types,
        }
    }

    pub fn get_operand_count(&self) -> usize {
        self.operand_types.len()
    }
}

impl InstructionKind {
    pub fn get_description(&self) -> InstructionKindDescription {
        match self {
            InstructionKind::Nop => IKD::new("nop", Vec::new()),
            InstructionKind::Mov => IKD::new(
                "mov",
                [OperandType::Valuable, OperandType::Destination].to_vec(),
            ),
            InstructionKind::End => IKD::new("end", Vec::new()),
        }
    }

    pub fn get_name(&self) -> &'static str {
        self.get_description().name
    }

    pub fn get_operand_count(&self) -> usize {
        self.get_description().get_operand_count()
    }

    pub fn get_operand_types(&self) -> Vec<OperandType> {
        self.get_description().operand_types
    }
}

impl OperandType {
    pub fn is_valid_operand(&self, operand: &Operand) -> bool {
        match self {
            OperandType::Register => match operand {
                Operand::Register(_) => true,
                _ => false,
            },
            OperandType::Immediate => match operand {
                Operand::Immediate(_) => true,
                _ => false,
            },
            OperandType::Address => match operand {
                Operand::Address(_) => true,
                _ => false,
            },
            OperandType::Label => match operand {
                Operand::Label(_) => true,
                _ => false,
            },
            OperandType::Data => match operand {
                Operand::Data(_) => true,
                _ => false,
            },
            OperandType::Valuable => match operand {
                Operand::Register(_) => true,
                Operand::Immediate(_) => true,
                Operand::Address(_) => true,
                Operand::Data(_) => true,
                _ => false,
            },
            OperandType::Destination => match operand {
                Operand::Register(_) => true,
                Operand::Address(_) => true,
                _ => false,
            },
            OperandType::Any => true,
            OperandType::Some(subset) => subset.iter().any(|ot| ot.is_valid_operand(operand)),
        }
    }
}

impl InstructionBuilder {
    pub fn new(instruction: InstructionKind) -> Self {
        Self {
            instruction,
            operands: Vec::new()
        }
    }

    pub fn add_operand(&mut self, operand: Operand) {
        self.operands.push(operand);
    }

    pub fn verify(&self) -> Result<(), InstructionBuildError> {
        let inst_desc = self.instruction.get_description();
        if self.operands.len() != inst_desc.get_operand_count() {
            return Err(WrongNumberOfOperands);
        }

        for (i, op) in self.operands.iter().enumerate() {
            let expected_type = &inst_desc.operand_types[i];
            if !expected_type.is_valid_operand(op) {
                return Err(WrongNumberOfOperands);
            }
        }

        Ok(())
    }

    pub fn build(self) -> Result<Instruction, InstructionBuildError> {
        self.verify()?;

        Ok(Instruction {
            kind: self.instruction,
            operands: self.operands
        })
    }
}

impl Instruction {
    pub fn new(kind: InstructionKind, operands: Vec<Operand>) -> Self {
        Self {
            kind,
            operands
        }
    }

    pub fn get_kind(&self) -> &InstructionKind {
        &self.kind
    }

    pub fn get_operands(&self) -> &Vec<Operand> {
        &self.operands
    }

    pub fn build(&self) -> Result<Vec<u8>, InstructionBuildError> {
        use InstructionKind::*;
        match &self.kind {
            Nop => build_functions::nop(),
            Mov => build_functions::mov(self),
            End => build_functions::end(),
        }
    }
}
