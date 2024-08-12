#[derive(PartialEq, Debug)]
pub enum TrapCode {
    GETC = 0x20,
    OUT = 0x21,
    PUTS = 0x22,
    IN = 0x23,
    PUTSP = 0x24,
    HALT = 0x25,
}

impl TryFrom<u16> for TrapCode {
    type Error = String;

    fn try_from(bits: u16) -> Result<Self, Self::Error> {
        match bits & 0xff {
            0x20 => Ok(Self::GETC),
            0x21 => Ok(Self::OUT),
            0x22 => Ok(Self::PUTS),
            0x23 => Ok(Self::IN),
            0x24 => Ok(Self::PUTSP),
            0x25 => Ok(Self::HALT),
            _ => Err("Bad trapcode".to_string()),
        }
    }
}
