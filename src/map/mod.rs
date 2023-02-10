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


#[derive(Serialize, Deserialize)]
pub struct Map {
	width: usize,
	height: usize,
	matrix: Vec<MapValue>,
}

impl Map {
	pub fn new(width: usize, height: usize) -> Self {
		let mut matrix: Vec<MapValue> = vec![GRAY.into(); width * height];

		for y in 0..height {
		for x in 0..width {
			matrix[y * width + x].pos = [x as u16, y as u16];
		}}

		let mut map = Map {
			width, height,
			matrix,
		};
		map.set_values();
		map
	}

	pub fn get_at_mx(&self, mx: &MxPos)-> Option<Color> {
		let idx = mx.ver * self.width + mx.hor % self.width;
		
		match idx < self.matrix.len() {
			true => Some(self.matrix[idx].color()),
			false => None,
		}
	}

	pub fn set_values(&mut self) {
		let perlin = Perlin::new(0u32);
		let qual = 3.0;
		let mut color = Color { r: 0.25, g: 0.5, b: 0.5, a: 1.0 };
		
		for i in 0..self.matrix.len() {
			let pos_x = i % self.width;
			let pos_y = i / self.width;
			
			let point: [f64; 2] = [pos_x as f64 / qual, pos_y as f64 / qual]; 
			let noise = 0.75 + (perlin.get(point) / 4.0) as f32;
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


			//println!("noise: {}", noise);

			self.matrix[i].set_color(&color);
		}
	}
}
