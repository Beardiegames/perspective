use crate::{shapes::Vertex, layout::PerspectiveShaderLayout};

use super::*;
use cgmath::prelude::*;
use wgpu::util::DeviceExt;


pub struct SpriteRenderObject {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_vertices: u32,
    pub num_indices: u32,
    pub texture_id: TextureID,
    pub animation: SpriteGpuHandle,
}

impl SpriteRenderObject {
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

        SpriteRenderObject {
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
            max_pool_size: 10_000,
            temp_offset: 0.0,
        }
    }
}

pub struct SpritePoolID {
    index: usize,
}

pub struct SpriteInstanceID {
    pool_idx: usize,
    instance_idx: usize,
}

pub struct SpritePool {
    pub sprite_obj: SpriteRenderObject,
    pub instances: Vec<ObjectInstance>,
    pub instance_buffer: wgpu::Buffer,

    pub num_spawns: usize,
}

impl SpritePool {

    pub fn new(
        device: &wgpu::Device, 
        bind_group_layouts: &PerspectiveShaderLayout,
        settings: &SpritePoolSetup,
    ) -> Self { 

        let sprite_obj = SpriteRenderObject::new(
            device, //queue, 
            bind_group_layouts,
            settings,
        );

        let instances = (0..settings.max_pool_size as u32)
            .map(|instance_idx| {
                // let hwidth = NUM_INSTANCES_PER_ROW as f32 * 0.5;
                // let x = ((instance_idx as f32).sin() / 5.0) + (instance_idx % NUM_INSTANCES_PER_ROW) as f32 - hwidth;
                // let z = ((instance_idx as f32).cos() / 2.0) + (instance_idx / NUM_INSTANCES_PER_ROW) as f32 - hwidth;

                // let position = cgmath::Vector3 { x, y: settings.temp_offset, z: z * 0.35 } - INSTANCE_DISPLACEMENT;
                // let rotation = cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0));
                
                ObjectInstance { 
                    instance_idx, 
                    position: cgmath::Vector3::zero(), 
                    rotation: cgmath::Quaternion::zero(), }
            })
            .collect::<Vec<_>>();           

        // let instances = (0..NUM_INSTANCES_PER_ROW * NUM_INSTANCES_PER_ROW)
        //     .map(|instance_idx| {
        //         let hwidth = NUM_INSTANCES_PER_ROW as f32 * 0.5;
        //         let x = ((instance_idx as f32).sin() / 5.0) + (instance_idx % NUM_INSTANCES_PER_ROW) as f32 - hwidth;
        //         let z = ((instance_idx as f32).cos() / 2.0) + (instance_idx / NUM_INSTANCES_PER_ROW) as f32 - hwidth;

        //         let position = cgmath::Vector3 { x, y: settings.temp_offset, z: z * 0.35 } - INSTANCE_DISPLACEMENT;
        //         let rotation = cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0));
                
        //         ObjectInstance { instance_idx, position, rotation, }
        //     })
        //     .collect::<Vec<_>>();           

        let instance_data = instances.iter().map(ObjectInstance::to_raw).collect::<Vec<_>>();

        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        SpritePool {
            sprite_obj,
            instances,
            instance_buffer,

            num_spawns: 0
        }
    }

    pub fn update_instance_buffer(&mut self, gx: &WgpuCore) {
        let instance_data = self.instances.iter().map(ObjectInstance::to_raw).collect::<Vec<_>>();

        gx.queue.write_buffer(
            &self.instance_buffer, 
            0, 
            bytemuck::cast_slice(&instance_data),
        );
    }
}

pub struct Renderer {
    pub shader: wgpu::ShaderModule,
    pub pipeline: wgpu::RenderPipeline,
    pub layout: wgpu::PipelineLayout,

    pub camera: Camera,
    pub ambient_light: AmbientLight,

    pub bindgroup_layouts: PerspectiveShaderLayout,
    pub textures: TexturePack,
    pub sprites: Vec<SpritePool>,
}

