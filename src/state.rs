use winit::{
    event::*,
    window::Window,
    dpi::PhysicalSize,
};
use crate::*;
use cgmath::prelude::*;



pub struct Renderer {
    pub render_pipeline: wgpu::RenderPipeline,
    
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
    vertex_index_buffer: wgpu::Buffer,
    num_indices: u32,
    
    diffuse_bind_group: wgpu::BindGroup,
    diffuse_texture: texture::Texture,

    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    
    instances: Vec<Instance>,
    instance_buffer: wgpu::Buffer,
    depth_texture: Texture,
    
    //camera_controller: CameraController,
}

impl Renderer {
    // Creating some of the wgpu types requires async code
    pub fn new(gfx: &mut Gfx) -> Self {                    
        // load texture image
        let diffuse_bytes = include_bytes!("../assets/models/GrassTile.png");
        let diffuse_texture = texture::Texture::from_bytes(&gfx.canvas, diffuse_bytes, "../assets/models/GrassTile.png").unwrap(); // CHANGED!
        let (texture_bind_group_layout, diffuse_bind_group) = texture::create_diffuse_bindgroup(&gfx.canvas, &diffuse_texture);
                  
        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&gfx.camera);
        
        let camera_buffer = create_camera_buffer(&gfx.canvas, camera_uniform);
        let (camera_bind_group_layout, camera_bind_group) = create_camera_bindgroup(&gfx.canvas, &camera_buffer);
                  
        // setup vertex and indices buffers
        let (vertex_buffer, num_vertices) = vertices::new_vertex_buffer(&gfx.canvas.device);
        let (vertex_index_buffer, num_indices) = vertices::new_index_buffer(&gfx.canvas.device);
        
        // insantiating multiple render objects
        let instances = setup_instance_data();
        let instance_buffer = create_instance_buffer(&gfx.canvas, &instances);
                  
        // render depth buffering
        let depth_texture = texture::Texture::create_depth_texture(&gfx.canvas, "depth_texture");
        
                  
        // setup render pipeline
        let shader = shaders::create_shader(&gfx.canvas.device);
        
        let buffer_layouts = &[Vertex::desc(), InstanceRaw::desc()];
        let bindgroup_layouts = &[&texture_bind_group_layout, &camera_bind_group_layout];
        
        let render_pipeline = render_pipeline::create_new(
            &gfx.canvas.device,
            &shader,
            gfx.canvas.config.format,
            buffer_layouts,
            bindgroup_layouts,
        );

        // controller for camera movement -> This needs to be refactored into user implementation
        //let camera_controller = CameraController::new(0.2);
        
        Self {
            render_pipeline,
            
            vertex_buffer,
            num_vertices,
            vertex_index_buffer,
            
            num_indices,
            diffuse_bind_group,
            diffuse_texture,
            
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            
            instances,
            instance_buffer,
            depth_texture,
            
            //camera_controller,
        }
    }
    
    // pub fn reconfigure_surface(&self, gfx: &mut Gfx) {
    //     gfx.canvas.reconfigure_surface();
    // }

    pub fn resize(&mut self, gfx: &mut Gfx, new_size: winit::dpi::PhysicalSize<u32>) {
        if gfx.canvas.resize(new_size) {
            self.depth_texture = Texture::create_depth_texture(&gfx.canvas, "depth_texture");  
        }
    }

    // pub fn input(&mut self, event: &WindowEvent) -> bool {
    //     false
    // }

    pub fn update(&mut self, gfx: &mut Gfx) {
        self.camera_uniform.update_view_proj(&gfx.camera);
        gfx.canvas.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));
    }

    pub fn draw(&mut self, gfx: &mut Gfx) -> Result<(), wgpu::SurfaceError> {
        let output = gfx.canvas.surface.get_current_texture()?;
        let canvas = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let mut encoder = gfx.canvas.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &canvas,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });
            
            render_pass.set_pipeline(&self.render_pipeline);
            
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.vertex_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            
            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.instances.len() as _);
        }
    
        // submit will accept anything that implements IntoIter
        gfx.canvas.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    }
}
