use std::ops::{Add, Sub, Mul, Div};
use macroquad::prelude::*;
use super::*;


#[derive(Debug, Clone, PartialEq)]
pub struct RealPos(Vec3);

impl RealPos {
	pub fn new(x: f32, y: f32, z: f32) -> Self {
		RealPos(Vec3::new(x, y, z))
	}

	pub fn as_vec3(&self) -> &Vec3 {
		&self.0
	}
}


// impl From Vec

impl From<&RealPos> for Vec3 {
	fn from(other: &RealPos) -> Vec3 {
		vec3(other.0.x, other.0.y, other.0.z)
	}
}

impl From<RealPos> for Vec3 {
	fn from(other: RealPos) -> Vec3 {
		vec3(other.0.x, other.0.y, other.0.z)
	}
}


// impl From TilePos

fn from_tile_to_real(tilepos: &TilePos) -> RealPos {
	RealPos::new(
		0.75 * tilepos.ver,
		0.0,
		tilepos.hor
	)
}

impl From<TilePos> for RealPos {
	fn from(other: TilePos) -> Self {
		from_tile_to_real(&other)
	}
}

impl From<&TilePos> for RealPos {
	fn from(other: &TilePos) -> Self {
		from_tile_to_real(other)
	}
}


// impl From MxPos

fn from_matrix_to_real(mxpos: &MxPos) -> RealPos {
	RealPos::new(
		0.75 * mxpos.ver as f32,
		0.0,
		mxpos.hor as f32
	)
}

impl From<MxPos> for RealPos {
	fn from(other: MxPos) -> Self {
		from_matrix_to_real(&other)
	}
}

impl From<&MxPos> for RealPos {
	fn from(other: &MxPos) -> Self {
		from_matrix_to_real(other)
	}
}


// impl From Vec3

fn from_vec_to_real(vec3: Vec3) -> RealPos {
	RealPos(vec3)
}

impl From<Vec3> for RealPos {
	fn from(other: Vec3) -> Self {
		from_vec_to_real(other)
	}
}


// impl Add

fn add_real_to_real(a: &RealPos, b: &RealPos) -> RealPos {
	RealPos::new(
		a.0.x + b.0.x,
		a.0.y + b.0.y,
		a.0.z + b.0.z
	)
}

impl Add<&RealPos> for &RealPos {
	type Output = RealPos;
    fn add(self, other: &RealPos) -> RealPos {
        add_real_to_real(self, other)
    }
}

impl Add<&RealPos> for RealPos {
	type Output = RealPos;	
    fn add(self, other: &RealPos) -> RealPos {
        add_real_to_real(&self, other)
    }
}

impl Add<RealPos> for RealPos {
	type Output = RealPos;	
    fn add(self, other: RealPos) -> RealPos {
        add_real_to_real(&self, &other)
    }
}

impl Add<RealPos> for &RealPos {
	type Output = RealPos;	
    fn add(self, other: RealPos) -> RealPos {
        add_real_to_real(self, &other)
    }
}


// impl Sub

fn sub_real_to_real(a: &RealPos, b: &RealPos) -> RealPos {
	RealPos::new(
		a.0.x - b.0.x,
		a.0.y - b.0.y,
		a.0.z - b.0.z
	)
}

impl Sub<&RealPos> for &RealPos {
	type Output = RealPos;
    fn sub(self, other: &RealPos) -> RealPos {
        sub_real_to_real(self, other)
    }
}

impl Sub<RealPos> for RealPos {
	type Output = RealPos;
    fn sub(self, other: RealPos) -> RealPos {
        sub_real_to_real(&self, &other)
    }
}

impl Sub<&RealPos> for RealPos {
	type Output = RealPos;
    fn sub(self, other: &RealPos) -> RealPos {
        sub_real_to_real(&self, other)
    }
}

impl Sub<RealPos> for &RealPos {
	type Output = RealPos;
    fn sub(self, other: RealPos) -> RealPos {
        sub_real_to_real(self, &other)
    }
}


// impl Mul traits

fn mul_reals(a: &RealPos, b: &RealPos) -> RealPos {
	RealPos::new(
		a.0.x * b.0.x,
		a.0.y * b.0.y,
		a.0.z * b.0.z
	)
}

impl Mul<&RealPos> for &RealPos {
	type Output = RealPos;
    fn mul(self, other: &RealPos) -> RealPos {
        mul_reals(self, other)
    }
}

impl Mul<RealPos> for RealPos {
	type Output = RealPos;
    fn mul(self, other: RealPos) -> RealPos {
        mul_reals(&self, &other)
    }
}

impl Mul<&RealPos> for RealPos {
	type Output = RealPos;
    fn mul(self, other: &RealPos) -> RealPos {
        mul_reals(&self, other)
    }
}

impl Mul<RealPos> for &RealPos {
	type Output = RealPos;
    fn mul(self, other: RealPos) -> RealPos {
        mul_reals(self, &other)
    }
}



// impl Div traits

fn div_reals(a: &RealPos, b: &RealPos) -> RealPos {
	RealPos::new(
		a.0.x / b.0.x,
		a.0.y / b.0.y,
		a.0.z / b.0.z
	)
}

impl Div<&RealPos> for &RealPos {
	type Output = RealPos;
    fn div(self, other: &RealPos) -> RealPos {
        div_reals(self, other)
    }
}

impl Div<RealPos> for RealPos {
	type Output = RealPos;
    fn div(self, other: RealPos) -> RealPos {
        div_reals(&self, &other)
    }
}

impl Div<&RealPos> for RealPos {
	type Output = RealPos;
    fn div(self, other: &RealPos) -> RealPos {
        div_reals(&self, other)
    }
}

impl Div<RealPos> for &RealPos {
	type Output = RealPos;
    fn div(self, other: RealPos) -> RealPos {
        div_reals(self, &other)
    }
}
