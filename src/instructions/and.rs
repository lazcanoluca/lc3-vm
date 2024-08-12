use crate::{
    registers::{Register, Registers},
    utils::sign_extend,
};

#[derive(Debug)]
pub struct AndImmediate {
    dr: Register,
    sr1: Register,
    imm5: u16,
}

#[derive(Debug)]
pub struct AndRegister {
    dr: Register,
    sr1: Register,
    sr2: Register,
}

#[derive(Debug)]
pub enum And {
    AddImm(AndImmediate),
    AddReg(AndRegister),
}

impl And {
    pub fn from_bits(bits: u16) -> Self {
        let dr = Register::try_from((bits >> 9) & 0b111).unwrap();
        let sr1 = Register::try_from((bits >> 6) & 0b111).unwrap();
        let imm = (bits >> 5) & 0b1 == 1;

        if imm {
            let imm5 = sign_extend(bits & 0b0000_0000_0001_1111, 5);

            Self::AddImm(AndImmediate { dr, sr1, imm5 })
        } else {
            let sr2 = Register::try_from(bits & 0b111).unwrap();

            Self::AddReg(AndRegister { dr, sr1, sr2 })
        }
    }

    pub fn execute(&self, registers: &mut Registers) {
        match self {
            And::AddImm(args) => {
                registers.set(args.dr, registers.get(args.sr1) & args.imm5);
                registers.update_flags(args.dr);
            }
            And::AddReg(args) => {
                registers.set(args.dr, registers.get(args.sr1) & registers.get(args.sr2));
                registers.update_flags(args.dr);
            }
        }
    }
}
