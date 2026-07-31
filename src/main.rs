use std::env;

mod machine;
mod parser;
mod renderer;

use machine::Machine;
use renderer::App;

fn main() -> iced::Result {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} <rom_path> [-t]", args[0]);
        return Ok(());
    }
    let rom_path = args[1].clone();

    iced::application(
        move || {
            let mut machine = Machine::new();
            machine.load(&rom_path).unwrap();
            App::new(machine)
        },
        renderer::update,
        renderer::view,
    )
    .subscription(renderer::subscription)
    .title("Chalice-8")
    .run()
}
