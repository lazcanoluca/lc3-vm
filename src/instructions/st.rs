use crate::{
    memory::Memory,
    registers::{Register, Registers},
    utils::sign_extend,
};

pub struct St {
    sr: Register,
    pc_offset9: u16,
}

impl St {
    pub fn from_bits(bits: u16) -> Self {
        let sr = Register::try_from((bits >> 9) & 0b111).unwrap();
        let pc_offset9 = sign_extend(bits & 0x1ff, 9);

        Self { sr, pc_offset9 }
    }

    pub fn execute(&self, registers: &mut Registers, memory: &mut Memory) {
        let val = registers.get(self.sr);
        memory.write(registers.get(Register::PC) + self.pc_offset9, val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_to_st() {
        // ST   R3, 0x7
        let bits = 0b0011_011_000000111;

        let instruction = St::from_bits(bits);

        assert_eq!(instruction.sr, Register::R3);
        assert_eq!(instruction.pc_offset9, 7);
    }

    #[test]
    fn test_st_works() {
        let mut memory = Memory::default();

        let mut registers = Registers::default();
        registers.set(Register::PC, 0x3000);
        registers.set(Register::R3, 0x99);

        let instruction = St {
            sr: Register::R3,
            pc_offset9: 0x7,
        };

        assert_eq!(memory.read(0x3007), 0);

        instruction.execute(&mut registers, &mut memory);

        assert_eq!(memory.read(0x3007), 0x99);
    }
}
