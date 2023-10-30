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
    cats: Vec<SpriteInstanceID>,
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
                max_pool_size: 100_000,
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
            cgmath::Vector3 { x: 0.0, y: 0.0, z: -5.0 },
            cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
        );

        let mut cats = Vec::new();
        for _ci in 0..100_000 {
            let cat = renderer.spawn_sprite(
                &cat_sprite_id,
                cgmath::Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
            );
            cats.push(cat);
        }

        RenderExample { renderer, log_counter: 0, frame_tot: 0.0, megaman, cats }
    }

    fn input(&mut self, gx: &mut WgpuCore, event: &WindowEvent) -> bool { 
        false 
    }

    fn update(&mut self, _gx: &mut WgpuCore, px: &mut Perspective) {
        self.renderer.camera.eye.x = ((px.timer.elapsed() as f32) / 5_000_000.0).sin();
        self.renderer.camera.eye.y = 10.0; 
        self.renderer.camera.eye.z = 48.0; 
        self.renderer.camera.target.y = -30.0;

        self.frame_tot += px.timer.frame_delta();

        self.renderer.ambient_light.uniform.direction[0] = 0.0;
        self.renderer.ambient_light.uniform.direction[1] = 0.0;
        self.renderer.ambient_light.uniform.direction[2] = 1.0;

        let mut offset = 0.0;
        for cat_instance in &self.cats {
            let xpos = 0.0 + (offset + (px.timer.elapsed() as f32) / 10_000_000.0).cos() * (1.0 + offset * 0.01);
            let ypos = -5.0 + (offset + (px.timer.elapsed() as f32) / 10_000_000.0).sin() * (1.0 + offset * 0.01);

            let cat = self.renderer.get_sprite(cat_instance);
            cat.position = cgmath::Vector3{ 
                x: xpos, 
                y: 0.0, 
                z: ypos
            };
            offset += 0.1;
        }

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