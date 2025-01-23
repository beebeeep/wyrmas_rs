// Gene only encodes wyrm's nn connections
// Connection is encoded as follows:
// 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
// │ │  7 bits   │ │ │   7 bits  │ │          16 bits            │
// │ └──src ID───┘ │ └──sink ID──┘ └─────────weight──────────────┘
// └─> src type    └─> sink type
// src or sink type of 1 means inner layer neuron
// 16 bit weight is normalized as float in range  (-4, 4]
// note: endiannes does not matter here

use rand::seq::SliceRandom;

#[derive(Clone)]
pub struct Gene(pub u32);

impl Gene {
    pub fn get_src(self: &Self) -> (bool, usize) {
        (self.0 >> 31 & 1 == 1, (self.0 >> 24 & 127) as usize)
    }

    pub fn get_sink(self: &Self) -> (bool, usize) {
        (self.0 >> 23 & 1 == 1, (self.0 >> 16 & 127) as usize)
    }

    pub fn get_weight(self: &Self) -> f32 {
        ((self.0 & 65535) as i32 - 32767) as f32 / 8192.0
    }

    pub fn mutate(self: &mut Self) {
        // flip from 1 to 3 random bits
        for _ in 0..=rand::random::<i32>() % 3 {
            self.0 = self.0 ^ (1 << rand::random::<u32>() % 32);
        }
    }
}

pub fn mix_genome(a: &Vec<Gene>, b: &Vec<Gene>) -> Vec<Gene> {
    let v = [a, b];
    let mut r = Vec::with_capacity(a.len());
    let mut n: Vec<usize> = (0..a.len()).collect();
    n.shuffle(&mut rand::thread_rng());
    for (i, idx) in n.iter().enumerate() {
        r.push(v[i % 2][*idx].clone()); // combine new genome by randomly selecting genes from a or b
    }
    return r;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weights() {
        assert_eq!(Gene(65535).get_weight(), 4.0);
        assert!(Gene(0).get_weight() < -3.99);
    }
}
