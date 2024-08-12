use crate::{
    registers::{Register, Registers},
    utils::sign_extend,
};

#[derive(Debug)]
pub struct JsrOffset {
    pc_offset11: u16,
}

#[derive(Debug)]
pub struct JsrRegister {
    base_r: Register,
}

#[derive(Debug)]
pub enum Jsr {
    JsrOffset(JsrOffset),
    JsrRegister(JsrRegister),
}

impl Jsr {
    pub fn from_bits(bits: u16) -> Self {
        let r = (bits >> 11) & 0x1 == 0;

        if r {
            let base_r = Register::try_from((bits >> 6) & 0b111).unwrap();

            Self::JsrRegister(JsrRegister { base_r })
        } else {
            let pc_offset11 = sign_extend(bits & 0b0000_0_11111111111, 11);

            Self::JsrOffset(JsrOffset { pc_offset11 })
        }
    }

    pub fn execute(&self, registers: &mut Registers) {
        let temp = registers.get(Register::PC);

        match self {
            Jsr::JsrOffset(args) => {
                registers.set(
                    Register::PC,
                    registers.get(Register::PC).wrapping_add(args.pc_offset11),
                );
            }
            Jsr::JsrRegister(args) => {
                registers.set(Register::PC, registers.get(args.base_r));
            }
        }

        registers.set(Register::R7, temp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_to_jsr_offset() {
        // JSR  0x1
        let bits = 0b0100_1_00000000001;

        let instruction = Jsr::from_bits(bits);

        match instruction {
            Jsr::JsrOffset(args) => {
                assert_eq!(args.pc_offset11, 1);
            }
            _ => panic!("Instruction should be in the offset mode variant."),
        }
    }

    #[test]
    fn test_bits_to_jsr_register() {
        // JSR  R1
        let bits = 0b0100_0_00_001_000000;

        let instruction = Jsr::from_bits(bits);

        match instruction {
            Jsr::JsrRegister(args) => {
                assert_eq!(args.base_r, Register::R1);
            }
            _ => panic!("Instruction should be in the register mode variant."),
        }
    }

    #[test]
    fn test_jsr_register() {
        let mut registers = Registers::default();

        registers.set(Register::R5, 0x7000);

        let instruction = Jsr::JsrRegister(JsrRegister {
            base_r: Register::R5,
        });

        assert_eq!(registers.get(Register::PC), 0);

        instruction.execute(&mut registers);

        assert_eq!(registers.get(Register::PC), 0x7000);
    }

    #[test]
    fn test_jsr_offset() {
        let mut registers = Registers::default();

        let instruction = Jsr::JsrOffset(JsrOffset {
            pc_offset11: 0x7000,
        });

        assert_eq!(registers.get(Register::PC), 0);

        instruction.execute(&mut registers);

        assert_eq!(registers.get(Register::PC), 0x7000);
    }
}
