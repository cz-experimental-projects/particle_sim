use std::time::Duration;
use crate::entity::Entity;
use crate::PARTICLES;

pub(crate) struct Simulation {
    delta: f32,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            delta: 0.,
        }
    }

    pub fn run(&mut self, elapsed: Duration, delta: f32) {
        self.delta = delta;
        
        for particle in PARTICLES.iter() {
            let mut particle = particle.write().unwrap();
            self.resolve_acc(&mut particle);
            self.resolve_vel(&mut particle);
            self.resolve_pos(&mut particle);
        }
    }
    
    fn resolve_acc(&self, particle: &mut Entity) {
        particle.acc += -9.807 * self.delta;
    }
    
    fn resolve_vel(&self, particle: &mut Entity) {
        particle.vel += particle.acc * self.delta;
    }
    
    fn resolve_pos(&self, particle: &mut Entity) {
        particle.pos.y += particle.vel * self.delta;
    }
}
