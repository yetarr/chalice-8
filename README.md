# Chalice-8

A CHIP-8 emulator written in Rust. The core machine (memory, registers, opcode decode/execute) is pure `std`; the only real dependencies are for the GUI (Iced) and audio (rodio).

## Features

- Full opcode support: display, flow control (jump/call/return), all conditional skips, the complete arithmetic/logic family, random numbers, BCD conversion, memory block read/write, index register operations, font lookup, and timers
- Windowed 64x32 display rendered with Iced's `Canvas`, driven by a 60Hz tick
- Keyboard input mapped to CHIP-8's 16-key hex keypad (`1234`/`QWER`/`ASDF`/`ZXCV` → `123C`/`456D`/`789E`/`A0BF`), with `FX0A` correctly resolving on the key-press event rather than polling held-key state
- Sound timer drives a real audio tone via rodio

## Usage

```bash
cargo run -- path/to/rom.ch8
```

Pass `-t` to print a dump of the state of the machine after each instruction executed:

```bash
cargo run -- path/to/rom.ch8 -d
```

## How it works

Chalice-8 loads a `.ch8` ROM's raw bytes into a 4KB memory buffer starting at `0x200`, alongside a built-in font set at `0x50`. Each cycle, it fetches a 2-byte instruction at the program counter, decodes it into an internal `Operation` enum, and executes it against 16 general-purpose registers, an index register, a call stack, and delay/sound timers. A GUI event loop (Iced) drives execution at a fixed rate, renders the display buffer to a canvas, forwards keyboard events into the machine's key state, and plays a tone while the sound timer is active.