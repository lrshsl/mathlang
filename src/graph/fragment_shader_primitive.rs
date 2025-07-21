use glam::{vec2, Vec2};
use iced::{
    Rectangle,
    widget::shader::{self, wgpu},
};

use super::ops::{Instruction, OP_INPUT_X, OP_SIN};
use crate::{
    graph::{
        Controls,
        graph_shader_pipeline::{FragmentShaderPipeline, Uniforms},
    },
    inst,
};

pub const N_INST: usize = 2;
pub const INSTRUCTIONS: [Instruction; N_INST] = [inst!(1, 0., 0.), inst!(2, 0., 0.)];

pub const STACK_SIZE: usize = 16;
pub const INITIAL_STACK: [f32; STACK_SIZE] = [0.; STACK_SIZE];

#[derive(Debug)]
pub struct FragmentShaderPrimitive {
    controls: Controls,
}

impl FragmentShaderPrimitive {
    pub fn new(controls: Controls) -> Self {
        Self { controls }
    }
}

impl shader::Primitive for FragmentShaderPrimitive {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        storage: &mut shader::Storage,
        bounds: &Rectangle,
        viewport: &shader::Viewport,
    ) {
        if !storage.has::<FragmentShaderPipeline>() {
            storage.store(FragmentShaderPipeline::new(
                device,
                format,
                &INSTRUCTIONS,
                STACK_SIZE,
            ));
        }

        let pipeline = storage.get_mut::<FragmentShaderPipeline>().unwrap();

        let viewport_size = Vec2::new(
            viewport.physical_width() as f32,
            viewport.physical_height() as f32,
        );
        pipeline.update_uniforms(
            queue,
            &Uniforms {
                resolution: viewport_size,
                center: self.controls.center.as_vec2(),
                scale: self.controls.scale() as f32,
                _pad0: 0.,
                viewport_origin: vec2(bounds.x, bounds.y),
                _pad1: Vec2::ZERO,
            },
        );
       if false {
            pipeline.update_program(queue, &INITIAL_STACK, &INSTRUCTIONS);
        }
    }

    fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        storage: &shader::Storage,
        target: &wgpu::TextureView,
        clip_bounds: &Rectangle<u32>,
    ) {
        let pipeline = storage.get::<FragmentShaderPipeline>().unwrap();
        pipeline.render(target, encoder, *clip_bounds);
    }
}
