mod add;

pub use add::Add;

use crate::{memory::Memory, opcodes::Opcode, registers::Registers};

pub enum Instruction {
    Add(Add),
}

impl Instruction {
    pub fn try_from_bits(bits: u16) -> Result<Self, String> {
        let opcode = Opcode::try_from(bits).unwrap();

        match opcode {
            Opcode::ADD => Ok(Self::Add(Add::from_bits(bits))),
            _ => todo!(),
        }
    }

    pub fn execute(&self, registers: &mut Registers, memory: &mut Memory) {
        match self {
            Instruction::Add(x) => x.execute(registers, memory),
            _ => todo!(),
        }
    }
}
