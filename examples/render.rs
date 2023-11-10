use perspective::prelude::*;


fn main() -> anyhow::Result<()> {
    PerspectiveBuilder::new()
        .set_window_size(PhysicalSize::new(1600, 1200))
        .set_camera(CameraSetup::default())
        .run::<RenderExample>()
}

pub struct RenderExample {
    log_counter: u8,
    frame_tot: f32,

    megamans: Vec<SpriteInstanceID>,
    cats: Vec<SpriteInstanceID>,
}

impl PerspectiveHandler for RenderExample {

    fn setup(mut sys: PerspectiveSystem) -> Self {
        
        let cat_sprite = sys.load_texture(
            include_bytes!("textures/cat-sprite.png"), 
            (0.5, 0.5),
            Some(SpritePoolSettings {
                image_size: (0, 0),
                tile_size: (0.5, 0.5),
                temp_offset: -0.5,
                max_pool_size: 100_000,
                ..Default::default()
            })
        );

        let mageman_sprite = sys.load_texture(
            include_bytes!("textures/megaman_running.png"), 
            (0.2, 0.5),
            Some(SpritePoolSettings {
                image_size: (0, 0),
                tile_size: (0.2, 0.5),
                animation_frames: vec![
                    [0.0, 0.0], [0.2, 0.0], [0.4, 0.0], [0.6, 0.0], [0.8, 0.0],
                    [0.0, 0.5], [0.2, 0.5], [0.4, 0.5], [0.6, 0.5], [0.8, 0.5],
                ],
                temp_offset: 0.0,
                max_pool_size: 100_000,
            })
        );

        let mut megamans = Vec::new();
        for _ci in 0..100_000 {
            megamans.push(sys.spawn_sprite(
                &mageman_sprite,
                Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                Quaternion::from_axis_angle(Vector3::unit_z(), Deg(0.0))
            ));
        }

        let mut cats = Vec::new();
        for _ci in 0..100_000 {
            cats.push(sys.spawn_sprite(
                &cat_sprite,
                Vector3 { x: 0.0, y: 0.0, z: 0.0 },
                Quaternion::from_axis_angle(Vector3::unit_z(), Deg(0.0))
            ));
        }

        RenderExample { log_counter: 0, frame_tot: 0.0, megamans, cats }
    }

    fn input(&mut self, _gx: &mut WgpuCore, _event: &WindowEvent) -> bool { 
        false 
    }

    fn update(&mut self, mut sys: PerspectiveSystem) {//_gx: &mut WgpuCore, px: &mut Perspective) {
        sys.set_camera_position(0.0, 5.0, 95.0);
        sys.camera().target.y = -70.0;

        self.frame_tot += sys.timer().frame_delta();

        sys.rnd.ambient_light.uniform.direction[0] = 0.0;
        sys.rnd.ambient_light.uniform.direction[1] = 0.0;
        sys.rnd.ambient_light.uniform.direction[2] = 1.0;

        let time_elapsed = sys.timer().average_step_time() as f32 / 100_000_000.0;

        let mut offset = 0.0;
        for cat_instance in &self.cats {
            let co_time = (offset + time_elapsed).cos();
            let si_time = (offset + time_elapsed).sin();

            let xpos = 0.0 + co_time * (1.0 + offset * 0.01);
            let ypos = -5.0 + si_time * (1.0 + offset * 0.01);

            let cat = sys.get_instance(cat_instance); //self.renderer.get_sprite(cat_instance);
            cat.position = Vector3 { x: xpos, y: 0.0, z: ypos };

            offset += 0.1;
        }

        let time_elapsed = sys.timer().average_step_time() as f32 / 75_000_000.0;

        offset = 0.0;
        for megaman_instance in &self.megamans {
            let co_time = (offset + time_elapsed).cos();
            let si_time = (offset + time_elapsed).sin();

            let xpos = 0.0 + co_time * (1.0 + offset * 0.01);
            let ypos = -5.0 + si_time * (1.0 + offset * 0.01);

            let megaman = sys.get_instance(megaman_instance);
            megaman.position = Vector3 { x: -xpos, y: 0.0, z: ypos };

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
}