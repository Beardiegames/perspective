use macroquad::prelude::*;
//use noise::*;
use crate::types::*;
use super::*;


pub const CAMERA_OFFSET_Z: f32 = -5.0;
pub const CAMERA_OFFSET_X: f32 = 5.0;
pub const GRID_WIDTH: i16 = 9;
pub const GRID_HEIGHT: i16 = 21;


pub struct Scene {
	pub camera: Camera3D,
	pub map: Map,

	tiles: DrawBuffer<HexTile>,
	map_offset: TilePos,
}

impl Scene {
	pub fn new(map: Map) -> Self {
		Scene {
			camera: Camera3D {
		        position: vec3(CAMERA_OFFSET_Z, CAMERA_OFFSET_X, 0.),
		        up: vec3(0., 1., 0.),
		        target: vec3(0., 0., 0.),
		        //projection: Projection::Orthographics,
		        //fovy: 10.0,
		        ..Default::default()
		    },
		    map,
		    tiles: setup_tiles(),
		    map_offset: TilePos { hor: 0.0, ver: 0.0 }
		}	
	}

	pub fn update_floor_tiles(&mut self) {
		self.map_offset = TilePos::from_real_position(self.camera.target)
			.round();

		//let mx_offset_ver = self.map_offset.ver + 8.0;
		//let tilepos_hor = &self.map_offset;
		
		for tile in self.tiles.mut_item_list() {
			tile.offset_pos(self.map_offset.clone());

			let mx_pos = tile.get_matrix_position();
			tile.set_color(self.map.get_at_mx(&mx_pos));
			
			
// 
			// let mut mxpos = tile.get_matrix_position().clone();
// 
			// if mxpos.hor as f32 + self.map_offset.hor < 0.0
			// {
				// tile.set_color(BLUE);
				// continue;
			// }
// 
			//if mxpos.hor as f32 + (mx_offset_hor + self.map_offset.hor) >= 0.0
			//&& mxpos.ver as f32 + self.map_offset.ver >= 0.0
			//{
				// mxpos.hor += self.map_offset.hor as usize;
				// mxpos.ver += self.map_offset.ver as usize;
				// 
				// if let Some(col) = self.map.get_at_mx(&mxpos) {
					// tile.set_color(col);
				// } else {
					// tile.set_color(RED);
				// }
			//} else {
			//	tile.set_color(BLACK);
			//}
		}
	}
	
	pub fn draw(&mut self) {
		set_camera(&self.camera);
	    
	    gl_use_default_material();
	    draw_grid(20, 1., BLACK, GRAY);
	    
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
	
	for hor in 0..grid_width_fact {

		let offset = match ver % 2 == grid_width_fact % 2 { true => 0.75, false => 0.25 };
		
		let screen_pos = TilePos {
			hor: hor as f32 - grid_width_fact as f32 / 2.0 + offset,
			ver: ver as f32 - GRID_HEIGHT as f32 / 5.0,
		};

		let mx_pos = MxPos { hor: hor + (GRID_HEIGHT - ver) / 2, ver };

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
