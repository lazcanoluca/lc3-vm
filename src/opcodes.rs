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

    #[test]
    fn test_bad_opcode_should_error() {
        assert!(Opcode::try_from(0b1111_0000_0000_0000).is_err());
    }
}
