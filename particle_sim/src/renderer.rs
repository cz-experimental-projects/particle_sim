use std::time::Duration;
use egui::Context;
use quarkstrom::RenderContext;
use winit_input_helper::WinitInputHelper;

use crate::PARTICLES;

pub(crate) struct Renderer {
}

impl quarkstrom::Renderer for Renderer {
    fn new() -> Self {
        Self {}
    }

    fn input(&mut self, _input: &WinitInputHelper, _elapsed: &Duration) {
    }

    fn render(&mut self, ctx: &mut RenderContext, _elapsed: &Duration) {
        ctx.clear_circles();

        for entity in PARTICLES.iter() {
            let entity = entity.read().unwrap();
            ctx.draw_circle(entity.pos, entity.radius, entity.col);
        }
    }

    fn gui(&mut self, _ctx: &Context) {
    }
}
