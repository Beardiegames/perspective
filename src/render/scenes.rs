use macroquad::prelude::*;
//use noise::*;
use crate::types::*;
use super::*;


pub const GRID_WIDTH: i16 = 17; // NOTE: must be odd numbers!!!
pub const GRID_HEIGHT: i16 = 35; // NOTE: must be odd numbers!!!


pub struct Scene {
	pub camera: CameraController,
	pub map: Map,

	tiles: DrawBuffer<HexTile>,
	map_offset: TilePos,
}

impl Scene {
	pub fn new(map: Map) -> Self {
		Scene {
			camera: CameraController::new(),
		    map,
		    tiles: setup_tiles(),
		    map_offset: TilePos { hor: 0.0, ver: 0.0 }
		}	
	}

	pub fn update_floor_tiles(&mut self) {
		self.map_offset = self.camera.position();

		// jump out of screen tiles
		self.map_offset.hor = self.map_offset.hor.round();
		self.map_offset.ver = (self.map_offset.ver * 0.5).round() * 2.0;

		for tile in self.tiles.mut_item_list() {
			tile.offset_pos(self.map_offset.clone());
			let mx_pos = tile.get_matrix_position();
			tile.set_color(self.map.get_at_mx(&mx_pos));

			if mx_pos == MxPos::from(self.camera.position()) {
				tile.set_color(Some(RED));
			}
		}
	}
	
	pub fn draw(&mut self) {
		set_camera(&self.camera.quad_cam);
		// set_camera(
			// &Camera3D {
		        // position: vec3(CAM_OFFSET_Z, CAM_OFFSET_X, 0.0),
		        // up: vec3(0., 1., 0.),
		        // target: vec3(0., 0., 0.),
		        // //projection: Projection::Orthographics,
		        // //fovy: 10.0,
		        // ..Default::default()
		    // }
		// );
	    
	    gl_use_default_material();
	    //draw_grid(20, 1., BLACK, GRAY);
	    
		for item in self.tiles.item_list() {
			if let Some(_c) = item.color() {
				draw_mesh(&item.mesh);
			}
			
			// let pos = item.position();
			// draw_text(text: &str, x: f32, y: f32, font_size: f32, color: Color)
	    }
	    self.tiles.clear_queue();
	}
}


fn setup_tiles() -> DrawBuffer<HexTile> {
	let mut hex_buffer = DrawBuffer::<HexTile>::new((GRID_WIDTH * GRID_HEIGHT) as usize);
	
	for ver in 0..GRID_HEIGHT {
	let grid_width_fact = GRID_WIDTH + ver;

	// let mx_offset = match (ver + 1) % 2 == 1 {
		// true => 0.0,
		// false => -0.5,
	// };
	
	for hor in 0..grid_width_fact {

		let row_offset = -0.5;

		let hor_offset = grid_width_fact as f32 / 2.0 + row_offset;
		let ver_offset = GRID_HEIGHT as f32 / 4.5;
		
		let screen_pos = TilePos {
			hor: hor as f32 - hor_offset,
			ver: ver as f32 - ver_offset,
		};

		let mx_pos = MxPos::new( 
			//(hor + (GRID_HEIGHT - ver) / 2) - (hor_offset * 2.0).round() as i16,
			hor - hor_offset.round() as i16, 
			ver - ver_offset.round() as i16
		);

		hex_buffer.define(HexTile::new(mx_pos, screen_pos, BLACK));
	}}
	
	hex_buffer
	
	//println!("mx_pos: {:?}", mx_pos);

	// let color = Color {
		// r: hor as f32 / GRID_HEIGHT as f32,
		// g: ver as f32 / grid_width_fact as f32,
		// b: 0.5,
		// a: 1.0,
	// };
}
