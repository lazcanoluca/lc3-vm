mod add;
mod and;
mod br;
mod jmp;
mod jsr;
mod ld;
mod ldi;
mod ldr;
mod lea;
mod not;
mod st;
mod sti;
mod str;
mod trap;

pub use add::Add;
pub use and::And;
pub use br::Br;
pub use jmp::Jmp;
pub use jsr::Jsr;
pub use ld::Ld;
pub use ldi::Ldi;
pub use ldr::Ldr;
pub use lea::Lea;
pub use not::Not;
pub use st::St;
pub use sti::Sti;
pub use str::Str;
pub use trap::Trap;

use crate::{memory::Memory, opcodes::Opcode, registers::Registers};

#[derive(Debug)]
pub enum Instruction {
    Add(Add),
    Br(Br),
    Jmp(Jmp),
    Jsr(Jsr),
    Ld(Ld),
    Ldi(Ldi),
    Ldr(Ldr),
    Lea(Lea),
    Not(Not),
    St(St),
    Sti(Sti),
    Str(Str),
    Trap(Trap),
    And(And),
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
            Opcode::LEA => Ok(Self::Lea(Lea::from_bits(bits))),
            Opcode::NOT => Ok(Self::Not(Not::from_bits(bits))),
            Opcode::ST => Ok(Self::St(St::from_bits(bits))),
            Opcode::STI => Ok(Self::Sti(Sti::from_bits(bits))),
            Opcode::STR => Ok(Self::Str(Str::from_bits(bits))),
            Opcode::TRAP => Ok(Self::Trap(Trap::from_bits(bits))),
            Opcode::AND => Ok(Self::And(And::from_bits(bits))),
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
            Instruction::Lea(x) => x.execute(registers),
            Instruction::Not(x) => x.execute(registers),
            Instruction::St(x) => x.execute(registers, memory),
            Instruction::Sti(x) => x.execute(registers, memory),
            Instruction::Str(x) => x.execute(registers, memory),
            Instruction::Trap(x) => x.execute(registers, memory),
            Instruction::And(x) => x.execute(registers),
        }
    }
}
