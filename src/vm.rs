use crate::{
    instructions::{Add, Instruction},
    memory::Memory,
    opcodes::Opcode,
    registers::{Register, Registers},
};

pub struct VM {
    registers: Registers,
    memory: Memory,
}

impl VM {
    pub fn new(registers: Registers, memory: Memory) -> Self {
        Self { registers, memory }
    }

    fn fetch(&mut self) -> u16 {
        let mar = self.registers.get(Register::PC);
        self.registers.program_counter_increment();

        self.memory.read(mar)
    }

    fn decode(&self, bits: u16) -> Box<dyn Instruction> {
        let opcode = Opcode::try_from(bits).unwrap();

        match opcode {
            Opcode::ADD => Box::new(Add::from_bits(bits)),
            _ => todo!(),
        }
    }

    pub fn instruction_cycle(&mut self) {
        let bits = self.fetch();
        let instruction = self.decode(bits);
        instruction.execute(&mut self.registers, &mut self.memory);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_cycle() {
        let mut registers = Registers::default();
        registers.set(Register::PC, 0x3000);

        let mut memory = Memory::default();
        // ADD  R0, R0, 1
        let i1 = 0b0001_000_000_1_00001;
        memory.write(0x3000, i1);
        // ADD  R0, R0, R0
        let i2 = 0b0001_000_000_0_00_000;
        memory.write(0x3001, i2);

        let mut vm = VM::new(registers, memory);

        assert_eq!(vm.registers.get(Register::PC), 0x3000);
        let fetched = vm.fetch();
        assert_eq!(fetched, i1);
        assert_eq!(vm.registers.get(Register::PC), 0x3001);
        let instruction = vm.decode(fetched);
        assert_eq!(vm.registers.get(Register::R0), 0);
        instruction.execute(&mut vm.registers, &mut vm.memory);
        assert_eq!(vm.registers.get(Register::R0), 1);
        let fetched = vm.fetch();
        assert_eq!(fetched, i2);
        let instruction = vm.decode(fetched);
        instruction.execute(&mut vm.registers, &mut vm.memory);
        assert_eq!(vm.registers.get(Register::R0), 2);
    }
}
