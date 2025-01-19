use std::cell::RefCell;
use std::rc::Rc;

use crate::genome;
use crate::neuron::{Kind, Neuron};
use crate::simulation::{Simulation, SimulationState};
use rand;
use rand::distributions::Standard;

pub struct WyrmState {
    pub x: i32,
    pub y: i32,
    pub dir: (i32, i32),
    pub age: i32,
    pub max_dist: i32,
    pub genome: Vec<genome::Gene>,
    pub responsiveness: f32,
}

pub struct Wyrm {
    state: WyrmState,
    sensorLayer: Vec<Rc<RefCell<Neuron>>>,
    innerLayer: Vec<Rc<RefCell<Neuron>>>,
    actionLayer: Vec<Rc<RefCell<Neuron>>>,
}

impl Wyrm {
    pub fn new(x: i32, y: i32, num_inner: usize, max_dist: i32, genome: Vec<genome::Gene>) -> Self {
        let mut w = Wyrm {
            state: WyrmState {
                x: x,
                y: y,
                age: 0,
                max_dist: max_dist,
                dir: (rand::random::<i32>() % 2, rand::random::<i32>() % 2),
                responsiveness: 1.0,
                genome: genome,
            },
            sensorLayer: Vec::with_capacity(10),
            innerLayer: Vec::with_capacity(num_inner),
            actionLayer: Vec::with_capacity(3),
        };
        for i in 0..w.sensorLayer.capacity() {
            w.sensorLayer.push(Rc::new(RefCell::new(Neuron::new(
                Kind::try_from(i).unwrap(),
            ))));
        }
        for i in 0..w.innerLayer.capacity() {
            w.innerLayer.push(Rc::new(RefCell::new(Neuron::new(
                Kind::try_from(i).unwrap(),
            ))));
        }
        for i in 0..w.actionLayer.capacity() {
            w.actionLayer.push(Rc::new(RefCell::new(Neuron::new(
                Kind::try_from(i).unwrap(),
            ))));
        }
        todo!("wire neurons");
    }

    pub fn simulationStep(self: &mut Self, state: &mut SimulationState) {
        self.sensorLayer
            .iter()
            .for_each(|n| n.borrow_mut().activate(state, &mut self.state));
    }
}
