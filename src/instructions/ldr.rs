use crate::{
    memory::Memory,
    registers::{Register, Registers},
    utils::sign_extend,
};

#[derive(Debug)]
pub struct Ldr {
    dr: Register,
    base_r: Register,
    offset6: u16,
}

impl Ldr {
    pub fn from_bits(bits: u16) -> Self {
        let dr = Register::try_from((bits >> 9) & 0b111).unwrap();
        let base_r = Register::try_from((bits >> 6) & 0b111).unwrap();
        let offset6 = sign_extend(bits & 0b111111, 6);

        Self {
            dr,
            base_r,
            offset6,
        }
    }

    pub fn execute(&self, registers: &mut Registers, memory: &mut Memory) {
        let val = memory.read(registers.get(self.base_r) + self.offset6);
        registers.set(self.dr, val);
        registers.update_flags(self.dr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_to_ldr() {
        // LDR  R3, R1, 0x7
        let bits = 0b0110_011_001_000111;

        let instruction = Ldr::from_bits(bits);

        assert_eq!(instruction.base_r, Register::R1);
        assert_eq!(instruction.dr, Register::R3);
        assert_eq!(instruction.offset6, 7);
    }

    #[test]
    fn test_ldr_works() {
        let mut memory = Memory::default();
        memory.write(0x8000, 9);

        let mut registers = Registers::default();
        registers.set(Register::R1, 0x1000);

        let instruction = Ldr {
            dr: Register::R3,
            base_r: Register::R1,
            offset6: 0x7000,
        };

        assert_eq!(registers.get(Register::R3), 0);
        assert_eq!(memory.read(0x8000), 9);

        instruction.execute(&mut registers, &mut memory);

        assert_eq!(registers.get(Register::R3), 9);
    }
}
