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
    pub light: Light,

    pub instances: Vec<ObjectInstance>,
    pub instance_buffer: wgpu::Buffer,
}

impl RenderProcessor {

    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, canvas: &Canvas, settings: &RenderSettings) -> RenderProcessor {
        // Setup uniform bindings

        let camera: Camera = Camera::new(device, &settings.camera_setup);
        let light: Light = Light::new(device);


        // Setup fragment bindings

        let textures = TexturePack::new(device, queue, settings.image_data);
        let texture_format = canvas.config.format;

        let uv_scale = [0.5, 0.5];


        // Setup vertex bindings

        let shape = crate::shapes::create_square(uv_scale);
        let (vertex_buffer, index_buffer) = shape.setup_wgpu_buffers(device);

        let num_vertices = shape.vertices.len() as u32;
        let num_indices = shape.indices.len() as u32;

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
        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );


        // Setup sprite animation bindings

        let sprite = SpriteGpuHandle::new(
            device, 
            vec![
                [0.0, 0.0], [0.5, 0.0], [0.0, 0.5], [0.5, 0.5]
            ],
            instances.len()
        );


        // Build pipeline

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&format!("Shader Module")),
            source: wgpu::ShaderSource::Wgsl(settings.shader_src.into()),
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&format!("Pipeline Layout")),
            bind_group_layouts: &[
                &textures.layout,
                &camera.binding.layout,
                &sprite.binding.layout,
                &light.binding.layout,
            ],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&format!("Render Pipeline")),

            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: settings.vertex_entry_point,
                buffers: &[Vertex::desc(), InstanceRaw::layout()],
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
            light,

            instances,
            instance_buffer,
        }
    }

    pub fn execute_render_pipeline(&mut self, mut ctx: RenderContext) {
        // pre render pass -- update buffer data
        self.camera.buffer_update(&ctx.gx);
        self.sprite.buffer_update(&ctx.gx, ctx.px.timer.sprite_frames());
        self.light.buffer_update(&ctx.gx);

        // start render pass
        {
            let mut render_pass = ctx.begin_render_pass();

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.textures.bindgroup, &[]);
            render_pass.set_bind_group(1, &self.camera.binding.bindgroup, &[]);
            render_pass.set_bind_group(2, &self.sprite.binding.bindgroup, &[]);
            render_pass.set_bind_group(3, &self.light.binding.bindgroup, &[]);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.instances.len() as _);
        }

        ctx.gx.queue.submit(std::iter::once(ctx.encoder.finish()));
        ctx.output.present(); 
    }
}
