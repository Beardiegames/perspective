use perspective::*;


pub struct RenderExample {
    render_processor: RenderProcessor,
}


impl PerspectiveHandler for RenderExample {

    fn startup(gfx: &mut WgpuCore) -> Self {

        let render_processor = gfx.setup_render_processor(
            &RenderSettings {
                label: "RenderExample", 
                group_index: 0,// represented within shader as @binding
                binding_index: 0,// represented within shader as @binding
    
                shader_src: include_str!("shaders/basic_shader.wgsl"),
                vertex_entry_point: "vertex_main",
                fragment_entry_point: "fragment_main",
            }
        );

        RenderExample { render_processor }
    }

    fn render_pipeline(&mut self, gx: &WgpuCore, mut encoder: wgpu::CommandEncoder) -> Result<(), wgpu::SurfaceError> {

        if let Some(c) = &gx.canvas {
            let output = c.surface.get_current_texture()?;
            let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
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

                render_pass.set_pipeline(&self.render_processor.pipeline);
                render_pass.set_vertex_buffer(0, self.render_processor.vertex_buffer.slice(..));
                render_pass.set_index_buffer(self.render_processor.index_buffer.slice(..), wgpu::IndexFormat::Uint16); // 1.
                render_pass.draw_indexed(0..self.render_processor.num_indices, 0, 0..1); // 2.
            }

            gx.queue.submit(std::iter::once(encoder.finish()));

            output.present();
        }
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {

    Perspective::new(800, 600)
        .run::<RenderExample>()?;
    
    Ok(())    
}