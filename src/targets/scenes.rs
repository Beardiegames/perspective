use macroquad::prelude::*;
use crate::position::*;
use crate::render::*;
use crate::map::*;
use crate::drawables::HexTile;


pub const GRID_WIDTH: i16 = 17; // NOTE: must be odd numbers!!!
pub const GRID_HEIGHT: i16 = 35; // NOTE: must be odd numbers!!!


pub struct Light {
	pub pos: TilePos,
	pub col: Color,
	pub range: f32, 
}

impl Light {
	pub fn add_to_color(&self, color: &mut Color, amount: f32) {
		color.r += (self.col.r * amount).clamp(0.0, 1.0);
		color.g += (self.col.g * amount).clamp(0.0, 1.0);
		color.b += (self.col.b * amount).clamp(0.0, 1.0);
	}
}


pub struct Scene {
	pub camera: CameraController,
	pub map: Map,
	pub lights: Vec<Light>,

	tiles: DrawBuffer<HexTile>,
	map_offset: TilePos,
}

impl Scene {
	pub fn new(map: Map) -> Self {
		Scene {
			camera: CameraController::new(),
		    map,
		    lights: Vec::new(),
		    tiles: setup_tiles(),
		    map_offset: TilePos { hor: 0.0, ver: 0.0 }
		}	
	}

	pub fn update_floor_tiles(&mut self) {
		self.map_offset = self.camera.position();
		self.map_offset.hor = self.map_offset.hor.round();
		self.map_offset.ver = (self.map_offset.ver * 0.5).round() * 2.0;

		for idx in 0..self.tiles.number_of_prefabs() {
			let mut tile_changed = false;
			
			if let Some(tile) = self.tiles.edit_prefab_at(&idx) {
			
				tile.offset_position = self.map_offset.clone();
				let mx_pos = tile.get_matrix_position();
				
				if let Some(map_value) = self.map.get_at_mx(&mx_pos) {

					// update lights & terrain color
					let mut map_color = map_value.color();
					
					for light in &self.lights {
						let dist_to_tile = tile.world_position().distance(&light.pos);
						
						if dist_to_tile < light.range {
							let p_range = light.range.powi(2);
							let amount = (1.0 / p_range) * (p_range - dist_to_tile.powi(2));
							light.add_to_color(&mut map_color, amount);
						}
					}
					tile.color = map_color;	

					// update terrain height
					tile.height = map_value.height;
				}

				for adj in self.map.adjacent_to(&mx_pos) {
					
				}
				
				tile.update_mesh();		 	
				tile_changed = true;

				
			}
			if tile_changed { 
				self.tiles.stage_by_index(idx);
			}
		}
	}

	pub fn draw(&mut self) {
		draw_buffer_3d(&mut self.tiles, &self.camera);
	}
}


fn setup_tiles() -> DrawBuffer<HexTile> {
	let mut hex_buffer = DrawBuffer::<HexTile>::new((GRID_WIDTH * GRID_HEIGHT) as usize);
	
	for ver in 0..GRID_HEIGHT {
	let grid_width_fact = GRID_WIDTH + ver;

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

		hex_buffer.define(HexTile::new(mx_pos, screen_pos));
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
