use perspective::*;


fn main() -> anyhow::Result<()> {
    let window_size = PhysicalSize::new(1600, 1200);
    
    Perspective::new(window_size).run::<RenderExample>()
}


pub struct RenderExample {
    renderer: Renderer,
    
    log_counter: u8,
    frame_tot: f32,
}

impl PerspectiveHandler for RenderExample {

    fn startup(gx: &mut WgpuCore) -> Self {

        //let tex_bind = gx.create_texture_binding(include_bytes!("textures/cat-sprite.png"));

        let mut textures = TexturePack::new();
        let texture_id = textures.load(
            &gx.device, 
            &gx.queue, 
            include_bytes!("textures/cat-sprite.png"), 
            (0.5, 0.5)
        );

        let renderer = gx.setup_render_processor(
            textures,
            CameraSetup::default(),
            SpritePoolSetup {
                custom_shader: None,
                max_pool_size: 100_000,
                texture_id,
                image_size: (0, 0),
                tile_size: (0.5, 0.5)
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