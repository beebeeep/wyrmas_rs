use std::cell::RefCell;
use std::rc::Rc;

use crate::genome;
use crate::misc::Dir;
use crate::neuron::{Neuron, ACTIONS, INNER, SENSORS};
use crate::simulation::SimulationState;
use rand;

pub struct WyrmState {
    pub x: i32,
    pub y: i32,
    pub dir: Dir,
    pub age: i32,
    pub max_dist: i32,
    pub genome: Vec<genome::Gene>,
    pub responsiveness: f32,
}

pub struct Wyrm {
    state: WyrmState,
    sensor_layer: Vec<Rc<RefCell<Neuron>>>,
    inner_layer: Vec<Rc<RefCell<Neuron>>>,
    action_layer: Vec<Rc<RefCell<Neuron>>>,
}

impl Wyrm {
    pub fn new(x: i32, y: i32, num_inner: usize, max_dist: i32, genome: Vec<genome::Gene>) -> Self {
        let mut w = Wyrm {
            state: WyrmState {
                x: x,
                y: y,
                age: 0,
                max_dist: max_dist,
                dir: Dir(rand::random::<i32>() % 2, rand::random::<i32>() % 2),
                responsiveness: 1.0,
                genome: genome,
            },
            sensor_layer: Vec::with_capacity(10),
            inner_layer: Vec::with_capacity(num_inner),
            action_layer: Vec::with_capacity(3),
        };

        // earch wyrm has full set of neurons that are not necessarily wired together
        for i in 0..w.sensor_layer.capacity() {
            w.sensor_layer.push(Rc::new(RefCell::new(Neuron::new(
                SENSORS[i % SENSORS.len()],
            ))));
        }
        for i in 0..w.inner_layer.capacity() {
            w.inner_layer
                .push(Rc::new(RefCell::new(Neuron::new(INNER[i % INNER.len()]))));
        }
        for i in 0..w.action_layer.capacity() {
            w.action_layer.push(Rc::new(RefCell::new(Neuron::new(
                ACTIONS[i % ACTIONS.len()],
            ))));
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
                (false, id) => self.sensor_layer[id % self.inner_layer.len()].clone(),
            };
            let sink = match g.get_sink() {
                (true, id) => self.inner_layer[id % self.inner_layer.len()].clone(),
                (false, id) => self.sensor_layer[id % self.inner_layer.len()].clone(),
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
}
