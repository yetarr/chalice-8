use std::env;

mod machine;
mod parser;
mod renderer;

use machine::Machine;
use renderer::App;

fn main() -> iced::Result {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} <rom_path> [-d]", args[0]);
        return Ok(());
    }
    let rom_path = args[1].clone();
    let mut with_trace = false;
    if args.len() > 2 {
        if args[2] == "-d" {
            with_trace = true;
        } else {
            eprintln!("usage: {} <rom_path> [-d]", args[0]);
            return Ok(());
        }
    }

    iced::application(
        move || {
            let mut machine = Machine::new();
            machine.load(&rom_path).unwrap();
            if with_trace { machine.debug(); }
            App::new(machine)
        },
        renderer::update,
        renderer::view,
    )
    .subscription(renderer::subscription)
    .title("Chalice-8")
    .run()
}
