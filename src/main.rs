use simulation::Simulation;

mod genome;
mod misc;
mod neuron;
mod simulation;
mod wyrm;

fn main() {
    let mut sim = Simulation::new(128, 128, 5, 3, 100, 30, 5, 1000, 0.05);
    loop {
        if sim.simulation_step() > 1000 {
            println!("done");
        }
    }
}
