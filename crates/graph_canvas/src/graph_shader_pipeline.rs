use glam::Vec2;
use iced::{
    widget::shader::wgpu::{self, util::DeviceExt as _},
    Rectangle,
};

use mth_common::ops::{Instruction, OP_CONST};

pub const ZOOM_PIXELS_FACTOR: f64 = 200.0;

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Uniforms {
    pub resolution: Vec2,       // 8 bytes
    pub center: Vec2,           // 8 bytes
    pub viewport_origin: Vec2,  // 8 bytes
    pub scale: f32,             // 4 bytes
    pub instruction_count: u32, // 4 bytes
    pub _pad0: f32,             // 4 bytes (alignment)
    pub _pad1: f32,             // 4 bytes
}

pub struct FragmentShaderPipeline {
    pipeline: wgpu::RenderPipeline,

    uniform_buffer: wgpu::Buffer,
    bind_group_0: wgpu::BindGroup,

    instruction_buffer: wgpu::Buffer,
    bind_group_1: wgpu::BindGroup,

    max_instructions: usize,
}

impl FragmentShaderPipeline {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        initial_instructions: &[Instruction],
        max_instructions: usize,
    ) -> Self {
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
                entry_point: "vs_main",
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("UniformBuffer"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create buffer with max_instructions capacity, initialized with zeros
        let mut buffer_data = vec![
            Instruction {
                opcode: OP_CONST,
                a: 0.0,
                b: 0.0,
            };
            max_instructions
        ];

        // Copy initial instructions (if any)
        let copy_len = std::cmp::min(initial_instructions.len(), max_instructions);
        for (i, instruction) in initial_instructions.iter().take(copy_len).enumerate() {
            buffer_data[i] = *instruction;
        }

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
            max_instructions,
        }
    }

    pub fn update_uniforms(&self, queue: &wgpu::Queue, uniforms: &Uniforms) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(uniforms));
    }

    pub fn update_program(&self, queue: &wgpu::Queue, instructions: &[Instruction]) {
        // Pad instructions to max_instructions with zeros
        let mut buffer_data = vec![
            Instruction {
                opcode: OP_CONST,
                a: 0.0,
                b: 0.0,
            };
            self.max_instructions
        ];

        let copy_len = std::cmp::min(instructions.len(), self.max_instructions);
        for (i, instruction) in instructions.iter().take(copy_len).enumerate() {
            buffer_data[i] = *instruction;
        }

        queue.write_buffer(
            &self.instruction_buffer,
            0,
            bytemuck::cast_slice(&buffer_data),
        );
    }

    pub fn get_max_instructions(&self) -> usize {
        self.max_instructions
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
