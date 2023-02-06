mod config;
mod sprite_loader;
mod shaders;
mod render;

use config::*;
use macroquad::prelude::*;
use render::*;
use sprite_loader::Sprites;

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
		
        
        draw_hex_tiles(&mut hex_buffer);
        draw_sprites(&mut sprite_buffer);
        
        next_frame().await
    }
}

fn setup_tiles() -> DrawBuffer<HexTile> {
	let mut hex_buffer = DrawBuffer::<HexTile>::new(1000);
		
	for i in -16..16 {
		for j in -4..16 {

			let xbnd = i as f32 + 0.5 * j as f32;
			if xbnd > -8.0 && xbnd < 8.0 {  
		
				hex_buffer.define(
					HexTile::new(
						TilePos {
							hor: i as f32,
							ver: j as f32,
						},
						Color { 
							r: (i + 10) as f32 / 32.0,
							g: (j + 10) as f32 / 32.0,
							b: 0.5,
							a: 1.0,
						}
					)
				);
			}
		}
	}
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