impl Renderer {

    pub fn new(
        device: &Device, 
        camera_setup: &CameraSetup, 
        textures: TexturePack,
        //sprite_setup: &[SpritePoolSetup]
    ) -> Self {

        let bindgroup_layouts = PerspectiveShaderLayout::new(device);

        let camera = Camera::new(device, bindgroup_layouts.camera_layout(), camera_setup);
        let ambient_light = AmbientLight::new(device, bindgroup_layouts.lights_layout());
        
        let sprites = Vec::new();
        //for s in sprite_setup {
            //sprites.push(SpritePool::new(device, &bindgroup_layouts, s));
        //}

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
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: true,
            },
            multiview: None,
        });

        Renderer {
            shader,
            pipeline,
            layout,
            camera,
            ambient_light,
            bindgroup_layouts,
            textures,
            sprites,
        }
    }

    pub fn create_sprite_pool(&mut self, gx: &WgpuCore, setup: &SpritePoolSetup) -> SpritePoolID {
        self.sprites.push(SpritePool::new(&gx.device, &self.bindgroup_layouts, setup));
        renderer::SpritePoolID { index: self.sprites.len() - 1 }
    }

    pub fn spawn_sprite(
        &mut self,
        sprite_pool_id: &SpritePoolID,

        position: cgmath::Vector3<f32>,
        rotation: cgmath::Quaternion<f32>,
    ) -> SpriteInstanceID {

        let instance_idx = self.sprites[sprite_pool_id.index].num_spawns;
        let sprite = &mut self.sprites[sprite_pool_id.index].instances[instance_idx];
            sprite.position = position;
            sprite.rotation = rotation;

        self.sprites[sprite_pool_id.index].num_spawns += 1;

        SpriteInstanceID { 
            pool_idx: sprite_pool_id.index, 
            instance_idx: self.sprites[sprite_pool_id.index].num_spawns - 1
        }
    }

    pub fn get_sprite(
        &mut self,
        sprite_instance_id: &SpriteInstanceID,
    ) -> &mut ObjectInstance {
        &mut self.sprites[sprite_instance_id.pool_idx].instances[sprite_instance_id.instance_idx]
    }

    pub fn execute_render_pipeline(&mut self, mut ctx: RenderContext) {
        self.camera.buffer_update(ctx.gx);
        self.ambient_light.buffer_update(ctx.gx);

        for spritepool in &mut self.sprites {
            spritepool.update_instance_buffer(&ctx.gx);
            spritepool.sprite_obj.animation.buffer_update(ctx.gx, ctx.px.timer.sprite_frames());
        }

        if let Some(mut render_pass) = ctx.begin_render_pass() {

            render_pass.set_pipeline(&self.pipeline);

            render_pass.set_bind_group(1, &self.camera.binding.bindgroup, &[]);
            render_pass.set_bind_group(2, &self.ambient_light.binding.bindgroup, &[]);

            for spritepool in &mut self.sprites {
                if let Some(tex) =  &self.textures.get(&spritepool.sprite_obj.texture_id) {
                    render_pass.set_bind_group(0, &tex.bindgroup, &[]);
                }
                render_pass.set_bind_group(3, &spritepool.sprite_obj.animation.binding.bindgroup, &[]);

                render_pass.set_vertex_buffer(0, spritepool.sprite_obj.vertex_buffer.slice(..));
                render_pass.set_vertex_buffer(1, spritepool.instance_buffer.slice(..));

                render_pass.set_index_buffer(spritepool.sprite_obj.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                //render_pass.draw_indexed(0..spritepool.sprite_obj.num_indices, 0, 0..spritepool.instances.len() as _);
                render_pass.draw_indexed(0..spritepool.sprite_obj.num_indices, 0, 0..spritepool.num_spawns as _);
            }
        } 

        ctx.gx.queue.submit(std::iter::once(ctx.encoder.finish()));

        if let Some(draw) = ctx.draw {
            draw.output.present(); 
        }
    }
}