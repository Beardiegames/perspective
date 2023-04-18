use macroquad::prelude::*;
use macroquad::models::Vertex;
use crate::position::*;
use crate::render::*;


pub struct HexTile {
	pub matrix_position: MxPos,
	pub screen_position: TilePos,
	pub offset_position: TilePos,
	pub color: Color,
	pub height: f32,

	pub floor_mesh: Mesh,
	pub walls_mesh: [Mesh; 6],
		
	floor_root: Mesh,
	walls_root: [Mesh; 6],
}

impl Drawable for HexTile {
	fn draw(&self) {
		gl_use_default_material();
		draw_mesh(&self.floor_mesh);

		for wall in &self.walls_mesh {
			draw_mesh(wall)
		}
	}
}

impl HexTile {
	pub fn new(matrix_pos: MxPos, screen_pos: TilePos) -> Self {
		let default_color = BLACK;
		let (floor, walls) = create_hex_mesh(default_color);
		
		let mut hex = HexTile {
			matrix_position: matrix_pos,
			screen_position: screen_pos,
			offset_position: TilePos::ZERO,
			color: default_color,
			height: 0.0,

			floor_mesh: floor,
			walls_mesh: walls,
			floor_root: floor,
			walls_root: walls,
		};
		hex.update_mesh();
		hex
	}

	pub fn get_matrix_position(&self) -> MxPos {
		self.matrix_position.clone() + MxPos::from(&self.offset_position)
	}
	
	pub fn update_mesh(&mut self) {
		let real = self.real_position();
		
		for i in 0..6 {
			let vx = self.floor_root.vertices[i].position.z + real.z;
			let vy = self.floor_root.vertices[i].position.x + real.x;
			
			self.floor_mesh.vertices[i].position.z = vx;
			self.floor_mesh.vertices[i].position.x = vy;
			self.floor_mesh.vertices[i].position.y = self.height;
			self.floor_mesh.vertices[i].color = self.color;
		}
	}

	pub fn world_position(&self) -> TilePos {
		&self.screen_position + &self.offset_position
	}

	pub fn real_position(&self) -> Vec3 {
		Vec3::from(RealPos::from(self.world_position()))
	}
}

pub fn create_hex_mesh(color: Color) -> (Mesh, [Mesh; 6]) {
	let w: f32 = 2.0;
	let h: f32 = 2.0; //1.155 * w;
	
	let hor = 0.5 * h;
	let ver = 0.5 * w;
	let rib = 0.25 * h;

	// NOTE: positions are => (Y,Z,-X) or (away,up,left)
	let (vp0, uv0) = (Vec3::new(hor, 0.0, 0.0), 	Vec2::new(0.5, 0.0)); // top
	let (vp1, uv1) = (Vec3::new(rib, 0.0, ver), 	Vec2::new(1.0, 0.0)); // top-right
	let (vp2, uv2) = (Vec3::new(-rib, 0.0, ver), 	Vec2::new(1.0, 0.0)); // btm-right
	let (vp3, uv3) = (Vec3::new(-hor, 0.0, 0.0), 	Vec2::new(0.5, 1.0)); // btm
	let (vp4, uv4) = (Vec3::new(-rib, 0.0, -ver), 	Vec2::new(0.0, 1.0)); // btm-left
	let (vp5, uv5) = (Vec3::new(rib, 0.0, -ver), 	Vec2::new(0.0, 1.0)); // top-left
	
	let floor = Mesh {
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
	};

	let mut wvp0 = vp0; wvp0.y += 1.0;
	let mut wvp1 = vp1; wvp1.y += 1.0;
	let mut wvp2 = vp2; wvp2.y += 1.0;
	let mut wvp3 = vp3; wvp3.y += 1.0;
	let mut wvp4 = vp4; wvp4.y += 1.0;
	let mut wvp5 = vp5; wvp5.y += 1.0;

	let walls = [
		Mesh {
		    vertices: vec![
		    	Vertex { position: vp0, uv: uv0, color },
		    	Vertex { position: vp1, uv: uv1, color },
		    	Vertex { position: wvp0, uv: uv1, color },
		    	Vertex { position: wvp1, uv: uv1, color },
		    ],
		    indices: vec![0,1,2, 2,1,3],
		    texture: None,
		},
		Mesh {
		    vertices: vec![
		    	Vertex { position: vp1, uv: uv0, color },
		    	Vertex { position: vp2, uv: uv1, color },
		    	Vertex { position: wvp1, uv: uv1, color },
		    	Vertex { position: wvp2, uv: uv1, color },
		    ],
		    indices: vec![0,1,2, 2,1,3],
		    texture: None,
		},
		Mesh {
		    vertices: vec![
		    	Vertex { position: vp2, uv: uv0, color },
		    	Vertex { position: vp3, uv: uv1, color },
		    	Vertex { position: wvp2, uv: uv1, color },
		    	Vertex { position: wvp3, uv: uv1, color },
		    ],
		    indices: vec![0,1,2, 2,1,3],
		    texture: None,
		},
		Mesh {
		    vertices: vec![
		    	Vertex { position: vp3, uv: uv0, color },
		    	Vertex { position: vp4, uv: uv1, color },
		    	Vertex { position: wvp3, uv: uv1, color },
		    	Vertex { position: wvp4, uv: uv1, color },
		    ],
		    indices: vec![0,1,2, 2,1,3],
		    texture: None,
		},
		Mesh {
		    vertices: vec![
		    	Vertex { position: vp4, uv: uv0, color },
		    	Vertex { position: vp5, uv: uv1, color },
		    	Vertex { position: wvp4, uv: uv1, color },
		    	Vertex { position: wvp5, uv: uv1, color },
		    ],
		    indices: vec![0,1,2, 2,1,3],
		    texture: None,
		},
		Mesh {
		    vertices: vec![
		    	Vertex { position: vp5, uv: uv0, color },
		    	Vertex { position: vp0, uv: uv1, color },
		    	Vertex { position: wvp5, uv: uv1, color },
		    	Vertex { position: wvp0, uv: uv1, color },
		    ],
		    indices: vec![0,1,2, 2,1,3],
		    texture: None,
		},
	];

	(floor, walls)
}
