mod add;
mod br;
mod jmp;

pub use add::Add;
pub use br::Br;
pub use jmp::Jmp;

use crate::{memory::Memory, opcodes::Opcode, registers::Registers};

pub enum Instruction {
    Add(Add),
    Br(Br),
    Jmp(Jmp),
}

impl Instruction {
    pub fn try_from_bits(bits: u16) -> Result<Self, String> {
        let opcode = Opcode::try_from(bits).unwrap();

        match opcode {
            Opcode::ADD => Ok(Self::Add(Add::from_bits(bits))),
            Opcode::BR => Ok(Self::Br(Br::from_bits(bits))),
            Opcode::JMP => Ok(Self::Jmp(Jmp::from_bits(bits))),
            _ => todo!(),
        }
    }

    pub fn execute(&self, registers: &mut Registers, memory: &mut Memory) {
        match self {
            Instruction::Add(x) => x.execute(registers, memory),
            Instruction::Br(x) => x.execute(registers),
            Instruction::Jmp(x) => x.execute(registers),
            _ => todo!(),
        }
    }
}
