use super::*;
use ascii::AsciiChar;

pub fn outr_provider(mem: &mut Memory, program_memory: &mut Memory, pc: &mut MemoryLocation) {
    let inst_code = match program_memory.read(pc.clone()) {
        Ok(val) => val,
        Err(t) => panic!(
            "Could not read instruction from memory at program counter in outr_provider: {:?}",
            t
        )
    };
    *pc += WORD_SIZE as u64;

    let value_to_print = match inst_code {
        OUTR_ADDR_CODE => {
            // TODO: some sort of generic method to read the address from an address
            let address = match program_memory.read(pc.clone()) {
                Ok(val) => val,
                Err(t) => panic!("{:?}", t)
            };

            // This is the value to print
            match mem.read(address) {
                Ok(val) => val,
                Err(t) => panic!("{:?}", t)
            }
        }
        OUTR_LIT_CODE => match program_memory.read(pc.clone()) {
            Ok(val) => val,
            Err(t) => panic!("{:?}", t)
        },
        _ => panic!("unexpected op code for outr method")
    };
    let value_to_print = (value_to_print & 0b0111_1111) as u8;
    let value_to_print = AsciiChar::from_ascii(value_to_print).unwrap();

    *pc += WORD_SIZE as u64;

    print!("{}", value_to_print);
}
