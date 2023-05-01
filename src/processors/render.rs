use crate::shapes::Vertex;

use super::*;
use wgpu::util::DeviceExt;


pub struct RenderSettings<'a> {
    pub label: &'a str, 
    pub group_index: u32,// represented within shader as @binding
    pub binding_index: u32,// represented within shader as @binding

    pub shader_src: &'a str, // string slice representation of the actual shader code
    pub vertex_entry_point: &'a str, // name of the vertex entry funcion/methode, called on vertex update
    pub fragment_entry_point: &'a str, // name of the fragment entry funcion/methode, called on fragment update
}

pub struct RenderProcessor {
    pub shader: wgpu::ShaderModule,
    pub pipeline: wgpu::RenderPipeline,

    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_vertices: u32,
    pub num_indices: u32,
}

impl RenderProcessor {

    pub fn new(core: &mut WgpuCore, settings: &RenderSettings) -> RenderProcessor {

        let (shader, pipeline) = build_render_pipe(
            core, 
            &format!("{}_render-pipeline", settings.label),
            settings.shader_src, 
            settings.vertex_entry_point,
            settings.fragment_entry_point
        );

        let shape = crate::shapes::HEXAGON;

        let vertex_buffer = core.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(shape.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let num_vertices = shape.vertices.len() as u32;

        let index_buffer = core.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(shape.indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        let num_indices = shape.indices.len() as u32;
 
        
        RenderProcessor { shader, pipeline, vertex_buffer, index_buffer, num_vertices, num_indices }
    }
}

pub fn build_render_pipe(
    core: &WgpuCore, 
    label: &str, 
    shader_src: &str, 
    vertex_entry_point: &str, 
    fragment_entry_point: &str, 
) -> (wgpu::ShaderModule, wgpu::RenderPipeline) 
{
    let texture_format = core.canvas.as_ref()
        .expect("Canvas not available! Try passing WindowSettings when creating a new WgpuCore.")
        .config.format;

    let shader = core.device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(&format!("{}_shader", label)),
        source: wgpu::ShaderSource::Wgsl(shader_src.into()),
    });

    let layout = core.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(&format!("{}_layout", label)),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let pipeline = core.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(&format!("{}_pipeline", label)),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: vertex_entry_point,
            buffers: &[Vertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: fragment_entry_point,
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
            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLIP_CONTROL
            unclipped_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
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
    
    (shader, pipeline)
}