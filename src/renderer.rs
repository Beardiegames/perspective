use crate::{shapes::Vertex, layout::PerspectiveShaderLayout};
use crate::spritepool::*;
use super::*;

pub struct Renderer {
    pub shader: wgpu::ShaderModule,
    pub pipeline: wgpu::RenderPipeline,
    pub layout: wgpu::PipelineLayout,
    pub bindgroup_layouts: PerspectiveShaderLayout,

    pub camera: Camera,
    pub ambient_light: AmbientLight,
    pub assets: AssetPack,
    pub sprites: Sprites,
}

impl Renderer {

    pub fn new(
        device: &Device, 
        camera_setup: &CameraSetup, 
        assets: AssetPack,
        ) -> Self 
    {
        let bindgroup_layouts = PerspectiveShaderLayout::new(device);
        let camera = Camera::new(device, bindgroup_layouts.camera_layout(), camera_setup);
        let ambient_light = AmbientLight::new(device, bindgroup_layouts.lights_layout());     
        let sprites = Sprites::new();

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
                    //format: wgpu::TextureFormat::Rgba8UnormSrgb,
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
            assets,
            sprites,
        }
    }

    pub fn create_sprite_pool(
        &mut self, 
        gx: &WgpuGrapics, 
        texture_id: &AssetID,
        setup: &SpritePoolSettings
        ) -> SpritePoolID 
    {
        self.sprites.add_sprite_pool(SpritePool::new(&gx.device, &self.bindgroup_layouts, texture_id.clone(), setup))
    }

    pub fn execute_render_pipeline(
        &mut self, 
        gx: &WgpuGrapics, 
        mut ctx: RenderContext
        ) 
    {
        self.camera.buffer_update(gx);
        self.ambient_light.buffer_update(gx);

        for spritepool in self.sprites.mut_pool_list() {
            spritepool.update_instance_buffer(gx);
            spritepool.sprite_obj.animation.buffer_update(gx, gx.timer.sprite_frames());
        }

        if let Some(mut render_pass) = ctx.begin_render_pass() {

            render_pass.set_pipeline(&self.pipeline);

            render_pass.set_bind_group(1, &self.camera.binding.bindgroup, &[]);
            render_pass.set_bind_group(2, &self.ambient_light.binding.bindgroup, &[]);

            for spritepool in self.sprites.mut_pool_list() {
                if let Some(tex) =  &self.assets.get_texture(&spritepool.sprite_obj.texture_id) {
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

        gx.queue.submit(std::iter::once(ctx.encoder.finish()));

        if let Some(draw) = ctx.draw {
            draw.output.present(); 
        }
    }
}