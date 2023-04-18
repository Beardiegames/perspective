use macroquad::prelude::*;
use crate::shaders;
use crate::{SPRITE_SIZE, PIXEL_SIZE};
use crate::render::*;


pub const TEXTURE_SIZE: f32 = SPRITE_SIZE * PIXEL_SIZE;


pub struct Sprite {
	pub texture: Texture2D,
	pub material: Material,
	pub pos: Vec2,
	pub color: Color,
	pub params: DrawTextureParams,
}

impl Drawable for Sprite {
	fn draw(&self) {
		draw_texture_ex(
	        self.texture,
	        //drawable.pos.x.round() * PIXEL_SIZE,
	        //drawable.pos.y.round() * PIXEL_SIZE,
	        self.pos.x * PIXEL_SIZE,
	       	self.pos.y * PIXEL_SIZE,
	        self.color,
	    	self.params.clone()
	    );
	}
}

impl Sprite {
	pub fn new(texture: Texture2D, material: Material) -> Self {
		Sprite {
			texture,
			material,
			pos: Vec2 {x: 0., y: 0.},
			color: WHITE,
			params: DrawTextureParams {
		        dest_size: Some(Vec2{x: TEXTURE_SIZE, y: TEXTURE_SIZE}),
		        source: Some(Rect{x: 0., y: 0., w: SPRITE_SIZE, h: SPRITE_SIZE}),
		        rotation: 0.,
		        flip_x: false,
		        flip_y: false,
		        pivot: Some(Vec2{x: 0., y:0.}),
		    }
		}
	}
}

pub fn create_sprite_material(pipeline_params: PipelineParams) -> Material {
	let (fragment_shader, vertex_shader) = shaders::default_shaders();

	load_material(
        &vertex_shader,
        &fragment_shader,
        MaterialParams {
            pipeline_params,
            ..Default::default()
        },
    )
    .unwrap()
}
