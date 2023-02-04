mod sprite_loader;
mod shaders;
mod render;

use macroquad::prelude::*;
use render::*;
use sprite_loader::Sprites;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
const PIXEL_SIZE: f32 = 8.;
const SPRITE_SIZE: f32 = 32.;


fn window_conf() -> Conf {
    Conf {
        window_title: "Game".to_owned(),
        fullscreen: true,
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

#[macroquad::main(window_conf)]
async fn main() {
    
    let render_pipeline = render::build_pipeline();
	let (sprites, mut sprite_buffer) = setup_sprites(&render_pipeline);
	let mut hex_buffer = setup_tiles();
			
    'game: loop {
    	// input
    	if is_key_down(KeyCode::Escape) { break 'game; }

    	// update gui sprites
		update_sprites(&mut sprite_buffer, &sprites);
		
		// draw
		clear_background(LIGHTGRAY);
		
        //draw_sprites(&mut sprite_buffer);
        draw_hex_tiles(&mut hex_buffer);
        
        next_frame().await
    }
}

fn setup_tiles() -> DrawBuffer<HexTile> {
	let mut hex_buffer = DrawBuffer::<HexTile>::new(1000);
		
	//for x in (0..WINDOW_WIDTH).step_by(50) {
		//for y in (25..WINDOW_HEIGHT).step_by(50) {
			hex_buffer.define(
				HexTile {
					mesh: create_hex_mesh(RED),
					// pos: Vec2 { x: x as f32, y: y as f32 },
					// rot: 0.,
					// col: RED,
				}
			);
		//}
	//}
	hex_buffer
}

// sprite gui interface
fn setup_sprites(render_pipeline: &PipelineParams) -> (Sprites, DrawBuffer<Sprite>)  {
	let mut sprite_buffer = DrawBuffer::<Sprite>::new(1000);
	let sprites = sprite_loader::create_sprite_objects(&mut sprite_buffer, render_pipeline);
	
	// preset textures
    {
   		let grass_obj = sprite_buffer.edit(&sprites.grass);
   		grass_obj.pos.x = 100.;
   		grass_obj.pos.y = 50.;
	}
	(sprites, sprite_buffer)
}

fn update_sprites(sprite_buffer: &mut DrawBuffer<Sprite>, sprites: &Sprites) {
	let time = macroquad::time::get_time() as f32;
	    	
   	let x1 = 100. + time.cos() * 50.;
   	let y1 = 50. + time.sin() * 50.;
   	sprite_buffer.edit(&sprites.wall).pos = Vec2::new(x1, y1);

   	let x2 = x1 + (time*2.5).cos() * 30.;
   	let y2 = y1 + (time*2.5).sin() * 30.;
   	sprite_buffer.edit(&sprites.wall2).pos = Vec2::new(x2, y2);

	sprite_buffer.stage(&sprites.grass);
	sprite_buffer.stage(&sprites.wall);
	sprite_buffer.stage(&sprites.wall2);
}
