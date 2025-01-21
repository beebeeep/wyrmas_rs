use std::cell::RefCell;
use std::rc::Rc;

use crate::genome::{self, mix_genome, Gene};
use crate::misc::{Dir, DIRECTIONS};
use crate::neuron::{Neuron, ACTIONS, INNER, SENSORS};
use crate::simulation::{Simulation, SimulationState};
use rand;
use rand::distributions::Standard;

pub struct WyrmState {
    pub dead: bool,
    pub x: i32,
    pub y: i32,
    pub dir: Dir,
    pub age: i32,
    pub max_dist: i32,
    pub genome: Vec<genome::Gene>,
    pub responsiveness: f32,
}

pub struct Wyrm {
    pub state: WyrmState,
    sensor_layer: Vec<Rc<RefCell<Neuron>>>,
    inner_layer: Vec<Rc<RefCell<Neuron>>>,
    action_layer: Vec<Rc<RefCell<Neuron>>>,
}

impl Wyrm {
    pub fn new(x: i32, y: i32, num_inner: usize, max_dist: i32, genome: Vec<genome::Gene>) -> Self {
        let mut w = Wyrm {
            state: WyrmState {
                dead: false,
                x: x,
                y: y,
                age: 0,
                max_dist: max_dist,
                dir: DIRECTIONS[rand::random::<usize>() % DIRECTIONS.len()].clone(),
                responsiveness: 1.0,
                genome: genome,
            },
            sensor_layer: Vec::with_capacity(10),
            inner_layer: Vec::with_capacity(num_inner),
            action_layer: Vec::with_capacity(3),
        };
        let mut id = 0;
        // earch wyrm has full set of neurons that are not necessarily wired together
        for i in 0..w.sensor_layer.capacity() {
            w.sensor_layer.push(Rc::new(RefCell::new(Neuron::new(
                id,
                SENSORS[i % SENSORS.len()],
            ))));
            id += 1;
        }
        for i in 0..w.inner_layer.capacity() {
            w.inner_layer.push(Rc::new(RefCell::new(Neuron::new(
                id,
                INNER[i % INNER.len()],
            ))));
            id += 1;
        }
        for i in 0..w.action_layer.capacity() {
            w.action_layer.push(Rc::new(RefCell::new(Neuron::new(
                id,
                ACTIONS[i % ACTIONS.len()],
            ))));
            id += 1
        }

        w.wire_neurons();

        return w;
    }

    pub fn wire_neurons(&mut self) {
        // as neurons will be reused accross generations, reset old links
        self.inner_layer.iter().for_each(|n| n.borrow_mut().reset());
        self.action_layer
            .iter()
            .for_each(|n| n.borrow_mut().reset());

        for g in &self.state.genome {
            let src = match g.get_src() {
                (true, id) => self.inner_layer[id % self.inner_layer.len()].clone(),
                (false, id) => self.sensor_layer[id % self.sensor_layer.len()].clone(),
            };
            let sink = match g.get_sink() {
                (true, id) => self.inner_layer[id % self.inner_layer.len()].clone(),
                (false, id) => self.action_layer[id % self.action_layer.len()].clone(),
            };
            let src = if src.borrow().id == sink.borrow().id {
                None
            } else {
                Some(src)
            };
            sink.borrow_mut().add_input(g.get_weight(), src);
        }
    }

    pub fn simulation_step(self: &mut Self, state: &mut SimulationState) {
        self.sensor_layer
            .iter()
            .for_each(|n| n.borrow_mut().activate(&mut self.state, state));

        self.inner_layer
            .iter()
            .for_each(|n| n.borrow_mut().activate(&mut self.state, state));

        self.action_layer
            .iter()
            .for_each(|n| n.borrow_mut().activate(&mut self.state, state));
    }

    pub fn breed(self: &Self, partner: &Self, mutation_rate: &f32) -> Vec<Gene> {
        let mut genome = mix_genome(&self.state.genome, &partner.state.genome);
        for gene in &mut genome {
            if rand::random::<f32>() < *mutation_rate {
                gene.mutate();
            }
        }
        return genome;
    }
}
