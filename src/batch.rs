use wgpu::{Device};
use crate::{material::Material, ObjectInstance, shapes::Shape};

pub struct Model {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_vertices: u32,
    pub num_indices: u32,
}

impl Model {
    pub fn new(device: &Device, shape: Shape, uv_scale: [f32; 2]) -> Self {
        // Setup vertex bindings
        //let shape = crate::shapes::create_square(uv_scale);
        let (vertex_buffer, index_buffer) = shape.setup_wgpu_buffers(device);

        let num_vertices = shape.vertices.len() as u32;
        let num_indices = shape.indices.len() as u32;

        Model {
            vertex_buffer,
            index_buffer,
            num_vertices,
            num_indices,
        }
    }
}

pub struct Batch {
    pub material: Material,
    pub model: Model,
    
    pub instances: Vec<ObjectInstance>,
    pub instance_buffer: wgpu::Buffer,
    
    pub pipeline: wgpu::PipelineLayout,
}

impl Batch {
    pub fn new(device: &Device, model: Model, material: Material) -> Self { 
        // // Setup fragment bindings
        // let texture = TexturePack::new(device, queue, texture_layout, image);
        // let uv_scale = [0.5, 0.5];       

        let instances = (0..NUM_INSTANCES_PER_ROW * NUM_INSTANCES_PER_ROW)
            .map(|instance_idx| {
                let hwidth = NUM_INSTANCES_PER_ROW as f32 * 0.5;
                let x = ((instance_idx as f32).sin() / 5.0) + (instance_idx % NUM_INSTANCES_PER_ROW) as f32 - hwidth;
                let z = ((instance_idx as f32).cos() / 2.0) + (instance_idx / NUM_INSTANCES_PER_ROW) as f32 - hwidth;

                let position = cgmath::Vector3 { x: x, y: 0.0, z: z * 0.35 } - INSTANCE_DISPLACEMENT;
                let rotation = cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0));
                
                ObjectInstance { instance_idx, position, rotation, }
            })
            .collect::<Vec<_>>();           

        let instance_data = instances.iter().map(ObjectInstance::to_raw).collect::<Vec<_>>();
        //let instance_data = Vec::<ObjectInstance>::new();
        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        // Setup sprite animation bindings
        // let sprite = SpriteGpuHandle::new(
        //     device, 
        //     sprite_layout,
        //     vec![
        //         [0.0, 0.0], [0.5, 0.0], [0.0, 0.5], [0.5, 0.5]
        //     ],
        //     instances.len()
        // );

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&format!("Render Pipeline")),

            layout: Some(&material.layout),
            vertex: wgpu::VertexState {
                module: &material.shader,
                entry_point: "vert",
                buffers: self.vertex_buffers.as_slice(), //&[Vertex::desc(), InstanceRaw::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &material.shader,
                entry_point: "frag",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba32Float,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        Batch {

        }
    }
}