use std::sync::Mutex;

use glam::Vec2;
use iced::{
    Rectangle,
    wgpu::{self, util::DeviceExt as _},
    widget::shader,
};

use mth_common::ops::Instruction;

pub const ZOOM_PIXELS_FACTOR: f64 = 200.0;
pub const N_INSTRUCTIONS: usize = 256;

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Uniforms {
    pub resolution: Vec2,       // 8 bytes
    pub center: Vec2,           // 8 bytes
    pub viewport_origin: Vec2,  // 8 bytes
    pub scale: f32,             // 4 bytes
    pub instruction_count: u32, // 4 bytes
}

pub struct FragmentShaderPipeline {
    pipeline: wgpu::RenderPipeline,

    uniform_buffer: wgpu::Buffer,
    bind_group_0: wgpu::BindGroup,

    instruction_buffer: wgpu::Buffer,
    bind_group_1: wgpu::BindGroup,
}

impl shader::Pipeline for FragmentShaderPipeline {
    fn new(device: &wgpu::Device, _queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("FragmentShaderPipeline shader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                "graph_shader.wgsl"
            ))),
        });

        let bind_group_layout_0 =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("BindGroupLayout0"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
        let bind_group_layout_1 =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("BindGroupLayout1"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("CustomPipelineLayout"),
            bind_group_layouts: &[&bind_group_layout_0, &bind_group_layout_1],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("FragmentShaderPipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            multiview: None,
            cache: None,
        });

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("UniformBuffer"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create buffer with N_INSTRUCTIONS capacity, initialized with nop instructions
        let buffer_data = [Instruction::default(); N_INSTRUCTIONS];
        let instruction_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("InstructionBuffer"),
            contents: bytemuck::cast_slice(&buffer_data),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_0 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("BindGroup0"),
            layout: &bind_group_layout_0,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });
        let bind_group_1 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("BindGroup1"),
            layout: &bind_group_layout_1,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: instruction_buffer.as_entire_binding(),
            }],
        });

        Self {
            pipeline,
            uniform_buffer,
            bind_group_0,
            instruction_buffer,
            bind_group_1,
        }
    }
}

impl FragmentShaderPipeline {
    pub fn update_uniforms(&self, queue: &wgpu::Queue, uniforms: &Uniforms) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(uniforms));
    }

    /// Note: instruction_count also needs to be set correctly in the uniforms
    pub fn update_program(
        &self,
        queue: &wgpu::Queue,
        instructions: &Mutex<[Instruction; N_INSTRUCTIONS]>,
        instruction_count: usize,
    ) {
        let guard = instructions
            .lock()
            .expect("Failed to lock instruction mutex");

        let slice = bytemuck::cast_slice(&guard[..instruction_count]);

        queue.write_buffer(&self.instruction_buffer, 0, slice);
    }

    pub fn render(
        &self,
        target: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        viewport: Rectangle<u32>,
    ) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("RenderPass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        pass.set_pipeline(&self.pipeline);
        pass.set_viewport(
            viewport.x as f32,
            viewport.y as f32,
            viewport.width as f32,
            viewport.height as f32,
            0.0,
            1.0,
        );
        pass.set_bind_group(0, &self.bind_group_0, &[]);
        pass.set_bind_group(1, &self.bind_group_1, &[]);

        pass.draw(0..3, 0..1);
    }
}
