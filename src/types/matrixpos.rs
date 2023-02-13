use std::ops::{Add, Sub, Mul, Div};
use super::*;


#[derive(Debug, Clone, PartialEq)]
pub struct MxPos {
	pub hor: i16,
	pub ver: i16,
}

impl MxPos {
	pub fn new(hor: i16, ver: i16) -> Self {
		Self { hor, ver }
	}
}

// impl From TilePos

fn from_tile(tile: &TilePos) -> MxPos {
	MxPos::new(
		tile.hor.round() as i16,
		tile.ver.round() as i16
	)
} 

impl From<TilePos> for MxPos {
	fn from(other: TilePos) -> Self {
		from_tile(&other)
	}
}

impl From<&TilePos> for MxPos {
	fn from(other: &TilePos) -> Self {
		from_tile(other)
	}
}

// impl Add

fn add_mx_to_mx(a: &MxPos, b: &MxPos) -> MxPos {
	MxPos::new(
		a.hor + b.hor,
		a.ver + b.ver
	)
}

impl Add<&MxPos> for &MxPos {
	type Output = MxPos;
    fn add(self, other: &MxPos) -> MxPos {
        add_mx_to_mx(self, other)
    }
}

impl Add<MxPos> for MxPos {
	type Output = MxPos;	
    fn add(self, other: MxPos) -> MxPos {
        add_mx_to_mx(&self, &other)
    }
}

impl Add<&MxPos> for MxPos {
	type Output = MxPos;
    fn add(self, other: &MxPos) -> MxPos {
        add_mx_to_mx(&self, other)
    }
}

impl Add<MxPos> for &MxPos {
	type Output = MxPos;	
    fn add(self, other: MxPos) -> MxPos {
        add_mx_to_mx(self, &other)
    }
}

// impl Sub

fn sub_mx_to_mx(a: &MxPos, b: &MxPos) -> MxPos {
	MxPos::new(
		a.hor - b.hor,
		a.ver - b.ver
	)
}

impl Sub<&MxPos> for &MxPos {
	type Output = MxPos;
    fn sub(self, other: &MxPos) -> MxPos {
        sub_mx_to_mx(self, other)
    }
}

impl Sub<MxPos> for MxPos {
	type Output = MxPos;
	
    fn sub(self, other: MxPos) -> MxPos {
        sub_mx_to_mx(&self, &other)
    }
}

impl Sub<&MxPos> for MxPos {
	type Output = MxPos;
    fn sub(self, other: &MxPos) -> MxPos {
        sub_mx_to_mx(&self, other)
    }
}

impl Sub<MxPos> for &MxPos {
	type Output = MxPos;
	
    fn sub(self, other: MxPos) -> MxPos {
        sub_mx_to_mx(self, &other)
    }
}



// impl Mul traits

fn mul_mxes(a: &MxPos, b: &MxPos) -> MxPos {
	MxPos::new(
		a.hor - b.hor,
		a.ver - b.ver
	)
}

impl Mul<&MxPos> for &MxPos {
	type Output = MxPos;
    fn mul(self, other: &MxPos) -> MxPos {
        mul_mxes(self, other)
    }
}

impl Mul<MxPos> for MxPos {
	type Output = MxPos;
    fn mul(self, other: MxPos) -> MxPos {
        mul_mxes(&self, &other)
    }
}

impl Mul<&MxPos> for MxPos {
	type Output = MxPos;
    fn mul(self, other: &MxPos) -> MxPos {
        mul_mxes(&self, other)
    }
}

impl Mul<MxPos> for &MxPos {
	type Output = MxPos;
    fn mul(self, other: MxPos) -> MxPos {
        mul_mxes(self, &other)
    }
}


// impl Div traits

fn div_mxes(a: &MxPos, b: &MxPos) -> MxPos {
	MxPos::new(
		a.hor / b.hor,
		a.ver / b.ver,
	)
}

impl Div<&MxPos> for &MxPos {
	type Output = MxPos;
    fn div(self, other: &MxPos) -> MxPos {
        div_mxes(self, other)
    }
}

impl Div<MxPos> for MxPos {
	type Output = MxPos;
    fn div(self, other: MxPos) -> MxPos {
        div_mxes(&self, &other)
    }
}

impl Div<&MxPos> for MxPos {
	type Output = MxPos;
    fn div(self, other: &MxPos) -> MxPos {
        div_mxes(&self, other)
    }
}

impl Div<MxPos> for &MxPos {
	type Output = MxPos;
    fn div(self, other: MxPos) -> MxPos {
        div_mxes(self, &other)
    }
}
