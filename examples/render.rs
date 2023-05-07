use perspective::*;


pub struct RenderExample {
    render: RenderProcessor,
}


impl PerspectiveHandler for RenderExample {

    fn startup(gx: &mut WgpuCore) -> Self {

        let render = gx.setup_render_processor(
            &RenderSettings {
                label: "RenderExample", 
                group_index: 0,// represented within shader as @binding
                binding_index: 0,// represented within shader as @binding
    
                shader_src: include_str!("shaders/basic_shader.wgsl"),
                vertex_entry_point: "vertex_main",
                fragment_entry_point: "fragment_main",

                image_data: include_bytes!("textures/cat-sprite.png"),
            }
        );

        RenderExample { render }
    }

    fn update(&mut self, gx: &mut WgpuCore) {
        self.render.vertex_buffer.slice(..);
    }

    #[allow(unused)]
    fn render_pipeline(&mut self, gx: &WgpuCore, mut ctx: RenderContext) {
        {
            let mut render_pass = ctx.begin_render_pass();

            render_pass.set_pipeline(&self.render.pipeline);
            render_pass.set_bind_group(0, &self.render.texture_pack.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.render.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.render.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..self.render.num_indices, 0, 0..1);
        }

        gx.queue.submit(std::iter::once(ctx.encoder.finish()));
        ctx.output.present();
    }
}

fn main() -> anyhow::Result<()> {
    Perspective::new(1600, 1200)
        .run::<RenderExample>()
}