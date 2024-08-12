use crate::{
    registers::{Register, Registers},
    utils::sign_extend,
};

#[derive(Debug)]
pub struct Lea {
    dr: Register,
    pc_offset9: u16,
}

impl Lea {
    pub fn from_bits(bits: u16) -> Self {
        let dr = Register::try_from((bits >> 9) & 0b111).unwrap();
        let pc_offset9 = sign_extend(bits & 0x1ff, 9);

        Self { dr, pc_offset9 }
    }

    pub fn execute(&self, registers: &mut Registers) {
        registers.set(self.dr, registers.get(Register::PC) + self.pc_offset9);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_to_lea() {
        // LEA  R3, 0x7
        let bits = 0b1110_011_000000111;

        let instruction = Lea::from_bits(bits);

        assert_eq!(instruction.dr, Register::R3);
        assert_eq!(instruction.pc_offset9, 7);
    }

    #[test]
    fn test_lea_works() {
        let mut registers = Registers::default();
        registers.set(Register::PC, 0x1000);

        let instruction = Lea {
            dr: Register::R3,
            pc_offset9: 0x7000,
        };

        assert_eq!(registers.get(Register::R3), 0);

        instruction.execute(&mut registers);

        assert_eq!(registers.get(Register::R3), 0x8000);
    }
}
