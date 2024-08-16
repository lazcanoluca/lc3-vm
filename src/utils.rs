use std::io::Read;

use crate::{memory::Memory, registers::MemoryMappedReg};

pub fn sign_extend(mut x: u16, bit_count: u8) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x |= 0xFFFF << bit_count;
    }
    x
}

pub fn handle_keyboard(memory: &mut Memory) {
    let mut buffer = [0; 1];
    std::io::stdin().read_exact(&mut buffer).unwrap();

    if buffer[0] != 0 {
        memory.write(MemoryMappedReg::Kbsr as u16, 1 << 15);
        memory.write(MemoryMappedReg::Kbdr as u16, buffer[0] as u16);
    } else {
        memory.write(MemoryMappedReg::Kbsr as u16, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_extend_positive() {
        assert_eq!(sign_extend(0b0000_0001, 2), 0b0000_0000_0000_0001);
        assert_eq!(sign_extend(0b0000_0100, 4), 0b0000_0000_0000_0100);
    }

    #[test]
    fn test_sign_extend_negative() {
        assert_eq!(sign_extend(0b0000_1001, 4), 0b1111_1111_1111_1001);
        assert_eq!(sign_extend(0b0000_0110, 3), 0b1111_1111_1111_1110);
    }
}
