use crate::shapes::Vertex;

use super::*;
use cgmath::prelude::*;
use wgpu::util::DeviceExt;


pub struct RenderSettings<'a> {
    pub label: &'a str, 
    pub group_index: u32,// represented within shader as @binding
    pub binding_index: u32,// represented within shader as @binding

    pub shader_src: &'a str, // string slice representation of the actual shader code
    pub vertex_entry_point: &'a str, // name of the vertex entry funcion/methode, called on vertex update
    pub fragment_entry_point: &'a str, // name of the fragment entry funcion/methode, called on fragment update

    pub image_data: &'static [u8],
    pub camera_setup: CameraSetup,
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
    pub sprite: SpriteGpuHandle,

    pub instances: Vec<ObjectInstance>,
    pub instance_buffer: wgpu::Buffer,
}

impl RenderProcessor {

    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, canvas: &Canvas, settings: &RenderSettings) -> RenderProcessor {

        let textures = TexturePack::new(device, queue, settings.image_data);
        let texture_format = canvas.config.format;

        let uv_scale = [0.5, 0.5];
        let sprite = SpriteGpuHandle::new(device, vec![
            [0.0, 0.0], [0.5, 0.0], [0.0, 0.5], [0.5, 0.5]
        ]);

        let camera = Camera::new(device, &settings.camera_setup);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&format!("render-shader")),
            source: wgpu::ShaderSource::Wgsl(settings.shader_src.into()),
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&format!("render-layout")),
            bind_group_layouts: &[
                &textures.layout,
                &camera.layout,
                &sprite.layout,
            ],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&format!("render-pipeline")),

            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: settings.vertex_entry_point,
                buffers: &[Vertex::desc(), InstanceRaw::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: settings.fragment_entry_point,
                targets: &[Some(wgpu::ColorTargetState {
                    format: texture_format,
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
                format: DepthTexture::DEPTH_FORMAT,
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

        let shape = crate::shapes::create_square(uv_scale);
        let (vertex_buffer, index_buffer) = shape.setup_wgpu_buffers(device);

        let num_vertices = shape.vertices.len() as u32;
        let num_indices = shape.indices.len() as u32;


        let instances = (0..NUM_INSTANCES_PER_ROW).flat_map(|z| {
            (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                let position = cgmath::Vector3 { x: x as f32, y: 0.0, z: z as f32 } - INSTANCE_DISPLACEMENT;

                let rotation = if position.is_zero() {
                    // this is needed so an object at (0, 0, 0) won't get scaled to zero
                    // as Quaternions can effect scale if they're not created correctly
                    cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
                } else {
                    cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
                };

                ObjectInstance { position, rotation, }
            })
        }).collect::<Vec<_>>();

        let instance_data = instances.iter().map(ObjectInstance::to_raw).collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

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
            sprite,

            instances,
            instance_buffer,
        }
    }

    // pub fn update_uniform(&mut self, ctx: &RenderContext) {
    //     self.uniform.update(&ctx.px.timer, &ctx.px.camera);

    //     ctx.gx.queue.write_buffer(
    //         &self.camera.buffer, 
    //         0, 
    //         bytemuck::cast_slice(&[self.uniform.data])
    //     );
    // }
}
