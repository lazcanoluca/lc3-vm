use crate::constants::MEMORY_MAX;

pub struct Memory([u16; MEMORY_MAX as usize]);

impl Memory {
    pub fn read(&self, addr: u16) -> u16 {
        todo!()
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self([0; MEMORY_MAX as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_memory() {
        let memory = Memory::default();

        assert_eq!(memory.0.len(), MEMORY_MAX as usize);
    }
}
