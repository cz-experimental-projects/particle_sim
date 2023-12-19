use std::time::Duration;
use crate::PARTICLES;

pub(crate) struct Simulation {
}

impl Simulation {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, elapsed: Duration, delta: f32) {
        for particle in PARTICLES.iter() {
            let mut particle = particle.write().unwrap();
            particle.pos.y -= delta * 10.;
        }
    }
}
