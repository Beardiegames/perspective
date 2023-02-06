use macroquad::prelude::*;
use macroquad::models::Vertex;
use super::*;

pub struct TilePos {
	pub hor: f32,
	pub ver: f32,	
}

impl TilePos {
	pub const ZERO: Self = TilePos { hor: 0.0, ver: 0.0, };
	
	pub fn from_real_position(pos: Vec3) -> Self {
		TilePos {
			hor: (pos.z - (0.5 / pos.x)) as f32,
			ver: (0.75 / pos.x) as f32,	
		}
	}

	pub fn to_real_position(&self) -> Vec3 {
		Vec3 {
			x: 0.75 * self.ver as f32,
			y: 0.0,
			z: self.hor as f32 + 0.5 * self.ver as f32,
		}
	}
}

pub struct HexTile {
	pub mesh: Mesh, 
	
	root: Mesh,
	pos: TilePos,
	col: Color,
}

impl HexTile {
	pub fn new(pos: TilePos, col: Color) -> Self {
		let mut tile = HexTile {
			mesh: create_hex_mesh(col),
			root: create_hex_mesh(col),
			pos: TilePos::ZERO,
			col,
		};
		tile.set_pos(pos);
		tile
	}
	
	pub fn set_pos(&mut self, pos: TilePos) {
		let real = pos.to_real_position();
		
		for i in 0..6 {	
			let vx = self.root.vertices[i].position.z + real.z;
			let vy = self.root.vertices[i].position.x + real.x;
			self.mesh.vertices[i].position.z = vx;
			self.mesh.vertices[i].position.x = vy;
		}
		self.pos = pos;
	}

	pub fn pos(&self) -> &TilePos {
		&self.pos
	}

	pub fn set_col(&mut self, col: Color) {
		for i in 0..6 {	
			self.mesh.vertices[i].color = col;
		}
		self.col = col;
	}
	
	pub fn col(&self) -> &Color {
		&self.col
	}
}

pub fn draw_hex_tiles(dbuff: &mut DrawBuffer<HexTile>) {
	set_camera(&Camera3D {
        position: vec3(-5., 5., 0.),
        up: vec3(0., 1., 0.),
        target: vec3(0., 0., 0.),
        //projection: Projection::Orthographics,
        //fovy: 10.0,
        ..Default::default()
    });
    
    //gl_use_default_material();
    //draw_grid(20, 1., BLACK, GRAY);
    
	for item in dbuff.item_list() {
		draw_mesh(&item.mesh);
    }
    dbuff.clear_queue();

	//draw_cube(vec3(2., 0., -2.), vec3(0.4, 0.4, 0.4), None, BLACK);
    //draw_mesh(&create_hex_mesh(RED));
}

pub fn create_hex_mesh(color: Color) -> Mesh {
	let w: f32 = 1.0;
	let h: f32 = 1.0; //1.155 * w;
	
	let hor = 0.5 * h;
	let ver = 0.5 * w;
	let rib = 0.25; //((hor * hor) - (ver * ver)).sqrt();
	// NOTE: positions are => (Y,Z,-X) or (away,up,left)
	
	//let (vp0, uv0) = (Vec3::new(0.0, 0.0, 0.0), 	Vec2::new(0.5, 0.5));
	let (vp0, uv0) = (Vec3::new(hor, 0.0, 0.0), 	Vec2::new(0.5, 0.0));
	let (vp1, uv1) = (Vec3::new(rib, 0.0, ver), 	Vec2::new(1.0, 0.0));
	let (vp2, uv2) = (Vec3::new(-rib, 0.0, ver), 	Vec2::new(1.0, 0.0));
	let (vp3, uv3) = (Vec3::new(-hor, 0.0, 0.0), 	Vec2::new(0.5, 1.0));
	let (vp4, uv4) = (Vec3::new(-rib, 0.0, -ver), 	Vec2::new(0.0, 1.0));
	let (vp5, uv5) = (Vec3::new(rib, 0.0, -ver), 	Vec2::new(0.0, 1.0));
	//let (vp6, uv6) = (Vec3::new(0.0, 10., 0.0), 	Vec2::new(0.5, 0.0));

	Mesh {
	    vertices: vec![
	    	Vertex { position: vp0, uv: uv0, color },
	    	Vertex { position: vp1, uv: uv1, color },
	    	Vertex { position: vp2, uv: uv2, color },
	    	Vertex { position: vp3, uv: uv3, color },
	    	Vertex { position: vp4, uv: uv4, color },
	    	Vertex { position: vp5, uv: uv5, color },
	    	//Vertex { position: vp6, uv: uv5, color },
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
