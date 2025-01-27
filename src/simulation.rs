use core::f32;
use rand::seq::SliceRandom;
use sdl2::{self, libc::DIR, pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::{
    genome::Gene,
    misc::{Dir, DIRECTIONS},
    wyrm::{self, Wyrm},
};

pub struct Simulation {
    pub state: SimulationState,
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

        s.create_selection_area();
        return s;
    }

    pub fn create_selection_area(&mut self) {
        self.state
            .selection_area
            .iter_mut()
            .for_each(|col| col.iter_mut().for_each(|c| *c = false));

        /*
        // selection area in the middle of the world
        for x in 0..self.state.size_x {
            for y in 0..self.state.size_y {
                self.state.selection_area[x as usize][y as usize] = x >= self.state.size_x * 3 / 8
                    && x < self.state.size_x * 5 / 8
                    && y >= self.state.size_y * 3 / 8
                    && y < self.state.size_y * 5 / 8;
            }
        }
        */

        // random spots at certain density
        /*
        let total_area = self.state.size_x as f32 * self.state.size_y as f32;
        let mut ok_count = 0.0;
        loop {
            let (mut sx, mut sy, mut dir) = (
                (rand::random::<i32>() % self.state.size_x).abs(),
                (rand::random::<i32>() % self.state.size_y).abs(),
                DIRECTIONS[rand::random::<usize>() % DIRECTIONS.len()].clone(),
            );
            for _ in 0..rand::random::<u32>() % 90 {
                let (x, y) = (sx + dir.0, sy + dir.1);
                if x < 0 || x >= self.state.size_x || y < 0 || y >= self.state.size_y {
                    continue;
                }
                if self.state.selection_area[x as usize][y as usize] {
                    continue;
                }
                self.state.selection_area[x as usize][y as usize] = true;
                ok_count += 1.0;
                (sx, sy) = (x, y);
                dir = DIRECTIONS[rand::random::<usize>() % DIRECTIONS.len()].clone();
                if ok_count / total_area >= 0.3 {
                    return;
                }
            }
        }
        */

        // random rectangles
        for _ in 0..10 {
            let (sx, sy, w, h) = (
                rand::random::<u32>() % self.state.size_x as u32,
                rand::random::<u32>() % self.state.size_y as u32,
                rand::random::<u32>() % 30,
                rand::random::<u32>() % 30,
            );
            for x in sx..(sx + w) {
                for y in sy..sy + h {
                    if x as i32 >= self.state.size_x || y as i32 >= self.state.size_y {
                        continue;
                    }
                    self.state.selection_area[x as usize][y as usize] = true;
                }
            }
        }
    }

    pub fn pick_free_cell(&mut self) -> (i32, i32) {
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

    pub fn simulation_step(&mut self) -> i32 {
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

    pub fn apply_selection(&mut self) -> usize {
        let mut died: usize = 0;
        self.wyrmas.iter_mut().for_each(|w| {
            if !self.state.selection_area[w.state.x as usize][w.state.y as usize] {
                died += 1;
                w.state.dead = true;
            }
        });
        return self.wyrmas.len() - died;
    }

    pub fn get_survivor(&self) -> Option<&Wyrm> {
        self.wyrmas.iter().filter(|w| !w.state.dead).next()
    }

    pub fn repopulate(&mut self) {
        let mut new_genomes = self.breed_survivors();
        self.state
            .world
            .iter_mut()
            .for_each(|ys| ys.iter_mut().for_each(|v| *v = false));

        // reuse old generation by re-placing them randomly
        // and rewiring neurons using new genome
        for i in 0..new_genomes.len() {
            let (x, y) = self.pick_free_cell();
            self.wyrmas[i].reset(new_genomes.pop().unwrap(), x, y);
        }
        self.state.tick = 0;
    }

    fn breed_survivors(&self) -> Vec<Vec<Gene>> {
        let survivors: Vec<&Wyrm> = self.wyrmas.iter().filter(|w| !w.state.dead).collect();
        if survivors.is_empty() {
            // nobody survived, generate random gene pool from scratch :(
            return (0..self.wyrmas.len())
                .map(|_| {
                    (0..self.wyrmas[0].state.genome.len())
                        .map(|_| Gene(rand::random()))
                        .collect::<Vec<Gene>>()
                })
                .collect();
        }

        let child_count = self.wyrmas.len() / survivors.len();
        let mut new_genomes = Vec::with_capacity(self.wyrmas.len());

        // generate random paris from survived population
        // each pair will have at least child_count children
        //survivors.shuffle(&mut rand::thread_rng());
        for i in perm(survivors.len()) {
            new_genomes.extend((0..child_count).map(|_| {
                survivors[i].breed(
                    &survivors[(i + 1) % survivors.len()],
                    &self.state.mutation_rate,
                )
            }));
        }

        // generate some more random pairs to top up to the target population
        for i in perm(survivors.len())
            .iter()
            .take(self.wyrmas.len() % survivors.len())
        {
            new_genomes.push(survivors[*i].breed(
                &survivors[(*i + 1) % survivors.len()],
                &self.state.mutation_rate,
            ));
        }

        return new_genomes;
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, cell_size: i16) {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // draw selection area
        canvas.set_draw_color(Color::RGB(0, 0x40, 0));
        for x in 0..self.state.size_x {
            for y in 0..self.state.size_y {
                if self.state.selection_area[x as usize][y as usize] {
                    canvas
                        .fill_rect(Rect::new(
                            x * cell_size as i32,
                            y * cell_size as i32,
                            cell_size as u32,
                            cell_size as u32,
                        ))
                        .unwrap();
                }
            }
        }

        // draw wyrmas
        canvas.set_draw_color(Color::RGB(0x80, 0, 0));
        for w in &self.wyrmas {
            canvas
                .fill_rect(Rect::new(
                    w.state.x * cell_size as i32,
                    w.state.y * cell_size as i32,
                    cell_size as u32,
                    cell_size as u32,
                ))
                .unwrap();
        }

        /*
        // draw grid
        for x in 1..self.state.size_x as i16 {
            canvas
                .vline(
                    x * cell_size,
                    0,
                    canvas.window().size().1 as i16,
                    Color::GRAY,
                )
                .unwrap();
        }
        for y in 1..self.state.size_y as i16 {
            canvas
                .hline(
                    0,
                    canvas.window().size().0 as i16,
                    y * cell_size,
                    Color::GRAY,
                )
                .unwrap();
        }
        */
    }
}

fn perm(n: usize) -> Vec<usize> {
    let mut r: Vec<usize> = (0..n).collect();
    r.shuffle(&mut rand::thread_rng());
    return r;
}
