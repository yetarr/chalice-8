# Chalice-8

A CHIP-8 emulator written in Rust, built with as few external crates as possible. the core machine (memory, registers, opcodes) is pure `std`.

## Status

In development. Implemented: machine state (memory, registers, stack, timers, display buffer), ROM loading, the fetch-decode-execute loop, simple cli display and all opcodes (except FX0A that needs a functional input handler).

Not yet implemented: display rendering and keyboard input.

## Usage

```bash
cargo run -- path/to/rom.ch8
```

Pass `-t` to print a trace of every fetched instruction, its decoded form, and register state after execution:

```bash
cargo run -- path/to/rom.ch8 -t
```

## How it works

Chalice-8 loads a `.ch8` ROM's raw bytes into a 4KB memory buffer starting at `0x200`, alongside a built-in font set at `0x50`. Each cycle, it fetches a 2-byte instruction at the program counter, decodes it into an internal `Operation` enum, and executes it against 16 general-purpose registers, an index register, a call stack, and delay/sound timers.
