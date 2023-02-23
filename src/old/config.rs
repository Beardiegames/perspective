use macroquad::prelude::*;

pub const WINDOW_WIDTH: i32 = 800;
pub const WINDOW_HEIGHT: i32 = 600;
pub const PIXEL_SIZE: f32 = 3.;
pub const SPRITE_SIZE: f32 = 32.;


pub fn window_conf() -> Conf {
    Conf {
        window_title: "Game".to_owned(),
        fullscreen: false,
        window_resizable: false,
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        sample_count: 1,
        high_dpi: true,
        icon: None,
        platform: miniquad::conf::Platform {
            framebuffer_alpha: true,
            ..Default::default()
        }
    }
}
