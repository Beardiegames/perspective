use macroquad::prelude::*;
use crate::render::*;

pub struct SpritePointers {
	pub grass: DrawPointer<Sprite>,
	pub wall1: DrawPointer<Sprite>,
	pub wall2: DrawPointer<Sprite>,
}

impl GuiCustoms for SpritePointers {

	fn gui_setup(sprites: &mut DrawBuffer<Sprite>, gl_pipe: &PipelineParams) -> SpritePointers {

		let grass = grass(sprites, gl_pipe);
		let wall1 = wall(sprites, gl_pipe);
		let wall2 = wall(sprites, gl_pipe);	
		{
	   		let grass_obj = sprites.edit(&grass);
	   		grass_obj.pos.x = 100.;
	   		grass_obj.pos.y = 50.;
		}
		
	 	SpritePointers {
			grass,
			wall1,
			wall2,
		}
	}	
}


pub fn wall(sprites: &mut DrawBuffer<Sprite>, pipe: &PipelineParams) -> DrawPointer<Sprite> {
	sprites.define(
		Sprite::new(
    		texture_from_file(include_bytes!("../assets/WallTiles.png")),
    		create_sprite_material(*pipe)
    	)
    )
}

pub fn grass(sprites: &mut DrawBuffer<Sprite>, pipe: &PipelineParams) -> DrawPointer<Sprite> {
	sprites.define(
		Sprite::new(
    		texture_from_file(include_bytes!("../assets/Grass-Sheet.png")),
    		create_sprite_material(*pipe)
    	)
    )
}

fn texture_from_file(bytes: &[u8]) -> Texture2D {
	let t = Texture2D::from_file_with_format(bytes, None);
	t.set_filter(FilterMode::Nearest);
	t
}
