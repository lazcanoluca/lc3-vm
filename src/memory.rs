use std::{fs::File, io::BufReader};

use byteorder::{BigEndian, ReadBytesExt};

use crate::{registers::MemoryMappedReg, utils::handle_keyboard};

pub const MEMORY_SIZE: usize = 0x10000;

pub struct Memory {
    memory: [u16; MEMORY_SIZE],
}

impl Memory {
    pub fn read(&mut self, addr: u16) -> u16 {
        if addr == MemoryMappedReg::Kbsr as u16 {
            handle_keyboard(self);
        }
        self.memory[addr as usize]
    }

    pub fn write(&mut self, addr: u16, data: u16) {
        self.memory[addr as usize] = data;
    }

    pub fn from_file(file: File) -> Self {
        let mut file: BufReader<File> = BufReader::new(file);

        let mut memory = Memory::default();

        let mut addr = file
            .read_u16::<BigEndian>()
            .expect("error reading base address");

        while let Ok(bits) = file.read_u16::<BigEndian>() {
            memory.write(addr, bits);
            addr = addr.checked_add(1).expect("file too large");
        }

        memory
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_memory() {
        let memory = Memory::default();

        assert_eq!(memory.memory.len(), MEMORY_SIZE);
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
        let mut memory = Memory::default();

        assert_eq!(memory.read(0xffff), 0);
    }
}
