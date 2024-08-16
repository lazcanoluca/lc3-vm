
An LC-3 virtual machine written in Rust, based on [Write your Own Virtual Machine](https://www.jmeiners.com/lc3-vm/).

## Usage

1. `just build` to build and install the binary, or manually use `cargo`.

2. Run:

    - For help:

    ```sh
    lc3-vm -h
    ```

    - With an object file:

    ```sh
    lc3-vm -i <path-to-obj>
    ```

## Justfile

Build and install binary:

```sh
just build
```

Alternatively, you can directly run the "2048" and "rogue" examples with:

```sh
just run-2048
```

```sh
just run-rogue
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