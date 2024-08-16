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

use crate::{memory::Memory, opcodes::Opcode, registers::Registers, traps::TrapCode};

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

pub enum InstructionType<T> {
    Continue(T),
    Halt,
}

impl Instruction {
    pub fn try_from_bits(bits: u16) -> Result<InstructionType<Self>, String> {
        let opcode = Opcode::from(bits);

        let instruction = match opcode {
            Opcode::ADD => InstructionType::Continue(Self::Add(Add::from_bits(bits))),
            Opcode::BR => InstructionType::Continue(Self::Br(Br::from_bits(bits))),
            Opcode::JMP => InstructionType::Continue(Self::Jmp(Jmp::from_bits(bits))),
            Opcode::JSR => InstructionType::Continue(Self::Jsr(Jsr::from_bits(bits))),
            Opcode::LD => InstructionType::Continue(Self::Ld(Ld::from_bits(bits))),
            Opcode::LDI => InstructionType::Continue(Self::Ldi(Ldi::from_bits(bits))),
            Opcode::LDR => InstructionType::Continue(Self::Ldr(Ldr::from_bits(bits))),
            Opcode::LEA => InstructionType::Continue(Self::Lea(Lea::from_bits(bits))),
            Opcode::NOT => InstructionType::Continue(Self::Not(Not::from_bits(bits))),
            Opcode::ST => InstructionType::Continue(Self::St(St::from_bits(bits))),
            Opcode::STI => InstructionType::Continue(Self::Sti(Sti::from_bits(bits))),
            Opcode::STR => InstructionType::Continue(Self::Str(Str::from_bits(bits))),
            Opcode::TRAP => match Trap::from_bits(bits) {
                trap if trap.trap_code == TrapCode::HALT => InstructionType::Halt,
                trap => InstructionType::Continue(Self::Trap(trap)),
            },
            Opcode::AND => InstructionType::Continue(Self::And(And::from_bits(bits))),
            _ => todo!(),
        };

        Ok(instruction)
    }

    pub fn execute(&self, registers: &mut Registers, memory: &mut Memory) {
        match self {
            Instruction::Add(x) => x.execute(registers),
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
