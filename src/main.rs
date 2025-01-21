use std::{
    thread,
    time::{Duration, Instant},
};

use sdl2::{self, event::Event, keyboard::Keycode};
use simulation::Simulation;

mod genome;
mod misc;
mod neuron;
mod simulation;
mod wyrm;

fn main() {
    let (size_x, size_y, cell_size, max_age) = (128, 128, 5, 1000);
    let mut sim = Simulation::new(size_x, size_y, 5, 3, max_age, 5, 10, 1000, 0.01);
    let mut generation: u64 = 0;
    /*
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(
            "wyrmas",
            cell_size * size_x as u32,
            cell_size * size_y as u32,
        )
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    */
    let mut step = 0;
    let mut t = Instant::now();
    'run: loop {
        step = sim.simulation_step();
        /*
        sim.render(&mut canvas, cell_size as i16);
        canvas.present();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'run,
                _ => {}
            }
        }
        // thread::sleep(Duration::from_millis(16));
        */
        if step >= max_age {
            generation += 1;
            let survivors = sim.apply_selection();
            println!(
                "generation {generation}: {survivors} survivors ({}%), took {}ms",
                survivors as f32 / 1000.0,
                (Instant::now() - t).as_millis()
            );
            sim.repopulate();
            t = Instant::now();
        }
    }
}
