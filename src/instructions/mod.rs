use crate::{memory::Memory, registers::Registers};

pub trait Instruction {
    fn from_bits(bits: u16) -> Self
    where
        Self: Sized;
    fn execute(&self, registers: &mut Registers, memory: &mut Memory);
}
