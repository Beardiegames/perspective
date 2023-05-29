use perspective::*;


fn main() -> anyhow::Result<()> {
    let window_size = PhysicalSize::new(1600, 1200);
    
    Perspective::new(window_size).run::<RenderExample>()
}


pub struct RenderExample {
    renderer: RenderProcessor,
    
    log_counter: u8,
    frame_tot: f32,
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
                camera_setup: CameraSetup::default(),
            }
        ).unwrap();

        RenderExample { renderer, log_counter: 0, frame_tot: 0.0 }
    }

    fn update(&mut self, _gx: &mut WgpuCore, px: &mut Perspective) {
        self.renderer.camera.eye.x = ((px.timer.elapsed() as f32) / 5_000_000.0).sin();
        self.frame_tot += px.timer.frame_delta();

        self.renderer.light.uniform.position[0] = 0.0 + ((px.timer.elapsed() as f32) / 500_000.0).cos() * 4.0;
        self.renderer.light.uniform.position[2] = -3.0 + ((px.timer.elapsed() as f32) / 500_000.0).sin() * 4.0;

        if self.log_counter == 255 {
            println!("frame_delta: {:?} secs", self.frame_tot / 256.0);
            self.frame_tot = 0.0;
            self.log_counter = 0;
        }
        else {
            self.log_counter += 1;
        }
    }

    fn render_pipeline(&mut self, ctx: RenderContext) { 
        self.renderer.execute_render_pipeline(ctx);
    }
}