use crate::constants::MEMORY_MAX;

pub struct Memory {
    memory: [u16; MEMORY_MAX as usize + 1],
}

impl Memory {
    pub fn read(&self, addr: u16) -> u16 {
        self.memory[addr as usize]
    }

    pub fn write(&mut self, addr: u16, data: u16) {
        self.memory[addr as usize] = data;
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            memory: [0; MEMORY_MAX as usize + 1],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_memory() {
        let memory = Memory::default();

        assert_eq!(memory.memory.len(), MEMORY_MAX as usize + 1);
    }

    #[test]
    fn test_read_memory() {
        let mut memory = Memory::default();

        memory.memory[0x3000] = 0xABCD;

        assert_eq!(memory.read(0x3000), 0xABCD);
    }

    #[test]
    fn test_write_memory() {
        let mut memory = Memory::default();

        memory.write(0x3000, 0xABCD);

        assert_eq!(memory.read(0x3000), 0xABCD);
    }

    #[test]
    fn test_index_last_memory_block() {
        let memory = Memory::default();

        assert_eq!(memory.read(0xffff), 0);
    }
}
