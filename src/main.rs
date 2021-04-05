use chrono::Timelike;
use iced::{
    canvas::{self, Cache, Canvas, Cursor, Geometry, LineCap, Path, Stroke},
    executor, time,
    window::Settings as WindowSettings,
    Align, Application, Color, Column, Command, Container, Element, Length, Point, Rectangle, Row,
    Settings, Subscription, Vector,
};
use iced_native::event::Event;
use iced_native::keyboard::Event as KeyboardEvent;

pub fn main() -> iced::Result {
    Clock::run(Settings {
        window: WindowSettings {
            size: (400, 200),
            ..WindowSettings::default()
        },
        antialiasing: true,
        ..Settings::default()
    })
}

struct Clock {
    count: u32,
    total_work: u32,
    total_rest: u32,
    work_sessions: u32,
    work: bool,
    now: chrono::DateTime<chrono::Local>,
    paused: bool,
    previous: u32,
    clock: Cache,
}

#[derive(Debug, Clone)]
enum Message {
    Tick(chrono::DateTime<chrono::Local>),
    EventOccured(iced_native::Event),
}

impl Application for Clock {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Clock {
                count: 0,
                total_work: 60,
                total_rest: 60,
                work_sessions: 0,
                work: true,
                now: chrono::Local::now(),
                paused: false,
                previous: chrono::Local::now().minute(),
                clock: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Rusty Ants")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(local_time) => {
                println!("tick");
            }
            Message::EventOccured(event) => {
                if let Event::Keyboard(keyboard_event) = event {
                    if let KeyboardEvent::CharacterReceived(ch) = keyboard_event {
                        println!("key pressed");
                        self.clock.clear();
                    }
                }
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            iced_native::subscription::events().map(Message::EventOccured),
            time::every(std::time::Duration::from_millis(500))
                .map(|_| Message::Tick(chrono::Local::now())),
        ])
    }

    fn view(&mut self) -> Element<Message> {
        let canvas = Container::new(
            Canvas::new(self)
                .width(Length::Units(100))
                .height(Length::Units(100)),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(5)
        .align_x(Align::End)
        .center_y();

        Column::new().padding(20).push(canvas).into()
    }
}

impl canvas::Program<Message> for Clock {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let clock = self.clock.draw(bounds.size(), |frame| {
            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.0;

            let background = Path::circle(center, radius);

            let color: Color = match self.paused {
                false => match self.work {
                    true => Color::from_rgb8(0xc2, 0x23, 0x30),
                    false => Color::from_rgb8(0x19, 0xa8, 0x5b),
                },
                true => Color::from_rgb8(0x77, 0x77, 0x77),
            };
            frame.fill(&background, color);

            let short_hand = Path::line(Point::ORIGIN, Point::new(0.0, -0.5 * radius));
            let long_hand = Path::line(Point::ORIGIN, Point::new(0.0, -0.8 * radius));

            let thin_stroke = Stroke {
                width: radius / 100.0,
                color: Color::WHITE,
                line_cap: LineCap::Round,
                ..Stroke::default()
            };

            let wide_stroke = Stroke {
                width: thin_stroke.width * 3.0,
                ..thin_stroke
            };

            frame.translate(Vector::new(center.x, center.y));

            frame.with_save(|frame| {
                frame.rotate(2f32);
                frame.stroke(&short_hand, wide_stroke);
            });
        });

        vec![clock]
    }
}
