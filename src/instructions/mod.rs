mod add;
mod br;
mod jmp;
mod jsr;
mod ld;
mod ldi;
mod ldr;

pub use add::Add;
pub use br::Br;
pub use jmp::Jmp;
pub use jsr::Jsr;
pub use ld::Ld;
pub use ldi::Ldi;
pub use ldr::Ldr;

use crate::{memory::Memory, opcodes::Opcode, registers::Registers};

pub enum Instruction {
    Add(Add),
    Br(Br),
    Jmp(Jmp),
    Jsr(Jsr),
    Ld(Ld),
    Ldi(Ldi),
    Ldr(Ldr),
}

impl Instruction {
    pub fn try_from_bits(bits: u16) -> Result<Self, String> {
        let opcode = Opcode::try_from(bits).unwrap();

        match opcode {
            Opcode::ADD => Ok(Self::Add(Add::from_bits(bits))),
            Opcode::BR => Ok(Self::Br(Br::from_bits(bits))),
            Opcode::JMP => Ok(Self::Jmp(Jmp::from_bits(bits))),
            Opcode::JSR => Ok(Self::Jsr(Jsr::from_bits(bits))),
            Opcode::LD => Ok(Self::Ld(Ld::from_bits(bits))),
            Opcode::LDI => Ok(Self::Ldi(Ldi::from_bits(bits))),
            Opcode::LDR => Ok(Self::Ldr(Ldr::from_bits(bits))),
            _ => todo!(),
        }
    }

    pub fn execute(&self, registers: &mut Registers, memory: &mut Memory) {
        match self {
            Instruction::Add(x) => x.execute(registers, memory),
            Instruction::Br(x) => x.execute(registers),
            Instruction::Jmp(x) => x.execute(registers),
            Instruction::Jsr(x) => x.execute(registers),
            Instruction::Ld(x) => x.execute(registers, memory),
            Instruction::Ldi(x) => x.execute(registers, memory),
            Instruction::Ldr(x) => x.execute(registers, memory),
            _ => todo!(),
        }
    }
}
