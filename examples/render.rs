use perspective::prelude::*;
use cgmath::Vector3;

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

impl Perspective for RenderExample {

    fn setup(mut ctl: ControlPanel) -> Self {
        
        let cat_sprite = ctl.create_sprite_pool(
            include_bytes!("textures/cat-sprite.png"),
            Some(SpritePoolSettings { image_aspect: 0.75, tile_size: (0.5, 0.5), ..Default::default()})
        );
        let mageman_sprite = ctl.create_sprite_pool(
            include_bytes!("textures/megaman_running.png"),
            Some(SpritePoolSettings {
                image_aspect: 1.25,
                tile_size: (0.2, 0.5),
                animation_frames: vec![
                    [0.0, 0.0], [0.2, 0.0], [0.4, 0.0], [0.6, 0.0], [0.8, 0.0],
                    [0.0, 0.5], [0.2, 0.5], [0.4, 0.5], [0.6, 0.5], [0.8, 0.5],
                ],
                max_pool_size: 100_000
            })
        );

        let mut megamans = Vec::new();
        let position = cgmath::Vector3::zero();
        for _ci in 0..50_000 {
            megamans.push(ctl.spawn_sprite(&mageman_sprite, position));
        }

        let mut cats = Vec::new();
        let rotation = cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0));
        let scale = 1.0;
        for _ci in 0..50_000 {
            cats.push(ctl.spawn_sprite_transform(&cat_sprite, position, rotation, scale));
        }

        ctl.set_camera_position(0.0, 2.5, 47.0);
        ctl.draw.camera.target.y = -30.0;

        ctl.draw.light.point_lights.push(
            PointLight::from(
                PointLightSetup {
                    position: Vector3::new(0.0, 0.0, 46.0),
                    color: [0.0, 1.0, 0.0, 0.0],
                    power: 4.0
                }
            )
        );
        ctl.draw.light.point_lights.push(
            PointLight::from(
                PointLightSetup {
                    position: Vector3::new(4.0, 0.0, 44.0),
                    color: [0.0, 0.0, 1.0, 0.0],
                    power: 6.0
                }
            )
        );
        ctl.draw.light.point_lights.push(
            PointLight::from(
                PointLightSetup {
                    position: Vector3::new(-4.0, 0.0, 45.0),
                    color: [1.0, 0.0, 0.0, 0.0],
                    power: 6.0
                }
            )
        );

        RenderExample { log_counter: 0, frame_tot: 0.0, megamans, cats }
    }

    fn input(&mut self, mut _ctl: ControlPanel, _event: &WindowEvent) {}

    fn update(&mut self, mut ctl: ControlPanel) {
        self.frame_tot += ctl.gfx.timer.frame_delta();

        let time_elapsed = ctl.gfx.timer.average_step_time() as f32 / 100_000_000.0;

        let mut offset = 0.0;
        for cat_instance in &self.cats {
            let co_time = (offset + time_elapsed).cos();
            let si_time = (offset + time_elapsed).sin();

            let xpos = 0.0 + co_time * (1.0 + offset * 0.01);
            let zpos = -5.0 + si_time * (1.0 + offset * 0.01);

            let cat = ctl.get_instance(cat_instance); //self.renderer.get_sprite(cat_instance);
            cat.position = cgmath::Vector3 { x: xpos, y: -0.2, z: zpos };

            offset += 0.1;
        }

        let time_elapsed = ctl.gfx.timer.average_step_time() as f32 / 75_000_000.0;

        offset = 0.0;
        for megaman_instance in &self.megamans {
            let co_time = (offset + time_elapsed).cos();
            let si_time = (offset + time_elapsed).sin();

            let xpos = 0.0 + co_time * (1.0 + offset * 0.01);
            let zpos = -5.0 + si_time * (1.0 + offset * 0.01);

            let megaman = ctl.get_instance(megaman_instance);
            megaman.position = cgmath::Vector3 { x: -xpos, y: 0.0, z: zpos };

            offset += 0.1;
        }

        let cp = 0.15 + (ctl.gfx.timer.average_step_time() as f32 / 2_000_000.0).sin() * 0.1;
        ctl.draw.light.ambient.color = [cp, cp, cp, 1.0];

        let mut time_elapsed = ctl.gfx.timer.average_step_time() as f32 / 500_000.0;

        ctl.draw.light.point_lights[0].position[0] = time_elapsed.cos() * 5.0;
        ctl.draw.light.point_lights[0].position[2] = 42.0 + time_elapsed.sin() * 5.0;

        time_elapsed += 2.1;
        ctl.draw.light.point_lights[1].position[0] = time_elapsed.cos() * 5.0;
        ctl.draw.light.point_lights[1].position[2] = 42.0 + time_elapsed.sin() * 5.0;

        time_elapsed += 2.1;
        ctl.draw.light.point_lights[2].position[0] = time_elapsed.cos() * 5.0;
        ctl.draw.light.point_lights[2].position[2] = 42.0 + time_elapsed.sin() * 5.0;

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