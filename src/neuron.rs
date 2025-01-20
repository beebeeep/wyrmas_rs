use std::{cell::RefCell, rc::Rc};

use crate::{
    misc::{Dir, DIRECTIONS},
    simulation::{self, SimulationState},
    wyrm::{self, WyrmState},
};

pub type ActivationFn =
    fn(&mut Neuron, &mut wyrm::WyrmState, &mut simulation::SimulationState) -> f32;

pub static SENSORS: &'static [ActivationFn] = &[
    s_age,
    s_rand,
    s_pop,
    s_dist_nearest,
    s_dir_nearest,
    s_dist_fwd,
    s_osc,
    s_good_fwd,
    s_good_around,
    s_good_dist,
];
pub static ACTIONS: &'static [ActivationFn] = &[a_resp, a_move, a_turn];

pub static INNER: &'static [ActivationFn] = &[tanh_activation];

pub struct Neuron {
    pub potential: f32,
    activate: ActivationFn,
    inputs: Vec<Link>,
}

struct Link {
    weight: f32,
    source: Rc<RefCell<Neuron>>,
}

impl Neuron {
    pub fn new(activate: ActivationFn) -> Self {
        Neuron {
            potential: 0.0,
            activate: activate,
            inputs: Vec::with_capacity(1),
        }
    }
    pub fn add_input(self: &mut Self, weight: f32, n: Rc<RefCell<Neuron>>) {
        self.inputs.push(Link {
            weight: weight,
            source: Rc::clone(&n),
        });
    }

    pub fn reset(self: &mut Self) {
        self.inputs.truncate(0);
    }

    pub fn activate(self: &mut Self, w: &mut wyrm::WyrmState, s: &mut simulation::SimulationState) {
        self.potential = w.responsiveness * (self.activate)(self, w, s);
    }
}

fn s_age(_: &mut Neuron, w: &mut wyrm::WyrmState, s: &mut simulation::SimulationState) -> f32 {
    // normalized wyrm age
    w.age as f32 / s.max_age as f32
}

fn s_rand(_: &mut Neuron, _: &mut wyrm::WyrmState, _: &mut simulation::SimulationState) -> f32 {
    rand::random()
}
fn s_pop(_: &mut Neuron, w: &mut wyrm::WyrmState, s: &mut simulation::SimulationState) -> f32 {
    // population density nearby, where 1 is max density
    let mut c = 0i32;
    for dx in -1..1 {
        let x = w.x + dx;
        if x < 0 || x >= s.size_x {
            continue;
        }
        for dy in -1..1 {
            let y = w.y + dy;
            if y < 0 || y >= s.size_y || (x == 0 && y == 0) {
                continue;
            }
            if s.world[x as usize][y as usize] {
                c += 1;
            }
        }
    }
    return c as f32 / 8.0;
}

fn s_dist_nearest(
    _: &mut Neuron,
    w: &mut wyrm::WyrmState,
    s: &mut simulation::SimulationState,
) -> f32 {
    // distance to nearest cell
    let (_, dist) = find_nearest(w, s);
    return 1.0 - (dist as f32) / (w.max_dist as f32);
}

fn s_dir_nearest(
    _: &mut Neuron,
    w: &mut wyrm::WyrmState,
    s: &mut simulation::SimulationState,
) -> f32 {
    // direction to nearest cell
    let (d, _) = find_nearest(w, s);
    return d.normalize();
}

fn s_dist_fwd(_: &mut Neuron, w: &mut wyrm::WyrmState, s: &mut simulation::SimulationState) -> f32 {
    // distance to nearest cell in forward direction
    for t in 0..=w.max_dist {
        let (x, y) = (w.x + t * w.dir.0, w.y + t * w.dir.1);
        if x < 0 || x >= s.size_x || y < 0 || y >= s.size_y {
            return 0.0;
        }
        if s.world[x as usize][y as usize] {
            return 1.0 - (t as f32) / (s.max_age as f32);
        }
    }
    return 0.0;
}

fn s_osc(_: &mut Neuron, _: &mut wyrm::WyrmState, s: &mut simulation::SimulationState) -> f32 {
    s.osc_value
}

