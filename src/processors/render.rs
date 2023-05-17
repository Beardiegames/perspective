use crate::shapes::Vertex;

use super::*;
//use wgpu::util::DeviceExt;


pub struct RenderSettings<'a> {
    pub label: &'a str, 
    pub group_index: u32,// represented within shader as @binding
    pub binding_index: u32,// represented within shader as @binding

    pub shader_src: &'a str, // string slice representation of the actual shader code
    pub vertex_entry_point: &'a str, // name of the vertex entry funcion/methode, called on vertex update
    pub fragment_entry_point: &'a str, // name of the fragment entry funcion/methode, called on fragment update

    pub image_data: &'static [u8],
}

pub struct RenderProcessor {
    pub shader: wgpu::ShaderModule,
    pub pipeline: wgpu::RenderPipeline,
    pub layout: wgpu::PipelineLayout,

    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_vertices: u32,
    pub num_indices: u32,

    pub textures: TexturePack,

    pub camera: Camera,
    pub camera_uniform: CameraUniform,
    pub camera_gpu_handle: GpuCameraHandle,
}

impl RenderProcessor {

    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, canvas: &Canvas, settings: &RenderSettings) -> RenderProcessor {

        let textures = TexturePack::new(device, queue, settings.image_data);
        let texture_format = canvas.config.format;

        let camera = Camera::default();
        let camera_uniform = CameraUniform::new();
        let camera_gpu_handle = GpuCameraHandle::new(device, camera_uniform);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&format!("{}_render-shader", settings.label)),
            source: wgpu::ShaderSource::Wgsl(settings.shader_src.into()),
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&format!("{}_render-layout", settings.label)),
            bind_group_layouts: &[
                &textures.layout,
                &camera_gpu_handle.layout,
            ],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&format!("{}_render-pipeline", settings.label)),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: settings.vertex_entry_point,
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: settings.fragment_entry_point,
                targets: &[Some(wgpu::ColorTargetState {
                    format: texture_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let uv_scale = [0.5, 0.5];
        let shape = crate::shapes::create_square(uv_scale);

        let (vertex_buffer, index_buffer) = shape.setup_wgpu_buffers(device);

        let num_vertices = shape.vertices.len() as u32;
        let num_indices = shape.indices.len() as u32;


        RenderProcessor { 
            shader, 
            pipeline, 
            layout,

            vertex_buffer, 
            index_buffer, 
            num_vertices, 
            num_indices,

            textures,

            camera,
            camera_uniform,
            camera_gpu_handle,
        }
    }

}