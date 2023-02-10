use super::*;

#[derive(Clone)]
pub struct MxPos {
	pub hor: usize,
	pub ver: usize,
}

impl From<TilePos> for MxPos {
	fn from(other: TilePos) -> Self {
		MxPos {
			hor: other.hor.abs() as usize,
			ver: other.ver.abs() as usize,
		}
	}
}

impl From<&TilePos> for MxPos {
	fn from(other: &TilePos) -> Self {
		MxPos {
			hor: other.hor.abs() as usize,
			ver: other.ver.abs() as usize,
		}
	}
}
