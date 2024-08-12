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

impl TryFrom<u16> for Opcode {
    type Error = String;

    fn try_from(bits: u16) -> Result<Self, Self::Error> {
        match bits >> 12 {
            0b0000 => Ok(Self::BR),
            0b0001 => Ok(Self::ADD),
            0b0010 => Ok(Self::LD),
            0b0011 => Ok(Self::ST),
            0b0100 => Ok(Self::JSR),
            0b0101 => Ok(Self::AND),
            0b0110 => Ok(Self::LDR),
            0b0111 => Ok(Self::STR),
            0b1000 => Ok(Self::RTI),
            0b1001 => Ok(Self::NOT),
            0b1010 => Ok(Self::LDI),
            0b1011 => Ok(Self::STI),
            0b1100 => Ok(Self::JMP),
            0b1101 => Ok(Self::RES),
            0b1110 => Ok(Self::LEA),
            0b1111 => Ok(Self::TRAP),
            _ => Err("Bad opcode.".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_opcode_from_bits() {
        assert_eq!(Opcode::try_from(0b0000_0000_0000_0000).unwrap(), Opcode::BR);
    }
}
