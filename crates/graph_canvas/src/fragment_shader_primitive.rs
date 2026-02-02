use std::sync::{Arc, Mutex};

use glam::{Vec2, vec2};
use iced::{
    Rectangle,
    widget::shader::{self, wgpu},
};
use mth_common::ops::Instruction;

use crate::{
    controls::Controls,
    graph_shader_pipeline::{FragmentShaderPipeline, N_INSTRUCTIONS, Uniforms},
};

#[derive(Debug)]
pub struct FragmentShaderPrimitive {
    controls: Controls,
    instructions: Arc<Mutex<[Instruction; N_INSTRUCTIONS]>>,
    instruction_count: usize,
    pub instructions_dirty: bool,
}

impl FragmentShaderPrimitive {
    pub fn new(
        controls: Controls,
        instructions: Arc<Mutex<[Instruction; N_INSTRUCTIONS]>>,
        instruction_count: usize,
        instructions_dirty: bool,
    ) -> Self {
        Self {
            controls,
            instructions,
            instruction_count,
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
            storage.store(FragmentShaderPipeline::new(device, format));
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

        // Always update uniforms
        pipeline.update_uniforms(
            queue,
            &Uniforms {
                resolution: viewport_size,
                center: self.controls.center.as_vec2(),
                scale: self.controls.scale() as f32,
                viewport_origin,
                instruction_count: self.instruction_count as u32,
            },
        );

        // Update instructions if necessary
        if self.instructions_dirty {
            // Note: Doesn't write instruction_count to uniforms
            pipeline.update_program(queue, &self.instructions, self.instruction_count);
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
