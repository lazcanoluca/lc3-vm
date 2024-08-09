use crate::{
    memory::Memory,
    registers::{Register, Registers},
    utils::sign_extend,
};

pub struct Str {
    sr: Register,
    base_r: Register,
    offset6: u16,
}

impl Str {
    pub fn from_bits(bits: u16) -> Self {
        let sr = Register::try_from((bits >> 9) & 0b111).unwrap();
        let base_r = Register::try_from((bits >> 6) & 0b111).unwrap();
        let pc_offset6 = sign_extend(bits & 0b111111, 6);

        Self {
            sr,
            base_r,
            offset6: pc_offset6,
        }
    }

    pub fn execute(&self, registers: &mut Registers, memory: &mut Memory) {
        let val = registers.get(self.sr);
        memory.write(registers.get(self.base_r) + self.offset6, val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_to_str() {
        // STR  R3, R6, 0x7
        let bits = 0b0111_011_110_000111;

        let instruction = Str::from_bits(bits);

        assert_eq!(instruction.sr, Register::R3);
        assert_eq!(instruction.base_r, Register::R6);
        assert_eq!(instruction.offset6, 7);
    }

    #[test]
    fn test_str_works() {
        let mut memory = Memory::default();

        let mut registers = Registers::default();
        registers.set(Register::R6, 0x3000);
        registers.set(Register::R3, 0x99);

        let instruction = Str {
            sr: Register::R3,
            base_r: Register::R6,
            offset6: 0x7,
        };

        assert_eq!(memory.read(0x3007), 0);

        instruction.execute(&mut registers, &mut memory);

        assert_eq!(memory.read(0x3007), 0x99);
    }
}
