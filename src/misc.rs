#[derive(Clone)]
pub struct Dir(pub i32, pub i32);

pub static DIRECTIONS: &'static [Dir] = &[
    Dir(1, 0),   // E
    Dir(1, -1),  // NE
    Dir(0, -1),  // N
    Dir(-1, -1), // NW
    Dir(-1, 0),  // W
    Dir(-1, 1),  // SW
    Dir(0, 1),   // S
    Dir(1, 1),   // SE
];

impl Dir {
    pub fn normalize(self: &Self) -> f32 {
        match DIRECTIONS
            .iter()
            .enumerate()
            .find(|(_, x)| self.0 == x.0 && self.1 == x.1)
        {
            None => 0.0,
            Some((i, _)) => i as f32 / 7.0,
        }
    }
}
