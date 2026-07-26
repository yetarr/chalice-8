use std::env;

mod machine;
mod parser;

use machine::Machine;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} <rom_path> [-t]", args[0]);
        return;
    }
    let rom_path = &args[1];
    let trace = args.iter().any(|a| a == "-t");

    let mut machine = Machine::new();
    machine.load(rom_path).unwrap();
    if trace {
        machine.run_with_dump();
    } else {
        machine.run();
    }
    machine.print_display();
}
