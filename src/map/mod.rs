mod map_reader;
mod map_writer;
mod map_result;
mod map_value;

pub use map_reader::*;
pub use map_writer::*;
pub use map_result::*;
pub use map_value::*;

use serde::*;
use macroquad::prelude::*;
use noise::*;
use crate::types::*;
//use super::*;


#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
	width: i16,
	height: i16,
	matrix: Vec<MapValue>,
}

impl Map {
	pub fn new(width: usize, height: usize) -> Result<Self, MapError> {
		if width > i16::MAX as usize { return Err(MapError::out_of_bounds("width", &width.to_string(), &i16::MAX.to_string())); }
		if height > i16::MAX as usize { return Err( MapError::out_of_bounds("height", &height.to_string(), &i16::MAX.to_string())); }
	
		let mut matrix: Vec<MapValue> = vec![GRAY.into(); width * height];

		for y in 0..height {
		for x in 0..width {
			matrix[y * width + x].pos = [x as u16, y as u16];
		}}

		let mut map = Map {
			width: width as i16, 
			height: height as i16,
			matrix,
		};
		map.random_matrix_values();
		Ok(map)
	}

	pub fn get_at_mx(&self, mx: &MxPos)-> Option<Color> {
		if mx.hor >= 0 && mx.hor < self.width 
		&& mx.ver >= 0 && mx.ver < self.height 
		{
			let idx = (mx.ver * self.width + mx.hor % self.width) as usize;
					
			match idx < self.matrix.len() {
				true => Some(self.matrix[idx].color()),
				false => None,
			}
		}
		else {
			None
		}		
	}

	pub fn random_matrix_values(&mut self) {
		let perlin = Perlin::new(0u32);
		let quality = 3.0;
		let amount = 10.0;
		let mut color = Color { r: 0.25, g: 0.5, b: 0.5, a: 1.0 };
		
		for i in 0..self.matrix.len() as i16 {
			let pos_x = i % self.width;
			let pos_y = i / self.width;
			
			let point: [f64; 2] = [pos_x as f64 / quality, pos_y as f64 / quality]; 
			let noise = 0.75 + (perlin.get(point) / amount) as f32;
			color.g = noise;

			if pos_x == 0 {
				color.r = 1.0;
			} 
			else if pos_x == self.width - 1 {
				color.b = 1.0;
			}
			
			if pos_y == 0 {
				color.r = 1.0;
			} 
			else if pos_y == self.height - 1 {
				color.b = 1.0;
			}

			if pos_x > 0 && pos_x < self.width - 1 && pos_y > 0 && pos_y < self.height - 1 {
				color.r = 0.25;
				color.b = 0.50;
			}

			if pos_x == 0 && pos_y == 0 {
				color = Color { r: 0.75, g: 0.25, b: 0.25, a: 1.0 }
			} 



			//println!("noise: {}", noise);

			if i >= 0 {
				self.matrix[i as usize].set_color(&color);
			}
		}
	}
}
