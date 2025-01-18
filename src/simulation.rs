use crate::wyrm;

pub struct Simulation {
    size_x: i32,
    size_y: i32,
    wyrmas: Vec<wyrm::Wyrm>,
}

impl Simulation {
    pub fn s_age(self: &Self, w: wyrm::Wyrm) -> f32 {
        0.0
    }
}
