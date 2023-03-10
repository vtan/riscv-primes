# riscv-primes

A toy prime searcher targeting qemu's RISC-V `virt` board.

To build and run:
```console
$ rustup target add riscv64imac-unknown-none-elf
$ cargo run --release  # calls qemu-system-riscv64
```
