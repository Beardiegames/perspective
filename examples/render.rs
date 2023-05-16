use cgmath::InnerSpace;
use perspective::*;


pub struct RenderExample {
    renderer: RenderProcessor,
}


impl PerspectiveHandler for RenderExample {

    fn startup(gx: &mut WgpuCore) -> Self {

        let renderer = gx.setup_render_processor(
            &RenderSettings {
                label: "RenderExample", 
                group_index: 0,// represented within shader as @binding
                binding_index: 0,// represented within shader as @binding
    
                shader_src: include_str!("shaders/basic_shader.wgsl"),
                vertex_entry_point: "vertex_main",
                fragment_entry_point: "fragment_main",

                image_data: include_bytes!("textures/cat-sprite.png"),
            }
        ).unwrap();

        RenderExample { renderer }
    }

    fn update(&mut self, _gx: &mut WgpuCore, px: &Perspective) {
        self.renderer.camera.eye.x = (px.runtime() as f32).sin();
        println!("eye: {:?}", self.renderer.camera.eye);
    }

    #[allow(unused)]
    fn render_pipeline(&mut self, gx: &WgpuCore, mut ctx: RenderContext) {
        self.renderer.camera_uniform.update_view_proj(&self.renderer.camera);
        gx.queue.write_buffer(&self.renderer.camera_gpu_handle.buffer, 0, bytemuck::cast_slice(&[self.renderer.camera_uniform]));

        {
            let mut render_pass = ctx.begin_render_pass();

            render_pass.set_pipeline(&self.renderer.pipeline);
            render_pass.set_bind_group(0, &self.renderer.textures.bindgroup, &[]);
            render_pass.set_bind_group(1, &self.renderer.camera_gpu_handle.bindgroup, &[]);
            render_pass.set_vertex_buffer(0, self.renderer.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.renderer.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..self.renderer.num_indices, 0, 0..1);
        }

        gx.queue.submit(std::iter::once(ctx.encoder.finish()));
        ctx.output.present();
    }
}

fn main() -> anyhow::Result<()> {
    Perspective::new(1600, 1200)
        .run::<RenderExample>()
}