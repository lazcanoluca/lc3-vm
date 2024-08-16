use std::{fs::File, path::PathBuf};
use termios::{
    tcsetattr, Termios, BRKINT, ECHO, ICANON, ICRNL, IGNBRK, IGNCR, INLCR, ISTRIP, IXON, PARMRK,
    TCSANOW,
};

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
    let matches = Command::new("lc3-vm")
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

    let memory = Memory::from_file(f);

    let registers = Registers::default();
    let mut vm = Vm::new(registers, memory);

    let termios = Termios::from_fd(STDIN).unwrap();

    let handler = TermiosHandler::new(termios);
    vm.run();
    drop(handler);

    println!("execution finished ok");
}
