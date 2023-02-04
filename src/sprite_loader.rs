use macroquad::prelude::*;
use crate::render::*;

pub struct Sprites {
	pub grass: DrawPointer<Sprite>,
	pub wall: DrawPointer<Sprite>,
	pub wall2: DrawPointer<Sprite>,
}

pub fn create_sprite_objects(sprite_buffer: &mut DrawBuffer<Sprite>, render_pipeline: &PipelineParams) -> Sprites {
 	Sprites {
		grass: grass(sprite_buffer, *render_pipeline),
		wall: wall(sprite_buffer, *render_pipeline),
		wall2: wall(sprite_buffer, *render_pipeline),
	}
}

pub fn wall(buffer: &mut DrawBuffer<Sprite>, pipeline: PipelineParams) -> DrawPointer<Sprite> {
	buffer.define(
		Sprite::new(
    		texture_from_file(include_bytes!("../assets/WallTiles.png")),
    		create_sprite_material(pipeline)
    	)
    )
}

pub fn grass(buffer: &mut DrawBuffer<Sprite>, pipeline: PipelineParams) -> DrawPointer<Sprite> {
	buffer.define(
		Sprite::new(
    		texture_from_file(include_bytes!("../assets/Grass-Sheet.png")),
    		create_sprite_material(pipeline)
    	)
    )
}

fn texture_from_file(bytes: &[u8]) -> Texture2D {
	let t = Texture2D::from_file_with_format(bytes, None);
	t.set_filter(FilterMode::Nearest);
	t
}
