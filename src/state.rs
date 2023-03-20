use winit::{
    event::*,
    window::Window,
    dpi::PhysicalSize,
};
use crate::shaders;
use crate::vertices;
use crate::textures;
use crate::render_pipeline;
use crate::surface::{ self, RenderSurface };


pub struct State {
    render_surface: RenderSurface,
    
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
    vertex_index_buffer: wgpu::Buffer,
    num_indices: u32,
    diffuse_bind_group: wgpu::BindGroup,
}

impl State {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: Window) -> Self {        
        let render_surface = RenderSurface::new(window).await;
            
        // load texture image
        let (texture_bind_group_layout, diffuse_bind_group) = textures::load_texture_image(&render_surface.device, &render_surface.queue);
                  
        // setup render pipeline
        let shader = shaders::create_shader(&render_surface.device);
        let render_pipeline = render_pipeline::create_new(
            &render_surface.device,
            &shader,
            render_surface.config.format,
            &[vertices::Vertex::buffer_layout(), ],
            &[&texture_bind_group_layout],
        );
        
        // setup vertex and indices buffers
        let (vertex_buffer, num_vertices) = vertices::new_vertex_buffer(&render_surface.device);
        let (vertex_index_buffer, num_indices) = vertices::new_index_buffer(&render_surface.device);
        
        Self {
            render_surface,
            render_pipeline,
            vertex_buffer,
            num_vertices,
            vertex_index_buffer,
            num_indices,
            diffuse_bind_group,
        }
    }

    pub fn window(&self) -> &Window {
        &self.render_surface.window
    }
    
    pub fn reconfigure_surface(&self) {
        self.render_surface.reconfigure_surface();
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.render_surface.resize(new_size);
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {
        //todo!()
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.render_surface.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let mut encoder = self.render_surface.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
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
                depth_stencil_attachment: None,
            });
            
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.vertex_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }
    
        // submit will accept anything that implements IntoIter
        self.render_surface.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    }
}