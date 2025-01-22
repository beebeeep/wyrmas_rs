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
    let (size_x, size_y, cell_size, ticks_per_gen) = (128, 128, 10, 100);
    let mut sim = Simulation::new(size_x, size_y, 5, 3, ticks_per_gen, 5, 10, 1000, 0.01);
    let mut generation: u64 = 0;
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
    let mut tick;
    let mut gen_start = Instant::now();
    'run: loop {
        tick = sim.simulation_step();
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
        sim.render(&mut canvas, cell_size as i16);
        canvas.present();
        if tick >= ticks_per_gen {
            // sim.render(&mut canvas, cell_size as i16);
            // canvas.present();
            generation += 1;
            let survivors = sim.apply_selection();
            let selection_area = sim
                .state
                .selection_area
                .iter()
                .fold(0, |a, ys| a + ys.iter().filter(|v| **v).count());

            let gen_time = Instant::now() - gen_start;
            println!(
                "generation {generation}: {survivors} survivors ({:.1}%), ({:.1}% of selection area taken), took {}ms ({:.1} ticks/sec)",
                100.0 * survivors as f32 / 1000.0,
                100.0 * survivors as f32 / selection_area as f32,
                gen_time.as_millis(),
                ticks_per_gen as f32 / gen_time.as_secs_f32(),
            );
            sim.repopulate();
            gen_start = Instant::now();
        }
    }
}
