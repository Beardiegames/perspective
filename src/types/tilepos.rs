use std::ops::{Add, Sub, Mul, Div};
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct TilePos {
	pub hor: f32,
	pub ver: f32,	
}

impl TilePos {
	pub const ZERO: Self = TilePos { hor: 0.0, ver: 0.0, };
	
	pub fn from_real_position(pos: Vec3) -> Self {
		TilePos {
			hor: pos.z,// - pos.x * 2.0,
			ver: pos.x * 1.3333333,
		}
	}

	pub fn to_real_position(&self) -> Vec3 {
		Vec3 {
			x: 0.75 * self.ver,
			y: 0.0,
			z: self.hor,// + 0.5 * self.ver
		}
	}

	pub fn round(mut self) -> Self {
		self.hor = self.hor.round();
		self.ver = self.ver.round();
		self
	}

}

impl Add<&TilePos> for &TilePos {
	type Output = TilePos;
	
    fn add(self, other: &TilePos) -> TilePos {
        TilePos {
            hor: self.hor + other.hor,
            ver: self.ver + other.ver,
        }
    }
}

impl Sub<&TilePos> for &TilePos {
	type Output = TilePos;
	
    fn sub(self, other: &TilePos) -> TilePos {
        TilePos {
            hor: self.hor - other.hor,
            ver: self.ver - other.ver,
        }
    }
}

impl Mul<&TilePos> for &TilePos {
	type Output = TilePos;
	
    fn mul(self, other: &TilePos) -> TilePos {
        TilePos {
            hor: self.hor * other.hor,
            ver: self.ver * other.ver,
        }
    }
}

impl Div<&TilePos> for &TilePos {
	type Output = TilePos;
	
    fn div(self, other: &TilePos) -> TilePos {
        TilePos {
            hor: self.hor / other.hor,
            ver: self.ver / other.ver,
        }
    }
}
