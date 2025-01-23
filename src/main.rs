use std::{
    env::{self, Args},
    thread,
    time::{Duration, Instant},
};

use sdl2::{self, event::Event, keyboard::Keycode, render::Canvas, video::Window, EventPump, Sdl};
use simulation::Simulation;

mod genome;
mod misc;
mod neuron;
mod simulation;
mod wyrm;

struct UI {
    canvas: Canvas<Window>,
    events: EventPump,
}

fn init_ui(w: i32, h: i32) -> Option<UI> {
    if env::args().find(|x| x == "-v").is_none() {
        return None;
    }

    let sdl_context = sdl2::init().unwrap();
    let events = sdl_context.event_pump().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("wyrmas", w as u32, h as u32)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();
    canvas.present();

    return Some(UI {
        canvas: canvas,
        events: events,
    });
}

fn main() {
    let (size_x, size_y, cell_size, ticks_per_gen) = (128, 128, 5, 100);
    let mut sim = Simulation::new(size_x, size_y, 5, 3, ticks_per_gen, 5, 10, 1000, 0.05);
    let mut generation: u64 = 0;
    let mut ui = init_ui(size_x * cell_size, size_y * cell_size);

    let mut tick;
    let mut gen_start = Instant::now();
    'run: loop {
        tick = sim.simulation_step();
        if tick >= ticks_per_gen {
            if let Some(ref mut ui) = ui {
                for event in ui.events.poll_iter() {
                    match event {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => break 'run,
                        _ => {}
                    }
                }
                sim.render(&mut ui.canvas, cell_size as i16);
                ui.canvas.present();
            }
            generation += 1;
            let survivors = sim.apply_selection();
            let selection_area = sim
                .state
                .selection_area
                .iter()
                .fold(0, |a, ys| a + ys.iter().filter(|v| **v).count());

            let gen_time = Instant::now() - gen_start;
            println!(
                "generation {generation}: {survivors} survivors ({:.1}%), ({:.1}% of selection area taken), took {}ms ({:.1} ticks/sec, {:.1} generations/sec)",
                100.0 * survivors as f32 / 1000.0,
                100.0 * survivors as f32 / selection_area as f32,
                gen_time.as_millis(),
                ticks_per_gen as f32 / gen_time.as_secs_f32(),
                1.0 / gen_time.as_secs_f32()
            );
            sim.repopulate();
            gen_start = Instant::now();
        }
    }
}
