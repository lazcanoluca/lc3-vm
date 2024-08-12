use crate::{
    memory::Memory,
    registers::{Register, Registers},
    utils::sign_extend,
};

#[derive(Debug)]
pub struct Ldi {
    dr: Register,
    pc_offset9: u16,
}

impl Ldi {
    pub fn from_bits(bits: u16) -> Self {
        let dr = Register::try_from((bits >> 9) & 0b111).unwrap();
        let pc_offset9 = sign_extend(bits & 0x1ff, 9);

        Self { dr, pc_offset9 }
    }

    pub fn execute(&self, registers: &mut Registers, memory: &mut Memory) {
        let inner = memory.read(registers.get(Register::PC).wrapping_add(self.pc_offset9));
        let val = memory.read(inner);
        registers.set(self.dr, val);
        registers.update_flags(self.dr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_to_ldi() {
        // LDI   R3, 0x7
        let bits = 0b0010_011_000000111;

        let instruction = Ldi::from_bits(bits);

        assert_eq!(instruction.dr, Register::R3);
        assert_eq!(instruction.pc_offset9, 7);
    }

    #[test]
    fn test_bits_to_ldi_works() {
        let mut memory = Memory::default();
        memory.write(0x8000, 0x9999);
        memory.write(0x9999, 5);

        let mut registers = Registers::default();
        registers.set(Register::PC, 0x1000);

        let instruction = Ldi {
            dr: Register::R3,
            pc_offset9: 0x7000,
        };

        assert_eq!(registers.get(Register::R3), 0);
        assert_eq!(memory.read(0x8000), 0x9999);

        instruction.execute(&mut registers, &mut memory);

        assert_eq!(registers.get(Register::R3), 5);
    }
}
