use crate::{shapes::Vertex, layout::PerspectiveShaderLayout};

use super::*;
use cgmath::prelude::*;
use wgpu::util::DeviceExt;


pub struct SpriteObject {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_vertices: u32,
    pub num_indices: u32,
    //pub texture: TexturePack,
    pub texture_id: TextureID,
    pub animation: SpriteGpuHandle,
}

impl SpriteObject {
    pub fn new(
        device: &Device, 
        queue: &Queue, 
        bind_group_layouts: &PerspectiveShaderLayout,
        //image_data: &[u8],
        // texture_id: TextureID,
        // uv_scale: (f32, f32),
        // 
        settings: SpritePoolSetup,
        //pool_size: usize
    ) -> Self 
    {
        // Setup fragment bindings
        // let texture = TexturePack::new(
        //     device, 
        //     queue, 
        //     bind_group_layouts.texture_layout(), 
        //     image_data,
        // );
        // let uv_scale = [0.5, 0.5];

        // Setup vertex bindings
        let shape = crate::shapes::create_square([settings.tile_size.0, settings.tile_size.1]);
        let (vertex_buffer, index_buffer) = shape.setup_wgpu_buffers(device);

        let num_vertices = shape.vertices.len() as u32;
        let num_indices = shape.indices.len() as u32;

        // Setup sprite animation bindings
        let animation = SpriteGpuHandle::new(
            device, 
            bind_group_layouts.sprite_layout(),
            vec![
                [0.0, 0.0], [0.5, 0.0], [0.0, 0.5], [0.5, 0.5]
            ],
            settings.max_pool_size //instances.len()
        );

        SpriteObject {
            vertex_buffer,
            index_buffer,
            num_vertices,
            num_indices,
            //texture,
            texture_id: settings.texture_id,
            animation,
        }
    }
}

// #[derive(Default)]
pub struct SpritePoolSetup {
    pub custom_shader: Option<&'static str>,
    pub texture_id: TextureID,
    // pub image_data: &'static [u8],
    pub image_size: (u32, u32),
    pub tile_size: (f32, f32),
    pub max_pool_size: usize,
}

pub struct SpritePool {
    pub sprite_obj: SpriteObject,
    pub instances: Vec<ObjectInstance>,
    pub instance_buffer: wgpu::Buffer,
}

impl SpritePool {

    pub fn new(
        device: &wgpu::Device, 
        queue: &wgpu::Queue, 
        bind_group_layouts: &PerspectiveShaderLayout,
        settings: SpritePoolSetup,
    ) -> Self { 

        let sprite_obj = SpriteObject::new(
            device, queue, 
            bind_group_layouts,
            settings,
        );

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

        SpritePool {
            sprite_obj,
            instances,
            instance_buffer,
        }
        
    }

    pub fn pre_render_pass(&mut self, ctx: &mut RenderContext) {
        self.sprite_obj.animation.buffer_update(&ctx.gx, ctx.px.timer.sprite_frames());
    }

    pub fn exec_render_pass(
        &self, 
        ctx: &mut RenderContext, 
        pipeline: &RenderPipeline,
        textures: &TexturePack,
        camera: &Camera,
        light: &Light,
    ) {
        let mut render_pass = ctx.begin_render_pass();

        render_pass.set_pipeline(pipeline);

        if let Some(tex) = textures.get(&self.sprite_obj.texture_id) {
            render_pass.set_bind_group(0, &tex.bindgroup, &[]);
        }
        render_pass.set_bind_group(1, &camera.binding.bindgroup, &[]);
        render_pass.set_bind_group(2, &light.binding.bindgroup, &[]);
        render_pass.set_bind_group(3, &self.sprite_obj.animation.binding.bindgroup, &[]);

        render_pass.set_vertex_buffer(0, self.sprite_obj.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

        render_pass.set_index_buffer(self.sprite_obj.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        render_pass.draw_indexed(0..self.sprite_obj.num_indices, 0, 0..self.instances.len() as _);
    }
}

pub struct Renderer {
    pub shader: wgpu::ShaderModule,
    pub pipeline: wgpu::RenderPipeline,
    pub layout: wgpu::PipelineLayout,

    pub camera: Camera,
    pub light: Light,

    pub bind_group_layouts: PerspectiveShaderLayout,
    pub textures: TexturePack,
    pub sprite_pool: SpritePool,
}

impl Renderer {

    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, canvas: &Canvas, textures: TexturePack, camera_setup: CameraSetup, sprite_setup: SpritePoolSetup) -> Renderer {

        let bind_group_layouts = PerspectiveShaderLayout::new(device);

        // Setup uniform bindings
        let camera: Camera = Camera::new(device, bind_group_layouts.camera_layout(), &camera_setup);
        let light: Light = Light::new(device, bind_group_layouts.effects_layout());
        let texture_format = canvas.config.format;

        // Build pipeline
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&format!("Shader Module")),
            source: wgpu::ShaderSource::Wgsl(
                match sprite_setup.custom_shader {
                    Some(s) => s.into(),
                    None => include_str!("shaders/sprite_shader.wgsl").into()
                }
            ),
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&format!("Pipeline Layout")),
            bind_group_layouts: &bind_group_layouts.as_slice(),
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&format!("Render Pipeline")),

            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vert",
                buffers: &[Vertex::desc(), InstanceRaw::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "frag",
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

        let sprite_pool = SpritePool::new(
            device, 
            queue,
            &bind_group_layouts,
            sprite_setup,
        );

        Renderer { 
            shader, 
            pipeline, 
            layout,

            camera,
            light,

            bind_group_layouts,
            textures,
            sprite_pool,
        }
    }

    pub fn execute_render_pipeline(&mut self, mut ctx: RenderContext) {
        self.camera.buffer_update(&ctx.gx);
        self.light.buffer_update(&ctx.gx);

        self.sprite_pool.pre_render_pass(&mut ctx);
        self.sprite_pool.exec_render_pass(&mut ctx, &self.pipeline,&self.textures, &self.camera, &self.light);

        ctx.gx.queue.submit(std::iter::once(ctx.encoder.finish()));
        ctx.output.present(); 
    }
}
