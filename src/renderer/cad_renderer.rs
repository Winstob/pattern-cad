use super::ViewportRect;
use super::bind_group::{BindGroup, BindGroupEntry, BindingDescriptor};
use super::buffer::Buffer;
use crate::cad::model::CadModel;
use crate::renderer::CadView;
use glam::*;

pub struct CadRenderer {
    device: wgpu::Device,

    // geometry generation stuff
    vertex_buffer: Buffer,
    indirect_draw_args_buffer: Buffer,
    view_transform_buffer: Buffer,
    points_buffer: Buffer,
    num_points_buffer: Buffer,
    num_points: u32,
    compute_bind_group: BindGroup,
    compute_pipeline: wgpu::ComputePipeline,

    // rendering stuff
    render_pipeline: wgpu::RenderPipeline,
    background_color: wgpu::Color,
}

impl CadRenderer {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let vertex_buffer = Buffer::builder()
            .label("CAD Vertex Buffer")
            .size(1024)
            .usage(
                wgpu::BufferUsages::STORAGE
                    | wgpu::BufferUsages::VERTEX
                    | wgpu::BufferUsages::COPY_DST,
            )
            .build(device);

        let indirect_draw_args_buffer = Buffer::builder()
            .label("CAD Indirect Draw Args Buffer")
            .size(16)
            .usage(
                wgpu::BufferUsages::STORAGE
                    | wgpu::BufferUsages::INDIRECT
                    | wgpu::BufferUsages::COPY_DST,
            )
            .build(device);

        let view_transform_buffer = Buffer::builder()
            .label("CAD View Transform Buffer")
            .size(96)
            .usage(wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST)
            .build(device);

        let points_buffer = Buffer::builder()
            .label("CAD Points Buffer")
            .size(1024)
            .usage(wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST)
            .build(device);

        let num_points_buffer = Buffer::builder()
            .label("CAD Num Points Buffer")
            .size(4)
            .usage(wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST)
            .build(device);

