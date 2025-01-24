use std::cell::RefCell;
use std::rc::Rc;

use crate::genome::{self, mix_genome, Gene};
use crate::misc::{Dir, DIRECTIONS};
use crate::neuron::{Neuron, ACTIONS, ACTION_NAMES, INNER, INNER_NAME, SENSORS, SENSOR_NAMES};
use crate::simulation::SimulationState;
use dot_writer::{Attributes, DotWriter};
use rand;

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
        // earch wyrm has full set of neurons that are not necessarily wired together
        for i in 0..w.sensor_layer.capacity() {
            let n = i % SENSORS.len();
            w.sensor_layer.push(Rc::new(RefCell::new(Neuron::new(
                String::from(SENSOR_NAMES[n]),
                SENSORS[n],
            ))));
        }
        for i in 0..w.inner_layer.capacity() {
            w.inner_layer.push(Rc::new(RefCell::new(Neuron::new(
                format!("{}{i}", INNER_NAME),
                INNER[i % INNER.len()],
            ))));
        }
        for i in 0..w.action_layer.capacity() {
            let n = i % ACTIONS.len();
            w.action_layer.push(Rc::new(RefCell::new(Neuron::new(
                String::from(ACTION_NAMES[n]),
                ACTIONS[n],
            ))));
        }

        w.wire_neurons();

        return w;
    }

    pub fn reset(&mut self, genome: Vec<Gene>, x: i32, y: i32) {
        (self.state.x, self.state.y) = (x, y);
        self.state.dead = false;
        self.state.age = 0;
        self.state.responsiveness = 1.0;
        self.state.dir = DIRECTIONS[rand::random::<usize>() % DIRECTIONS.len()].clone();
        self.state.genome = genome;
        self.wire_neurons();
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
            let src = if src.borrow().name == sink.borrow().name {
                None
            } else {
                Some(src)
            };
            sink.borrow_mut().add_input(g.get_weight(), src);
        }
    }

    pub fn simulation_step(&mut self, state: &mut SimulationState) {
        self.state.age += 1;
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

    pub fn breed(&self, partner: &Self, mutation_rate: &f32) -> Vec<Gene> {
        let mut genome = mix_genome(&self.state.genome, &partner.state.genome);
        for gene in &mut genome {
            if rand::random::<f32>() < *mutation_rate {
                gene.mutate();
            }
        }
        return genome;
    }

    pub fn dump_genome(&self) -> Vec<u8> {
        let mut dot = Vec::new();
        {
            let mut writer = DotWriter::from(&mut dot);
            let mut digraph = writer.digraph();
            for n in &self.inner_layer {
                for (input, weight) in n.borrow().get_inputs() {
                    digraph
                        .edge(input, n.borrow().name.clone())
                        .attributes()
                        .set_label(&format!("{weight:.1}"));
                }
            }
            for n in &self.action_layer {
                for (input, weight) in n.borrow().get_inputs() {
                    digraph
                        .edge(input, n.borrow().name.clone())
                        .attributes()
                        .set_label(&format!("{weight:.1}"));
                }
            }
        }
        return dot;
    }
}
