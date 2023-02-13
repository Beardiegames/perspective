use macroquad::prelude::*;
//use noise::*;
use crate::types::*;


pub const CAM_OFFSET_Z: f32 = -6.0;
pub const CAM_OFFSET_X: f32 = 6.0;


pub struct CameraController{
	pub quad_cam: Camera3D,
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
	
	pub fn set_zoom(&mut self, mut to_zoom: f32) {
		if to_zoom < 0.0 { to_zoom = 0.0; }
		else if to_zoom > 10.0 { to_zoom = 10.0; }
		
		self.zoom_pos.x = -0.25 - to_zoom * 1.5;
		self.zoom_pos.y = 0.25 + to_zoom * 1.5;

		self.quad_cam.position = self.quad_cam.target + self.zoom_pos;
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
// 
	// pub fn horzontal_mut(&mut self) -> &mut f32 {
		// &mut self.quad_cam.target.x
	// }
// 
	// pub fn vertical_mut(&mut self) -> &mut f32 {
		// &mut self.quad_cam.target.x
	// }
// 
	// pub fn horzontal(&self) -> &f32 {
		// &self.quad_cam.target.x
	// }
// 
	// pub fn vertical(&self) -> &f32 {
		// &self.quad_cam.target.x
	// }
}