        let compute_bind_group = BindGroup::builder()
            .label("CAD Compute Bind Group")
            .entry(BindGroupEntry {
                visibility: wgpu::ShaderStages::COMPUTE,
                binding: BindingDescriptor::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    resource: &vertex_buffer,
                },
            })
            .entry(BindGroupEntry {
                visibility: wgpu::ShaderStages::COMPUTE,
                binding: BindingDescriptor::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    resource: &indirect_draw_args_buffer,
                },
            })
            .entry(BindGroupEntry {
                visibility: wgpu::ShaderStages::COMPUTE,
                binding: BindingDescriptor::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    resource: &view_transform_buffer,
                },
            })
            .entry(BindGroupEntry {
                visibility: wgpu::ShaderStages::COMPUTE,
                binding: BindingDescriptor::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    resource: &points_buffer,
                },
            })
            .entry(BindGroupEntry {
                visibility: wgpu::ShaderStages::COMPUTE,
                binding: BindingDescriptor::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    resource: &num_points_buffer,
                },
            })
            .build(device);

        // compute pipeline setup
        let compute_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("CAD Compute Shader"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("shaders/cad_geometry_compute.wgsl").into(),
            ),
        });
        let compute_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("CAD Compute Pipeline Layout"),
                bind_group_layouts: &[Some(compute_bind_group.layout())],
                immediate_size: 0,
            });
        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("CAD Compute Pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &compute_shader,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        });
        // render pipeline setup
        const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
            0 => Float32x4,
            1 => Float32x4
        ];
        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<GpuVertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &VERTEX_ATTRIBUTES,
        };
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("CAD Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/cad.wgsl").into()),
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("CAD Pipeline Layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("CAD Pipeline"),
            layout: Some(&pipeline_layout),

            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[Some(vertex_buffer_layout)],
            },

            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        Self {
            device: device.clone(),
            vertex_buffer,
            indirect_draw_args_buffer,
            view_transform_buffer,
            points_buffer,
            num_points_buffer,
            num_points: 0,
            compute_bind_group,
            compute_pipeline,
            render_pipeline,
            background_color: wgpu::Color::BLACK,
        }
    }

    fn recreate_render_bind_group(&mut self) {}

    pub fn prepare(
        &mut self,
        queue: &wgpu::Queue,
        model: Option<&CadModel>,
        model_view: &CadView,
        viewport_size: &Vec2,
        pixels_per_mm: f32,
    ) {
        let mut recreate_bind_groups = false;

        self.background_color = wgpu::Color::BLACK;
        let mut points: Vec<GpuPoint> = Vec::new();
        if model.is_some() {
            self.background_color = wgpu::Color::WHITE;
            points = model
                .unwrap()
                .points
                .iter()
                .map(|(element_id, point)| GpuPoint {
                    location: [point.location.x, point.location.y],
                    element_id: element_id.0.into(),
                    _padding: 0,
                })
                .collect();
        }
        self.num_points = points.len() as u32;
        let points_buffer_size = (points.len() * std::mem::size_of::<GpuPoint>()) as u64;
        if self.points_buffer.size() < points_buffer_size {
            self.points_buffer.resize(points_buffer_size);
            recreate_bind_groups = true;
        }
        queue.write_buffer(&self.points_buffer.data(), 0, bytemuck::cast_slice(&points));

        queue.write_buffer(
            &self.num_points_buffer.data(),
            0,
            &self.num_points.to_ne_bytes(),
        );

        let vertex_buffer_size = (3 * 16 * points.len() * std::mem::size_of::<GpuVertex>()) as u64;
        if self.vertex_buffer.size() < vertex_buffer_size {
            self.vertex_buffer.resize(vertex_buffer_size);
            recreate_bind_groups = true;
        }
        queue.write_buffer(
            &self.indirect_draw_args_buffer.data(),
            0,
            &(0u32).to_ne_bytes(),
        );
        queue.write_buffer(
            &self.indirect_draw_args_buffer.data(),
            4,
            &(1u32).to_ne_bytes(),
        );
        queue.write_buffer(
            &self.indirect_draw_args_buffer.data(),
            8,
            &(0u32).to_ne_bytes(),
        );
        queue.write_buffer(
            &self.indirect_draw_args_buffer.data(),
            12,
            &(0u32).to_ne_bytes(),
        );

        let transform = ViewTransform {
            world_to_ndc_matrix: model_view
                .world_to_ndc_matrix(viewport_size, pixels_per_mm)
                .to_cols_array_2d(),
            viewport_size: [viewport_size.x, viewport_size.y],
            _padding: [0.0; 2],
        };
        queue.write_buffer(
            &self.view_transform_buffer.data(),
            0,
            bytemuck::bytes_of(&transform),
        );

        if recreate_bind_groups {
            self.compute_bind_group.recreate();
            self.recreate_render_bind_group();
        }
    }

    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        viewport: ViewportRect,
    ) {
        self.device
            .on_uncaptured_error(std::sync::Arc::new(|error| {
                eprintln!("wgpu error: {error}");
            }));
        // compute pass - generate geometry
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("CAD Compute Pass"),
                timestamp_writes: None,
            });

            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, self.compute_bind_group.data(), &[]);

            let workgroup_count = self.num_points.div_ceil(64);
            compute_pass.dispatch_workgroups(workgroup_count, 1, 1);
        }

        // render pass
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("CAD Viewport"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.background_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            // render pass
            pass.set_viewport(
                viewport.x as f32,
                viewport.y as f32,
                viewport.width as f32,
                viewport.height as f32,
                0.0,
                1.0,
            );

            pass.set_scissor_rect(viewport.x, viewport.y, viewport.width, viewport.height);

            pass.set_pipeline(&self.render_pipeline);
            pass.set_vertex_buffer(0, self.vertex_buffer.data().slice(..));
            pass.draw_indirect(&self.indirect_draw_args_buffer.data(), 0);
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuVertex {
    pub position: [f32; 4],
    pub color: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct DrawIndirectArgs {
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct ViewTransform {
    pub world_to_ndc_matrix: [[f32; 4]; 4],
    pub viewport_size: [f32; 2],
    pub _padding: [f32; 2],
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuPoint {
    pub location: [f32; 2],
    pub element_id: u32,
    pub _padding: u32,
}
