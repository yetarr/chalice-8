# Chalice-8

A CHIP-8 emulator written in Rust, built with as few external crates as possible. the core machine (memory, registers, opcodes) is pure `std`.

## Status

In development. Implemented: machine state (memory, registers, stack, timers, display buffer), ROM loading, the fetch-decode-execute loop, simple cli display and the following opcodes:

- `00E0` - clear display
- `1NNN` - jump
- `6XNN` - set register
- `8XY0`–`8XYE` - full arithmetic/logic family (OR, AND, XOR, add/sub with carry, shifts)
- `3XNN`, `4XNN`, `5XY0`, `9XY0` - conditional skips
- `2NNN`, `00ee` - subroutine calls
- `ANNN` - set index register
- `DXYN` - sprite drawing

Not yet implemented: display rendering, keyboard input, and timers.

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
