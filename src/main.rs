use std::{
    env,
    fs::File,
    io::{self, BufReader},
    process::exit,
};

use byteorder::{BigEndian, ReadBytesExt};
use memory::Memory;
use registers::Registers;

use termios::Termios;
use utils::{restore_terminal, setup_terminal, STDIN};
use vm::Vm;

mod instructions;
mod memory;
mod opcodes;
mod registers;
mod traps;
mod utils;
mod vm;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run [image-file1] ...\n");
        exit(2);
    }

    let mut memory = Memory::default();

    for arg in args[1..].iter() {
        let f = File::open(arg).unwrap_or_else(|_| panic!("couldn't open: {}", arg));
        let mut f = BufReader::new(f);

        let base_addr = f
            .read_u16::<BigEndian>()
            .expect("error reading base address");

        let mut addr = base_addr;

        loop {
            match f.read_u16::<BigEndian>() {
                Ok(bits) => {
                    memory.write(addr, bits);
                    addr += 1;
                }
                Err(e) => {
                    if e.kind() == io::ErrorKind::UnexpectedEof {
                        println!("program loaded successfully")
                    } else {
                        println!("failed loading program: {}", e)
                    }
                    break;
                }
            }
        }
    }

    let registers = Registers::default();

    let mut vm = Vm::new(registers, memory);

    let termios = Termios::from_fd(STDIN).unwrap();

    setup_terminal(termios);

    vm.instruction_cycle();

    restore_terminal(termios);
}
