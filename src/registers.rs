#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    COND,
}

impl TryFrom<u16> for Register {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::R0),
            1 => Ok(Self::R1),
            2 => Ok(Self::R2),
            3 => Ok(Self::R3),
            4 => Ok(Self::R4),
            5 => Ok(Self::R5),
            6 => Ok(Self::R6),
            7 => Ok(Self::R7),
            8 => Ok(Self::PC),
            9 => Ok(Self::COND),
            _ => Err("Invalid register".to_string()),
        }
    }
}

#[derive(Default)]
pub struct Registers {
    r0: u16,
    r1: u16,
    r2: u16,
    r3: u16,
    r4: u16,
    r5: u16,
    r6: u16,
    r7: u16,
    pc: u16,
    cond: u16,
}

impl Registers {
    pub fn get(&self, register: Register) -> u16 {
        match register {
            Register::R0 => self.r0,
            Register::R1 => self.r1,
            Register::R2 => self.r2,
            Register::R3 => self.r3,
            Register::R4 => self.r4,
            Register::R5 => self.r5,
            Register::R6 => self.r6,
            Register::R7 => self.r7,
            Register::PC => self.pc,
            Register::COND => self.cond,
        }
    }

    pub fn set(&mut self, register: Register, new: u16) {
        match register {
            Register::R0 => self.r0 = new,
            Register::R1 => self.r1 = new,
            Register::R2 => self.r2 = new,
            Register::R3 => self.r3 = new,
            Register::R4 => self.r4 = new,
            Register::R5 => self.r5 = new,
            Register::R6 => self.r6 = new,
            Register::R7 => self.r7 = new,
            Register::PC => self.pc = new,
            Register::COND => self.cond = new,
        };
    }

    pub fn update_flags(&mut self, register: Register) {
        let flag = match self.get(register) {
            0 => CondFlag::ZRO,
            x if x >> 15 == 1 => CondFlag::NEG,
            _ => CondFlag::POS,
        };

        self.set(Register::COND, flag as u16);
    }

    pub fn program_counter_increment(&mut self) {
        self.pc += 1;
    }
}

#[derive(PartialEq)]
pub enum CondFlag {
    POS = 1 << 0 as u16,
    ZRO = 1 << 1 as u16,
    NEG = 1 << 2 as u16,
}

impl TryFrom<u16> for CondFlag {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            value if value == (1 << 0) => Ok(CondFlag::POS),
            value if value == (1 << 1) => Ok(CondFlag::ZRO),
            value if value == (1 << 2) => Ok(CondFlag::NEG),
            _ => Err(format!("{} is not a condition flag.", value).to_owned()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::u16;

    use super::*;

    #[test]
    fn test_value_to_register_should_return_register_variant() {
        assert_eq!(
            Register::try_from(0b0000_0000_0000_0000).unwrap(),
            Register::R0
        );
        assert_eq!(
            Register::try_from(0b0000_0000_0000_0001).unwrap(),
            Register::R1
        );
        assert_eq!(
            Register::try_from(0b0000_0000_0000_1000).unwrap(),
            Register::PC
        );
        assert_eq!(
            Register::try_from(0b0000_0000_0000_1001).unwrap(),
            Register::COND
        );
    }

    #[test]
    fn test_value_to_unexistant_register_should_fail() {
        assert!(Register::try_from(0b0000_0000_0000_1111).is_err())
    }

    #[test]
    fn test_get_register_gets_correctly() {
        let registers = Registers {
            r0: 5,
            pc: 0x3000,
            ..Default::default()
        };

        assert_eq!(registers.get(Register::R0), 5);
        assert_eq!(registers.get(Register::PC), 0x3000);
    }

    #[test]
    fn test_set_register_updates_correctly() {
        let mut registers = Registers::default();

        registers.set(Register::R5, 5);
        registers.set(Register::R7, 7);

        assert_eq!(registers.get(Register::R5), 5);
        assert_eq!(registers.get(Register::R7), 7);
    }

    #[test]
    fn test_flag_update() {
        let mut registers = Registers::default();

        registers.set(Register::R0, u16::MAX);
        registers.update_flags(Register::R0);
        assert_eq!(registers.get(Register::COND), CondFlag::NEG as u16);

        registers.set(Register::R0, 0);
        registers.update_flags(Register::R0);
        assert_eq!(registers.get(Register::COND), CondFlag::ZRO as u16);

        registers.set(Register::R0, 10);
        registers.update_flags(Register::R0);
        assert_eq!(registers.get(Register::COND), CondFlag::POS as u16);
    }
}
