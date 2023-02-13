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


pub trait PerspectiveHandler {
	fn update_scene(&mut self, scene: &mut Scene);
	fn update_gui(&mut self, gui: &mut Gui<SpritePointers>);
}


#[macroquad::main(window_conf)]
async fn main() -> Result<(), MapError> {

	let mut engine = Perspective::new()?;
	engine.run(Game::new()).await
}


pub struct Perspective {
	pub gui: Gui::<SpritePointers>,
	pub scene: Scene,
}

impl Perspective {

	pub fn new() -> Result<Perspective, MapError> {
	
		let gui = Gui::<SpritePointers>::new();
		
		// let map = Map::new(142, 230)?;
		// map.write_to_file("./assets/maps/world.toml")?;
		// return Err(MapError::GenericError);
		
		let map = Map::read_from_file("./assets/maps/world.toml")?;
		let scene = Scene::new(map);
		
		Ok(Perspective { gui, scene })
	}

	pub async fn run<T>(&mut self, mut game: T) -> Result<(), MapError> 
		where T: PerspectiveHandler
	{
	    'game: loop {
	    	// input
	    	if is_key_down(KeyCode::Escape) { break 'game; }

	    	// update gui sprites
			game.update_scene(&mut self.scene);
			game.update_gui(&mut self.gui);

			// pre-draw update
			self.scene.update_floor_tiles();
			
			// draw
			clear_background(LIGHTGRAY);        
	        self.scene.draw();
	        self.gui.draw();
	        
	        next_frame().await
	    }
	    Ok(())
	}
}

pub struct Game {
	time: f32,
}

impl Game {
	pub fn new() -> Self {
		Self {
			time: 0.0,
		}
	}
}

impl PerspectiveHandler for Game {

	fn update_scene(&mut self, scene: &mut Scene) {
		let new_time = macroquad::time::get_time() as f32;
		let delta_time = new_time - self.time;
		self.time = new_time;
		
		//scene.camera.set_zoom(2.0 + (time * 0.5).sin() * 2.0);
		scene.camera.set_zoom(100.0);
		
		// scene.camera.set_position(
			// &TilePos::new(
				// 32.0 + (time * 0.5).sin() * 44.0,
				// 35.0 + (time * 0.5).cos() * 36.0,
			// )
		// );

		let mut cam_pos = scene.camera.position();

		if is_key_down(KeyCode::Right) { 
			cam_pos.hor += delta_time * 10.0;
		}
		else if is_key_down(KeyCode::Left) { 
			cam_pos.hor -= delta_time * 10.0	
		}
		if is_key_down(KeyCode::Up) { 
			cam_pos.ver += delta_time * 10.0
		}
		else if is_key_down(KeyCode::Down) { 
			cam_pos.ver -= delta_time * 10.0	
		}
		scene.camera.set_position(&cam_pos);
	}

	fn update_gui(&mut self, gui: &mut Gui<SpritePointers>) {
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
}
