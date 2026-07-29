use iced::widget::canvas;
use iced::widget::canvas::{Path, Frame};
use iced::{Color, Element, Rectangle, Renderer, Theme, mouse};
use iced::time::{self, Duration};

use crate::machine::Machine;

pub struct App {
    machine: Machine
}

impl App {
    pub fn new(machine: Machine) -> Self {
        App { machine }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
}

pub fn update(app: &mut App, message: Message) {
    match message {
        Message::Tick => {
            for _ in 0..10 {
                app.machine.cycle();
            }
            app.machine.sync_timers();
        }
    }
}

pub fn view(app: &App) -> Element<Message> {
    canvas(ChaliceDisplay { buf: app.machine.display_buf() })
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
}

pub fn subscription(_app: &App) -> iced::Subscription<Message> {
    time::every(Duration::from_millis(16))
        .map(|_| Message::Tick)
}

struct ChaliceDisplay<'a> {
    buf: &'a [bool; 64 * 32],
}

impl<'a> canvas::Program<Message> for ChaliceDisplay<'a> {
    type State = ();
    fn draw(&self, _state: &(), renderer: &Renderer, _theme: &Theme, bounds: Rectangle, _cursor: mouse::Cursor) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let pixel_size = bounds.width / 64.0;
        for y in 0..32 {
            for x in 0..64 {
                if self.buf[y * 64 + x] {
                    let rect = Path::rectangle(
                        iced::Point::new(x as f32 * pixel_size, y as f32 * pixel_size),
                        iced::Size::new(pixel_size, pixel_size),
                    );
                    frame.fill(&rect, Color::WHITE);
                }
            }
        }
        vec![frame.into_geometry()]
    }
}