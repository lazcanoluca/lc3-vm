# runs the lc-3 in debug mode with the 2048 obj file
default: run-2048

# build the lc-3 in release mode and install the binary
build:
    cargo build --release
    cargo install --path .

test:
    cargo test

run-2048:
    cargo run -- -i ./images/2048.obj

run-rogue:
    cargo run -- -i ./images/rogue.obj
