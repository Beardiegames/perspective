use perspective::prelude::*;


fn main() -> anyhow::Result<()> {
    let window_size = PhysicalSize::new(1600, 1200);
    
    Perspective::new(window_size).run_winit::<RenderExample>()
}


pub struct RenderExample {
    renderer: Renderer,
    
    log_counter: u8,
    frame_tot: f32,

    megaman: SpriteInstanceID,
    cat: SpriteInstanceID,
}

impl PerspectiveHandler for RenderExample {

    fn startup(gx: &mut WgpuCore) -> Self {

        //let tex_bind = gx.create_texture_binding(include_bytes!("textures/cat-sprite.png"));

        let mut textures = TexturePack::default();
        let cat_texture_id = textures.load(
            &gx.device, 
            &gx.queue, 
            include_bytes!("textures/cat-sprite.png"), 
            (0.5, 0.5)
        );

        let mageman_texture_id = textures.load(
            &gx.device, 
            &gx.queue, 
            include_bytes!("textures/megaman_running.png"), 
            (0.2, 0.5)
        );

        let mut renderer = gx.setup_render_processor(&CameraSetup::default(),textures);

        let cat_sprite_id = renderer.create_sprite_pool(
            gx,
            &SpritePoolSetup {
                texture_id: cat_texture_id,
                image_size: (0, 0),
                tile_size: (0.5, 0.5),
                temp_offset: -0.5,
                ..Default::default()
            }
        );

        let megaman_sprite_id = renderer.create_sprite_pool(
            gx,
            &SpritePoolSetup {
                texture_id: mageman_texture_id.clone(),
                image_size: (0, 0),
                tile_size: (0.2, 0.5),
                animation_frames: vec![
                    [0.0, 0.0], [0.2, 0.0], [0.4, 0.0], [0.6, 0.0], [0.8, 0.0],
                    [0.0, 0.5], [0.2, 0.5], [0.4, 0.5], [0.6, 0.5], [0.8, 0.5],
                ],
                temp_offset: 0.0,
                ..Default::default()
            });

        let megaman = renderer.spawn_sprite(
            &megaman_sprite_id,
            cgmath::Vector3 { x: 1.0, y: 0.0, z: 0.0 },
            cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
        );

        let cat = renderer.spawn_sprite(
            &cat_sprite_id,
            cgmath::Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
        );

        RenderExample { renderer, log_counter: 0, frame_tot: 0.0, megaman, cat }
    }

    fn update(&mut self, _gx: &mut WgpuCore, px: &mut Perspective) {
        self.renderer.camera.eye.x = ((px.timer.elapsed() as f32) / 5_000_000.0).sin();
        self.frame_tot += px.timer.frame_delta();

        let xpos = 0.0 + ((px.timer.elapsed() as f32) / 500_000.0).cos() * 4.0;
        let ypos = -3.0 + ((px.timer.elapsed() as f32) / 500_000.0).sin() * 4.0;

        self.renderer.light.uniform.position[0] = xpos;
        self.renderer.light.uniform.position[2] = ypos;

        let cat = self.renderer.get_sprite(&self.cat);
            cat.position = cgmath::Vector3{ 
                x: xpos, 
                y: 0.0, 
                z: ypos
            };

        if self.log_counter == 255 {
            println!("frame_delta: {:?} secs", self.frame_tot / 256.0);
            self.frame_tot = 0.0;
            self.log_counter = 0;
        }
        else {
            self.log_counter += 1;
        }
    }

    fn draw(&mut self, ctx: RenderContext) { 
        self.renderer.execute_render_pipeline(ctx);
    }
}