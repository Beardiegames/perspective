pub mod sprites;
pub mod buffer;

use macroquad::prelude::*;
use macroquad::models::Vertex;
pub use sprites::*;
pub use buffer::*;


pub fn build_pipeline() -> PipelineParams {
    PipelineParams {
        depth_write: true,
        depth_test: Comparison::LessOrEqual,
        ..Default::default()
    }
}

// --

pub struct HexTile {
	pub mesh: Mesh, 
	// pub pos: Vec2,
	// pub rot: f32,
	// pub col: Color,
}

// impl HexTile {
	// pub fn new(col: Color) -> Self {
		// HexTile {
			// pos: Vec2 {x: 0., y: 0.},
			// rot: 0.,
			// col,
		// }
	// }
// }

pub fn draw_hex_tiles(dbuff: &mut DrawBuffer<HexTile>) {
	set_camera(&Camera3D {
        position: vec3(-20., 15., 0.),
        up: vec3(0., 1., 0.),
        target: vec3(0., 0., 0.),
        ..Default::default()
    });
    
    //gl_use_default_material();
    draw_grid(20, 1., BLACK, GRAY);
// 
	// for item in dbuff.item_list() {
		// //let item = dbuff.by_index(idx);
	    // //draw_poly(item.pos.x, item.pos.y, 6, 50., item.rot, item.col);
	    // //println!("item: {}", item.pos);
	    // draw_cube(vec3(2., 0., -2.), vec3(0.4, 0.4, 0.4), None, BLACK);
	    // //draw_mesh(&item.mesh);
    // }
    // dbuff.clear_queue();

	draw_cube(vec3(2., 0., -2.), vec3(0.4, 0.4, 0.4), None, BLACK);
    draw_mesh(&create_hex_mesh(RED));
}

pub fn create_hex_mesh(color: Color) -> Mesh {

	// NOTE: positions are => (Y,Z,-X) or (away,up,left)
	
	//let (vp0, uv0) = (Vec3::new(0.0, 0.0, 0.0), 	Vec2::new(0.5, 0.5));
	let (vp0, uv0) = (Vec3::new(0.0, 0.0, 10.), 	Vec2::new(0.5, 0.0));
	let (vp1, uv1) = (Vec3::new(6.6, 0.0, 3.3), 	Vec2::new(1.0, 0.0));
	let (vp2, uv2) = (Vec3::new(6.6, 0.0, -3.3), 	Vec2::new(1.0, 0.0));
	let (vp3, uv3) = (Vec3::new(0.0, 0.0, -10.), 	Vec2::new(0.5, 1.0));
	let (vp4, uv4) = (Vec3::new(-6.6, 0.0, -3.3), 	Vec2::new(0.0, 1.0));
	let (vp5, uv5) = (Vec3::new(-6.6, 0.0, 3.3), 	Vec2::new(0.0, 1.0));
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
