use iced::canvas::Cache;
use rand::random;

const STEP_SIZE: f32 = 5f32;
const DEFAULT_SPREAD: f32 = 20f32;
const HEADING_CHANGE: f32 = 20f32;
pub const ANT_SIZE: f32 = 2f32;

pub struct Ground {
    pub running: bool,
    pub cache: Cache,
    pub ants: Vec<Ant>,
    pub food: Vec<Food>,
    pub pheromones: Pheromones,
}

#[derive(Debug)]
pub struct Pheromones {
    location: Vec<u8>,
    width: u32,
}

#[derive(Debug)]
pub struct Ant {
    pub x: f32,
    pub y: f32,
    pub heading: f32,
    pub carrying: bool,
}

#[derive(Debug)]
pub struct Food {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Pheromones {
    pub fn new(size: (u32, u32)) -> Pheromones {
        Pheromones {
            location: Vec::with_capacity(size.0 as usize * size.1 as usize),
            width: size.0,
        }
    }
}

impl Food {
    /// positions: (x, y, width, height)
    pub fn spawn(locations: Vec<(f32, f32, f32, f32)>) -> Vec<Food> {
        let mut food: Vec<Food> = Vec::new();
        for loc in locations {
            food.push(Food {
                x: loc.0,
                y: loc.1,
                width: loc.2,
                height: loc.3,
            });
        }
        food
    }
}

impl Ant {
    #[allow(dead_code)]
    pub fn new(x: f32, y: f32) -> Self {
        Ant {
            x,
            y,
            heading: random::<f32>().trunc(),
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
                heading: (random::<i32>() % 360) as f32,
                carrying: false,
            });
        }
        ant_group
    }

    pub fn step(&mut self) {
        let heading = self.heading * std::f32::consts::PI / 180f32;
        self.x = self.x + (STEP_SIZE * heading.cos());
        self.y = self.y - (STEP_SIZE * heading.sin());
        let new_heading = random::<f32>() * 100f32;
        self.heading += new_heading % HEADING_CHANGE;
    }
}
