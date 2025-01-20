use simulation::Simulation;

mod genome;
mod misc;
mod neuron;
mod simulation;
mod wyrm;

fn main() {
    let mut sim = Simulation::new(128, 128, 5, 3, 100, 5, 5, 1000, 0.05);
    let mut generation: u64 = 0;
    loop {
        if sim.simulation_step() > 1000 {
            generation += 1;
            let survivors = sim.apply_selection();
            println!(
                "generation {generation}: {survivors} survivors ({}%)",
                survivors as f32 / 1000.0
            );
            sim.repopulate();
        }
    }
}
