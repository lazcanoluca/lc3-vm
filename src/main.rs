use std::{fs::File, io::BufReader, path::PathBuf};

use byteorder::{BigEndian, ReadBytesExt};
use clap::{value_parser, Arg, Command};
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

fn load_memory_from_file(memory: &mut Memory, file: File) {
    let mut file = BufReader::new(file);

    let mut addr = file
        .read_u16::<BigEndian>()
        .expect("error reading base address");

    while let Ok(bits) = file.read_u16::<BigEndian>() {
        memory.write(addr, bits);
        addr = addr.checked_add(1).expect("file too large");
    }
}

fn main() {
    let matches = Command::new("cargo run")
        .about("An LC-3 virtual machine")
        .arg(
            Arg::new("image")
                .short('i')
                .long("img")
                .value_parser(value_parser!(PathBuf))
                .required(true)
                .help("The path to the object file"),
        )
        .get_matches();

    let obj_file = matches.get_one::<PathBuf>("image").unwrap();
    let f = File::open(obj_file).unwrap();

    let mut memory = Memory::default();
    load_memory_from_file(&mut memory, f);

    let registers = Registers::default();
    let mut vm = Vm::new(registers, memory);

    let termios = Termios::from_fd(STDIN).unwrap();
    setup_terminal(termios);

    vm.instruction_cycle();

    restore_terminal(termios);
}
