use super::*;

#[derive(Debug)]
enum JumpType {
    Address,
    Literal
}

pub fn jmp_provider(mem: &mut Memory, program_memory: &mut Memory, pc: &mut MemoryLocation) {
    let t = match program_memory.read(pc.clone()) {
        Ok(val) => match val {
            JMP_ADDR_CODE => JumpType::Address,
            JMP_LIT_CODE => JumpType::Literal,
            _ => panic!("The jump provider is can't handle a non-jump instructions")
        },
        Err(t) => panic!("{:?}", t)
    };

    *pc += WORD_SIZE as u64;

    let destination: Word = match program_memory.read(pc.clone()) {
        // TODO: subtact the 8 before hand again
        Ok(val) => val - 8, // subtract 8 because of stripping the signature
        Err(t) => panic!("{:?}", t)
    };

    let destination: Word = match t {
        JumpType::Address => match mem.read(destination) {
            Ok(val) => val,
            Err(t) => panic!("{:?}", t)
        },
        JumpType::Literal => destination
    };

    println!("Going to location {:#X}", &destination);
    *pc = destination;
}
