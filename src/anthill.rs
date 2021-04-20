use iced::canvas::Cache;
use rand::random;

const STEP_SIZE: f32 = 5f32;
const DEFAULT_SPREAD: f32 = 20f32;
const HEADING_CHANGE: f32 = 60f32;
pub const ANT_SIZE: f32 = 2f32;

const TO_LOOK_POSITIONS: [(f32, f32, f32); 8] = [
    (-1f32, -1f32, 135f32), // top left
    (0f32, -1f32, 90f32),   // top
    (1f32, -1f32, 45f32),   // top right
    (1f32, 0f32, 0f32),     // right
    (1f32, 1f32, 315f32),   // bottom right
    (0f32, 1f32, 270f32),   // bottom
    (-1f32, 1f32, 225f32),  // bottom left
    (-1f32, 0f32, 180f32),  // left
];

pub struct Ground {
    pub running: bool,
    pub cache: Cache,
    pub ants: Vec<Ant>,
    pub food: Vec<Food>,
    pub pheromones: Pheromones,
}

#[derive(Debug)]
pub struct Pheromones {
    pub location: Vec<Vec<PheromoneTypes>>,
}

#[derive(Debug, Clone)]
pub enum PheromoneTypes {
    None,
    ToFood,
    ToHome,
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
            location: vec![vec![PheromoneTypes::None; size.0 as usize]; size.1 as usize],
        }
    }
    pub fn update(&mut self, ant: &Ant) {
        let pheromone: PheromoneTypes = match ant.carrying {
            true => PheromoneTypes::ToFood,
            false => PheromoneTypes::ToHome,
        };
        self.location[ant.y as usize][ant.x as usize] = pheromone;
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

    pub fn step(&mut self, pheromones: &Pheromones) {
        let heading = self.heading * std::f32::consts::PI / 180f32;
        self.x = self.x + (STEP_SIZE * heading.cos());
        self.y = self.y - (STEP_SIZE * heading.sin());

        let mut new_heading: Option<f32> = None;

        let max_x = pheromones.location[0].len() as f32 - 1f32;
        let max_y = pheromones.location.len() as f32 - 1f32;
        for (pos_x, pos_y, pos_head) in TO_LOOK_POSITIONS.iter() {
            let x = self.x + pos_x;
            let y = self.y + pos_y;
            if x < 0f32 || y < 0f32 || x > max_x || y > max_y {
                continue;
            }
            match pheromones.location[y as usize][x as usize] {
                PheromoneTypes::ToHome => {
                    new_heading = Some(*pos_head);
                    break;
                }
                PheromoneTypes::ToFood => {
                    new_heading = Some(*pos_head);
                    break;
                }
                PheromoneTypes::None => {}
            }
        }
        match new_heading {
            Some(x) => self.heading = x,
            None => {
                let new_heading = random::<f32>() * 100f32;
                self.heading += new_heading % (2f32 * HEADING_CHANGE) - HEADING_CHANGE;
            }
        }
    }
}
