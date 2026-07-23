# Chalice-8

A CHIP-8 emulator written in Rust, built with as few external crates as possible, the core machine (memory, registers, opcodes) is pure `std`.

## Status

In development. Machine state (memory, registers, stack, timers, display buffer) and ROM loading are implemented. The fetch-decode-execute loop and opcode set are not yet implemented.

## Usage

```bash
cargo run -- path/to/rom.ch8
```

## How it works

Chalice-8 loads a `.ch8` ROM's raw bytes into a 4KB memory buffer starting at `0x200`, alongside a built-in font set at `0x50`, and will execute instructions via a fetch-decode-execute loop against 16 general-purpose registers, an index register, a call stack, and delay/sound timers.
