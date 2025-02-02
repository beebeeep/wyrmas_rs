use std::{
    fmt::Display,
    fs::File,
    io::Write,
    process::{self, Stdio},
    thread,
    time::Instant,
};

use anyhow::{anyhow, Result};
use clap::Parser;
use sdl2::{self, event::Event, keyboard::Keycode, render::Canvas, video::Window, EventPump};
use simulation::Simulation;
use wyrm::Wyrm;

mod genome;
mod misc;
mod neuron;
mod simulation;
mod wyrm;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short)]
    visualize: bool,
    #[arg(short, default_value_t = 0.05)]
    mutation_rate: f32,
    #[arg(short, default_value_t = 10)]
    genome_size: usize,
    #[arg(short, default_value_t = 3)]
    inner_neurons: usize,
}

struct UI {
    canvas: Canvas<Window>,
    events: EventPump,
}

fn init_ui(args: &Args, w: i32, h: i32) -> Option<UI> {
    if !args.visualize {
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

fn dump_survivor(generation: u64, w: Option<&Wyrm>) -> Result<String> {
    let s = w.ok_or(anyhow!("nobody survived :("))?;
    let filename = format!("./survivor-{generation}.png");
    let mut dot = process::Command::new("dot")
        .args(["-Tpng", "-o", &filename])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    let mut stdin = dot.stdin.take().ok_or(anyhow!("failed to pipe stdin"))?;
    stdin.write_all(&s.dump_genome())?;
    drop(stdin);
    dot.wait()?;
    Ok(filename)
}

fn view_survivor(generation: u64, w: Option<&Wyrm>) -> Result<String> {
    match dump_survivor(generation, w) {
        Ok(file) => {
            let f = file.clone();
            thread::spawn(move || {
                match process::Command::new("xdg-open")
                    .arg(file.clone())
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .spawn()
                {
                    Err(err) => println!("error opening viewer: {err}"),
                    Ok(mut viewer) => {
                        println!("dumping survivor to {file}");
                        let _ = viewer.wait();
                    }
                }
            });
            Ok(f)
        }
        Err(err) => Err(anyhow!("error dumping survivor: {err}")),
    }
}

fn main() {
    let args = Args::parse();
    let (size_x, size_y, cell_size, ticks_per_gen) = (128, 128, 5, 100);
    let mut sim = Simulation::new(
        size_x,
        size_y,
        5,
        args.inner_neurons,
        ticks_per_gen,
        50,
        args.genome_size,
        1000,
        args.mutation_rate,
    );
    let mut generation: u64 = 0;
    let mut ui = init_ui(&args, size_x * cell_size, size_y * cell_size);

    let mut tick;
    let mut gen_start = Instant::now();
    let mut dump = false;
    let mut view = false;
    'run: loop {
        tick = sim.simulation_step();
        if tick >= ticks_per_gen {
            generation += 1;
            if let Some(ref mut ui) = ui {
                for event in ui.events.poll_iter() {
                    match event {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => break 'run,
                        Event::KeyDown {
                            keycode: Some(Keycode::D),
                            ..
                        } => dump = true,
                        Event::KeyDown {
                            keycode: Some(Keycode::V),
                            ..
                        } => view = true,
                        _ => {}
                    }
                }
                sim.render(&mut ui.canvas, cell_size as i16);
                ui.canvas.present();
            }
            let survivors = sim.apply_selection();
            if dump {
                dump = false;
                match dump_survivor(generation, sim.get_survivor()) {
                    Ok(file) => println!("dumping survivor to {file}"),
                    Err(err) => println!("error dumping survivor: {err}"),
                }
            }
            if view {
                view = false;
                match view_survivor(generation, sim.get_survivor()) {
                    Ok(file) => println!("dumping survivor to {file}"),
                    Err(err) => println!("error dumping survivor: {err}"),
                }
            }
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
            sim.create_selection_area();
            gen_start = Instant::now();
        }
    }
}
