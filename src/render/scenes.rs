use macroquad::prelude::*;
use noise::*;
use super::*;


pub struct Scene {
	pub camera: Camera3D,
	pub tiles: DrawBuffer<HexTile>,
}

impl Scene {
	pub fn new() -> Self {
		Scene {
			camera: Camera3D {
		        position: vec3(-5., 5., 0.),
		        up: vec3(0., 1., 0.),
		        target: vec3(0., 0., 0.),
		        //projection: Projection::Orthographics,
		        //fovy: 10.0,
		        ..Default::default()
		    },
		    tiles: setup_tiles(),
		}	
	}

	pub fn move_tiles_into_view(&mut self) {
		let offset = (self.camera.position.z).round();
		
		for tile in self.tiles.mut_item_list() {
			let mut pos = tile.pos().clone();
			pos.hor -= offset;

			tile.set_pos(pos);
		}
	}
	
	pub fn draw(&mut self) {
		set_camera(&self.camera);
	    
	    //gl_use_default_material();
	    //draw_grid(20, 1., BLACK, GRAY);
	    
		for item in self.tiles.item_list() {
			draw_mesh(&item.mesh);
	    }
	    self.tiles.clear_queue();
	}
}


fn setup_tiles() -> DrawBuffer<HexTile> {
	let mut hex_buffer = DrawBuffer::<HexTile>::new(1000);
	let perlin = Perlin::new(0u32);
		
	for i in -16..16 {
		for j in -4..16 {

			let xbnd = i as f32 + 0.5 * j as f32;
			if xbnd > -8.0 && xbnd < 8.0 {  

				let point: [f64; 2] = [i as f64 / 1.5, j as f64 / 1.5]; 
				let noise = perlin.get(point);
				println!("noise: {}", noise);
				let rgb = (noise * 0.03) as f32;
				
				hex_buffer.define(
					HexTile::new(
						TilePos {
							hor: i as f32,
							ver: j as f32,
						},
						//Color { r: rgb, g: rgb, b: 0.5, a: 1.0 },
						Color {
							r: (i + 16) as f32 / 32.0,
							g: (j + 4) as f32 / 32.0,
							b: 0.5 + rgb,
							a: 1.0,
						}
					)
				);
			}
		}
	}
	hex_buffer
}
