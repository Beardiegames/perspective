mod config;
mod sprite_loader;
mod shaders;
mod render;
mod types;
mod map;

use config::*;
use macroquad::prelude::*;
use render::*;
use sprite_loader::*;
use map::*;
use types::*;


#[macroquad::main(window_conf)]
async fn main() -> Result<(), MapError> {

	let mut gui = Gui::<SpritePointers>::new();
	// let map = Map::new(100, 100)?;
	// map.write_to_file("./assets/maps/world.toml")?;
	// return Ok(());
	let map = Map::read_from_file("./assets/maps/world.toml")?;
	let mut scene = Scene::new(map);
			
    'game: loop {
    	// input
    	if is_key_down(KeyCode::Escape) { break 'game; }

    	// update gui sprites
		update_scene(&mut scene);
		update_sprites(&mut gui);

		// pre-draw update
		scene.update_floor_tiles();
		
		// draw
		clear_background(LIGHTGRAY);        
        scene.draw();
        gui.draw();
        
        next_frame().await
    }
    Ok(())
}

fn update_scene(scene: &mut Scene) {
	let time = macroquad::time::get_time() as f32;

	scene.camera.set_position(
		&TilePos::new(
			32.0 + (time * 0.4).sin() * 44.0,
			35.0 + (time * 0.4).cos() * 36.0,
		)
	);
}

fn update_sprites(gui: &mut Gui<SpritePointers>) {
	let time = macroquad::time::get_time() as f32;
	    	
   	let x1 = 100. + time.cos() * 50.;
   	let y1 = 50. + time.sin() * 50.;
   	gui.sprites.edit(&gui.customs.wall1).pos = Vec2::new(x1, y1);

   	let x2 = x1 + (time*2.5).cos() * 30.;
   	let y2 = y1 + (time*2.5).sin() * 30.;
   	gui.sprites.edit(&gui.customs.wall2).pos = Vec2::new(x2, y2);

	gui.sprites.stage(&gui.customs.grass);
	gui.sprites.stage(&gui.customs.wall1);
	gui.sprites.stage(&gui.customs.wall2);
}
