use crate::neuron;

// Gene only encodes wyrm's nn connections
// Connection is encoded as follows:
// 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
// │ │  7 bits   │ │ │   7 bits  │ │          16 bits            │
// │ └──src ID───┘ │ └──sink ID──┘ └─────────weight──────────────┘
// └─> src type    └─> sink type
// src or sink type of 1 means inner layer neuron
// 16 bit weight is normalized as float in range  (-4, 4]
// note: endiannes does not matter here

pub struct Gene(pub u32);

impl Gene {
    fn get_src(self: &Self) -> (bool, usize) {
        (self.0 >> 31 & 1 == 1, (self.0 >> 24 & 127) as usize)
    }

    fn get_sink(self: &Self) -> (bool, usize) {
        (self.0 >> 23 & 1 == 1, (self.0 >> 16 & 127) as usize)
    }

    fn get_weight(self: &Self) -> f32 {
        (self.0 & 65536 - 32767) as f32 / 8192.0
    }

    fn mutate(self: &mut Self) {
        // flip from 1 to 3 random bits
        for _ in 0..=rand::random::<i32>() % 3 {
            self.0 = self.0 ^ (1 << rand::random::<u32>() % 32);
        }
    }
}
