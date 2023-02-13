use macroquad::prelude::*;
use macroquad::models::Vertex;
use crate::types::*;


pub struct HexTile {
	pub mesh: Mesh,
	root: Mesh,
	mx_pos: MxPos,
	screen_pos: TilePos,
	offset: TilePos,
	col: Option<Color>,
}

impl HexTile {
	pub fn new(mx_pos: MxPos, screen_pos: TilePos, col: Color) -> Self {		
		let mut hex = HexTile {
			mesh: create_hex_mesh(col),
			root: create_hex_mesh(col),
			mx_pos,
			screen_pos,
			offset: TilePos::ZERO,
			col: None,
		};
		hex.offset_pos(TilePos::ZERO);
		hex
	}

	pub fn get_matrix_position(&self) -> MxPos {
		self.mx_pos.clone() + MxPos::from(&self.offset)
	}
	
	pub fn offset_pos(&mut self, offset: TilePos) {
		self.offset = offset;
		let real = Vec3::from(RealPos::from(self.position()));
		
		for i in 0..6 {
			let vx = self.root.vertices[i].position.z + real.z;
			let vy = self.root.vertices[i].position.x + real.x;
			self.mesh.vertices[i].position.z = vx;
			self.mesh.vertices[i].position.x = vy;
		}
	}

	pub fn position(&self) -> TilePos {
		&self.screen_pos + &self.offset
	}

	pub fn set_color(&mut self, col: Option<Color>) {
		self.col = col;

		if let Some(c) = col {
			for i in 0..6 {	
				self.mesh.vertices[i].color = c;
			}
		}
	}
	
	pub fn color(&self) -> &Option<Color> {
		&self.col
	}
}

pub fn create_hex_mesh(color: Color) -> Mesh {
	let w: f32 = 2.0;
	let h: f32 = 2.0; //1.155 * w;
	
	let hor = 0.5 * h;
	let ver = 0.5 * w;
	let rib = 0.25 * h;

	// NOTE: positions are => (Y,Z,-X) or (away,up,left)
	let (vp0, uv0) = (Vec3::new(hor, 0.0, 0.0), 	Vec2::new(0.5, 0.0));
	let (vp1, uv1) = (Vec3::new(rib, 0.0, ver), 	Vec2::new(1.0, 0.0));
	let (vp2, uv2) = (Vec3::new(-rib, 0.0, ver), 	Vec2::new(1.0, 0.0));
	let (vp3, uv3) = (Vec3::new(-hor, 0.0, 0.0), 	Vec2::new(0.5, 1.0));
	let (vp4, uv4) = (Vec3::new(-rib, 0.0, -ver), 	Vec2::new(0.0, 1.0));
	let (vp5, uv5) = (Vec3::new(rib, 0.0, -ver), 	Vec2::new(0.0, 1.0));

	Mesh {
	    vertices: vec![
	    	Vertex { position: vp0, uv: uv0, color },
	    	Vertex { position: vp1, uv: uv1, color },
	    	Vertex { position: vp2, uv: uv2, color },
	    	Vertex { position: vp3, uv: uv3, color },
	    	Vertex { position: vp4, uv: uv4, color },
	    	Vertex { position: vp5, uv: uv5, color },
	    ],
	    indices: vec![
	    	0,5,1,
	    	3,2,4,
	    	2,5,1,
	    	4,5,2,
	    ],
	    texture: None,
	}
}
