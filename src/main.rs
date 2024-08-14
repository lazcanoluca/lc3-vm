use std::{fs::File, io::BufReader, path::PathBuf};
use termios::{
    tcsetattr, Termios, BRKINT, ECHO, ICANON, ICRNL, IGNBRK, IGNCR, INLCR, ISTRIP, IXON, PARMRK,
    TCSANOW,
};

use byteorder::{BigEndian, ReadBytesExt};
use clap::{value_parser, Arg, Command};
use memory::Memory;
use registers::Registers;

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

pub const STDIN: i32 = 0;

struct TermiosHandler(Termios);

impl TermiosHandler {
    pub fn new(termios: Termios) -> TermiosHandler {
        println!("terminal settings modified");
        let mut new_termios = termios;
        new_termios.c_iflag &= IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON;
        new_termios.c_lflag &= !(ICANON | ECHO);

        tcsetattr(STDIN, TCSANOW, &new_termios).unwrap();

        TermiosHandler(termios)
    }
}

impl Drop for TermiosHandler {
    fn drop(&mut self) {
        println!("terminal settings restored");
        tcsetattr(STDIN, TCSANOW, &self.0).unwrap();
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

    {
        let _ = TermiosHandler::new(termios);
        vm.run();
    }

    println!("execution finished ok");
}
