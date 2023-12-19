mod renderer;
mod sim;
mod entity;

use std::{sync::{Arc, RwLock}, time::Instant};
use std::f32::consts::PI;
use entity::Entity;
use lazy_static::lazy_static;
use ultraviolet::Vec2;
use renderer::Renderer;
use quarkstrom::{run, WindowMode};
use sim::Simulation;

lazy_static! {
    static ref PARTICLES: Arc<Vec<RwLock<Entity>>> = {
        let mut vec = vec![];
        let mut prev_size = 0.;
        
        for i in 0..=180 {
            let radians = i * PI / 180.;
            
            vec.push(RwLock::new(Entity {
                pos: Vec2::new(radians),
                col: 0xffffff,
                radius: 2,
                acc: 0.,
                vel: 0.
            }));
            
            prev_size = size;
        }
        
        Arc::new(vec)
    };
}

fn main() {
    run::<Renderer>(WindowMode::Windowed(1280, 720), "Particle Sim", 256.);

    let elapsed = Instant::now();
    let mut delta = Instant::now();
    let mut sim = Simulation::new();

    loop {
        let previous_delta = delta;
        delta = Instant::now();
        let delta_time = delta.duration_since(previous_delta).as_secs_f32();
        sim.run(elapsed.elapsed(), delta_time);
    }
}
