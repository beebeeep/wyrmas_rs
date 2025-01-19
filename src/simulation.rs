use std::os;

use crate::{
    genome::Gene,
    wyrm::{self, Wyrm},
};

pub struct Simulation {
    state: SimulationState,
    wyrmas: Vec<wyrm::Wyrm>,
}

pub struct SimulationState {
    pub size_x: i32,
    pub size_y: i32,
    pub max_age: i32,
    pub osc_period: i32,
    pub mutation_rate: f32,
    pub tick: u64,
}

impl Simulation {
    pub fn new(
        size_x: i32,
        size_y: i32,
        osc_period: i32,
        num_inner_neurons: usize,
        max_age: i32,
        max_dist: i32,
        genome_len: usize,
        population: usize,
        mutation_rate: f32,
    ) -> Self {
        let mut s = Simulation {
            state: SimulationState {
                tick: 0,
                size_x: size_x,
                size_y: size_y,
                max_age: max_age,
                osc_period: osc_period,
                mutation_rate: mutation_rate,
            },
            wyrmas: Vec::with_capacity(population),
        };
        for _ in 0..population {
            s.wyrmas.push(Wyrm::new(
                rand::random::<i32>() % s.state.size_x,
                rand::random::<i32>() % s.state.size_y,
                num_inner_neurons,
                max_dist,
                (0..genome_len).map(|_| Gene(rand::random())).collect(),
            ));
        }
        return s;
    }

    pub fn simulationStep(self: &mut Self) -> u64 {
        for w in &mut self.wyrmas {
            w.simulationStep(&mut self.state);
        }
        self.state.tick += 1;
        return self.state.tick;
    }
}
