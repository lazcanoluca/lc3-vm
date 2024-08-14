
An LC-3 virtual machine written in Rust, based on [Write your Own Virtual Machine](https://www.jmeiners.com/lc3-vm/).

## Usage

1. Clone the repository.

2. Run:

```sh
cargo run <image-file>
```

Two example images are provided, to run:

```sh
cargo run images/rogue.obj
```

and

```sh
cargo run images/2048.obj
```

## TODO
- Unify the construction and execution of the instructions. The current design makes is unnecesarily complex
- Handle errors correctly, by using the corresponding traps and error handling
- Better indexing of registers. No need to always do `.get()`, can acceso directly `registers.r0`
- Wrapping add tests
- Documentation
- Implement `Index` and `IndexMut` for `Memory` access
- Cond flags as enum

## Credits
- Meiners, J. and Pendleton, R. (no date) Write your Own Virtual Mach, Write your own virtual machine. Available at: https://www.jmeiners.com/lc3-vm/ (Accessed: 12 August 2024). 
- Patt, Y.N. and Patel, S.J. (2020) Introduction to computing systems: From bits & gates to C. New York, NY: McGraw-Hill.