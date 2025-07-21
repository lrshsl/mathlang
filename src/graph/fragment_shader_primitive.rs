use std::sync::Arc;

use glam::{Vec2, vec2};
use iced::{
    Rectangle,
    widget::shader::{self, wgpu},
};

use super::ops::*;
use crate::{
    graph::{
        Controls,
        graph_shader_pipeline::{FragmentShaderPipeline, Uniforms},
    },
    inst,
};

pub const N_INST: usize = 3;
pub const INSTRUCTIONS: [Instruction; N_INST] = [
    inst!(OP_X_POLY, -1., 3.),
    inst!(OP_CONST, -1.),
    inst!(OP_ADD),
];

pub const STACK_SIZE: usize = 16;
pub const INITIAL_STACK: [f32; STACK_SIZE] = [0.; STACK_SIZE];

#[derive(Debug)]
pub struct FragmentShaderPrimitive {
    controls: Controls,
    instructions: Arc<Vec<Instruction>>,
    pub instructions_dirty: bool,
}

impl FragmentShaderPrimitive {
    pub fn new(
        controls: Controls,
        instructions: Arc<Vec<Instruction>>,
        instructions_dirty: bool,
    ) -> Self {
        Self {
            controls,
            instructions,
            instructions_dirty,
        }
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
            storage.store(FragmentShaderPipeline::new(device, format, &INSTRUCTIONS));
        }

        let pipeline = storage.get_mut::<FragmentShaderPipeline>().unwrap();

        let viewport_size = Vec2::new(
            viewport.physical_width() as f32,
            viewport.physical_height() as f32,
        );

        let scale_factor = viewport.scale_factor();
        let viewport_origin = vec2(
            bounds.x * scale_factor as f32,
            bounds.y * scale_factor as f32,
        );
        pipeline.update_uniforms(
            queue,
            &Uniforms {
                resolution: viewport_size,
                center: self.controls.center.as_vec2(),
                scale: self.controls.scale() as f32,
                _pad0: 0.,
                viewport_origin,
                _pad1: Vec2::ZERO,
            },
        );
        if self.instructions_dirty {
            pipeline.update_program(queue, &self.instructions);
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
