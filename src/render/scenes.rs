use macroquad::prelude::*;
//use noise::*;
use crate::types::*;
use super::*;


pub const CAM_OFFSET_Z: f32 = -5.0;
pub const CAM_OFFSET_X: f32 = 5.0;
pub const GRID_WIDTH: i16 = 9;
pub const GRID_HEIGHT: i16 = 21;

pub struct CameraController{
	quad_cam: Camera3D,
	zoom_pos: Vec3,
}

impl CameraController{
	pub fn new() -> Self {
		let zoom_pos = Vec3::new(CAM_OFFSET_Z, CAM_OFFSET_X, 0.0);
	
		Self {
			quad_cam: Camera3D {
		        position: zoom_pos,
		        up: vec3(0., 1., 0.),
		        target: vec3(0., 0., 0.),
		        //projection: Projection::Orthographics,
		        //fovy: 10.0,
		        ..Default::default()
		    },
		    zoom_pos
		}
	}

	pub fn zoom(&self) -> &f32 {
		&self.zoom_pos.y
	}
	
	pub fn set_zoom(&mut self, to_zoom: f32) {
		self.zoom_pos.x = -to_zoom;
		self.zoom_pos.y = to_zoom;
	}

	pub fn position(&mut self) -> TilePos {
		TilePos::from(self.real_position())
	}

	pub fn real_position(&mut self) -> RealPos {
		RealPos::from(self.quad_cam.target)
	}

	pub fn set_position(&mut self, pos: &TilePos) {
		self.set_real_position(&RealPos::from(pos))
	}

	pub fn set_real_position(&mut self, pos: &RealPos) {
		let target = Vec3::from(pos.clone());
		self.quad_cam.target = target;
		
		let vec_pos = Vec3::from(pos.clone()) + self.zoom_pos;
		self.quad_cam.position = vec_pos;
	}

	pub fn horzontal_mut(&mut self) -> &mut f32 {
		&mut self.quad_cam.target.x
	}

	pub fn vertical_mut(&mut self) -> &mut f32 {
		&mut self.quad_cam.target.x
	}

	pub fn horzontal(&self) -> &f32 {
		&self.quad_cam.target.x
	}

	pub fn vertical(&self) -> &f32 {
		&self.quad_cam.target.x
	}
}


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
