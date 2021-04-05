use iced::canvas::Cache;

const STEP_SIZE: f32 = 5f32;
pub const ANT_SIZE: f32 = 2f32;

pub struct Ground {
    pub running: bool,
    pub cache: Cache,
    pub ant: Ant,
}

pub struct Ant {
    pub x: f32,
    pub y: f32,
    pub heading: f32,
    pub carrying: bool,
}

impl Ant {
    pub fn new(x: f32, y: f32) -> Self {
        Ant {
            x,
            y,
            heading: 80f32,
            carrying: false,
        }
    }
    pub fn step(&mut self) {
        let heading = self.heading * std::f32::consts::PI / 180f32;
        self.x = self.x + (STEP_SIZE * heading.cos());
        self.y = self.y - (STEP_SIZE * heading.sin());
    }
}
