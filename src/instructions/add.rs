use crate::{
    memory::Memory,
    registers::{Register, Registers},
    sign_extend,
};

use super::Instruction;

pub struct AddImmediate {
    dr: Register,
    sr1: Register,
    imm5: u16,
}

pub struct AddRegister {
    dr: Register,
    sr1: Register,
    sr2: Register,
}

pub enum Add {
    AddImm(AddImmediate),
    AddReg(AddRegister),
}

impl Instruction for Add {
    fn from_bits(bits: u16) -> Self
    where
        Self: Sized,
    {
        let dr = Register::try_from((bits >> 9) & 0b111).unwrap();
        let sr1 = Register::try_from((bits >> 6) & 0b111).unwrap();
        let imm = (bits >> 5) & 0b1 == 1;

        if imm {
            let imm5 = sign_extend(bits & 0b0000_0000_0001_1111, 5);

            Self::AddImm(AddImmediate { dr, sr1, imm5 })
        } else {
            let sr2 = Register::try_from(bits & 0b111).unwrap();

            Self::AddReg(AddRegister { dr, sr1, sr2 })
        }
    }

    fn execute(&self, registers: &mut Registers, memory: &mut Memory) {
        match self {
            Add::AddImm(args) => {
                registers.set(args.dr, registers.get(args.sr1) + args.imm5);
                registers.update_flags(args.dr);
            }
            Add::AddReg(args) => {
                registers.set(args.dr, registers.get(args.sr1) + registers.get(args.sr2));
                registers.update_flags(args.dr);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_to_add_imm() {
        // ADD  R3, R2, 31
        let bits = 0b0001_011_010_1_00001;

        let instruction = Add::from_bits(bits);

        match instruction {
            Add::AddImm(args) => {
                assert_eq!(args.dr, Register::R3);
                assert_eq!(args.sr1, Register::R2);
                assert_eq!(args.imm5, 0b0000_0000_0000_0001);
            }
            _ => panic!("Instruction should be in the immediate mode variant."),
        }
    }

    #[test]
    fn test_bits_to_add_reg() {
        // ADD  R3, R2, R5
        let bits = 0b0001_011_010_0_00_101;

        let instruction = Add::from_bits(bits);

        match instruction {
            Add::AddReg(args) => {
                assert_eq!(args.dr, Register::R3);
                assert_eq!(args.sr1, Register::R2);
                assert_eq!(args.sr2, Register::R5);
            }
            _ => panic!("Instruction should be in the register mode variant."),
        }
    }

    #[test]
    fn test_add_immediate_execute() {
        let mut registers = Registers::default();
        let mut memory = Memory::default();

        registers.set(Register::R1, 5);

        let instruction = Add::AddImm(AddImmediate {
            dr: Register::R0,
            sr1: Register::R1,
            imm5: 10,
        });

        instruction.execute(&mut registers, &mut memory);

        assert_eq!(registers.get(Register::R0), 15);
    }

    #[test]
    fn test_add_register_execute() {
        let mut registers = Registers::default();
        let mut memory = Memory::default();

        registers.set(Register::R1, 5);
        registers.set(Register::R2, 10);

        let instruction = Add::AddReg(AddRegister {
            dr: Register::R0,
            sr1: Register::R1,
            sr2: Register::R2,
        });

        instruction.execute(&mut registers, &mut memory);

        assert_eq!(registers.get(Register::R0), 15);
    }
}
