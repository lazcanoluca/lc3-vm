use crate::registers::{Register, Registers};

pub struct Not {
    dr: Register,
    sr: Register,
}

impl Not {
    pub fn from_bits(bits: u16) -> Self {
        let dr = Register::try_from((bits >> 9) & 0b111).unwrap();
        let sr = Register::try_from((bits >> 6) & 0b111).unwrap();

        Self { dr, sr }
    }

    pub fn execute(&self, registers: &mut Registers) {
        registers.set(self.dr, !registers.get(self.sr));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_to_not() {
        // NOT  R3, R1
        let bits = 0b1001_011_001_1_11111;

        let instruction = Not::from_bits(bits);

        assert_eq!(instruction.dr, Register::R3);
        assert_eq!(instruction.sr, Register::R1);
    }

    #[test]
    fn test_not() {
        let mut registers = Registers::default();
        registers.set(Register::R3, 0);

        let instruction = Not {
            dr: Register::R3,
            sr: Register::R1,
        };

        instruction.execute(&mut registers);

        assert_eq!(registers.get(Register::R3), 0xffff);

        registers.set(Register::R4, 0b1010_1010_0101_0101);

        let instruction2 = Not {
            dr: Register::R4,
            sr: Register::R4,
        };

        instruction2.execute(&mut registers);

        assert_eq!(registers.get(Register::R4), 0b0101_0101_1010_1010);
    }
}
