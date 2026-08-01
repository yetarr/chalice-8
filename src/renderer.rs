use iced::event::Event::Keyboard;
use iced::keyboard;
use iced::time::{self, Duration};
use iced::widget::canvas;
use iced::widget::canvas::{Frame, Path};
use iced::{Color, Element, Rectangle, Renderer, Theme, mouse};
use rodio::source::SineWave;
use rodio::{MixerDeviceSink, Player};

use crate::machine::{DISPLAY_PIXELS, Machine};

pub struct App {
    machine: Machine,
    _sink: MixerDeviceSink,
    beep_player: Player,
    is_playing: bool,
}

impl App {
    pub fn new(machine: Machine) -> Self {
        let mut sink = rodio::DeviceSinkBuilder::open_default_sink()
            .expect("open default audio stream");
        sink.log_on_drop(false);
        let beep_player = Player::connect_new(sink.mixer());
        beep_player.set_volume(0.5);
        App { machine, _sink: sink, beep_player, is_playing: false }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    Event(iced::event::Event),
}

pub fn update(app: &mut App, message: Message) {
    match message {
        Message::Tick => {
            for _ in 0..10 {
                app.machine.cycle();
            }
            app.machine.sync_timers();

            let should_play = app.machine.can_play_sound();
            if should_play && !app.is_playing {
                app.beep_player.append(SineWave::new(440.0));
                app.is_playing = true;
            } else if !should_play && app.is_playing {
                app.beep_player.stop();
                app.is_playing = false;
            }
        }
        Message::Event(event) => {
            if let Keyboard(keyboard::Event::KeyPressed {
                key,
                modified_key: _,
                physical_key,
                location: _,
                modifiers: _,
                text: _,
                repeat: _,
            }) = &event
            {
                let key = &key.to_latin(*physical_key).unwrap_or('0').to_string();
                app.machine.key_pressed(key);
            }

            if let Keyboard(keyboard::Event::KeyReleased {
                key,
                modified_key: _,
                physical_key,
                location: _,
                modifiers: _,
            }) = &event
            {
                let key = &key.to_latin(*physical_key).unwrap_or('0').to_string();
                app.machine.set_key(key, false);
            }
        }
    }
}

pub fn view(app: &App) -> Element<'_, Message> {
    canvas(ChaliceDisplay {
        buf: app.machine.display_buf(),
    })
    .width(iced::Length::Fill)
    .height(iced::Length::Fill)
    .into()
}

pub fn subscription(_app: &App) -> iced::Subscription<Message> {
    iced::Subscription::batch([
        time::every(Duration::from_millis(16)).map(|_| Message::Tick),
        iced::event::listen().map(Message::Event),
    ])
}

struct ChaliceDisplay<'a> {
    buf: &'a [bool; DISPLAY_PIXELS],
}

impl<'a> canvas::Program<Message> for ChaliceDisplay<'a> {
    type State = ();
    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
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
