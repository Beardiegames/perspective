use macroquad::prelude::*;
use noise::*;
use crate::types::*;
use super::*;


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
		        position: vec3(-5., 5., 0.),
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
		self.map_offset.hor = self.camera.position.z.round() + 0.0;
		self.map_offset.ver = self.camera.position.x.round() + 5.0;

		//let mx_offset_ver = self.map_offset.ver + 8.0;
		//let tilepos_hor = &self.map_offset;
		
		for tile in self.tiles.mut_item_list() {
			tile.offset_pos(self.map_offset.clone());

			let mut mxpos = tile.get_matrix_position().clone();

			if mxpos.hor as f32 + self.map_offset.hor < 0.0
			{
				tile.set_color(BLUE);
				continue;
			}

			//if mxpos.hor as f32 + (mx_offset_hor + self.map_offset.hor) >= 0.0
			//&& mxpos.ver as f32 + self.map_offset.ver >= 0.0
			//{
				mxpos.hor += self.map_offset.hor as usize;
				mxpos.ver += self.map_offset.ver as usize;
				
				if let Some(col) = self.map.get_at_mx(&mxpos) {
					tile.set_color(col);
				} else {
					tile.set_color(RED);
				}
			//} else {
			//	tile.set_color(BLACK);
			//}
		}
	}
	
	pub fn draw(&mut self) {
		set_camera(&self.camera);
	    
	    //gl_use_default_material();
	    //draw_grid(20, 1., BLACK, GRAY);
	    
		for item in self.tiles.item_list() {
			draw_mesh(&item.mesh);
			
			// let pos = item.position();
			// draw_text(text: &str, x: f32, y: f32, font_size: f32, color: Color)
	    }
	    self.tiles.clear_queue();
	}
}


fn setup_tiles() -> DrawBuffer<HexTile> {
	let mut hex_buffer = DrawBuffer::<HexTile>::new(1000);

	let x_start: i32 = -16;
	let x_count: usize = 32;
	let y_start: i32 = -4;
	let y_count: usize = 20;
		
	for i in 0..x_count {
		for j in 0..y_count {

			let pos_x = i as i32 + x_start;
			let pos_y = j as i32 + y_start;

			let xbnd = pos_x as f32 + 0.5 * pos_y as f32;
			if xbnd > -8.0 && xbnd < 8.0 {  
				
				hex_buffer.define(
					HexTile::new(
						MxPos {
							hor: i,
							ver: j,	
						},
						TilePos {
							hor: pos_x as f32,
							ver: pos_y as f32,
						},
						//Color { r: rgb, g: rgb, b: 0.5, a: 1.0 },
						Color {
							r: (pos_x + 16) as f32 / 32.0,
							g: (pos_y + 4) as f32 / 32.0,
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
