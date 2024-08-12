use crate::{
    registers::{CondFlag, Register, Registers},
    utils::sign_extend,
};

#[derive(Debug)]
pub struct Br {
    n: bool,
    z: bool,
    p: bool,
    pc_offset9: u16,
}

impl Br {
    pub fn from_bits(bits: u16) -> Self {
        let n = (bits >> 11) & 0b1 == 1;
        let z = (bits >> 10) & 0b1 == 1;
        let p = (bits >> 9) & 0b1 == 1;

        let pc_offset9 = sign_extend(bits & 0b0000_0001_1111_1111, 9);

        Self {
            n,
            z,
            p,
            pc_offset9,
        }
    }

    pub fn execute(&self, registers: &mut Registers) {
        let cond = CondFlag::try_from(registers.get(Register::COND)).unwrap();

        let branch = match cond {
            CondFlag::POS if self.p => true,
            CondFlag::ZRO if self.z => true,
            CondFlag::NEG if self.n => true,
            _ => false,
        };

        // If the branch condition is satisfied, or branch unconditional.
        if branch || !(self.n || self.p || self.z) {
            registers.set(
                Register::PC,
                registers.get(Register::PC).wrapping_add(self.pc_offset9),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_to_br() {
        // BRnzp    0x9
        let bits = 0b0000_111_000001001;

        let instruction = Br::from_bits(bits);

        assert!(instruction.n);
        assert!(instruction.p);
        assert!(instruction.z);
        assert_eq!(instruction.pc_offset9, 9);
    }

    #[test]
    fn test_br_unconditional() {
        // BR   0x9
        let bits = 0b0000_000_000001001;

        let instruction = Br::from_bits(bits);
        assert!(!instruction.n);
        assert!(!instruction.p);
        assert!(!instruction.z);
        assert_eq!(instruction.pc_offset9, 9);

        let mut registers = Registers::default();
        registers.set(Register::PC, 0x9000);
        registers.set(Register::COND, CondFlag::ZRO as u16);

        instruction.execute(&mut registers);

        assert_eq!(registers.get(Register::PC), 0x9009);
    }

    #[test]
    fn test_br_neg_flag() {
        // BRn   0x9
        let bits = 0b0000_100_000001001;

        let instruction = Br::from_bits(bits);
        assert_eq!(instruction.pc_offset9, 9);

        let mut registers = Registers::default();
        registers.set(Register::PC, 0x9000);
        registers.set(Register::COND, CondFlag::NEG as u16);

        instruction.execute(&mut registers);

        assert_eq!(registers.get(Register::PC), 0x9009);
    }

    #[test]
    fn test_br_pos_flag() {
        // BRp   0x9
        let bits = 0b0000_001_000001001;

        let instruction = Br::from_bits(bits);
        assert_eq!(instruction.pc_offset9, 9);

        let mut registers = Registers::default();
        registers.set(Register::PC, 0x9000);
        registers.set(Register::COND, CondFlag::POS as u16);

        instruction.execute(&mut registers);

        assert_eq!(registers.get(Register::PC), 0x9009);
    }

    #[test]
    fn test_br_zro_flag() {
        // BRz   0x9
        let bits = 0b0000_010_000001001;

        let instruction = Br::from_bits(bits);
        assert_eq!(instruction.pc_offset9, 9);

        let mut registers = Registers::default();
        registers.set(Register::PC, 0x9000);
        registers.set(Register::COND, CondFlag::ZRO as u16);

        instruction.execute(&mut registers);

        assert_eq!(registers.get(Register::PC), 0x9009);
    }
}
