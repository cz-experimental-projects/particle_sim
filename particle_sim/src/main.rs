mod renderer;
mod sim;
mod entity;

use std::{sync::{Arc, RwLock}, time::Instant};
use entity::Entity;
use lazy_static::lazy_static;
use renderer::Renderer;
use quarkstrom::{run, WindowMode};
use sim::Simulation;

lazy_static! {
    static ref PARTICLES: RwLock<Vec<Arc<Entity>>> = RwLock::new(Vec::new());
}

fn main() {
    run::<Renderer>(WindowMode::Windowed(1280, 720), "Particle Sim", 256.);

    let sim = Simulation::new();
    let elapsed = Instant::now();

    loop {
        sim.run(elapsed);
    }
}
