use simulation::Simulation;

mod genome;
mod neuron;
mod simulation;
mod wyrm;

fn main() {
    let mut sim = Simulation::new(128, 128, 5, 3, 100, 30, 5, 1000, 0.05);
    loop {
        if sim.simulationStep() > 1000 {
            println!("done");
        }
    }
}
