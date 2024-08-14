use crate::{
    instructions::{Instruction, InstructionType},
    memory::Memory,
    registers::{CondFlag, Register, Registers},
};

const PC_START: u16 = 0x3000;

pub struct Vm {
    registers: Registers,
    memory: Memory,
}

impl Vm {
    pub fn new(mut registers: Registers, memory: Memory) -> Self {
        registers.set(Register::PC, PC_START);
        registers.set(Register::COND, CondFlag::ZRO as u16);

        Self { registers, memory }
    }

    fn fetch_next_instruction(&mut self) -> u16 {
        let mar = self.registers.get(Register::PC);
        self.registers.program_counter_increment();

        self.memory.read(mar)
    }

    pub fn run(&mut self) {
        while let InstructionType::Continue(instruction) =
            Instruction::try_from_bits(self.fetch_next_instruction()).unwrap()
        {
            instruction.execute(&mut self.registers, &mut self.memory);
        }
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

        let mut vm = Vm::new(registers, memory);

        assert_eq!(vm.registers.get(Register::PC), 0x3000);
        let fetched = vm.fetch_next_instruction();
        assert_eq!(fetched, i1);
        assert_eq!(vm.registers.get(Register::PC), 0x3001);
        let instruction = Instruction::try_from_bits(fetched).unwrap();
        assert_eq!(vm.registers.get(Register::R0), 0);
        match instruction {
            InstructionType::Continue(ins) => ins.execute(&mut vm.registers, &mut vm.memory),
            InstructionType::Halt => panic!("halted"),
        }
        assert_eq!(vm.registers.get(Register::R0), 1);
        let fetched = vm.fetch_next_instruction();
        assert_eq!(fetched, i2);
        let instruction = Instruction::try_from_bits(fetched).unwrap();
        match instruction {
            InstructionType::Continue(ins) => ins.execute(&mut vm.registers, &mut vm.memory),
            InstructionType::Halt => panic!("halted"),
        }
        assert_eq!(vm.registers.get(Register::R0), 2);
    }
}
