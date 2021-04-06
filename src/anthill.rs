use iced::canvas::Cache;
use rand::random;

const STEP_SIZE: f32 = 5f32;
const DEFAULT_SPREAD: f32 = 150f32;
const HEADING_CHANGE: f32 = 180f32;
pub const ANT_SIZE: f32 = 2f32;

pub struct Ground {
    pub running: bool,
    pub cache: Cache,
    pub ants: Vec<Ant>,
}

pub struct Ant {
    pub x: f32,
    pub y: f32,
    pub heading: f32,
    pub carrying: bool,
}

impl Ant {
    #[allow(dead_code)]
    pub fn new(x: f32, y: f32) -> Self {
        Ant {
            x,
            y,
            heading: random::<u32>() as f32, // to remove remainder
            carrying: false,
        }
    }
    pub fn spawn(count: i32, center_x: f32, center_y: f32, spread: Option<f32>) -> Vec<Ant> {
        let mut ant_group: Vec<Ant> = Vec::new();
        for _ in 0..count {
            let spread = match spread {
                Some(val) => val,
                None => DEFAULT_SPREAD,
            };
            ant_group.push(Ant {
                x: center_x + (random::<f32>() % spread),
                y: center_y + (random::<f32>() % spread),
                heading: random::<u32>() as f32,
                carrying: false,
            });
        }
        ant_group
    }

    pub fn step(&mut self) {
        let heading = self.heading * std::f32::consts::PI / 180f32;
        self.x = self.x + (STEP_SIZE * heading.cos());
        self.y = self.y - (STEP_SIZE * heading.sin());
        self.heading += (random::<u32>() as f32) % HEADING_CHANGE;
    }
}
