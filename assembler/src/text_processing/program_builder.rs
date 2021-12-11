#[cfg(test)]
mod tests;

pub mod build_functions;

use std::collections::HashMap;
use clasp_common::io::CCLASP_SIGNATURE;
use clasp_common::data_constants::get_register_address;

/// A utility for building a program.
#[derive(Debug)]
pub struct ProgramBuilder {
    pub program: Vec<InstructionBuilder>
}

/// A utility for building an instruction.
#[derive(Debug)]
pub struct InstructionBuilder {
    pub instruction: InstructionKind,
    pub operands: Vec<OperandBuilder>,
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

/// The type of operand that an assembly instruction argument can have.
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

/// The operand in the assembly language. It may be a reference to a register,
/// label, or data block; or it may be an immediate or address value.
#[derive(Debug)]
pub enum OperandBuilder {
    Register(String),
    Immediate(u64),
    Address(u64),
    Label(String),
    Data(String)
}

/// The compiled and dereferenced operand that can directly be used in the
/// instruction without the context of the program.
#[derive(Debug)]
pub enum Operand {
    Immediate(u64),
    Address(u64)
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
    ExpectedAddressableOperand,
    ExpectedExactOperand,
    InvalidRegister(String),
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
    pub fn is_valid_operand(&self, operand: &OperandBuilder) -> bool {
        match self {
            OperandType::Register => match operand {
                OperandBuilder::Register(_) => true,
                _ => false,
            },
            OperandType::Immediate => match operand {
                OperandBuilder::Immediate(_) => true,
                _ => false,
            },
            OperandType::Address => match operand {
                OperandBuilder::Address(_) => true,
                _ => false,
            },
            OperandType::Label => match operand {
                OperandBuilder::Label(_) => true,
                _ => false,
            },
            OperandType::Data => match operand {
                OperandBuilder::Data(_) => true,
                _ => false,
            },
            OperandType::Valuable => match operand {
                OperandBuilder::Register(_) => true,
                OperandBuilder::Immediate(_) => true,
                OperandBuilder::Address(_) => true,
                OperandBuilder::Data(_) => true,
                _ => false,
            },
            OperandType::Destination => match operand {
                OperandBuilder::Register(_) => true,
                OperandBuilder::Address(_) => true,
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

    pub fn add_operand(&mut self, operand: OperandBuilder) {
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
        let built_operands = self.build_operands()?;

        Ok(Instruction {
            kind: self.instruction,
            operands: built_operands,
        })
    }

    fn build_operands(&self) -> Result<Vec<Operand>, InstructionBuildError> {
        let mut operands = Vec::new();
        for operand in self.operands.iter() {
            match operand {
                OperandBuilder::Register(reg) => {
                    let addr = match get_register_address(reg) {
                        Some(addr) => addr,
                        None => return Err(InvalidRegister(reg.clone())),
                    };
                    operands.push(Operand::Address(addr));
                }
                OperandBuilder::Immediate(imm) => {
                    operands.push(Operand::Immediate(*imm));
                }
                OperandBuilder::Address(addr) => {
                    operands.push(Operand::Address(*addr));
                }
                // TODO: Implement a way to get label and data information
                //       from the program builder. At this point all labels and
                //       data are already in the program builder.
                OperandBuilder::Label(label) => {
                    return Err(NotImplemented);
                }
                OperandBuilder::Data(data) => {
                    return Err(NotImplemented);
                }
            }
        }

        Ok(operands)
    }
}

impl Operand {
    fn to_be_bytes(&self) -> [u8; 8] {
        match self {
            Operand::Immediate(imm) => imm.to_be_bytes(),
            Operand::Address(addr) => addr.to_be_bytes()
        }
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
