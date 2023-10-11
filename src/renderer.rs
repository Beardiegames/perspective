use crate::{shapes::Vertex, layout::PerspectiveShaderLayout};

use super::*;
use cgmath::prelude::*;
use wgpu::util::DeviceExt;


pub struct SpriteObject {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_vertices: u32,
    pub num_indices: u32,
    pub texture_id: TextureID,
    pub animation: SpriteGpuHandle,
}

impl SpriteObject {
    pub fn new(
        device: &Device, 
        bind_group_layouts: &PerspectiveShaderLayout,
        settings: &SpritePoolSetup,
    ) -> Self 
    {
        // Setup vertex bindings
        let shape = crate::shapes::create_square([settings.tile_size.0, settings.tile_size.1]);
        let (vertex_buffer, index_buffer) = shape.setup_wgpu_buffers(device);

        let num_vertices = shape.vertices.len() as u32;
        let num_indices = shape.indices.len() as u32;

        // Setup sprite animation bindings
        let animation = SpriteGpuHandle::new(
            device, 
            bind_group_layouts.sprite_layout(),
            settings.animation_frames.clone(),
            settings.max_pool_size //instances.len()
        );

        SpriteObject {
            vertex_buffer,
            index_buffer,
            num_vertices,
            num_indices,
            texture_id: settings.texture_id.clone(),
            animation,
        }
    }
}

#[derive(Clone)]
pub struct SpritePoolSetup {
    pub texture_id: TextureID,
    pub image_size: (u32, u32),
    pub tile_size: (f32, f32),
    pub animation_frames: Vec<[f32; 2]>,
    pub max_pool_size: usize,
    pub temp_offset: f32,
}

impl Default for SpritePoolSetup {
    fn default() -> Self {
        SpritePoolSetup {
            texture_id: TextureID::Index(0),
            image_size: (1024, 1024),
            tile_size: (0.0625, 0.0625),
            animation_frames: vec![[0.0, 0.0], [0.5, 0.0], [0.0, 0.5], [0.5, 0.5]],
            max_pool_size: 100_000,
            temp_offset: 0.0,
        }
    }
}

pub struct SpritePool {
    pub sprite_obj: SpriteObject,
    pub instances: Vec<ObjectInstance>,
    pub instance_buffer: wgpu::Buffer,
}

impl SpritePool {

    pub fn new(
        device: &wgpu::Device, 
        bind_group_layouts: &PerspectiveShaderLayout,
        settings: &SpritePoolSetup,
    ) -> Self { 

        let sprite_obj = SpriteObject::new(
            device, //queue, 
            bind_group_layouts,
            settings,
        );

        let instances = (0..NUM_INSTANCES_PER_ROW * NUM_INSTANCES_PER_ROW)
            .map(|instance_idx| {
                let hwidth = NUM_INSTANCES_PER_ROW as f32 * 0.5;
                let x = ((instance_idx as f32).sin() / 5.0) + (instance_idx % NUM_INSTANCES_PER_ROW) as f32 - hwidth;
                let z = ((instance_idx as f32).cos() / 2.0) + (instance_idx / NUM_INSTANCES_PER_ROW) as f32 - hwidth;

                let position = cgmath::Vector3 { x, y: settings.temp_offset, z: z * 0.35 } - INSTANCE_DISPLACEMENT;
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
}

pub struct Renderer {
    pub shader: wgpu::ShaderModule,
    pub pipeline: wgpu::RenderPipeline,
    pub layout: wgpu::PipelineLayout,

    pub camera: Camera,
    pub light: Light,

    pub bindgroup_layouts: PerspectiveShaderLayout,
    pub textures: TexturePack,
    pub sprites: Vec<SpritePool>,
}

impl Renderer {

    pub fn new(
        device: &Device, 
        camera_setup: &CameraSetup, 
        textures: TexturePack,
        sprite_setup: &[SpritePoolSetup]
    ) -> Self {

        let bindgroup_layouts = PerspectiveShaderLayout::new(device);

        let camera = Camera::new(device, bindgroup_layouts.camera_layout(), camera_setup);
        let light = Light::new(device, bindgroup_layouts.effects_layout());
        
        let mut sprites = Vec::new();
        for s in sprite_setup {
            sprites.push(SpritePool::new(device, &bindgroup_layouts, s));
        }

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/sprite_shader.wgsl").into()),
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &bindgroup_layouts.as_slice(),
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
    
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
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
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

        Renderer {
            shader,
            pipeline,
            layout,
            camera,
            light,
            bindgroup_layouts,
            textures,
            sprites,
        }
    }

    pub fn execute_render_pipeline(&mut self, mut ctx: RenderContext) {
        self.camera.buffer_update(ctx.gx);
        self.light.buffer_update(ctx.gx);

        for spritepool in &mut self.sprites {
            spritepool.sprite_obj.animation.buffer_update(ctx.gx, ctx.px.timer.sprite_frames());
        }

        if let Some(mut render_pass) = ctx.begin_render_pass() {

            render_pass.set_pipeline(&self.pipeline);

            render_pass.set_bind_group(1, &self.camera.binding.bindgroup, &[]);
            render_pass.set_bind_group(2, &self.light.binding.bindgroup, &[]);

            for spritepool in &mut self.sprites {
                if let Some(tex) =  &self.textures.get(&spritepool.sprite_obj.texture_id) {
                    render_pass.set_bind_group(0, &tex.bindgroup, &[]);
                }
                render_pass.set_bind_group(3, &spritepool.sprite_obj.animation.binding.bindgroup, &[]);

                render_pass.set_vertex_buffer(0, spritepool.sprite_obj.vertex_buffer.slice(..));
                render_pass.set_vertex_buffer(1, spritepool.instance_buffer.slice(..));

                render_pass.set_index_buffer(spritepool.sprite_obj.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..spritepool.sprite_obj.num_indices, 0, 0..spritepool.instances.len() as _);
            }
        } 

        ctx.gx.queue.submit(std::iter::once(ctx.encoder.finish()));

        if let Some(draw) = ctx.draw {
            draw.output.present(); 
        }
    }
}