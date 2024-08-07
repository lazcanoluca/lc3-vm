pub fn sign_extend(x: u16, bit_count: u32) -> u16 {
    if (x >> (bit_count - 1)) & 1 == 1 {
        x | (0xFFFF << bit_count)
    } else {
        x
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
