use core::f32;
use std::sync::Arc;

use rand::seq::SliceRandom;

use crate::{
    genome::{mix_genome, Gene},
    misc::DIRECTIONS,
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
    pub tick: i32,
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
                world: vec![vec![false; size_y as usize]; size_x as usize],
                selection_area: vec![vec![false; size_y as usize]; size_x as usize],
            },
            wyrmas: Vec::with_capacity(population),
        };

        // spawn wyrmae
        for _ in 0..population {
            let (x, y) = s.pick_free_cell();
            s.wyrmas.push(Wyrm::new(
                x,
                y,
                num_inner_neurons,
                max_dist,
                (0..genome_len).map(|_| Gene(rand::random())).collect(),
            ));
        }

        // create selection area in the middle of the world
        for x in 0..size_x {
            for y in 0..size_y {
                s.state.selection_area[x as usize][y as usize] = x >= size_x * 3 / 8
                    && x < size_x * 5 / 8
                    && y >= size_y * 3 / 8
                    && y < size_y * 5 / 8;
            }
        }

        return s;
    }

    pub fn pick_free_cell(self: &mut Self) -> (i32, i32) {
        let (mut x, mut y): (i32, i32);
        loop {
            (x, y) = (
                (rand::random::<i32>() % self.state.size_x).abs(),
                (rand::random::<i32>() % self.state.size_y).abs(),
            );
            if !self.state.world[x as usize][y as usize] {
                self.state.world[x as usize][y as usize] = true;
                return (x, y);
            }
        }
    }

    pub fn simulation_step(self: &mut Self) -> i32 {
        self.state.tick += 1;
        self.state.osc_value = 0.5
            + 0.5
                * f32::cos(
                    2.0 * f32::consts::PI * ((self.state.tick % self.state.osc_period) as f32)
                        / self.state.osc_period as f32,
                );
        for w in &mut self.wyrmas {
            w.simulation_step(&mut self.state);
        }
        return self.state.tick;
    }

    pub fn apply_selection(self: &mut Self) -> usize {
        let mut died: usize = 0;
        self.wyrmas.iter_mut().for_each(|w| {
            if !self.state.selection_area[w.state.x as usize][w.state.y as usize] {
                died += 1;
                w.state.dead = true;
            }
        });
        return self.wyrmas.len() - died;
    }

    pub fn repopulate(self: &mut Self) {
        let mut new_genomes = self.breed_survivors();
        self.state
            .world
            .iter_mut()
            .for_each(|ys| ys.iter_mut().for_each(|v| *v = false));

        // reuse old generation by re-placing them randomly
        // and rewiring neurons using new genome
        assert_eq!(new_genomes.len(), self.wyrmas.len());
        for i in 0..new_genomes.len() {
            (self.wyrmas[i].state.x, self.wyrmas[i].state.y) = self.pick_free_cell();
            self.wyrmas[i].state.dead = false;
            self.wyrmas[i].state.dir =
                DIRECTIONS[rand::random::<usize>() % DIRECTIONS.len()].clone();
            self.wyrmas[i].state.genome = new_genomes.pop().unwrap();
            self.wyrmas[i].wire_neurons();
        }
        self.state.tick = 0;
    }

    fn breed_survivors(self: &Self) -> Vec<Vec<Gene>> {
        let mut survivors: Vec<&Wyrm> = self.wyrmas.iter().filter(|w| !w.state.dead).collect();
        let child_count = self.wyrmas.len() / survivors.len();
        let mut new_genomes = Vec::with_capacity(self.wyrmas.len());

        // generate random paris from survived population
        // each pair will have at least child_count children
        survivors.shuffle(&mut rand::thread_rng());
        for i in perm(survivors.len()) {
            new_genomes.extend((0..child_count).map(|_| {
                survivors[i].breed(&survivors[i % survivors.len()], &self.state.mutation_rate)
            }));
        }

        // generate some more random pairs to top up to the target population
        for i in perm(survivors.len())
            .iter()
            .take(self.wyrmas.len() % survivors.len())
        {
            new_genomes.push(
                survivors[*i].breed(&survivors[*i % survivors.len()], &self.state.mutation_rate),
            );
        }

        return new_genomes;
    }
}

fn perm(n: usize) -> Vec<usize> {
    let mut r: Vec<usize> = (0..n).collect();
    r.shuffle(&mut rand::thread_rng());
    return r;
}
