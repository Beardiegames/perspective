use serde::*;
use macroquad::prelude::*;


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MapValue {
	pub pos: [u16; 2],
	pub color: [u8; 4], 
}

impl MapValue {
	pub fn color(&self) -> Color {
		Color {
			r: self.color[0] as f32 / 255.0,
			g: self.color[1] as f32 / 255.0,
			b: self.color[2] as f32 / 255.0,
			a: self.color[3] as f32 / 255.0,
		}		
	}

	pub fn set_color(&mut self, col: &Color) {
		self.color[0] = (col.r * 255.0) as u8;
		self.color[1] = (col.g * 255.0) as u8;
		self.color[2] = (col.b * 255.0) as u8;
		self.color[3] = (col.a * 255.0) as u8;
	}
}

impl From<&Color> for MapValue {
	fn from(val: &Color) -> MapValue {
		let mut map_val = MapValue::default();
		map_val.set_color(val);
		map_val
	}
}

impl From<Color> for MapValue {
	fn from(val: Color) -> MapValue {
		let mut map_val = MapValue::default();
		map_val.set_color(&val);
		map_val
	}
}
