use std::rc::Rc;

use crate::{simulation, wyrm};

pub type ActivationFn = fn(&mut simulation::Simulation, &mut wyrm::Wyrm, &Neuron) -> f32;

pub struct Neuron {
    potential: f32,
    responsiveness: f32,
    activationFn: ActivationFn,
    inputs: Vec<Link>,
}

struct Link {
    weight: f32,
    source: Rc<Neuron>,
}

impl Neuron {
    fn activate(self: &mut Self, s: &mut simulation::Simulation, w: &mut wyrm::Wyrm) {
        self.potential = (self.activationFn)(s, w, self);
    }
}
