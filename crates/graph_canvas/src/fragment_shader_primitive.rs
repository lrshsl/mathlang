use std::sync::{Arc, Mutex};

use glam::{vec2, Vec2};
use iced::{wgpu, widget::shader, Rectangle};

use crate::{
    controls::Controls,
    graph_shader_pipeline::{FragmentShaderPipeline, Uniforms, N_INSTRUCTIONS},
};
use mth_common::{ops::Instruction, plot_desc::PlotDesc, N_PLOTS};

#[derive(Debug)]
pub struct FragmentShaderPrimitive {
    controls: Controls,
    instructions: Arc<Mutex<[Instruction; N_INSTRUCTIONS]>>,
    plot_desc: [PlotDesc; N_PLOTS],
    pub instructions_dirty: bool,
}

impl FragmentShaderPrimitive {
    pub fn new(
        controls: Controls,
        instructions: Arc<Mutex<[Instruction; N_INSTRUCTIONS]>>,
        plot_desc: [PlotDesc; N_PLOTS],
        instructions_dirty: bool,
    ) -> Self {
        Self {
            controls,
            instructions,
            plot_desc,
            instructions_dirty,
        }
    }
}

impl shader::Primitive for FragmentShaderPrimitive {
    type Pipeline = FragmentShaderPipeline;

    fn prepare(
        &self,
        pipeline: &mut Self::Pipeline,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        bounds: &Rectangle,
        viewport: &shader::Viewport,
    ) {
        let scale_factor = viewport.scale_factor();
        let viewport_size = Vec2::new(
            bounds.width * scale_factor as f32,
            bounds.height * scale_factor as f32,
        );
        let viewport_origin = vec2(
            bounds.x * scale_factor as f32,
            bounds.y * scale_factor as f32,
        );

        // Always update uniforms
        pipeline.update_uniforms(
            queue,
            &Uniforms {
                viewport_origin,
                viewport_size,
                pan_offset: self.controls.offset.as_vec2(),
                pixel_ratio: self.controls.pixel_ratio() as f32,
                _pad: 0,
            },
        );

        // Update instructions if necessary
        if self.instructions_dirty {
            let n_instructions = self.plot_desc.iter().fold(0, |acc, desc| acc + desc.length);
            pipeline.update_plot_desc(queue, &self.plot_desc);
            pipeline.update_program(queue, &self.instructions, n_instructions as usize);
        }
    }

    fn render(
        &self,
        pipeline: &Self::Pipeline,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        clip_bounds: &Rectangle<u32>,
    ) {
        pipeline.render(target, encoder, *clip_bounds);
    }
}
