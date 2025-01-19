use std::{cell::RefCell, rc::Rc};

use crate::{simulation, wyrm};

pub type ActivationFn = fn(&mut simulation::Simulation, &mut wyrm::Wyrm, &Neuron) -> f32;

pub enum Kind {
    SensorAge,
    SensorRand,
    SensorPop,
    SensorDistN,
    SensorDirN,
    SensorDistF,
    SensorOsc,
    SensorGoodFwd,
    SensorGoodAround,
    SensorGoodDist,
    ActionResp,
    ActionMove,
    ActionTurn,
    Inner,
}

pub struct Neuron {
    pub potential: f32,
    kind: Kind,
    inputs: Vec<Link>,
}

struct Link {
    weight: f32,
    source: Rc<RefCell<Neuron>>,
}

impl Neuron {
    pub fn new(kind: Kind) -> Self {
        Neuron {
            potential: 0.0,
            kind: kind,
            inputs: Vec::with_capacity(1),
        }
    }
    pub fn add_input(self: &mut Self, weight: f32, n: Rc<RefCell<Neuron>>) {
        self.inputs.push(Link {
            weight: weight,
            source: Rc::clone(&n),
        });
    }

    pub fn activate(self: &mut Self, s: &mut simulation::SimulationState, w: &mut wyrm::WyrmState) {
        self.potential = w.responsiveness
            * match self.kind {
                Kind::SensorAge => todo!(),
                Kind::SensorRand => todo!(),
                Kind::SensorPop => todo!(),
                Kind::SensorDistN => todo!(),
                Kind::SensorDirN => todo!(),
                Kind::SensorDistF => todo!(),
                Kind::SensorOsc => todo!(),
                Kind::SensorGoodFwd => todo!(),
                Kind::SensorGoodAround => todo!(),
                Kind::SensorGoodDist => todo!(),
                Kind::ActionResp => todo!(),
                Kind::ActionMove => todo!(),
                Kind::ActionTurn => todo!(),
                Kind::Inner => self
                    .inputs
                    .iter()
                    .fold(0.0, |a, x| a + x.weight * x.source.borrow().potential)
                    .tanh(),
            }
    }
}

impl TryFrom<usize> for Kind {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Kind::SensorAge),
            1 => Ok(Kind::SensorRand),
            3 => Ok(Kind::SensorPop),
            4 => Ok(Kind::SensorDistN),
            5 => Ok(Kind::SensorDirN),
            6 => Ok(Kind::SensorDistF),
            7 => Ok(Kind::SensorOsc),
            8 => Ok(Kind::SensorGoodFwd),
            9 => Ok(Kind::SensorGoodAround),
            10 => Ok(Kind::SensorGoodDist),
            11 => Ok(Kind::ActionResp),
            12 => Ok(Kind::ActionMove),
            13 => Ok(Kind::ActionTurn),
            14 => Ok(Kind::Inner),
            _ => Err(String::from(format!("invalid neuron kind {value}"))),
        }
    }
}
