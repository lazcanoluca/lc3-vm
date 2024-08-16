#[derive(Debug, PartialEq)]
pub enum Opcode {
    BR = 0b0000,
    ADD,
    LD,
    ST,
    JSR,
    AND,
    LDR,
    STR,
    RTI,
    NOT,
    LDI,
    STI,
    JMP,
    RES,
    LEA,
    TRAP,
}

impl From<u16> for Opcode {
    fn from(bits: u16) -> Self {
        match bits >> 12 {
            0b0000 => Self::BR,
            0b0001 => Self::ADD,
            0b0010 => Self::LD,
            0b0011 => Self::ST,
            0b0100 => Self::JSR,
            0b0101 => Self::AND,
            0b0110 => Self::LDR,
            0b0111 => Self::STR,
            0b1000 => Self::RTI,
            0b1001 => Self::NOT,
            0b1010 => Self::LDI,
            0b1011 => Self::STI,
            0b1100 => Self::JMP,
            0b1101 => Self::RES,
            0b1110 => Self::LEA,
            0b1111 => Self::TRAP,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_opcode_from_bits() {
        assert_eq!(Opcode::from(0b0000_0000_0000_0000), Opcode::BR);
    }
}
