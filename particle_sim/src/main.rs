mod renderer;
mod sim;
mod entity;

use std::{sync::{Arc, RwLock}, time::Instant};
use entity::Entity;
use lazy_static::lazy_static;
use ultraviolet::Vec2;
use renderer::Renderer;
use quarkstrom::{run, WindowMode};
use sim::Simulation;

lazy_static! {
    static ref PARTICLES: Arc<Vec<RwLock<Entity>>> = {
        let mut vec = vec![];
        
        for i in 0..=32 {
            vec.push(RwLock::new(Entity {
                pos: Vec2::new(i as f32, 0.),
                col: 0xffffff,
                size: 1.,
            }));
        }
        
        Arc::new(vec)
    };
}

fn main() {
    run::<Renderer>(WindowMode::Windowed(1280, 720), "Particle Sim", 256.);

    let sim = Simulation::new();
    let elapsed = Instant::now();
    let mut delta = Instant::now();

    loop {
        let previous_delta = delta;
        delta = Instant::now();
        let delta_time = delta.duration_since(previous_delta).as_secs_f32();
        sim.run(elapsed.elapsed(), delta_time);
    }
}
