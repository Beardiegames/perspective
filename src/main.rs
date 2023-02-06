mod config;
mod sprite_loader;
mod shaders;
mod render;

use config::*;
use macroquad::prelude::*;
use render::*;
use sprite_loader::*;

#[macroquad::main(window_conf)]
async fn main() {

	let mut gui = Gui::<SpritePointers>::new();
	let mut scene = Scene::new();
	//let mut map = create_map();
			
    'game: loop {
    	// input
    	if is_key_down(KeyCode::Escape) { break 'game; }

    	// update gui sprites
		update_scene(&mut scene);
		update_sprites(&mut gui);

		// pre-draw update
		scene.move_tiles_into_view();
		
		// draw
		clear_background(LIGHTGRAY);        
        scene.draw();
        gui.draw();
        
        next_frame().await
    }
}

fn update_scene(scene: &mut Scene) {
	let time = macroquad::time::get_time() as f32;

	scene.camera.position.z = (time * 0.6).sin() * 10.0;
	scene.camera.target.z = (time * 0.6).sin() * 10.0;
}

fn create_map() -> [[f32; 20]; 100] {
	let mut map = [[0.0; 20]; 100];
	//let perlin = Perlin::new(0u32);

	// for x in 0..100 {
		// for y in 0..20 {
					// 
		// }
	// }
	map
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