fn s_good_fwd(_: &mut Neuron, w: &mut wyrm::WyrmState, s: &mut simulation::SimulationState) -> f32 {
    // count good places in forward direction
    let mut c = 0;
    for t in 0..=w.max_dist {
        let (x, y) = (w.x + t * w.dir.0, w.y + t * w.dir.1);
        if x < 0 || x >= s.size_x || y < 0 || y >= s.size_y {
            break;
        }
        if s.selection_area[x as usize][y as usize] {
            c += 1;
        }
    }
    return c as f32 / w.max_dist as f32;
}

fn s_good_around(
    _: &mut Neuron,
    w: &mut wyrm::WyrmState,
    s: &mut simulation::SimulationState,
) -> f32 {
    // count of good places around
    let mut c = 0;
    for dx in -1..1 {
        let x = w.x + dx;
        if x < 0 || x >= s.size_x {
            continue;
        }
        for dy in -1..1 {
            let y = w.y + dy;
            if y < 0 || y >= s.size_y {
                continue;
            }
            if s.selection_area[x as usize][y as usize] {
                c += 1;
            }
        }
    }
    return c as f32 / 9.0;
}

fn s_good_dist(
    _: &mut Neuron,
    w: &mut wyrm::WyrmState,
    s: &mut simulation::SimulationState,
) -> f32 {
    // distance to good place in forward direction
    for t in 0..=w.max_dist {
        let (x, y) = (w.x + t * w.dir.0, w.y + t * w.dir.1);
        if x < 0 || x >= s.size_x || y < 0 || y >= s.size_y {
            return 0.0;
        }
        if s.selection_area[x as usize][y as usize] {
            return 1.0 - t as f32 / w.max_dist as f32;
        }
    }
    return 0.0;
}

fn a_resp(n: &mut Neuron, w: &mut wyrm::WyrmState, s: &mut simulation::SimulationState) -> f32 {
    // set wyrm responsiveness (how agitated it is)
    let p = tanh_activation(n, w, s);
    if let Some(sgn) = activate_threshold(&p) {
        w.responsiveness += 0.05 * sgn as f32
    }
    return p;
}

fn a_move(n: &mut Neuron, w: &mut wyrm::WyrmState, s: &mut simulation::SimulationState) -> f32 {
    let p = tanh_activation(n, w, s);
    if let Some(sgn) = activate_threshold(&p) {
        let (mut x, mut y) = (w.x + w.dir.0 * sgn, w.y + w.dir.1 * sgn);
        if x < 0 {
            x = 0
        }
        if y < 0 {
            y = 0
        }
        if x >= s.size_x {
            x = s.size_x - 1
        }
        if y >= s.size_y {
            y = s.size_y - 1
        }
        s.world[w.x as usize][w.y as usize] = false;
        (w.x, w.y) = (x, y);
        s.world[w.x as usize][w.y as usize] = true;
    }
    return p;
}

fn a_turn(n: &mut Neuron, w: &mut wyrm::WyrmState, s: &mut simulation::SimulationState) -> f32 {
    let p = tanh_activation(n, w, s);
    if let Some(sgn) = activate_threshold(&p) {
        match DIRECTIONS
            .iter()
            .enumerate()
            .find(|(_, x)| x.0 == w.dir.0 && x.1 == w.dir.1)
        {
            Some((i, _)) => {
                w.dir = DIRECTIONS[(i as i32 + sgn).rem_euclid(DIRECTIONS.len() as i32) as usize]
                    .clone();
            }
            None => return p,
        }
    }
    return p;
}

fn tanh_activation(
    n: &mut Neuron,
    _: &mut wyrm::WyrmState,
    _: &mut simulation::SimulationState,
) -> f32 {
    n.inputs
        .iter()
        .fold(0.0, |a, x| a + x.weight + x.source.borrow().potential)
        .tanh()
}

fn activate_threshold(p: &f32) -> Option<i32> {
    if rand::random::<f32>() < p.abs() {
        Some(if *p > 0.0 { 1 } else { -1 })
    } else {
        None
    }
}

fn find_nearest(w: &WyrmState, s: &SimulationState) -> (Dir, i32) {
    for t in 0..=w.max_dist {
        for dir in DIRECTIONS {
            let (x, y) = (w.x + t * dir.0, w.y + t * dir.1);
            if x >= 0 && x < s.size_x && y >= 0 && y < s.size_y && s.world[x as usize][y as usize] {
                return (dir.clone(), t);
            }
        }
    }
    return (DIRECTIONS[0].clone(), 0);
}
