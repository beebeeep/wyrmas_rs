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
    pub osc_value: f32,
    pub mutation_rate: f32,
    pub tick: u64,
    pub world: Vec<Vec<bool>>,
    pub selection_area: Vec<Vec<bool>>,
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
                osc_value: 0.0,
                mutation_rate: mutation_rate,
                world: vec![vec![true; size_y as usize]; size_x as usize],
                selection_area: vec![vec![true; size_y as usize]; size_x as usize],
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

    pub fn simulation_step(self: &mut Self) -> u64 {
        for w in &mut self.wyrmas {
            w.simulation_step(&mut self.state);
        }
        self.state.tick += 1;
        return self.state.tick;
    }
}
