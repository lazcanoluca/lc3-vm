use crate::registers::{Register, Registers};

pub struct Jmp {
    base_r: Register,
}

impl Jmp {
    pub fn from_bits(bits: u16) -> Self {
        let base_r = Register::try_from((bits >> 6) & 0b111).unwrap();

        Self { base_r }
    }

    pub fn execute(&self, registers: &mut Registers) {
        registers.set(Register::PC, registers.get(self.base_r));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_to_jmp() {
        // JMP  R3
        let bits = 0b1100_000_011_000000;

        let instruction = Jmp::from_bits(bits);

        let mut registers = Registers::default();

        registers.set(Register::R3, 0x3000);

        assert_eq!(registers.get(Register::PC), 0x0000);

        instruction.execute(&mut registers);

        assert_eq!(registers.get(Register::PC), 0x3000);
    }

    #[test]
    fn test_bits_to_ret() {
        // RET
        let bits = 0b1100_000_111_000000;

        let instruction = Jmp::from_bits(bits);

        let mut registers = Registers::default();

        registers.set(Register::R7, 0x3000);

        assert_eq!(registers.get(Register::PC), 0x0000);

        instruction.execute(&mut registers);

        assert_eq!(registers.get(Register::PC), 0x3000);
    }
}
