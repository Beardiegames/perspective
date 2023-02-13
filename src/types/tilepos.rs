use std::ops::{Add, Sub, Mul, Div};
use macroquad::prelude::*;
use super::*;


#[derive(Debug, Clone, PartialEq)]
pub struct TilePos {
	pub hor: f32,
	pub ver: f32,	
}

impl TilePos {
	pub const ZERO: Self = TilePos { hor: 0.0, ver: 0.0, };

	pub fn new(hor: f32, ver: f32) -> Self {
		TilePos { hor, ver }
	}
	
	pub fn round(mut self) -> Self {
		self.hor = self.hor.round();
		self.ver = self.ver.round();
		self
	}
}


// From RealPos

fn from_real_to_tile(tilepos: &RealPos) -> TilePos {
	let rpos = tilepos.as_vec3();	
	TilePos {
		hor: rpos.z * 0.5, //  rpos.z,
		ver: rpos.x * 0.6666666, //rpos.x * 1.3333333,
	}
}	

impl From<&RealPos> for TilePos {
	fn from(other: &RealPos) -> Self {
		from_real_to_tile(other)
	}
}

impl From<RealPos> for TilePos {
	fn from(other: RealPos) -> Self {
		from_real_to_tile(&other)
	}
}


// From MxPos

fn from_matrix_to_tile(mxpos: &MxPos) -> TilePos {
	TilePos::new(
		mxpos.hor as f32,
		mxpos.ver as f32,
	)
}

impl From<&MxPos> for TilePos {
	fn from(other: &MxPos) -> Self {
		from_matrix_to_tile(other)
	}
}

impl From<MxPos> for TilePos {
	fn from(other: MxPos) -> Self {
		from_matrix_to_tile(&other)
	}
}


// impl Add traits

fn add_tile_to_tile(a: &TilePos, b: &TilePos) -> TilePos {
	TilePos::new(
		a.hor + b.hor,
		a.ver + b.ver,
	)
}

impl Add<&TilePos> for &TilePos {
	type Output = TilePos;
    fn add(self, other: &TilePos) -> TilePos {
        add_tile_to_tile(self, other)
    }
}

impl Add<TilePos> for TilePos {
	type Output = TilePos;
    fn add(self, other: TilePos) -> TilePos {
        add_tile_to_tile(&self, &other)
    }
}

impl Add<&TilePos> for TilePos {
	type Output = TilePos;
    fn add(self, other: &TilePos) -> TilePos {
        add_tile_to_tile(&self, other)
    }
}

impl Add<TilePos> for &TilePos {
	type Output = TilePos;
    fn add(self, other: TilePos) -> TilePos {
        add_tile_to_tile(self, &other)
    }
}


// impl Sub traits

fn sub_tile_to_tile(a: &TilePos, b: &TilePos) -> TilePos {
	TilePos::new(
		a.hor - b.hor,
		a.ver - b.ver,
	)
}

impl Sub<&TilePos> for &TilePos {
	type Output = TilePos;
    fn sub(self, other: &TilePos) -> TilePos {
        sub_tile_to_tile(self, other)
    }
}

impl Sub<TilePos> for TilePos {
	type Output = TilePos;
    fn sub(self, other: TilePos) -> TilePos {
        sub_tile_to_tile(&self, &other)
    }
}

impl Sub<&TilePos> for TilePos {
	type Output = TilePos;
    fn sub(self, other: &TilePos) -> TilePos {
        sub_tile_to_tile(&self, other)
    }
}

impl Sub<TilePos> for &TilePos {
	type Output = TilePos;
    fn sub(self, other: TilePos) -> TilePos {
        sub_tile_to_tile(self, &other)
    }
}


// impl Mul traits

fn mul_tiles(a: &TilePos, b: &TilePos) -> TilePos {
	TilePos::new(
		a.hor * b.hor,
		a.ver * b.ver,
	)
}

impl Mul<&TilePos> for &TilePos {
	type Output = TilePos;
    fn mul(self, other: &TilePos) -> TilePos {
        mul_tiles(self, other)
    }
}

impl Mul<TilePos> for TilePos {
	type Output = TilePos;
    fn mul(self, other: TilePos) -> TilePos {
        mul_tiles(&self, &other)
    }
}

impl Mul<&TilePos> for TilePos {
	type Output = TilePos;
    fn mul(self, other: &TilePos) -> TilePos {
        mul_tiles(&self, other)
    }
}

impl Mul<TilePos> for &TilePos {
	type Output = TilePos;
    fn mul(self, other: TilePos) -> TilePos {
        mul_tiles(self, &other)
    }
}



// impl Div traits

fn div_tiles(a: &TilePos, b: &TilePos) -> TilePos {
	TilePos::new(
		a.hor / b.hor,
		a.ver / b.ver,
	)
}

impl Div<&TilePos> for &TilePos {
	type Output = TilePos;
    fn div(self, other: &TilePos) -> TilePos {
        div_tiles(self, other)
    }
}

impl Div<TilePos> for TilePos {
	type Output = TilePos;
    fn div(self, other: TilePos) -> TilePos {
        div_tiles(&self, &other)
    }
}

impl Div<&TilePos> for TilePos {
	type Output = TilePos;
    fn div(self, other: &TilePos) -> TilePos {
        div_tiles(&self, other)
    }
}

impl Div<TilePos> for &TilePos {
	type Output = TilePos;
    fn div(self, other: TilePos) -> TilePos {
        div_tiles(self, &other)
    }
}
