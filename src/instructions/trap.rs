use std::{
    io::{self, Read, Write},
    process,
};

use crate::{
    memory::Memory,
    registers::{Register, Registers},
    traps::TrapCode,
};

#[derive(Debug)]
pub struct Trap {
    pub trap_code: TrapCode,
}

impl Trap {
    pub fn from_bits(bits: u16) -> Self {
        let trap_code = TrapCode::try_from(bits).unwrap();

        Self { trap_code }
    }

    pub fn execute(&self, registers: &mut Registers, memory: &mut Memory) {
        registers.set(Register::R7, registers.get(Register::PC));

        match self.trap_code {
            TrapCode::GETC => {
                let mut buffer = [0; 1];
                std::io::stdin().read_exact(&mut buffer).unwrap();
                registers.set(Register::R0, buffer[0] as u16);
            }
            TrapCode::OUT => {
                let c = registers.get(Register::R0) as u8;
                print!("{}", c as char);
            }
            TrapCode::PUTS => {
                let mut index = registers.get(Register::R0);
                let mut c = memory.read(index);
                while c != 0x0000 {
                    print!("{}", (c as u8) as char);
                    index += 1;
                    c = memory.read(index);
                }
                io::stdout().flush().expect("failed to flush");
            }
            TrapCode::IN => {
                print!("Enter a  character : ");
                io::stdout().flush().expect("failed to flush");
                let char = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as u16)
                    .unwrap();
                registers.set(Register::R0, char);
            }
            TrapCode::PUTSP => {
                let mut index = registers.get(Register::R0);
                let mut c = memory.read(index);
                while c != 0x0000 {
                    let c1 = ((c & 0xFF) as u8) as char;
                    print!("{}", c1);
                    let c2 = ((c >> 8) as u8) as char;
                    if c2 != '\0' {
                        print!("{}", c2);
                    }
                    index += 1;
                    c = memory.read(index);
                }
                io::stdout().flush().expect("failed to flush");
            }
            TrapCode::HALT => {
                io::stdout().flush().expect("failed to flush");
                process::exit(1);
            }
        }
    }
}
