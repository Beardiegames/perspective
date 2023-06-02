use crate::{shapes::Vertex, layout::PerspectiveShaderLayout};

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

    pub image_data: &'a [u8],
    pub camera_setup: CameraSetup,
}

pub struct RenderObject {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_vertices: u32,
    pub num_indices: u32,

    pub instances: Vec<ObjectInstance>,
    pub instance_buffer: wgpu::Buffer,

    pub texture: TexturePack,
    pub sprite: SpriteGpuHandle,
}

impl RenderObject {

    pub fn new(
        device: &wgpu::Device, 
        queue: &wgpu::Queue, 
        texture_layout: &wgpu::BindGroupLayout, 
        sprite_layout: &wgpu::BindGroupLayout, 
        image: &[u8]
    ) -> Self { 

        // Setup fragment bindings
        let texture = TexturePack::new(device, queue, texture_layout, image);
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
        //let instance_data = Vec::<ObjectInstance>::new();
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
            sprite_layout,
            vec![
                [0.0, 0.0], [0.5, 0.0], [0.0, 0.5], [0.5, 0.5]
            ],
            instances.len()
        );

        RenderObject {
            vertex_buffer,
            index_buffer,
            num_vertices,
            num_indices,
        
            instances,
            instance_buffer,
        
            texture,
            sprite,
        }
        
    }

    pub fn pre_render_pass(
        &mut self, 
        ctx: &mut RenderContext, 
    ) {
        self.sprite.buffer_update(&ctx.gx, ctx.px.timer.sprite_frames());
    }

    pub fn exec_render_pass(
        &self, 
        ctx: &mut RenderContext, 
        pipeline: &RenderPipeline,
        camera: &Camera,
        light: &Light,
    ) {
        
        let mut render_pass = ctx.begin_render_pass();

        render_pass.set_pipeline(pipeline);
        render_pass.set_bind_group(0, &self.texture.bindgroup, &[]);
        render_pass.set_bind_group(1, &camera.binding.bindgroup, &[]);
        render_pass.set_bind_group(2, &light.binding.bindgroup, &[]);
        render_pass.set_bind_group(3, &self.sprite.binding.bindgroup, &[]);
        // for i in 0..self.bindings.len() {
        //     render_pass.set_bind_group(i as u32, &self.bindings[i].bindgroup, &[]);
        // }

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        render_pass.draw_indexed(0..self.num_indices, 0, 0..self.instances.len() as _);
    }
}

pub struct RenderProcessor {
    pub shader: wgpu::ShaderModule,
    pub pipeline: wgpu::RenderPipeline,
    pub layout: wgpu::PipelineLayout,

    pub camera: Camera,
    pub light: Light,

    pub bind_group_layouts: PerspectiveShaderLayout,
    pub render_objects: Vec<RenderObject>,
}

impl RenderProcessor {

    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, canvas: &Canvas, settings: &RenderSettings) -> RenderProcessor {

        let bind_group_layouts = PerspectiveShaderLayout::new(device);

        // Setup uniform bindings
        let camera: Camera = Camera::new(device, bind_group_layouts.camera_layout(), &settings.camera_setup);
        let light: Light = Light::new(device, bind_group_layouts.effects_layout());
        let texture_format = canvas.config.format;

        // Build pipeline
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&format!("Shader Module")),
            source: wgpu::ShaderSource::Wgsl(settings.shader_src.into()),
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

        let mut render_objects = Vec::new();

        render_objects.push(
            RenderObject::new(
                device, 
                queue, 
                bind_group_layouts.texture_layout(),
                bind_group_layouts.sprite_layout(), 
                settings.image_data
            )
        );

        RenderProcessor { 
            shader, 
            pipeline, 
            layout,

            camera,
            light,

            bind_group_layouts,
            render_objects,
        }
    }

    pub fn create_render_object(&mut self, gx: &WgpuCore, image_data: &[u8]) {
        self.render_objects.push(
            RenderObject::new(
                &gx.device, 
                &gx.queue, 
                self.bind_group_layouts.texture_layout(),
                self.bind_group_layouts.sprite_layout(), 
                image_data
            )
        )
    }

    // pub fn instantiate_object(&mut self, gx: &WgpuCore, image_data: &'static [u8]) {
    //     self.render_objects.push(
    //         RenderObject::new(
    //             &gx.device, 
    //             &gx.queue, 
    //             self.bind_group_layouts.texture_layout(),
    //             self.bind_group_layouts.sprite_layout(), 
    //             image_data
    //         )
    //     )
    // }

    pub fn execute_render_pipeline(&mut self, mut ctx: RenderContext) {
        self.camera.buffer_update(&ctx.gx);
        self.light.buffer_update(&ctx.gx);

        if self.render_objects.len() > 0 {

            // pre render pass -- update buffer data
            for pr in &mut self.render_objects {
                pr.pre_render_pass(&mut ctx);
            }

            // execute render passes
            for er in &mut self.render_objects {
                er.exec_render_pass(&mut ctx, &self.pipeline, &self.camera, &self.light);
            }
        }
        ctx.gx.queue.submit(std::iter::once(ctx.encoder.finish()));
        ctx.output.present(); 
    }
}
