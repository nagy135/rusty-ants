use iced::{
    canvas::{self, Canvas, Cursor, Geometry, Path},
    executor, time,
    window::Settings as WindowSettings,
    Align, Application, Color, Column, Command, Container, Element, Length, Point, Rectangle,
    Settings, Subscription, Vector,
};
use iced_native::event::Event;
use iced_native::keyboard::Event as KeyboardEvent;

mod anthill;

const SPEED: u64 = 100;
const WINDOW_SIZE: (u32, u32) = (600, 600);

pub fn main() -> iced::Result {
    anthill::Ground::run(Settings {
        window: WindowSettings {
            size: WINDOW_SIZE,
            ..WindowSettings::default()
        },
        antialiasing: true,
        ..Settings::default()
    })
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick(chrono::DateTime<chrono::Local>),
    EventOccured(iced_native::Event),
}

impl Application for anthill::Ground {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            anthill::Ground {
                running: true,
                cache: Default::default(),
                ant: anthill::Ant::new(0f32, 100f32),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Rusty Ants")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(_) => {
                self.ant.step();
                self.cache.clear();
            }
            Message::EventOccured(event) => {
                if let Event::Keyboard(keyboard_event) = event {
                    if let KeyboardEvent::CharacterReceived(_ch) = keyboard_event {
                        println!("key pressed");
                        self.cache.clear();
                    }
                }
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            iced_native::subscription::events().map(Message::EventOccured),
            time::every(std::time::Duration::from_millis(SPEED))
                .map(|_| Message::Tick(chrono::Local::now())),
        ])
    }

    fn view(&mut self) -> Element<Message> {
        let canvas = Container::new(
            Canvas::new(self)
                .width(Length::Units(WINDOW_SIZE.0 as u16))
                .height(Length::Units(WINDOW_SIZE.1 as u16)),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Align::End)
        .center_y();

        Column::new().push(canvas).into()
    }
}

impl canvas::Program<Message> for anthill::Ground {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let ground = self.cache.draw(bounds.size(), |frame| {
            let center = frame.center();

            let ant = Path::circle(Point::new(self.ant.x, self.ant.y), anthill::ANT_SIZE);

            frame.translate(Vector::new(center.x, center.y));
            let red: Color = Color::from_rgb8(0xc2, 0x23, 0x30);

            frame.fill(&ant, red);
        });

        vec![ground]
    }
}
