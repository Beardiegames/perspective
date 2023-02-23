pub mod buffer;
pub mod camera;

use macroquad::prelude::*;
pub use buffer::*;
pub use camera::*;


pub trait Drawable { 
	fn draw(&self); 
}

pub fn draw_buffer_2d<T>(buffer: &mut DrawBuffer<T>)
	where T: Drawable
{
	set_default_camera();
	draw_buffer(buffer);
}

pub fn draw_buffer_3d<T>(buffer: &mut DrawBuffer<T>, camera: &CameraController)
	where T: Drawable
{	
	set_camera(&camera.quad_cam);

	gl_use_default_material();
	draw_grid(20, 1., BLACK, GRAY);

	draw_buffer(buffer);
}

fn draw_buffer<T>(draw_buffer: &mut DrawBuffer<T>)
	where T: Drawable
{
	match draw_buffer.base_material {
		Some(m) => gl_use_material(m),
		None => gl_use_default_material(),
	};

	draw_buffer.draw_queue();
    draw_buffer.render_queue.clear();
}
