use perspective::*;


fn main() -> anyhow::Result<()> {
    let window_size = PhysicalSize::new(1600, 1200);
    let camera_setup = CameraSetup::default();

    Perspective::new(window_size, camera_setup)
        .run::<RenderExample>()
}


pub struct RenderExample {
    renderer: RenderProcessor,
    
    log_counter: u8,
    frame_tot: f64,
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

        RenderExample { renderer, log_counter: 0, frame_tot: 0.0 }
    }

    fn update(&mut self, _gx: &mut WgpuCore, px: &mut Perspective) {
        px.camera.eye.x = ((px.timer.elapsed() as f32) / 1_000_000.0).sin();
        self.frame_tot += px.timer.frame_delta();

        if self.log_counter == 255 {
            println!("frame_delta: {:?} secs", self.frame_tot / 256.0);
            self.frame_tot = 0.0;
            self.log_counter = 0;
        }
        else {
            self.log_counter += 1;
        }
    }

    fn render_pipeline(&mut self, mut ctx: RenderContext) { 
        // update uniform data 
        self.renderer.update_uniform(&ctx);

        // start render pass
        {
            let mut render_pass = ctx.begin_render_pass();

            render_pass.set_pipeline(&self.renderer.pipeline);
            render_pass.set_bind_group(0, &self.renderer.textures.bindgroup, &[]);
            render_pass.set_bind_group(1, &self.renderer.uniform.bindgroup, &[]);

            render_pass.set_vertex_buffer(0, self.renderer.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.renderer.instance_buffer.slice(..));

            render_pass.set_index_buffer(self.renderer.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..self.renderer.num_indices, 0, 0..self.renderer.instances.len() as _);
        }

        ctx.gx.queue.submit(std::iter::once(ctx.encoder.finish()));
        ctx.output.present(); 
    }
}