use iced::{
    canvas::{self, Canvas, Cursor, Geometry, Path},
    executor, time,
    window::Settings as WindowSettings,
    Align, Application, Color, Column, Command, Container, Element, Length, Point, Rectangle,
    Settings, Size, Subscription,
};
use iced_native::event::Event;
use iced_native::keyboard::Event as KeyboardEvent;

mod anthill;

const SPEED: u64 = 200;
const SWARM_SIZE: i32 = 10;
const WINDOW_SIZE: (u32, u32) = (600, 600);
const ANTS_LOCATION: (f32, f32) = (200f32, 200f32);
const FOOD_LOCATION: (f32, f32) = (100f32, 100f32);
const FOOD_SIZE: (f32, f32) = (40f32, 40f32);

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
                ants: anthill::Ant::spawn(SWARM_SIZE, ANTS_LOCATION.0, ANTS_LOCATION.1, None),
                food: anthill::Food::spawn(vec![(
                    FOOD_LOCATION.0,
                    FOOD_LOCATION.1,
                    FOOD_SIZE.0,
                    FOOD_SIZE.1,
                )]),
                pheromones: anthill::Pheromones::new(WINDOW_SIZE),
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
                for i in 0..self.ants.len() {
                    self.ants[i].step(&self.pheromones);
                    for food in &self.food {
                        let x = self.ants[i].x;
                        let y = self.ants[i].y;
                        if !self.ants[i].carrying
                            && x > food.x
                            && y > food.y
                            && x < food.x + food.width
                            && y < food.y + food.height
                        {
                            self.ants[i].carrying = true;
                        }
                    }
                    self.pheromones.update(&self.ants[i]);
                }
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
            let _center = frame.center();

            let red: Color = Color::from_rgb8(0xc2, 0x23, 0x30);
            let blue: Color = Color::from_rgb8(0x02, 0x13, 0xca);
            let green: Color = Color::from_rgb8(0x02, 0xc3, 0x2a);

            for food in &self.food {
                let food_circle = Path::rectangle(
                    Point::new(food.x, food.y),
                    Size::new(food.width, food.height),
                );
                frame.fill(&food_circle, blue);
            }

            for y in 0..self.pheromones.location.len() {
                for x in 0..self.pheromones.location[0].len() {
                    match self.pheromones.location[y][x] {
                        anthill::PheromoneTypes::None => {}
                        _ => {
                            let pheromone_spot = Path::rectangle(
                                Point::new(x as f32, y as f32),
                                Size::new(3f32, 3f32),
                            );
                            frame.fill(&&pheromone_spot, green);
                        }
                    }
                }
            }

            for ant in &self.ants {
                let ant_circle = Path::circle(Point::new(ant.x, ant.y), anthill::ANT_SIZE);
                frame.fill(
                    &ant_circle,
                    match ant.carrying {
                        true => green,
                        false => red,
                    },
                );
            }
        });

        vec![ground]
    }
}
