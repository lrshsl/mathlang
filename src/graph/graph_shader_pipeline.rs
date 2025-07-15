use glam::{DVec2, Vec2};
use iced::{
    Rectangle,
    widget::shader::{self, wgpu},
};

pub const ZOOM_PIXELS_FACTOR: f64 = 200.0;

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Uniforms {
    resolution: Vec2,
    center: Vec2,
    scale: f32,
    f2: f32,
    f1: f32,
    f0: f32,
}

struct FragmentShaderPipeline {
    pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
}

impl FragmentShaderPipeline {
    fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("FragmentShaderPipeline shader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                "graph_shader.wgsl"
            ))),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("FragmentShaderPipeline"),
            layout: None,
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
            label: Some("shader_quad uniform buffer"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uniform_bind_group_layout = pipeline.get_bind_group_layout(0);
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("shader_quad uniform bind group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        Self {
            pipeline,
            uniform_buffer,
            uniform_bind_group,
        }
    }

    fn update(&mut self, queue: &wgpu::Queue, uniforms: &Uniforms) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(uniforms));
    }

    fn render(
        &self,
        target: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        viewport: Rectangle<u32>,
    ) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("fill color test"),
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
        pass.set_bind_group(0, &self.uniform_bind_group, &[]);

        pass.draw(0..3, 0..1);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Controls {
    pub zoom: f64,
    pub center: DVec2,
    pub factors: [f64; 3],
}

impl Controls {
    pub fn scale(&self) -> f64 {
        1.0 / 2.0_f64.powf(self.zoom) / ZOOM_PIXELS_FACTOR
    }
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            zoom: 1.,
            center: DVec2::ZERO,
            factors: [1., 0., 0.],
        }
    }
}

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
        _bounds: &Rectangle,
        viewport: &shader::Viewport,
    ) {
        if !storage.has::<FragmentShaderPipeline>() {
            storage.store(FragmentShaderPipeline::new(device, format));
        }

        let pipeline = storage.get_mut::<FragmentShaderPipeline>().unwrap();

        let (f2, f1, f0) = self.controls.factors.into();
        pipeline.update(
            queue,
            &Uniforms {
                resolution: Vec2::new(
                    viewport.physical_width() as f32,
                    viewport.physical_height() as f32,
                ),
                center: self.controls.center.as_vec2(),
                scale: self.controls.scale() as f32,
                f2: f2 as f32,
                f1: f1 as f32,
                f0: f0 as f32,
            },
        );
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
