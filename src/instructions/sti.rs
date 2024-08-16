use crate::{
    memory::Memory,
    registers::{Register, Registers},
    utils::sign_extend,
};

#[derive(Debug)]
pub struct Sti {
    sr: Register,
    pc_offset9: u16,
}

impl Sti {
    pub fn from_bits(bits: u16) -> Self {
        let sr = Register::try_from((bits >> 9) & 0b111).unwrap();
        let pc_offset9 = sign_extend(bits & 0x1ff, 9);

        Self { sr, pc_offset9 }
    }

    pub fn execute(&self, registers: &mut Registers, memory: &mut Memory) {
        let val = registers.get(self.sr);
        let inner = memory.read(registers.get(Register::PC).wrapping_add(self.pc_offset9));

        memory.write(inner, val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_to_sti() {
        // ST   R3, 0x7
        let bits = 0b1011_011_000000111;

        let instruction = Sti::from_bits(bits);

        assert_eq!(instruction.sr, Register::R3);
        assert_eq!(instruction.pc_offset9, 7);
    }

    #[test]
    fn test_sti_works() {
        let mut memory = Memory::default();
        memory.write(0x1007, 0x2000);

        let mut registers = Registers::default();
        registers.set(Register::PC, 0x1000);
        registers.set(Register::R3, 0x99);

        let instruction = Sti {
            sr: Register::R3,
            pc_offset9: 0x7,
        };

        assert_eq!(memory.read(0x2000), 0);

        instruction.execute(&mut registers, &mut memory);

        assert_eq!(memory.read(0x2000), 0x99);
    }
}
