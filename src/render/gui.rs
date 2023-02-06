use macroquad::prelude::*;
use super::*;


pub trait GuiCustoms {
	fn gui_setup(sprites: &mut DrawBuffer<Sprite>, pipeline: &PipelineParams) -> Self;
}

pub struct Gui<T: GuiCustoms> {
	pub render_pipeline: PipelineParams,
	pub sprites: DrawBuffer<Sprite>,
	pub customs: T,
}

impl<T: GuiCustoms> Gui<T> {
	pub fn new() -> Self {
		let render_pipeline = build_gl_pipeline();
		let mut sprites = DrawBuffer::new(1000);
		let customs = T::gui_setup(&mut sprites, &render_pipeline);

		Gui {
			render_pipeline,
			sprites,
			customs, 
		}		
	}
	
	pub fn draw(&mut self) {
		set_default_camera();

		for idx in self.sprites.queue() {
			let drawable = self.sprites.by_index(idx);
			
		    gl_use_material(drawable.material);

		    draw_texture_ex(
		        drawable.texture,
		        //drawable.pos.x.round() * PIXEL_SIZE, 
		        //drawable.pos.y.round() * PIXEL_SIZE,
		        drawable.pos.x * PIXEL_SIZE, 
		       	drawable.pos.y * PIXEL_SIZE,
		        drawable.color,
		    	drawable.params.clone()
		    );
	    }
	    self.sprites.clear_queue();
	}
}
