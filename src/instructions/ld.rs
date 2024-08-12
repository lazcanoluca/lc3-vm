use crate::{
    memory::Memory,
    registers::{Register, Registers},
    utils::sign_extend,
};

#[derive(Debug)]
pub struct Ld {
    dr: Register,
    pc_offset9: u16,
}

impl Ld {
    pub fn from_bits(bits: u16) -> Self {
        let dr = Register::try_from((bits >> 9) & 0b111).unwrap();
        let pc_offset9 = sign_extend(bits & 0x1ff, 9);

        Self { dr, pc_offset9 }
    }

    pub fn execute(&self, registers: &mut Registers, memory: &mut Memory) {
        let val = memory.read(registers.get(Register::PC).wrapping_add(self.pc_offset9));
        registers.set(self.dr, val);
        registers.update_flags(self.dr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_to_ld() {
        // LD   R3, 0x7
        let bits = 0b0010_011_000000111;

        let instruction = Ld::from_bits(bits);

        assert_eq!(instruction.dr, Register::R3);
        assert_eq!(instruction.pc_offset9, 7);
    }

    #[test]
    fn test_ld_works() {
        let mut memory = Memory::default();
        memory.write(0x8000, 9);

        let mut registers = Registers::default();
        registers.set(Register::PC, 0x1000);

        let instruction = Ld {
            dr: Register::R3,
            pc_offset9: 0x7000,
        };

        assert_eq!(registers.get(Register::R3), 0);
        assert_eq!(memory.read(0x8000), 9);

        instruction.execute(&mut registers, &mut memory);

        assert_eq!(registers.get(Register::R3), 9);
    }
}
